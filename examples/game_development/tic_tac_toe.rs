#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TicTacToe {
    pub board: Vec<Vec<String>>,
    pub current_player: String,
}
impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: Vec::new(),
            current_player: String::new(),
        }
    }
    pub fn make_move(&mut self, row: i32, col: i32) -> bool {
        if !self.is_valid_move(row, col) {
            return false;
        };
        self.board
            .clone()
            .get_mut(&row)
            .unwrap()
            .insert(col, self.current_player.clone());
        self.current_player = if self.current_player.clone() == "X".to_string() {
            "O".to_string()
        } else {
            "X".to_string()
        };
        return true;
    }
    pub fn is_valid_move(&self, row: i32, col: i32) -> bool {
        if row < 0 || row >= 3 || col < 0 || col >= 3 {
            return false;
        };
        return self.board.clone()[row as usize][col as usize] == " ".to_string();
    }
    pub fn check_winner(&self) -> Option<String> {
        for row in self.board.clone() {
            if row[0 as usize] == row[1 as usize]
                && row[1 as usize] == row[2 as usize]
                && row[2 as usize] != " ".to_string()
            {
                return Some(row[0 as usize]);
            };
        }
        for col in 0..3 {
            if self.board.clone()[0 as usize][col as usize]
                == self.board.clone()[1 as usize][col as usize]
                && self.board.clone()[1 as usize][col as usize]
                    == self.board.clone()[2 as usize][col as usize]
                && self.board.clone()[2 as usize][col as usize] != " ".to_string()
            {
                return Some(self.board.clone()[0 as usize][col as usize]);
            };
        }
        if self.board.clone()[0 as usize][0 as usize] == self.board.clone()[1 as usize][1 as usize]
            && self.board.clone()[1 as usize][1 as usize]
                == self.board.clone()[2 as usize][2 as usize]
            && self.board.clone()[2 as usize][2 as usize] != " ".to_string()
        {
            return Some(self.board.clone()[0 as usize][0 as usize]);
        };
        if self.board.clone()[0 as usize][2 as usize] == self.board.clone()[1 as usize][1 as usize]
            && self.board.clone()[1 as usize][1 as usize]
                == self.board.clone()[2 as usize][0 as usize]
            && self.board.clone()[2 as usize][0 as usize] != " ".to_string()
        {
            return Some(self.board.clone()[0 as usize][2 as usize]);
        };
        return None;
    }
    pub fn is_board_full(&self) -> bool {
        for row in self.board.clone() {
            for cell in row {
                if cell == " ".to_string() {
                    return false;
                };
            }
        }
        return true;
    }
    pub fn is_game_over(&self) -> bool {
        return self.check_winner().is_some() || self.is_board_full();
    }
    pub fn get_empty_positions(&self) -> Vec<(i32, i32)> {
        let mut positions = vec![];
        for row in 0..3 {
            for col in 0..3 {
                if self.board.clone()[row as usize][col as usize] == " ".to_string() {
                    positions.push((row, col));
                };
            }
        }
        return positions;
    }
    pub fn board_to_string(&self) -> String {
        let mut lines = vec![];
        for (i, row) in self
            .board
            .clone()
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (i as i32, x))
        {
            let line = format!(
                " {} | {} | {} ",
                row[0 as usize], row[1 as usize], row[2 as usize]
            );
            lines.push(line);
            if i < 2 {
                lines.push("-----------".to_string());
            };
        }
        return lines.join("\n".to_string());
    }
}
#[derive(Debug, Clone)]
pub struct AIPlayer {
    pub symbol: String,
    pub opponent: DepylerValue,
}
impl AIPlayer {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            opponent: Default::default(),
        }
    }
    pub fn get_best_move(&self, game: &TicTacToe) -> (i32, i32) {
        let mut best_score = -1000;
        let mut best_move = (0, 0);
        for (row, col) in game.get_empty_positions() {
            game.board
                .get_mut(&row)
                .unwrap()
                .insert(col, self.symbol.clone());
            let score = self.minimax(game, 0, false);
            game.board
                .get_mut(&row)
                .unwrap()
                .insert(col, " ".to_string());
            if score > best_score {
                let best_score = score;
                let best_move = (row, col);
            };
        }
        return best_move;
    }
    pub fn minimax(&self, game: &TicTacToe, depth: i32, is_maximizing: bool) -> i32 {
        let winner = game.check_winner();
        if winner == self.symbol.clone() {
            return 10 - depth;
        } else {
            if winner == self.opponent.clone() {
                return depth - 10;
            } else {
                if game.is_board_full() {
                    return 0;
                };
            };
        };
        if is_maximizing {
            let mut best_score = -1000;
            for (row, col) in game.get_empty_positions() {
                game.board
                    .get_mut(&row)
                    .unwrap()
                    .insert(col, self.symbol.clone());
                let score = self.minimax(game, depth + 1, false);
                game.board
                    .get_mut(&row)
                    .unwrap()
                    .insert(col, " ".to_string());
                let best_score = (best_score).max(score);
            }
            return best_score;
        } else {
            let mut best_score = 1000;
            for (row, col) in game.get_empty_positions() {
                game.board
                    .get_mut(&row)
                    .unwrap()
                    .insert(col, self.opponent.clone());
                let score = self.minimax(game, depth + 1, true);
                game.board
                    .get_mut(&row)
                    .unwrap()
                    .insert(col, " ".to_string());
                let best_score = (best_score).min(score);
            }
            return best_score;
        };
    }
}
#[doc = "Play a complete game against computer"]
pub fn play_computer_game() -> Result<String, Box<dyn std::error::Error>> {
    let mut game = TicTacToe::new();
    let ai = AIPlayer::new("O".to_string());
    let mut moves_log: Vec<String> = vec![];
    while !game.is_game_over() {
        let mut col;
        let mut row;
        if game.current_player == "X" {
            let empty_positions = game.get_empty_positions();
            if empty_positions {
                (row, col) = empty_positions
                    .get(0usize)
                    .cloned()
                    .expect("IndexError: list index out of range");
                game.make_move(row, col);
                moves_log.push(format!("Human plays at({:?}, {:?})", row, col));
            }
        } else {
            (row, col) = ai.get_best_move(&game);
            game.make_move(row, col);
            moves_log.push(format!("AI plays at({:?}, {:?})", row, col));
        }
    }
    let winner = game.check_winner();
    if winner {
        moves_log.push(format!("Winner: {:?}", winner));
    } else {
        moves_log.push("Game ended in a tie".to_string().to_string());
    }
    Ok(moves_log.join("\n"))
}
#[doc = "Count how many ways a player can win from current state"]
pub fn count_winning_positions<'b, 'a>(
    board_state: &'a Vec<Vec<String>>,
    player: &'b str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count: i32 = Default::default();
    let winning_positions = vec![
        vec![(0, 0), (0, 1), (0, 2)],
        vec![(1, 0), (1, 1), (1, 2)],
        vec![(2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0)],
        vec![(0, 1), (1, 1), (2, 1)],
        vec![(0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 1), (2, 2)],
        vec![(0, 2), (1, 1), (2, 0)],
    ];
    count = 0;
    for positions in winning_positions.iter().cloned() {
        let mut can_win = true;
        for (row, col) in positions.iter().cloned() {
            let cell = board_state
                .get(row as usize)
                .cloned()
                .expect("IndexError: list index out of range")
                .get(col as usize)
                .cloned()
                .expect("IndexError: list index out of range");
            if (cell != " ") && (cell != player) {
                can_win = false;
                break;
            }
        }
        if can_win {
            count = count + 1;
        }
    }
    Ok(count)
}
