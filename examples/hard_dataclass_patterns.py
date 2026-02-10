"""Hard dataclass patterns: nested, frozen, slots, field factories, post_init, inheritance."""

from dataclasses import dataclass, field, fields, asdict, astuple
from typing import List, Dict, Optional, Tuple, Any


@dataclass
class Coordinate:
    """Simple dataclass with default values."""
    x: float = 0.0
    y: float = 0.0

    def distance_to(self, other: "Coordinate") -> float:
        dx = self.x - other.x
        dy = self.y - other.y
        return (dx * dx + dy * dy) ** 0.5

    def midpoint(self, other: "Coordinate") -> "Coordinate":
        return Coordinate(
            x=(self.x + other.x) / 2.0,
            y=(self.y + other.y) / 2.0,
        )


@dataclass(frozen=True)
class ImmutablePoint:
    """Frozen dataclass - all fields are read-only."""
    x: float
    y: float
    label: str = "origin"

    def magnitude(self) -> float:
        return (self.x * self.x + self.y * self.y) ** 0.5

    def scaled_label(self, factor: float) -> str:
        mag = self.magnitude() * factor
        return f"{self.label}@{mag:.2f}"


@dataclass
class Color:
    """Color with clamping in post_init."""
    r: int
    g: int
    b: int
    alpha: float = 1.0
    hex_value: str = field(init=False)

    def __post_init__(self) -> None:
        self.r = max(0, min(255, self.r))
        self.g = max(0, min(255, self.g))
        self.b = max(0, min(255, self.b))
        self.alpha = max(0.0, min(1.0, self.alpha))
        self.hex_value = f"#{self.r:02x}{self.g:02x}{self.b:02x}"

    def brightness(self) -> float:
        return (0.299 * self.r + 0.587 * self.g + 0.114 * self.b) / 255.0

    def blend(self, other: "Color", ratio: float) -> "Color":
        r = int(self.r * (1.0 - ratio) + other.r * ratio)
        g = int(self.g * (1.0 - ratio) + other.g * ratio)
        b = int(self.b * (1.0 - ratio) + other.b * ratio)
        a = self.alpha * (1.0 - ratio) + other.alpha * ratio
        return Color(r, g, b, a)


@dataclass
class BoundingBox:
    """Bounding box with computed fields."""
    min_corner: Coordinate
    max_corner: Coordinate
    tags: List[str] = field(default_factory=list)

    def width(self) -> float:
        return abs(self.max_corner.x - self.min_corner.x)

    def height(self) -> float:
        return abs(self.max_corner.y - self.min_corner.y)

    def area(self) -> float:
        return self.width() * self.height()

    def contains(self, point: Coordinate) -> bool:
        return (
            self.min_corner.x <= point.x <= self.max_corner.x
            and self.min_corner.y <= point.y <= self.max_corner.y
        )

    def center(self) -> Coordinate:
        return self.min_corner.midpoint(self.max_corner)


@dataclass
class Sprite:
    """Complex nested dataclass with multiple field types."""
    name: str
    position: Coordinate
    color: Color
    bounds: BoundingBox
    velocity: Coordinate = field(default_factory=lambda: Coordinate(0.0, 0.0))
    health: int = 100
    visible: bool = True
    tags: List[str] = field(default_factory=list)
    properties: Dict[str, float] = field(default_factory=dict)

    def __post_init__(self) -> None:
        self.health = max(0, min(100, self.health))
        if not self.tags:
            self.tags = [self.name.lower()]

    def update_position(self, dt: float) -> None:
        self.position = Coordinate(
            x=self.position.x + self.velocity.x * dt,
            y=self.position.y + self.velocity.y * dt,
        )

    def is_alive(self) -> bool:
        return self.health > 0 and self.visible

    def damage(self, amount: int) -> int:
        actual = min(amount, self.health)
        self.health -= actual
        if self.health <= 0:
            self.visible = False
        return actual


@dataclass
class GameEntity(Sprite):
    """Inherited dataclass with additional fields."""
    entity_id: int = 0
    team: str = "neutral"
    score: int = 0
    inventory: List[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        super().__post_init__()
        if self.team not in ("red", "blue", "neutral"):
            self.team = "neutral"

    def collect_item(self, item: str) -> None:
        self.inventory.append(item)
        self.score += 10

    def drop_item(self, item: str) -> bool:
        if item in self.inventory:
            self.inventory.remove(item)
            return True
        return False


@dataclass(frozen=True)
class GameConfig:
    """Frozen config dataclass with nested frozen."""
    map_width: int
    map_height: int
    max_entities: int
    spawn_point: ImmutablePoint
    gravity: float = 9.81
    friction: float = 0.1
    team_colors: Tuple[str, ...] = ("red", "blue")

    def is_in_bounds(self, x: float, y: float) -> bool:
        return 0.0 <= x <= float(self.map_width) and 0.0 <= y <= float(self.map_height)

    def effective_gravity(self, mass: float) -> float:
        return self.gravity * mass


@dataclass
class SpriteSheet:
    """Dataclass with complex field factories."""
    name: str
    frame_count: int
    frame_durations: List[float] = field(default_factory=lambda: [0.1])
    current_frame: int = field(init=False, default=0)
    elapsed: float = field(init=False, default=0.0)

    def __post_init__(self) -> None:
        if len(self.frame_durations) < self.frame_count:
            last = self.frame_durations[-1] if self.frame_durations else 0.1
            while len(self.frame_durations) < self.frame_count:
                self.frame_durations.append(last)

    def advance(self, dt: float) -> int:
        self.elapsed += dt
        threshold = self.frame_durations[self.current_frame]
        if self.elapsed >= threshold:
            self.elapsed -= threshold
            self.current_frame = (self.current_frame + 1) % self.frame_count
        return self.current_frame

    def reset(self) -> None:
        self.current_frame = 0
        self.elapsed = 0.0


def create_sprite_grid(rows: int, cols: int, spacing: float) -> List[Sprite]:
    """Create a grid of sprites with computed positions."""
    sprites: List[Sprite] = []
    for r in range(rows):
        for c in range(cols):
            pos = Coordinate(c * spacing, r * spacing)
            color = Color(
                r=(r * 50) % 256,
                g=(c * 50) % 256,
                b=((r + c) * 30) % 256,
            )
            bounds = BoundingBox(
                min_corner=Coordinate(pos.x - 5.0, pos.y - 5.0),
                max_corner=Coordinate(pos.x + 5.0, pos.y + 5.0),
            )
            name = f"sprite_{r}_{c}"
            sprite = Sprite(
                name=name,
                position=pos,
                color=color,
                bounds=bounds,
                tags=[f"row{r}", f"col{c}"],
            )
            sprites.append(sprite)
    return sprites


def find_sprites_in_area(
    sprites: List[Sprite], region: BoundingBox
) -> List[Sprite]:
    """Find all sprites whose position is within a bounding box."""
    return [s for s in sprites if region.contains(s.position)]


# Untyped function 1: test inference on dataclass access patterns
def compute_sprite_stats(sprites):
    total_health = 0
    live_count = 0
    positions = []
    for sprite in sprites:
        total_health += sprite.health
        if sprite.is_alive():
            live_count += 1
        positions.append((sprite.position.x, sprite.position.y))

    avg_health = total_health / len(sprites) if sprites else 0.0
    cx = sum(p[0] for p in positions) / len(positions) if positions else 0.0
    cy = sum(p[1] for p in positions) / len(positions) if positions else 0.0
    return {
        "avg_health": avg_health,
        "live_count": live_count,
        "centroid": (cx, cy),
    }


# Untyped function 2: test inference on dataclass construction
def clone_sprite_with_offset(sprite, dx, dy):
    new_pos = Coordinate(sprite.position.x + dx, sprite.position.y + dy)
    new_bounds = BoundingBox(
        min_corner=Coordinate(
            sprite.bounds.min_corner.x + dx,
            sprite.bounds.min_corner.y + dy,
        ),
        max_corner=Coordinate(
            sprite.bounds.max_corner.x + dx,
            sprite.bounds.max_corner.y + dy,
        ),
    )
    return Sprite(
        name=sprite.name + "_clone",
        position=new_pos,
        color=sprite.color,
        bounds=new_bounds,
        health=sprite.health,
        visible=sprite.visible,
        tags=list(sprite.tags),
    )


def dataclass_to_flat_dict(entity: GameEntity) -> Dict[str, str]:
    """Convert a game entity to a flat string dictionary."""
    result: Dict[str, str] = {}
    result["name"] = entity.name
    result["x"] = f"{entity.position.x:.2f}"
    result["y"] = f"{entity.position.y:.2f}"
    result["health"] = str(entity.health)
    result["team"] = entity.team
    result["score"] = str(entity.score)
    result["color"] = entity.color.hex_value
    result["inventory"] = ",".join(entity.inventory)
    result["tags"] = ",".join(entity.tags)
    return result


def simulate_frame(
    entities: List[GameEntity], dt: float, config: GameConfig
) -> int:
    """Simulate one frame of the game, return number of alive entities."""
    alive = 0
    for entity in entities:
        entity.update_position(dt)
        x, y = entity.position.x, entity.position.y
        if not config.is_in_bounds(x, y):
            entity.damage(5)
        if entity.is_alive():
            alive += 1
    return alive


def main() -> None:
    # Test basic coordinate operations
    c1 = Coordinate(3.0, 4.0)
    c2 = Coordinate(0.0, 0.0)
    dist = c1.distance_to(c2)
    assert abs(dist - 5.0) < 0.01

    mid = c1.midpoint(c2)
    assert abs(mid.x - 1.5) < 0.01

    # Test frozen dataclass
    ip = ImmutablePoint(3.0, 4.0, "test")
    assert abs(ip.magnitude() - 5.0) < 0.01

    # Test Color post_init
    color = Color(300, -10, 128, 0.5)
    assert color.r == 255
    assert color.g == 0
    assert color.hex_value == "#ff0080"

    # Test nested dataclasses
    bbox = BoundingBox(Coordinate(0.0, 0.0), Coordinate(10.0, 10.0))
    assert abs(bbox.area() - 100.0) < 0.01
    assert bbox.contains(Coordinate(5.0, 5.0))
    assert not bbox.contains(Coordinate(15.0, 15.0))

    # Test sprite grid
    sprites = create_sprite_grid(3, 3, 20.0)
    assert len(sprites) == 9

    stats = compute_sprite_stats(sprites)
    assert stats["live_count"] == 9

    # Test game entity inheritance
    entity = GameEntity(
        name="hero",
        position=Coordinate(50.0, 50.0),
        color=Color(255, 0, 0),
        bounds=BoundingBox(Coordinate(45.0, 45.0), Coordinate(55.0, 55.0)),
        entity_id=1,
        team="red",
    )
    entity.collect_item("sword")
    assert entity.score == 10
    assert "sword" in entity.inventory

    # Test config
    config = GameConfig(
        map_width=800,
        map_height=600,
        max_entities=100,
        spawn_point=ImmutablePoint(400.0, 300.0),
    )
    assert config.is_in_bounds(400.0, 300.0)
    assert not config.is_in_bounds(-1.0, 0.0)

    # Test sprite sheet animation
    sheet = SpriteSheet("walk", 4, [0.1, 0.15])
    frame = sheet.advance(0.05)
    assert frame == 0
    frame = sheet.advance(0.06)
    assert frame == 1


if __name__ == "__main__":
    main()
