"""Small graph tests, written out because networkx is not available.

A graph is a dict mapping each vertex to the set of its neighbours.
"""

import itertools


def induced(graph, vertices):
    vertices = set(vertices)
    return {v: graph[v] & vertices for v in vertices}


def complement(graph):
    vertices = set(graph)
    return {v: vertices - graph[v] - {v} for v in graph}


def edges(graph):
    return [(a, b) for a in graph for b in graph[a] if a < b]


def components(graph):
    seen, out = set(), []
    for start in graph:
        if start in seen:
            continue
        stack, part = [start], []
        seen.add(start)
        while stack:
            v = stack.pop()
            part.append(v)
            for w in graph[v]:
                if w not in seen:
                    seen.add(w)
                    stack.append(w)
        out.append(sorted(part))
    return out


def is_bipartite(graph):
    colour = {}
    for start in graph:
        if start in colour:
            continue
        colour[start] = 0
        stack = [start]
        while stack:
            v = stack.pop()
            for w in graph[v]:
                if w not in colour:
                    colour[w] = 1 - colour[v]
                    stack.append(w)
                elif colour[w] == colour[v]:
                    return False
    return True


def is_forest(graph):
    return len(edges(graph)) == len(graph) - len(components(graph))


def is_chordal(graph):
    """Maximum cardinality search, then a check that its reverse is a
    perfect elimination ordering (Tarjan and Yannakakis)."""
    weight = {v: 0 for v in graph}
    order = []
    left = set(graph)
    while left:
        v = max(left, key=lambda u: (weight[u], -u))
        order.append(v)
        left.remove(v)
        for w in graph[v]:
            if w in left:
                weight[w] += 1
    position = {v: k for k, v in enumerate(order)}
    # Each vertex's earlier neighbours must form a clique: it suffices that
    # they are all adjacent to the latest of them.
    for v in order:
        earlier = [w for w in graph[v] if position[w] < position[v]]
        if not earlier:
            continue
        parent = max(earlier, key=lambda w: position[w])
        if any(w != parent and w not in graph[parent] for w in earlier):
            return False
    return True


def is_comparability(graph):
    """Golumbic: a graph is a comparability graph if and only if no
    implication class of its oriented edges holds an edge both ways."""
    index = {}
    for a, b in edges(graph):
        index[(a, b)] = len(index)
        index[(b, a)] = len(index)
    parent = list(range(len(index)))

    def find(k):
        while parent[k] != k:
            parent[k] = parent[parent[k]]
            k = parent[k]
        return k

    def union(i, j):
        i, j = find(i), find(j)
        if i != j:
            parent[i] = j

    for a in graph:
        for b, c in itertools.combinations(graph[a], 2):
            if c not in graph[b]:
                # ab forces ac the same way: both away from a, or both towards.
                union(index[(a, b)], index[(a, c)])
                union(index[(b, a)], index[(c, a)])
    return all(find(index[(a, b)]) != find(index[(b, a)]) for a, b in edges(graph))


def is_interval(graph):
    """Gilmore and Hoffman: chordal and the complement of a comparability
    graph."""
    return is_chordal(graph) and is_comparability(complement(graph))


def odd_hole(graph, budget=2_000_000):
    """An induced cycle of odd length at least five, or None; and whether
    the search was exhaustive within `budget` extensions of induced paths."""
    order = sorted(graph)
    steps = 0
    for start in order:
        # Induced paths start, ..., v with every vertex above start; a cycle
        # closes when v is adjacent to start and the path has at least five
        # vertices.
        stack = [[start, w] for w in graph[start] if w > start]
        while stack:
            path = stack.pop()
            steps += 1
            if steps > budget:
                return None, False
            last = path[-1]
            inner = set(path[1:-1])
            for w in graph[last]:
                if w <= start or w in path:
                    continue
                # Induced: w adjacent to no vertex of the path but `last`,
                # except possibly `start` which closes the cycle.
                touches = graph[w] & set(path[:-1])
                if touches - {start}:
                    continue
                if start in touches:
                    if len(path) + 1 >= 5 and (len(path) + 1) % 2 == 1:
                        return path + [w], True
                    continue
                stack.append(path + [w])
            del inner
    return None, True


def maximal_cliques(graph):
    """Bron and Kerbosch with a pivot."""
    out = []

    def expand(r, p, x):
        if not p and not x:
            out.append(sorted(r))
            return
        pivot = max(p | x, key=lambda u: len(graph[u] & p))
        for v in list(p - graph[pivot]):
            expand(r | {v}, p & graph[v], x & graph[v])
            p = p - {v}
            x = x | {v}

    expand(set(), set(graph), set())
    return out


def classes(graph, budget=2_000_000):
    """The classes of `graph` tested, as a dict of booleans (None: the
    search did not finish)."""
    hole, done_hole = odd_hole(graph, budget)
    antihole, done_antihole = odd_hole(complement(graph), budget)
    perfect = None
    if hole is not None or antihole is not None:
        perfect = False
    elif done_hole and done_antihole:
        perfect = True
    return {
        "forest": is_forest(graph),
        "bipartite": is_bipartite(graph),
        "chordal": is_chordal(graph),
        "comparability": is_comparability(graph),
        "cocomparability": is_comparability(complement(graph)),
        "interval": is_interval(graph),
        "perfect": perfect,
        "odd_hole": hole,
        "odd_antihole": antihole,
    }
