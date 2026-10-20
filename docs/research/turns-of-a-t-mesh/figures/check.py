"""Checks the numbers drawn on the TikZ figures of this directory.

Each figure draws a flat surface cut open into one chart: unit squares of the
plane whose border segments are glued in pairs by motions x -> R^k x + t, with
R the quarter-turn (x, y) -> (-y, x). This script builds the same surfaces,
then checks the valence of every vertex, the length and shift of every turn
drawn (by walking the strips of the quad mesh), the Burgers sum of
holonomy.tex, the staircase and the winding separatrix of dislocation.tex and
separatrix.tex, and the comb identity of comb.tex.

    python3 check.py

prints the numbers and stops on the first one that differs from the figures.
Borders left unglued are the edges of the picture (the surface goes on there),
and vertices on them are not checked.
"""

from fractions import Fraction


def rotate(turns, vector):
    x, y = vector
    for _ in range(turns % 4):
        x, y = -y, x
    return (x, y)


def plus(a, b):
    return (a[0] + b[0], a[1] + b[1])


def minus(a, b):
    return (a[0] - b[0], a[1] - b[1])


def times(factor, a):
    return (factor * a[0], factor * a[1])


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1]


def half(n):
    return Fraction(2 * n + 1, 2)


class Motion:
    """x -> R^turns x + translation."""

    def __init__(self, turns, translation):
        self.turns = turns % 4
        self.translation = translation

    def __call__(self, point):
        return plus(rotate(self.turns, point), self.translation)

    def inverse(self):
        turns = (-self.turns) % 4
        return Motion(turns, times(-1, rotate(turns, self.translation)))

    def after(self, other):
        return Motion(self.turns + other.turns,
                      plus(rotate(self.turns, other.translation), self.translation))


def edge(a, b):
    return (a, b) if a <= b else (b, a)


class Surface:
    def __init__(self, cells):
        self.cells = set(cells)
        self.cells_of_edge = {}
        for (i, j) in self.cells:
            for side in [edge((i, j), (i + 1, j)), edge((i, j + 1), (i + 1, j + 1)),
                         edge((i, j), (i, j + 1)), edge((i + 1, j), (i + 1, j + 1))]:
                self.cells_of_edge.setdefault(side, []).append((i, j))
        self.glued = {}  # border unit edge -> (motion, image edge)

    def border(self, side):
        return len(self.cells_of_edge.get(side, [])) == 1

    def glue(self, start, end, image_start, image_end):
        """Glue the segment start -> end onto image_start -> image_end."""
        length = abs(end[0] - start[0]) + abs(end[1] - start[1])
        assert length == abs(image_end[0] - image_start[0]) + abs(image_end[1] - image_start[1])
        step = times(Fraction(1, length), minus(end, start))
        image_step = times(Fraction(1, length), minus(image_end, image_start))
        turns = [k for k in range(4) if rotate(k, step) == image_step][0]
        motion = Motion(turns, minus(image_start, rotate(turns, start)))
        assert motion(start) == image_start and motion(end) == image_end
        for s in range(length):
            a, b = plus(start, times(s, step)), plus(start, times(s + 1, step))
            a, b = tuple(map(int, a)), tuple(map(int, b))
            side, image = edge(a, b), edge(motion(a), motion(b))
            assert self.border(side) and self.border(image), ("not a border", side, image)
            assert side not in self.glued and image not in self.glued, ("glued twice", side)
            self.glued[side] = (motion, image)
            self.glued[image] = (motion.inverse(), side)

    def vertices(self):
        """The vertices of the surface, each as the list of its points in the chart."""
        parent = {}

        def root(point):
            parent.setdefault(point, point)
            while parent[point] != point:
                parent[point] = parent[parent[point]]
                point = parent[point]
            return point

        for side, (motion, _) in self.glued.items():
            for point in side:
                a, b = root(point), root(motion(point))
                if a != b:
                    parent[a] = b
        for (i, j) in self.cells:
            for point in [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)]:
                root(point)
        groups = {}
        for point in list(parent):
            groups.setdefault(root(point), []).append(point)
        return list(groups.values())

    def quads_at(self, point):
        x, y = point
        return sum(cell in self.cells for cell in [(x, y), (x - 1, y), (x, y - 1), (x - 1, y - 1)])

    def inside(self, vertex):
        """False if the vertex touches an unglued border (the edge of the picture)."""
        for point in vertex:
            x, y = point
            for other in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]:
                side = edge(point, other)
                if self.border(side) and side not in self.glued:
                    return False
        return True

    def singular(self):
        """{valence: [vertex, ...]} for the vertices inside of valence other than 4."""
        found = {}
        for vertex in self.vertices():
            if self.inside(vertex):
                valence = sum(self.quads_at(point) for point in vertex)
                if valence != 4:
                    found.setdefault(valence, []).append(sorted(vertex))
        return found

    def turn(self, start, direction=(1, 0)):
        """Walk the strip from the edge midpoint start along direction until it is
        back on the line of start, going the same way: (length, shift), the shift
        counted along the chart (positive: it comes back at a larger coordinate)."""
        midpoint, heading = start, direction
        for length in range(1, 10000):
            centre = plus(midpoint, times(Fraction(1, 2), heading))
            cell = (int(centre[0] - Fraction(1, 2)), int(centre[1] - Fraction(1, 2)))
            assert cell in self.cells, ("the walk left the surface", midpoint, heading)
            midpoint = plus(midpoint, heading)
            across = rotate(1, heading)
            ends = [plus(midpoint, times(Fraction(sign, 2), across)) for sign in (-1, 1)]
            side = edge(*[tuple(map(int, point)) for point in ends])
            if self.border(side):
                assert side in self.glued, ("the walk left the picture", midpoint)
                motion, _ = self.glued[side]
                midpoint, heading = motion(midpoint), rotate(motion.turns, heading)
            if heading == direction and dot(minus(midpoint, start), direction) == 0:
                return length, dot(minus(midpoint, start), rotate(1, direction))
        raise RuntimeError("the walk does not come back")


def burgers(first_visits):
    """Translation of rho_m o ... o rho_1, rho_i(x) = R_i (x - p_i) + p_i with
    R_i = R^(4 - valence), for first_visits = [(p_i, valence_i)] in order."""
    total = Motion(0, (0, 0))
    for point, valence in first_visits:
        turns = 4 - valence
        total = Motion(turns, minus(point, rotate(turns, point))).after(total)
    assert total.turns == 0
    return total.translation


def check_singular(surface, expected):
    found = surface.singular()
    expected = {valence: sorted(sorted(vertex) for vertex in vertices)
                for valence, vertices in expected.items()}
    assert {valence: sorted(vertices) for valence, vertices in found.items()} == expected, found
    print("  singular vertices (valence: points of each in the chart):", found)


def check_turn(surface, start, length, shift):
    assert surface.turn(start) == (length, shift), (start, surface.turn(start))
    print(f"  turn from ({start[0]}, {float(start[1])}): length {length}, shift {shift}")


def holonomy():
    print("holonomy.tex")
    surface = Surface({(i, j) for i in range(11) for j in range(4)}
                      | {(i, 4) for i in range(3, 11)}
                      | {(i, j) for i in range(3) for j in range(4, 6)}
                      | {(i, 6) for i in range(2)})
    surface.glue((0, 0), (0, 4), (11, 1), (11, 5))
    # The top of the block, from c_6 rightwards, onto the border of the top.
    for start, end, image_start, image_end in [
            ((3, 5), (4, 5), (3, 5), (3, 6)), ((4, 5), (5, 5), (3, 6), (2, 6)),
            ((5, 5), (6, 5), (2, 6), (2, 7)), ((6, 5), (8, 5), (2, 7), (0, 7)),
            ((8, 5), (11, 5), (0, 7), (0, 4))]:
        surface.glue(start, end, image_start, image_end)
    check_singular(surface, {3: [[(0, 4), (11, 5)], [(0, 7), (8, 5)], [(2, 7), (6, 5)],
                                 [(3, 6), (4, 5)], [(3, 5)]],
                             5: [[(2, 6), (5, 5)]]})
    for o in (1, 2, 3):
        check_turn(surface, (0, half(o)), 11, -1)
    first_visits = [((11, 5), 3), ((8, 5), 3), ((6, 5), 3), ((5, 5), 5), ((4, 5), 3), ((3, 5), 3)]
    # D lies on the left of the walk: B_D = T_gamma = (-length, shift).
    length, shift = surface.turn((0, half(3)))
    assert burgers(first_visits) == (-length, shift) == (-11, -1)
    print("  B_D =", burgers(first_visits), "from the first visits p_1, ..., p_6")
    other_order = [((0, 4), 3), ((0, 7), 3), ((2, 7), 3), ((2, 6), 5), ((3, 6), 3), ((3, 5), 3)]
    print("  (walking round T the other way gives", burgers(other_order), "instead)")


def straight_chain():
    print("dislocation.tex (a)")
    top = 8
    surface = Surface({(i, j) for i in range(10) for j in range(3)}
                      | {(i, j) for i in range(2, 10) for j in range(3, top)})
    surface.glue((10, 0), (10, 3), (0, 0), (0, 3))
    surface.glue((2, 3), (0, 3), (2, 3), (2, 5))  # quarter-turn about c_3
    surface.glue((10, 3), (10, top - 2), (2, 5), (2, top))
    check_singular(surface, {5: [[(0, 3), (10, 3), (2, 5)]], 3: [[(2, 3)]]})
    check_turn(surface, (0, half(1)), 10, 0)
    check_turn(surface, (2, half(5)), 8, 2)
    # c_3 = (2,3) to c_5 = (0,3) along y = 3: delta = 2 = the shift difference.
    assert abs(minus((0, 3), (2, 3))[0]) == 2 - 0


def staircase():
    print("dislocation.tex (b), separatrix.tex (b), comb.tex")
    top = 8
    surface = Surface({(i, j) for i in range(10) for j in range(3)}
                      | {(i, 3) for i in range(1, 10)}
                      | {(i, j) for i in range(10) for j in range(4, top)})
    surface.glue((10, 0), (10, 3), (0, 0), (0, 3))
    surface.glue((1, 4), (1, 3), (1, 4), (0, 4))  # quarter-turn about c_3
    surface.glue((1, 3), (0, 3), (0, 4), (0, 5))
    surface.glue((10, 3), (10, top - 2), (0, 5), (0, top))
    check_singular(surface, {5: [[(0, 3), (10, 3), (0, 5)]], 3: [[(1, 4)]]})
    check_turn(surface, (0, half(1)), 10, 0)
    check_turn(surface, (0, half(5)), 10, 2)
    # Staircase from c_3 = (1,4): arc b down, arc c left, to c_5 = (0,3).
    separation, epsilon = minus((0, 3), (1, 4)), (-1, -1)
    assert all(dot(epsilon, step) == 1 for step in [(0, -1), (-1, 0)])
    assert dot(epsilon, separation) == 2 - 0
    print("  staircase b, c: epsilon . d =", dot(epsilon, separation), "= s_2 - s_1")
    # The separatrix sigma along y = 5 leaves v at (0,5) and stops on tau at
    # q = (10,5), which is (0,7) on the left copy of tau: delta = 7 - 5.
    motion, _ = surface.glued[edge((10, 4), (10, 5))]
    assert motion((10, 5)) == (0, 7)
    print("  sigma lands on tau at (10,5) = (0,7): delta = 2 = s_2")
    # The comb of a = [(1,5), (10,5)]: feet c_3 = (1,4) and v = (10,3).
    assert dot((1, 0), minus((10, 3), (1, 4))) == 9
    print("  comb of a: <e_a, p_c1 - p_c0> = 9 = l_a")


if __name__ == "__main__":
    holonomy()
    straight_chain()
    staircase()
    print("all figures check")
