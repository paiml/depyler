# Hard graph algorithm patterns for Python-to-Rust transpiler stress testing
# All functions are pure: no imports, no I/O, no side effects
# Graphs represented as adjacency lists: dict[int, list[int]]


# =============================================================================
# 1. BFS with distance tracking
# =============================================================================


def bfs_distances(graph: dict[int, list[int]], start: int) -> dict[int, int]:
    """BFS from start node, returns distances to all reachable nodes."""
    dist: dict[int, int] = {start: 0}
    queue: list[int] = [start]
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head = head + 1
        if node in graph:
            neighbors: list[int] = graph[node]
            i: int = 0
            while i < len(neighbors):
                nb: int = neighbors[i]
                if nb not in dist:
                    dist[nb] = dist[node] + 1
                    queue.append(nb)
                i = i + 1
    return dist


def bfs_shortest_path_length(
    graph: dict[int, list[int]], start: int, end: int
) -> int:
    """Return shortest path length from start to end, or -1 if unreachable."""
    if start == end:
        return 0
    dist: dict[int, int] = bfs_distances(graph, start)
    if end in dist:
        return dist[end]
    return -1


def bfs_level_sizes(graph: dict[int, list[int]], start: int) -> list[int]:
    """Return the number of nodes at each BFS level from start."""
    dist: dict[int, int] = bfs_distances(graph, start)
    if len(dist) == 0:
        return []
    max_dist: int = 0
    for node in dist:
        d: int = dist[node]
        if d > max_dist:
            max_dist = d
    sizes: list[int] = []
    level: int = 0
    while level <= max_dist:
        sizes.append(0)
        level = level + 1
    for node in dist:
        d2: int = dist[node]
        sizes[d2] = sizes[d2] + 1
    return sizes


# =============================================================================
# 2. DFS with discovery/finish times
# =============================================================================


def dfs_times(
    graph: dict[int, list[int]], start: int
) -> dict[int, list[int]]:
    """Iterative DFS returning {node: [discovery_time, finish_time]}."""
    times: dict[int, list[int]] = {}
    clock: list[int] = [0]
    stack: list[list[int]] = [[start, 0]]
    visited: dict[int, int] = {}
    while len(stack) > 0:
        top: list[int] = stack[len(stack) - 1]
        node: int = top[0]
        idx: int = top[1]
        if node not in visited:
            visited[node] = 1
            clock[0] = clock[0] + 1
            times[node] = [clock[0], 0]
        neighbors: list[int] = []
        if node in graph:
            neighbors = graph[node]
        if idx < len(neighbors):
            top[1] = idx + 1
            nb: int = neighbors[idx]
            if nb not in visited:
                stack.append([nb, 0])
        else:
            clock[0] = clock[0] + 1
            times[node][1] = clock[0]
            stack.pop()
    return times


def dfs_reachable_count(graph: dict[int, list[int]], start: int) -> int:
    """Count nodes reachable from start using DFS."""
    times: dict[int, list[int]] = dfs_times(graph, start)
    return len(times)


# =============================================================================
# 3. Connected components
# =============================================================================


def all_nodes(graph: dict[int, list[int]]) -> list[int]:
    """Extract all unique nodes from a graph adjacency list."""
    node_set: dict[int, int] = {}
    for node in graph:
        node_set[node] = 1
        neighbors: list[int] = graph[node]
        j: int = 0
        while j < len(neighbors):
            node_set[neighbors[j]] = 1
            j = j + 1
    result: list[int] = []
    for n in node_set:
        result.append(n)
    return result


def make_undirected(graph: dict[int, list[int]]) -> dict[int, list[int]]:
    """Ensure graph has edges in both directions without duplicates."""
    ug: dict[int, list[int]] = {}
    edge_set: dict[int, dict[int, int]] = {}
    for node in graph:
        if node not in ug:
            ug[node] = []
        if node not in edge_set:
            edge_set[node] = {}
        neighbors: list[int] = graph[node]
        k: int = 0
        while k < len(neighbors):
            nb: int = neighbors[k]
            if nb not in ug:
                ug[nb] = []
            if nb not in edge_set:
                edge_set[nb] = {}
            if nb not in edge_set[node]:
                edge_set[node][nb] = 1
                ug[node].append(nb)
            if node not in edge_set[nb]:
                edge_set[nb][node] = 1
                ug[nb].append(node)
            k = k + 1
    return ug


def connected_components_count(graph: dict[int, list[int]]) -> int:
    """Count connected components in an undirected graph."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    visited: dict[int, int] = {}
    count: int = 0
    idx: int = 0
    while idx < len(nodes):
        node: int = nodes[idx]
        if node not in visited:
            count = count + 1
            queue: list[int] = [node]
            visited[node] = 1
            qh: int = 0
            while qh < len(queue):
                cur: int = queue[qh]
                qh = qh + 1
                if cur in ug:
                    nbs: list[int] = ug[cur]
                    ni: int = 0
                    while ni < len(nbs):
                        nb2: int = nbs[ni]
                        if nb2 not in visited:
                            visited[nb2] = 1
                            queue.append(nb2)
                        ni = ni + 1
        idx = idx + 1
    return count


def largest_component_size(graph: dict[int, list[int]]) -> int:
    """Return size of the largest connected component."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    visited: dict[int, int] = {}
    max_size: int = 0
    idx: int = 0
    while idx < len(nodes):
        node: int = nodes[idx]
        if node not in visited:
            size: int = 0
            queue: list[int] = [node]
            visited[node] = 1
            qh: int = 0
            while qh < len(queue):
                cur: int = queue[qh]
                qh = qh + 1
                size = size + 1
                if cur in ug:
                    nbs: list[int] = ug[cur]
                    ni: int = 0
                    while ni < len(nbs):
                        nb2: int = nbs[ni]
                        if nb2 not in visited:
                            visited[nb2] = 1
                            queue.append(nb2)
                        ni = ni + 1
            if size > max_size:
                max_size = size
        idx = idx + 1
    return max_size


# =============================================================================
# 4. Cycle detection
# =============================================================================


def has_cycle_directed(graph: dict[int, list[int]]) -> int:
    """Detect cycle in directed graph. Returns 1 if cycle exists, 0 otherwise."""
    white: int = 0
    gray: int = 1
    black: int = 2
    color: dict[int, int] = {}
    nodes: list[int] = all_nodes(graph)
    ci: int = 0
    while ci < len(nodes):
        color[nodes[ci]] = white
        ci = ci + 1
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        if color[node] == white:
            stack: list[list[int]] = [[node, 0]]
            color[node] = gray
            while len(stack) > 0:
                top: list[int] = stack[len(stack) - 1]
                cur: int = top[0]
                idx: int = top[1]
                neighbors: list[int] = []
                if cur in graph:
                    neighbors = graph[cur]
                if idx < len(neighbors):
                    top[1] = idx + 1
                    nb: int = neighbors[idx]
                    if nb in color and color[nb] == gray:
                        return 1
                    if nb not in color or color[nb] == white:
                        if nb not in color:
                            color[nb] = white
                        color[nb] = gray
                        stack.append([nb, 0])
                else:
                    color[cur] = black
                    stack.pop()
        ni = ni + 1
    return 0


def has_cycle_undirected(graph: dict[int, list[int]]) -> int:
    """Detect cycle in undirected graph using union-find. Returns 1/0."""
    parent: dict[int, int] = {}
    nodes: list[int] = all_nodes(graph)
    pi: int = 0
    while pi < len(nodes):
        parent[nodes[pi]] = nodes[pi]
        pi = pi + 1

    def find_root(x: int) -> int:
        r: int = x
        while parent[r] != r:
            r = parent[r]
        cur2: int = x
        while cur2 != r:
            nxt: int = parent[cur2]
            parent[cur2] = r
            cur2 = nxt
        return r

    for u in graph:
        neighbors: list[int] = graph[u]
        ei: int = 0
        while ei < len(neighbors):
            v: int = neighbors[ei]
            if u < v:
                ru: int = find_root(u)
                rv: int = find_root(v)
                if ru == rv:
                    return 1
                parent[ru] = rv
            ei = ei + 1
    return 0


# =============================================================================
# 5. Shortest path (Dijkstra-like with weighted edges as dict[int, list[list[int]]])
# =============================================================================


def dijkstra_distances(
    weighted_graph: dict[int, list[list[int]]], start: int
) -> dict[int, int]:
    """Dijkstra using sorted list as priority queue. Edges: [[neighbor, weight], ...]."""
    dist: dict[int, int] = {start: 0}
    pq: list[list[int]] = [[0, start]]
    visited: dict[int, int] = {}
    while len(pq) > 0:
        best_idx: int = 0
        bi: int = 1
        while bi < len(pq):
            if pq[bi][0] < pq[best_idx][0]:
                best_idx = bi
            bi = bi + 1
        entry: list[int] = pq[best_idx]
        pq[best_idx] = pq[len(pq) - 1]
        pq.pop()
        d: int = entry[0]
        u: int = entry[1]
        if u in visited:
            continue
        visited[u] = 1
        if u in weighted_graph:
            edges: list[list[int]] = weighted_graph[u]
            ei: int = 0
            while ei < len(edges):
                v: int = edges[ei][0]
                w: int = edges[ei][1]
                nd: int = d + w
                if v not in dist or nd < dist[v]:
                    dist[v] = nd
                    pq.append([nd, v])
                ei = ei + 1
    return dist


def dijkstra_shortest(
    weighted_graph: dict[int, list[list[int]]], start: int, end: int
) -> int:
    """Return shortest weighted distance from start to end, or -1."""
    dist: dict[int, int] = dijkstra_distances(weighted_graph, start)
    if end in dist:
        return dist[end]
    return -1


# =============================================================================
# 6. Bipartite checking
# =============================================================================


def is_bipartite(graph: dict[int, list[int]]) -> int:
    """Check if undirected graph is bipartite. Returns 1 if yes, 0 if no."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    color: dict[int, int] = {}
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        if node not in color:
            color[node] = 0
            queue: list[int] = [node]
            qh: int = 0
            while qh < len(queue):
                cur: int = queue[qh]
                qh = qh + 1
                if cur in ug:
                    nbs: list[int] = ug[cur]
                    nbi: int = 0
                    while nbi < len(nbs):
                        nb: int = nbs[nbi]
                        if nb not in color:
                            color[nb] = 1 - color[cur]
                            queue.append(nb)
                        elif color[nb] == color[cur]:
                            return 0
                        nbi = nbi + 1
        ni = ni + 1
    return 1


# =============================================================================
# 7. Bridge detection (Tarjan-like iterative)
# =============================================================================


def count_bridges(graph: dict[int, list[int]]) -> int:
    """Count bridges in an undirected graph using iterative Tarjan."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    disc: dict[int, int] = {}
    low: dict[int, int] = {}
    parent: dict[int, int] = {}
    timer: list[int] = [0]
    bridge_count: int = 0
    ni: int = 0
    while ni < len(nodes):
        root: int = nodes[ni]
        if root not in disc:
            stack: list[list[int]] = [[root, 0]]
            parent[root] = -1
            while len(stack) > 0:
                top: list[int] = stack[len(stack) - 1]
                u: int = top[0]
                idx: int = top[1]
                if idx == 0:
                    timer[0] = timer[0] + 1
                    disc[u] = timer[0]
                    low[u] = timer[0]
                nbs: list[int] = []
                if u in ug:
                    nbs = ug[u]
                if idx < len(nbs):
                    top[1] = idx + 1
                    v: int = nbs[idx]
                    if v not in disc:
                        parent[v] = u
                        stack.append([v, 0])
                    elif v != parent[u]:
                        if disc[v] < low[u]:
                            low[u] = disc[v]
                else:
                    stack.pop()
                    if len(stack) > 0:
                        pu: int = stack[len(stack) - 1][0]
                        if low[u] < low[pu]:
                            low[pu] = low[u]
                        if low[u] > disc[pu]:
                            bridge_count = bridge_count + 1
        ni = ni + 1
    return bridge_count


def count_articulation_points(graph: dict[int, list[int]]) -> int:
    """Count articulation points (cut vertices) in an undirected graph."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    disc: dict[int, int] = {}
    low: dict[int, int] = {}
    parent: dict[int, int] = {}
    is_ap: dict[int, int] = {}
    timer: list[int] = [0]
    ni: int = 0
    while ni < len(nodes):
        root: int = nodes[ni]
        if root not in disc:
            stack: list[list[int]] = [[root, 0]]
            parent[root] = -1
            child_count: dict[int, int] = {root: 0}
            while len(stack) > 0:
                top: list[int] = stack[len(stack) - 1]
                u: int = top[0]
                idx: int = top[1]
                if idx == 0:
                    timer[0] = timer[0] + 1
                    disc[u] = timer[0]
                    low[u] = timer[0]
                nbs: list[int] = []
                if u in ug:
                    nbs = ug[u]
                if idx < len(nbs):
                    top[1] = idx + 1
                    v: int = nbs[idx]
                    if v not in disc:
                        parent[v] = u
                        if u not in child_count:
                            child_count[u] = 0
                        child_count[u] = child_count[u] + 1
                        stack.append([v, 0])
                    elif v != parent[u]:
                        if disc[v] < low[u]:
                            low[u] = disc[v]
                else:
                    stack.pop()
                    if len(stack) > 0:
                        pu: int = stack[len(stack) - 1][0]
                        if low[u] < low[pu]:
                            low[pu] = low[u]
                        if parent[pu] == -1:
                            cc: int = 0
                            if pu in child_count:
                                cc = child_count[pu]
                            if cc > 1:
                                is_ap[pu] = 1
                        else:
                            if low[u] >= disc[pu]:
                                is_ap[pu] = 1
        ni = ni + 1
    return len(is_ap)


# =============================================================================
# 8. Strongly connected components (Kosaraju's)
# =============================================================================


def transpose_graph(graph: dict[int, list[int]]) -> dict[int, list[int]]:
    """Return the transpose (reverse edges) of a directed graph."""
    tg: dict[int, list[int]] = {}
    nodes: list[int] = all_nodes(graph)
    gi: int = 0
    while gi < len(nodes):
        tg[nodes[gi]] = []
        gi = gi + 1
    for u in graph:
        neighbors: list[int] = graph[u]
        ei: int = 0
        while ei < len(neighbors):
            v: int = neighbors[ei]
            if v not in tg:
                tg[v] = []
            tg[v].append(u)
            ei = ei + 1
    return tg


def kosaraju_scc_count(graph: dict[int, list[int]]) -> int:
    """Count strongly connected components using Kosaraju's algorithm."""
    nodes: list[int] = all_nodes(graph)
    visited: dict[int, int] = {}
    finish_order: list[int] = []
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        if node not in visited:
            stack: list[list[int]] = [[node, 0]]
            visited[node] = 1
            while len(stack) > 0:
                top: list[int] = stack[len(stack) - 1]
                u: int = top[0]
                idx: int = top[1]
                nbs: list[int] = []
                if u in graph:
                    nbs = graph[u]
                if idx < len(nbs):
                    top[1] = idx + 1
                    v: int = nbs[idx]
                    if v not in visited:
                        visited[v] = 1
                        stack.append([v, 0])
                else:
                    finish_order.append(u)
                    stack.pop()
        ni = ni + 1
    tg: dict[int, list[int]] = transpose_graph(graph)
    visited2: dict[int, int] = {}
    scc_count: int = 0
    fi: int = len(finish_order) - 1
    while fi >= 0:
        node2: int = finish_order[fi]
        if node2 not in visited2:
            scc_count = scc_count + 1
            queue: list[int] = [node2]
            visited2[node2] = 1
            qh: int = 0
            while qh < len(queue):
                cur: int = queue[qh]
                qh = qh + 1
                if cur in tg:
                    tnbs: list[int] = tg[cur]
                    ti: int = 0
                    while ti < len(tnbs):
                        tv: int = tnbs[ti]
                        if tv not in visited2:
                            visited2[tv] = 1
                            queue.append(tv)
                        ti = ti + 1
        fi = fi - 1
    return scc_count


def largest_scc_size(graph: dict[int, list[int]]) -> int:
    """Return size of largest SCC via Kosaraju's."""
    nodes: list[int] = all_nodes(graph)
    visited: dict[int, int] = {}
    finish_order: list[int] = []
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        if node not in visited:
            stack: list[list[int]] = [[node, 0]]
            visited[node] = 1
            while len(stack) > 0:
                top: list[int] = stack[len(stack) - 1]
                u: int = top[0]
                idx: int = top[1]
                nbs: list[int] = []
                if u in graph:
                    nbs = graph[u]
                if idx < len(nbs):
                    top[1] = idx + 1
                    v: int = nbs[idx]
                    if v not in visited:
                        visited[v] = 1
                        stack.append([v, 0])
                else:
                    finish_order.append(u)
                    stack.pop()
        ni = ni + 1
    tg: dict[int, list[int]] = transpose_graph(graph)
    visited2: dict[int, int] = {}
    max_size: int = 0
    fi: int = len(finish_order) - 1
    while fi >= 0:
        node2: int = finish_order[fi]
        if node2 not in visited2:
            size: int = 0
            queue: list[int] = [node2]
            visited2[node2] = 1
            qh: int = 0
            while qh < len(queue):
                cur: int = queue[qh]
                qh = qh + 1
                size = size + 1
                if cur in tg:
                    tnbs: list[int] = tg[cur]
                    ti: int = 0
                    while ti < len(tnbs):
                        tv: int = tnbs[ti]
                        if tv not in visited2:
                            visited2[tv] = 1
                            queue.append(tv)
                        ti = ti + 1
            if size > max_size:
                max_size = size
        fi = fi - 1
    return max_size


# =============================================================================
# 9. Graph coloring (greedy)
# =============================================================================


def greedy_coloring(graph: dict[int, list[int]]) -> dict[int, int]:
    """Greedy graph coloring. Returns {node: color} with colors starting at 0."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    coloring: dict[int, int] = {}
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        used_colors: dict[int, int] = {}
        if node in ug:
            nbs: list[int] = ug[node]
            nbi: int = 0
            while nbi < len(nbs):
                nb: int = nbs[nbi]
                if nb in coloring:
                    used_colors[coloring[nb]] = 1
                nbi = nbi + 1
        c: int = 0
        while c in used_colors:
            c = c + 1
        coloring[node] = c
        ni = ni + 1
    return coloring


def chromatic_number_upper(graph: dict[int, list[int]]) -> int:
    """Upper bound on chromatic number via greedy coloring."""
    coloring: dict[int, int] = greedy_coloring(graph)
    max_color: int = -1
    for node in coloring:
        if coloring[node] > max_color:
            max_color = coloring[node]
    return max_color + 1


# =============================================================================
# 10. Minimum spanning tree (Kruskal's with union-find)
# =============================================================================


def kruskal_mst_weight(
    num_nodes: int, edges: list[list[int]]
) -> int:
    """Kruskal's MST. edges = [[u, v, weight], ...]. Returns total MST weight."""
    sorted_edges: list[list[int]] = []
    ei: int = 0
    while ei < len(edges):
        sorted_edges.append([edges[ei][0], edges[ei][1], edges[ei][2]])
        ei = ei + 1
    si: int = 0
    while si < len(sorted_edges):
        sj: int = si + 1
        while sj < len(sorted_edges):
            if sorted_edges[sj][2] < sorted_edges[si][2]:
                tmp: list[int] = sorted_edges[si]
                sorted_edges[si] = sorted_edges[sj]
                sorted_edges[sj] = tmp
            sj = sj + 1
        si = si + 1
    parent: list[int] = []
    rank: list[int] = []
    pi: int = 0
    while pi < num_nodes:
        parent.append(pi)
        rank.append(0)
        pi = pi + 1

    def find_uf(x: int) -> int:
        r: int = x
        while parent[r] != r:
            r = parent[r]
        cur2: int = x
        while cur2 != r:
            nxt: int = parent[cur2]
            parent[cur2] = r
            cur2 = nxt
        return r

    def union_uf(a: int, b: int) -> int:
        ra: int = find_uf(a)
        rb: int = find_uf(b)
        if ra == rb:
            return 0
        if rank[ra] < rank[rb]:
            parent[ra] = rb
        elif rank[ra] > rank[rb]:
            parent[rb] = ra
        else:
            parent[rb] = ra
            rank[ra] = rank[ra] + 1
        return 1

    total_weight: int = 0
    edge_count: int = 0
    ki: int = 0
    while ki < len(sorted_edges) and edge_count < num_nodes - 1:
        e: list[int] = sorted_edges[ki]
        if union_uf(e[0], e[1]) == 1:
            total_weight = total_weight + e[2]
            edge_count = edge_count + 1
        ki = ki + 1
    return total_weight


def prim_mst_weight(
    weighted_graph: dict[int, list[list[int]]], start: int
) -> int:
    """Prim's MST using simple min-extraction. Returns total MST weight."""
    visited: dict[int, int] = {start: 1}
    pq: list[list[int]] = []
    if start in weighted_graph:
        edges: list[list[int]] = weighted_graph[start]
        ei: int = 0
        while ei < len(edges):
            pq.append([edges[ei][1], edges[ei][0]])
            ei = ei + 1
    total: int = 0
    while len(pq) > 0:
        best_idx: int = 0
        bi: int = 1
        while bi < len(pq):
            if pq[bi][0] < pq[best_idx][0]:
                best_idx = bi
            bi = bi + 1
        entry: list[int] = pq[best_idx]
        pq[best_idx] = pq[len(pq) - 1]
        pq.pop()
        w: int = entry[0]
        u: int = entry[1]
        if u in visited:
            continue
        visited[u] = 1
        total = total + w
        if u in weighted_graph:
            ue: list[list[int]] = weighted_graph[u]
            uei: int = 0
            while uei < len(ue):
                v: int = ue[uei][0]
                vw: int = ue[uei][1]
                if v not in visited:
                    pq.append([vw, v])
                uei = uei + 1
    return total


# =============================================================================
# 11. Floyd-Warshall (all-pairs shortest paths)
# =============================================================================


def floyd_warshall(
    num_nodes: int, edges: list[list[int]]
) -> list[list[int]]:
    """Floyd-Warshall. edges=[[u,v,w],...]. Returns 2D distance matrix (999999=inf)."""
    inf: int = 999999
    dist: list[list[int]] = []
    i: int = 0
    while i < num_nodes:
        row: list[int] = []
        j: int = 0
        while j < num_nodes:
            if i == j:
                row.append(0)
            else:
                row.append(inf)
            j = j + 1
        dist.append(row)
        i = i + 1
    ei: int = 0
    while ei < len(edges):
        u: int = edges[ei][0]
        v: int = edges[ei][1]
        w: int = edges[ei][2]
        if w < dist[u][v]:
            dist[u][v] = w
        ei = ei + 1
    k: int = 0
    while k < num_nodes:
        ii: int = 0
        while ii < num_nodes:
            jj: int = 0
            while jj < num_nodes:
                through_k: int = dist[ii][k] + dist[k][jj]
                if through_k < dist[ii][jj]:
                    dist[ii][jj] = through_k
                jj = jj + 1
            ii = ii + 1
        k = k + 1
    return dist


def floyd_shortest(
    num_nodes: int, edges: list[list[int]], start: int, end: int
) -> int:
    """Return shortest path from start to end using Floyd-Warshall, or -1."""
    dist: list[list[int]] = floyd_warshall(num_nodes, edges)
    result: int = dist[start][end]
    if result >= 999999:
        return -1
    return result


# =============================================================================
# 12. Degree sequence analysis
# =============================================================================


def degree_sequence(graph: dict[int, list[int]]) -> list[int]:
    """Return sorted (descending) degree sequence of an undirected graph."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    degrees: list[int] = []
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        deg: int = 0
        if node in ug:
            deg = len(ug[node])
        degrees.append(deg)
        ni = ni + 1
    si: int = 0
    while si < len(degrees):
        sj: int = si + 1
        while sj < len(degrees):
            if degrees[sj] > degrees[si]:
                tmp: int = degrees[si]
                degrees[si] = degrees[sj]
                degrees[sj] = tmp
            sj = sj + 1
        si = si + 1
    return degrees


def max_degree(graph: dict[int, list[int]]) -> int:
    """Return maximum degree in the graph."""
    seq: list[int] = degree_sequence(graph)
    if len(seq) == 0:
        return 0
    return seq[0]


def is_regular(graph: dict[int, list[int]]) -> int:
    """Check if graph is regular (all nodes same degree). Returns 1/0."""
    seq: list[int] = degree_sequence(graph)
    if len(seq) <= 1:
        return 1
    i: int = 1
    while i < len(seq):
        if seq[i] != seq[0]:
            return 0
        i = i + 1
    return 1


def sum_degrees(graph: dict[int, list[int]]) -> int:
    """Sum of all degrees (should be 2 * num_edges for undirected)."""
    seq: list[int] = degree_sequence(graph)
    total: int = 0
    i: int = 0
    while i < len(seq):
        total = total + seq[i]
        i = i + 1
    return total


# =============================================================================
# 13. Eulerian path/circuit detection
# =============================================================================


def is_eulerian_circuit(graph: dict[int, list[int]]) -> int:
    """Check if undirected graph has Eulerian circuit (all degrees even, connected)."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    if len(nodes) == 0:
        return 1
    has_edge: int = 0
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        if node in ug and len(ug[node]) > 0:
            has_edge = 1
            if len(ug[node]) % 2 != 0:
                return 0
        ni = ni + 1
    if has_edge == 0:
        return 1
    if connected_components_count(graph) > 1:
        nodes_with_edges: int = 0
        comp_nodes: list[int] = all_nodes(ug)
        ci: int = 0
        while ci < len(comp_nodes):
            nd: int = comp_nodes[ci]
            if nd in ug and len(ug[nd]) > 0:
                nodes_with_edges = nodes_with_edges + 1
            ci = ci + 1
        visited_e: dict[int, int] = {}
        first_e: int = -1
        fi: int = 0
        while fi < len(comp_nodes):
            if comp_nodes[fi] in ug and len(ug[comp_nodes[fi]]) > 0:
                first_e = comp_nodes[fi]
                fi = len(comp_nodes)
            fi = fi + 1
        if first_e == -1:
            return 1
        queue: list[int] = [first_e]
        visited_e[first_e] = 1
        qh: int = 0
        while qh < len(queue):
            cur: int = queue[qh]
            qh = qh + 1
            if cur in ug:
                nbs: list[int] = ug[cur]
                nbi: int = 0
                while nbi < len(nbs):
                    nb: int = nbs[nbi]
                    if nb not in visited_e:
                        visited_e[nb] = 1
                        queue.append(nb)
                    nbi = nbi + 1
        reachable_with_edges: int = 0
        for rn in visited_e:
            if rn in ug and len(ug[rn]) > 0:
                reachable_with_edges = reachable_with_edges + 1
        if reachable_with_edges < nodes_with_edges:
            return 0
    return 1


def count_odd_degree_nodes(graph: dict[int, list[int]]) -> int:
    """Count nodes with odd degree in undirected graph."""
    ug: dict[int, list[int]] = make_undirected(graph)
    nodes: list[int] = all_nodes(ug)
    count: int = 0
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        if node in ug and len(ug[node]) % 2 != 0:
            count = count + 1
        ni = ni + 1
    return count


def has_eulerian_path(graph: dict[int, list[int]]) -> int:
    """Check if undirected graph has Eulerian path. Returns 1/0."""
    odd: int = count_odd_degree_nodes(graph)
    if odd == 0:
        return is_eulerian_circuit(graph)
    if odd == 2:
        if connected_components_count(graph) <= 1:
            return 1
        ug: dict[int, list[int]] = make_undirected(graph)
        nodes: list[int] = all_nodes(ug)
        edge_nodes: int = 0
        ni: int = 0
        while ni < len(nodes):
            nd: int = nodes[ni]
            if nd in ug and len(ug[nd]) > 0:
                edge_nodes = edge_nodes + 1
            ni = ni + 1
        if edge_nodes <= 2:
            return 1
        first_with_edge: int = -1
        fi: int = 0
        while fi < len(nodes):
            if nodes[fi] in ug and len(ug[nodes[fi]]) > 0:
                first_with_edge = nodes[fi]
                fi = len(nodes)
            fi = fi + 1
        if first_with_edge == -1:
            return 1
        visited: dict[int, int] = {}
        queue: list[int] = [first_with_edge]
        visited[first_with_edge] = 1
        qh: int = 0
        while qh < len(queue):
            cur: int = queue[qh]
            qh = qh + 1
            if cur in ug:
                nbs: list[int] = ug[cur]
                nbi: int = 0
                while nbi < len(nbs):
                    nb: int = nbs[nbi]
                    if nb not in visited:
                        visited[nb] = 1
                        queue.append(nb)
                    nbi = nbi + 1
        reachable_edges: int = 0
        for rn in visited:
            if rn in ug and len(ug[rn]) > 0:
                reachable_edges = reachable_edges + 1
        if reachable_edges >= edge_nodes:
            return 1
    return 0


# =============================================================================
# 14. DAG longest path
# =============================================================================


def topological_sort(graph: dict[int, list[int]]) -> list[int]:
    """Kahn's algorithm for topological sort. Returns empty list if cycle."""
    nodes: list[int] = all_nodes(graph)
    in_degree: dict[int, int] = {}
    ni: int = 0
    while ni < len(nodes):
        in_degree[nodes[ni]] = 0
        ni = ni + 1
    for u in graph:
        nbs: list[int] = graph[u]
        ei: int = 0
        while ei < len(nbs):
            v: int = nbs[ei]
            if v in in_degree:
                in_degree[v] = in_degree[v] + 1
            else:
                in_degree[v] = 1
            ei = ei + 1
    queue: list[int] = []
    for n in in_degree:
        if in_degree[n] == 0:
            queue.append(n)
    result: list[int] = []
    qh: int = 0
    while qh < len(queue):
        u2: int = queue[qh]
        qh = qh + 1
        result.append(u2)
        if u2 in graph:
            nbs2: list[int] = graph[u2]
            ei2: int = 0
            while ei2 < len(nbs2):
                v2: int = nbs2[ei2]
                in_degree[v2] = in_degree[v2] - 1
                if in_degree[v2] == 0:
                    queue.append(v2)
                ei2 = ei2 + 1
    if len(result) != len(nodes):
        return []
    return result


def dag_longest_path(graph: dict[int, list[int]]) -> int:
    """Return longest path length in a DAG. -1 if graph has cycle."""
    topo: list[int] = topological_sort(graph)
    if len(topo) == 0 and len(all_nodes(graph)) > 0:
        return -1
    dist: dict[int, int] = {}
    ti: int = 0
    while ti < len(topo):
        dist[topo[ti]] = 0
        ti = ti + 1
    ti2: int = 0
    while ti2 < len(topo):
        u: int = topo[ti2]
        if u in graph:
            nbs: list[int] = graph[u]
            ei: int = 0
            while ei < len(nbs):
                v: int = nbs[ei]
                nd: int = dist[u] + 1
                if nd > dist[v]:
                    dist[v] = nd
                ei = ei + 1
        ti2 = ti2 + 1
    max_d: int = 0
    for n in dist:
        if dist[n] > max_d:
            max_d = dist[n]
    return max_d


def dag_longest_weighted_path(
    weighted_graph: dict[int, list[list[int]]]
) -> int:
    """Longest weighted path in DAG. Edges: [[neighbor, weight], ...]."""
    unweighted: dict[int, list[int]] = {}
    for u in weighted_graph:
        unweighted[u] = []
        edges: list[list[int]] = weighted_graph[u]
        ei: int = 0
        while ei < len(edges):
            unweighted[u].append(edges[ei][0])
            ei = ei + 1
    topo: list[int] = topological_sort(unweighted)
    if len(topo) == 0 and len(all_nodes(unweighted)) > 0:
        return -1
    dist: dict[int, int] = {}
    ti: int = 0
    while ti < len(topo):
        dist[topo[ti]] = 0
        ti = ti + 1
    ti2: int = 0
    while ti2 < len(topo):
        u2: int = topo[ti2]
        if u2 in weighted_graph:
            edges2: list[list[int]] = weighted_graph[u2]
            ei2: int = 0
            while ei2 < len(edges2):
                v: int = edges2[ei2][0]
                w: int = edges2[ei2][1]
                nd: int = dist[u2] + w
                if nd > dist[v]:
                    dist[v] = nd
                ei2 = ei2 + 1
        ti2 = ti2 + 1
    max_d: int = 0
    for n in dist:
        if dist[n] > max_d:
            max_d = dist[n]
    return max_d


# =============================================================================
# Extra utilities: in-degree, edge count, density
# =============================================================================


def in_degree_map(graph: dict[int, list[int]]) -> dict[int, int]:
    """Compute in-degree for each node in a directed graph."""
    nodes: list[int] = all_nodes(graph)
    indeg: dict[int, int] = {}
    ni: int = 0
    while ni < len(nodes):
        indeg[nodes[ni]] = 0
        ni = ni + 1
    for u in graph:
        nbs: list[int] = graph[u]
        ei: int = 0
        while ei < len(nbs):
            v: int = nbs[ei]
            if v in indeg:
                indeg[v] = indeg[v] + 1
            else:
                indeg[v] = 1
            ei = ei + 1
    return indeg


def count_edges_directed(graph: dict[int, list[int]]) -> int:
    """Count total edges in a directed graph."""
    total: int = 0
    for u in graph:
        total = total + len(graph[u])
    return total


def count_source_nodes(graph: dict[int, list[int]]) -> int:
    """Count nodes with in-degree 0 in directed graph."""
    indeg: dict[int, int] = in_degree_map(graph)
    count: int = 0
    for n in indeg:
        if indeg[n] == 0:
            count = count + 1
    return count


def count_sink_nodes(graph: dict[int, list[int]]) -> int:
    """Count nodes with out-degree 0 in directed graph."""
    nodes: list[int] = all_nodes(graph)
    count: int = 0
    ni: int = 0
    while ni < len(nodes):
        node: int = nodes[ni]
        if node not in graph or len(graph[node]) == 0:
            count = count + 1
        ni = ni + 1
    return count


def graph_density_x1000(graph: dict[int, list[int]]) -> int:
    """Return density * 1000 (integer) for directed graph. density = E / (V*(V-1))."""
    nodes: list[int] = all_nodes(graph)
    v: int = len(nodes)
    if v <= 1:
        return 0
    e: int = count_edges_directed(graph)
    return (e * 1000) // (v * (v - 1))


# =============================================================================
# TEST FUNCTIONS — each returns an int for verification
# =============================================================================


def test_bfs_distances() -> int:
    """Test BFS distance computation."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [3], 2: [3], 3: []}
    d: dict[int, int] = bfs_distances(g, 0)
    ok: int = 1
    if d[0] != 0:
        ok = 0
    if d[1] != 1:
        ok = 0
    if d[2] != 1:
        ok = 0
    if d[3] != 2:
        ok = 0
    return ok


def test_bfs_shortest_path() -> int:
    """Test BFS shortest path length."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [3], 2: [3], 3: [4], 4: []}
    r1: int = bfs_shortest_path_length(g, 0, 4)
    r2: int = bfs_shortest_path_length(g, 0, 99)
    if r1 != 3:
        return 0
    if r2 != -1:
        return 0
    return 1


def test_bfs_level_sizes() -> int:
    """Test BFS level sizes."""
    g: dict[int, list[int]] = {0: [1, 2, 3], 1: [4], 2: [4], 3: [], 4: []}
    sizes: list[int] = bfs_level_sizes(g, 0)
    if len(sizes) != 3:
        return 0
    if sizes[0] != 1:
        return 0
    if sizes[1] != 3:
        return 0
    if sizes[2] != 1:
        return 0
    return 1


def test_dfs_times() -> int:
    """Test DFS discovery and finish times."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [3], 2: [], 3: []}
    times: dict[int, list[int]] = dfs_times(g, 0)
    if 0 not in times:
        return 0
    if 1 not in times:
        return 0
    if times[0][0] >= times[0][1]:
        return 0
    if times[0][0] != 1:
        return 0
    return 1


def test_dfs_reachable() -> int:
    """Test DFS reachable count."""
    g: dict[int, list[int]] = {0: [1], 1: [2], 2: [3], 3: [], 5: [6], 6: []}
    r1: int = dfs_reachable_count(g, 0)
    r2: int = dfs_reachable_count(g, 5)
    if r1 != 4:
        return 0
    if r2 != 2:
        return 0
    return 1


def test_connected_components() -> int:
    """Test connected component counting."""
    g: dict[int, list[int]] = {0: [1], 1: [0], 2: [3], 3: [2], 4: []}
    c: int = connected_components_count(g)
    if c != 3:
        return 0
    return 1


def test_largest_component() -> int:
    """Test largest component size."""
    g: dict[int, list[int]] = {0: [1], 1: [2], 2: [0], 3: [4], 4: [3], 5: []}
    s: int = largest_component_size(g)
    if s != 3:
        return 0
    return 1


def test_cycle_directed() -> int:
    """Test directed cycle detection."""
    acyclic: dict[int, list[int]] = {0: [1], 1: [2], 2: [3], 3: []}
    cyclic: dict[int, list[int]] = {0: [1], 1: [2], 2: [0]}
    r1: int = has_cycle_directed(acyclic)
    r2: int = has_cycle_directed(cyclic)
    if r1 != 0:
        return 0
    if r2 != 1:
        return 0
    return 1


def test_cycle_undirected() -> int:
    """Test undirected cycle detection."""
    tree: dict[int, list[int]] = {0: [1], 1: [2]}
    cyclic: dict[int, list[int]] = {0: [1, 2], 1: [2]}
    r1: int = has_cycle_undirected(tree)
    r2: int = has_cycle_undirected(cyclic)
    if r1 != 0:
        return 0
    if r2 != 1:
        return 0
    return 1


def test_dijkstra() -> int:
    """Test Dijkstra shortest path."""
    g: dict[int, list[list[int]]] = {
        0: [[1, 4], [2, 1]],
        1: [[3, 1]],
        2: [[1, 2], [3, 5]],
        3: [],
    }
    r1: int = dijkstra_shortest(g, 0, 3)
    r2: int = dijkstra_shortest(g, 0, 99)
    if r1 != 4:
        return 0
    if r2 != -1:
        return 0
    return 1


def test_bipartite() -> int:
    """Test bipartite checking."""
    bp: dict[int, list[int]] = {0: [1, 3], 1: [0, 2], 2: [1, 3], 3: [0, 2]}
    not_bp: dict[int, list[int]] = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
    r1: int = is_bipartite(bp)
    r2: int = is_bipartite(not_bp)
    if r1 != 1:
        return 0
    if r2 != 0:
        return 0
    return 1


def test_bridges() -> int:
    """Test bridge counting."""
    g: dict[int, list[int]] = {0: [1], 1: [0, 2], 2: [1, 3, 4], 3: [2, 4], 4: [2, 3]}
    b: int = count_bridges(g)
    if b != 2:
        return 0
    return 1


def test_articulation_points() -> int:
    """Test articulation point counting."""
    g: dict[int, list[int]] = {
        0: [1],
        1: [0, 2, 3],
        2: [1],
        3: [1, 4],
        4: [3],
    }
    ap: int = count_articulation_points(g)
    if ap != 2:
        return 0
    return 1


def test_transpose() -> int:
    """Test graph transpose."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [2], 2: []}
    tg: dict[int, list[int]] = transpose_graph(g)
    if 0 not in tg or len(tg[0]) != 0:
        return 0
    if 1 not in tg or len(tg[1]) != 1:
        return 0
    if 2 not in tg or len(tg[2]) != 2:
        return 0
    return 1


def test_kosaraju_scc() -> int:
    """Test Kosaraju SCC count."""
    g: dict[int, list[int]] = {
        0: [1],
        1: [2],
        2: [0, 3],
        3: [4],
        4: [5],
        5: [3],
    }
    c: int = kosaraju_scc_count(g)
    if c != 2:
        return 0
    return 1


def test_largest_scc() -> int:
    """Test largest SCC size."""
    g: dict[int, list[int]] = {
        0: [1],
        1: [2],
        2: [0],
        3: [4],
        4: [5],
        5: [3],
        6: [],
    }
    s: int = largest_scc_size(g)
    if s != 3:
        return 0
    return 1


def test_greedy_coloring() -> int:
    """Test greedy graph coloring."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
    c: dict[int, int] = greedy_coloring(g)
    if len(c) != 3:
        return 0
    for u in g:
        nbs: list[int] = g[u]
        ni: int = 0
        while ni < len(nbs):
            v: int = nbs[ni]
            if c[u] == c[v]:
                return 0
            ni = ni + 1
    return 1


def test_chromatic_upper() -> int:
    """Test chromatic number upper bound."""
    g: dict[int, list[int]] = {0: [1], 1: [0, 2], 2: [1]}
    cn: int = chromatic_number_upper(g)
    if cn != 2:
        return 0
    return 1


def test_kruskal() -> int:
    """Test Kruskal's MST."""
    edges: list[list[int]] = [[0, 1, 4], [0, 2, 1], [1, 2, 2], [1, 3, 5], [2, 3, 8]]
    w: int = kruskal_mst_weight(4, edges)
    if w != 8:
        return 0
    return 1


def test_prim() -> int:
    """Test Prim's MST."""
    g: dict[int, list[list[int]]] = {
        0: [[1, 4], [2, 1]],
        1: [[0, 4], [2, 2], [3, 5]],
        2: [[0, 1], [1, 2], [3, 8]],
        3: [[1, 5], [2, 8]],
    }
    w: int = prim_mst_weight(g, 0)
    if w != 8:
        return 0
    return 1


def test_floyd_warshall() -> int:
    """Test Floyd-Warshall all-pairs shortest paths."""
    edges: list[list[int]] = [[0, 1, 3], [0, 2, 8], [1, 2, 2], [2, 3, 1]]
    r: int = floyd_shortest(4, edges, 0, 3)
    if r != 6:
        return 0
    r2: int = floyd_shortest(4, edges, 3, 0)
    if r2 != -1:
        return 0
    return 1


def test_degree_sequence() -> int:
    """Test degree sequence computation."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
    seq: list[int] = degree_sequence(g)
    if len(seq) != 3:
        return 0
    total: int = 0
    i: int = 0
    while i < len(seq):
        total = total + seq[i]
        i = i + 1
    if total != 6:
        return 0
    return 1


def test_max_degree() -> int:
    """Test max degree computation."""
    g: dict[int, list[int]] = {0: [1, 2, 3], 1: [0], 2: [0], 3: [0]}
    md: int = max_degree(g)
    if md != 3:
        return 0
    return 1


def test_is_regular() -> int:
    """Test regularity check."""
    reg: dict[int, list[int]] = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
    not_reg: dict[int, list[int]] = {0: [1, 2, 3], 1: [0], 2: [0], 3: [0]}
    r1: int = is_regular(reg)
    r2: int = is_regular(not_reg)
    if r1 != 1:
        return 0
    if r2 != 0:
        return 0
    return 1


def test_sum_degrees() -> int:
    """Test sum of degrees (handshake lemma)."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [2]}
    sd: int = sum_degrees(g)
    if sd != 6:
        return 0
    return 1


def test_eulerian_circuit() -> int:
    """Test Eulerian circuit detection."""
    euler: dict[int, list[int]] = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
    not_euler: dict[int, list[int]] = {0: [1], 1: [0, 2], 2: [1]}
    r1: int = is_eulerian_circuit(euler)
    r2: int = is_eulerian_circuit(not_euler)
    if r1 != 1:
        return 0
    if r2 != 0:
        return 0
    return 1


def test_odd_degree() -> int:
    """Test odd degree node counting."""
    g: dict[int, list[int]] = {0: [1], 1: [0, 2], 2: [1]}
    odd: int = count_odd_degree_nodes(g)
    if odd != 2:
        return 0
    return 1


def test_eulerian_path() -> int:
    """Test Eulerian path detection."""
    path_g: dict[int, list[int]] = {0: [1], 1: [0, 2], 2: [1]}
    no_path: dict[int, list[int]] = {0: [1], 1: [0], 2: [3], 3: [2, 4], 4: [3]}
    r1: int = has_eulerian_path(path_g)
    r2: int = has_eulerian_path(no_path)
    if r1 != 1:
        return 0
    if r2 != 0:
        return 0
    return 1


def test_topological_sort() -> int:
    """Test topological sort."""
    dag: dict[int, list[int]] = {0: [1, 2], 1: [3], 2: [3], 3: []}
    topo: list[int] = topological_sort(dag)
    if len(topo) != 4:
        return 0
    pos: dict[int, int] = {}
    ti: int = 0
    while ti < len(topo):
        pos[topo[ti]] = ti
        ti = ti + 1
    if pos[0] >= pos[1]:
        return 0
    if pos[0] >= pos[2]:
        return 0
    if pos[1] >= pos[3]:
        return 0
    return 1


def test_dag_longest_path() -> int:
    """Test DAG longest path."""
    dag: dict[int, list[int]] = {0: [1, 2], 1: [3], 2: [3], 3: [4], 4: []}
    lp: int = dag_longest_path(dag)
    if lp != 3:
        return 0
    return 1


def test_dag_longest_weighted() -> int:
    """Test DAG longest weighted path."""
    wg: dict[int, list[list[int]]] = {
        0: [[1, 5], [2, 3]],
        1: [[3, 6]],
        2: [[3, 4]],
        3: [],
    }
    lp: int = dag_longest_weighted_path(wg)
    if lp != 11:
        return 0
    return 1


def test_in_degree() -> int:
    """Test in-degree computation."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [2], 2: [3], 3: []}
    indeg: dict[int, int] = in_degree_map(g)
    if indeg[0] != 0:
        return 0
    if indeg[1] != 1:
        return 0
    if indeg[2] != 2:
        return 0
    if indeg[3] != 1:
        return 0
    return 1


def test_edge_count() -> int:
    """Test directed edge counting."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [2, 3], 2: [], 3: []}
    e: int = count_edges_directed(g)
    if e != 4:
        return 0
    return 1


def test_source_sink() -> int:
    """Test source and sink node counting."""
    g: dict[int, list[int]] = {0: [1, 2], 1: [3], 2: [3], 3: []}
    src: int = count_source_nodes(g)
    snk: int = count_sink_nodes(g)
    if src != 1:
        return 0
    if snk != 1:
        return 0
    return 1


def test_density() -> int:
    """Test graph density computation."""
    g: dict[int, list[int]] = {0: [1, 2, 3], 1: [0, 2, 3], 2: [0, 1, 3], 3: [0, 1, 2]}
    d: int = graph_density_x1000(g)
    if d != 1000:
        return 0
    return 1


def test_empty_graph() -> int:
    """Test various functions on empty graph."""
    g: dict[int, list[int]] = {}
    c: int = connected_components_count(g)
    if c != 0:
        return 0
    lc: int = largest_component_size(g)
    if lc != 0:
        return 0
    cyc: int = has_cycle_directed(g)
    if cyc != 0:
        return 0
    return 1


def test_single_node() -> int:
    """Test functions on single-node graph."""
    g: dict[int, list[int]] = {0: []}
    d: dict[int, int] = bfs_distances(g, 0)
    if len(d) != 1:
        return 0
    if d[0] != 0:
        return 0
    c: int = connected_components_count(g)
    if c != 1:
        return 0
    return 1


def test_self_loop_cycle() -> int:
    """Test cycle detection with self-loop."""
    g: dict[int, list[int]] = {0: [0]}
    r: int = has_cycle_directed(g)
    if r != 1:
        return 0
    return 1


def test_disconnected_scc() -> int:
    """Test SCC on disconnected directed graph."""
    g: dict[int, list[int]] = {0: [1], 1: [0], 2: [3], 3: [2], 4: []}
    c: int = kosaraju_scc_count(g)
    if c != 3:
        return 0
    return 1


def test_complete_bipartite() -> int:
    """Test bipartite on K2,3."""
    g: dict[int, list[int]] = {
        0: [2, 3, 4],
        1: [2, 3, 4],
        2: [0, 1],
        3: [0, 1],
        4: [0, 1],
    }
    r: int = is_bipartite(g)
    if r != 1:
        return 0
    return 1


def test_long_chain_bfs() -> int:
    """Test BFS on a long chain graph 0->1->2->...->9."""
    g: dict[int, list[int]] = {}
    i: int = 0
    while i < 10:
        if i < 9:
            g[i] = [i + 1]
        else:
            g[i] = []
        i = i + 1
    r: int = bfs_shortest_path_length(g, 0, 9)
    if r != 9:
        return 0
    return 1


def test_topo_cycle_detection() -> int:
    """Topological sort returns empty on cyclic graph."""
    g: dict[int, list[int]] = {0: [1], 1: [2], 2: [0]}
    topo: list[int] = topological_sort(g)
    if len(topo) != 0:
        return 0
    return 1


# =============================================================================
# run_all_tests
# =============================================================================


def run_all_tests() -> int:
    """Run all tests and return sum of results."""
    total: int = 0
    total = total + test_bfs_distances()
    total = total + test_bfs_shortest_path()
    total = total + test_bfs_level_sizes()
    total = total + test_dfs_times()
    total = total + test_dfs_reachable()
    total = total + test_connected_components()
    total = total + test_largest_component()
    total = total + test_cycle_directed()
    total = total + test_cycle_undirected()
    total = total + test_dijkstra()
    total = total + test_bipartite()
    total = total + test_bridges()
    total = total + test_articulation_points()
    total = total + test_transpose()
    total = total + test_kosaraju_scc()
    total = total + test_largest_scc()
    total = total + test_greedy_coloring()
    total = total + test_chromatic_upper()
    total = total + test_kruskal()
    total = total + test_prim()
    total = total + test_floyd_warshall()
    total = total + test_degree_sequence()
    total = total + test_max_degree()
    total = total + test_is_regular()
    total = total + test_sum_degrees()
    total = total + test_eulerian_circuit()
    total = total + test_odd_degree()
    total = total + test_eulerian_path()
    total = total + test_topological_sort()
    total = total + test_dag_longest_path()
    total = total + test_dag_longest_weighted()
    total = total + test_in_degree()
    total = total + test_edge_count()
    total = total + test_source_sink()
    total = total + test_density()
    total = total + test_empty_graph()
    total = total + test_single_node()
    total = total + test_self_loop_cycle()
    total = total + test_disconnected_scc()
    total = total + test_complete_bipartite()
    total = total + test_long_chain_bfs()
    total = total + test_topo_cycle_detection()
    return total


if __name__ == "__main__":
    result: int = run_all_tests()
    expected: int = 42
    if result == expected:
        pass
    else:
        raise ValueError(
            "FAIL: expected " + str(expected) + " but got " + str(result)
        )
