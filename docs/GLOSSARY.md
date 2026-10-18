# Glossary

The geometric objects and terms used by openclare, its bench and the research note
[Closing Edge Loops at Quantization Time](research/closing-helices/paper.tex). Each entry
gives the French term in parentheses. Names in `code` are those of the engine's source.

The entries follow the chain: the mesh, the field, the map, the layout, the quantization, the
loops, then the theory of turns and the measures.

## 1. Meshes

**Triangle mesh** (*maillage triangulaire*). The input: a surface made of triangles, such as
a sculpt or a scan.

**Quad mesh** (*maillage de quads*). The output: a surface made only of quadrilaterals.
openclare never produces triangles or n-gons.

**Valence** (*valence*). The number of edges that meet at a vertex.

**Regular vertex** (*sommet régulier*). An inner vertex of valence 4 in a quad mesh, or of
valence 3 on a border.

**Irregular vertex, singular vertex, pole** (*sommet irrégulier, singularité, pôle*). An inner
vertex whose valence is not 4. In practice it has valence 3 or 5, rarely 2 or 6. Edge flow
turns around it. Artists call these vertices poles.

**Index** (*indice*). 4 minus the valence: +1 for valence 3, −1 for valence 5. On a closed
surface the indices add up to 4 × (2 − 2g), where g is the genus (Gauss–Bonnet). On a sphere
they add up to 8, so a quad mesh of a sphere has at least eight poles of valence 3.

**Genus** (*genre*). The number of handles of a closed surface: 0 for a sphere, 1 for a torus.

**Closed surface, border** (*surface fermée, bord*). A surface with no border edge, such as a
ball. Suzanne and jerry have borders. Several theorems of the note hold only on closed
surfaces of genus 0.

**Piece** (*pièce*). A connected part of the input. Pieces are meshed separately and share the
quad budget.

**Work mesh** (*maillage de travail*). The input remeshed into regular triangles before
anything else. All later stages compute on it.

**Base complex** (*complexe de base*). The partition of a quad mesh into the largest
rectangular blocks of quads bounded by the lines leaving the poles. Fewer patches mean a
simpler structure. A cube-like sphere has 6.

**Edge loop** (*boucle d'arêtes*). A line of edges that goes straight through every regular
vertex. It stops at a pole or a border, or comes back to where it started.

**Strip, quad strip** (*bande de quads*). A row of quads joined by their opposite edges: the
quads between two neighbouring edge loops.

**Ring, closed loop** (*anneau, boucle fermée*). An edge loop or strip that comes back to its
start and closes on itself.

**Spiral, helix** (*spirale, hélice*). An edge loop or strip that comes back beside its start,
one or more rows further along, and winds round the part like a thread on a screw. It is the
main defect openclare fights. Bommes et al. call it a *q-helix*.

**Open loop** (*boucle ouverte*). An edge loop that ends at a pole or a border.

**Inverted face, fold** (*face retournée, pli*). A quad that faces the opposite way from the
source surface. A single one makes the mesh invalid.

**Degenerate face** (*face dégénérée*). A face with no area.

## 2. The cross field

**Principal curvature directions** (*directions principales de courbure*). At each point of
a surface, the two directions in which it bends most and least.

**Cross field** (*champ de croix*). At each point, a cross with four arms at right angles that
says which way the edges of the quads should run. It is also called a 4-RoSy field (four-fold
rotational symmetry).

**N-RoSy field** (*champ N-symétrique*). The same idea with N arms. A 6-RoSy field, a star with
six arms, gives triangle and hexagonal meshes (DT-068).

**Power field** (*champ en puissance*). How openclare computes the cross field (Knöppel et al.
2013): the cross is written as the complex number e^{4iθ}, and the smoothest field that also
follows the curvature is solved as a linear system.

**Confidence** (*confiance*). How far the two principal curvatures differ at a point. Where
they are nearly equal, as on a sphere, their directions mean nothing and are not followed (the
confidence floor, `CONFIDENCE_FLOOR`).

**Singularity of the field** (*singularité du champ*). A point round which the cross turns by
a quarter turn or more. It becomes a pole of the mesh.

**Period jump** (*saut de période*). The number of quarter turns by which the cross is
re-labelled across an edge of the work mesh.

**Pair cancellation, singularity reach** (*annulation de paires, portée*). Removing a valence-3
and a valence-5 singularity that are close to each other, to simplify the mesh.
`singularity_reach` sets how far apart they may be.

## 3. The map

**Seamless map, integer-grid map** (*paramétrisation sans couture, à grille entière*). A
flattening of the surface into the plane in which the cross field becomes the horizontal and
vertical directions. Across a cut it changes only by a quarter-turn and a translation.

**Seam** (*couture*). A cut in the surface along which the map is opened flat.

**Cone point** (*point conique*). A singular vertex seen in the flat map: the angle round it
is 270° for valence 3 and 450° for valence 5, instead of 360°.

**Flat metric with cone points** (*métrique plate à singularités coniques*). A quad mesh seen
as a surface made of unit squares: flat everywhere except at its poles.

**Developing map** (*application développante*). The map obtained by unfolding the surface
patch by patch into the plane along a path. Following different paths can bring different
positions back.

**Holonomy** (*holonomie*). What a closed path brings back after it is unfolded: a rotation by
quarter turns and a translation. The translation of a turn is its shift.

## 4. The layout

**Separatrix** (*séparatrice*). A line of the field that leaves a singularity: 3 lines leave a
valence-3 vertex and 5 leave a valence-5 vertex. Separatrices become edge loops that end at
poles.

**Trace** (*trace*). A separatrix followed across the surface until it meets another trace or
a border. It is called a *motorcycle* because it stops where it hits another one, like the
bikes in the film Tron.

**Head-on merge** (*fusion frontale*). Two traces that run into each other from opposite
directions are joined into one, instead of one stopping on the other.

**T-mesh** (*T-mesh, maillage en T*). The layout made by the traces: a partition of the
surface into rectangles whose corners may sit in the middle of another rectangle's side.

**Patch** (*patch*). One rectangle of the T-mesh. It becomes a grid of quads.

**Node** (*nœud*). A point where traces meet or stop: a singular vertex, or a T-junction.

**T-junction** (*jonction en T*). A node where one trace stops on another, so that it is a
corner of some patches and lies in the middle of a side of another.

**Arc** (*arc*). A piece of a trace between two nodes: a side, or a part of a side, of a
patch. Quantization gives each arc a whole length.

**Wall** (*mur*). A border or hard edge that traces may not cross.

**Patch grid meshing** (*maillage par grille de patchs*). Turning every patch, once it has
whole side lengths, into its grid of quads.

## 5. Quantization

**Quantization** (*quantification*). Choosing a whole number of quad edges for every arc of
the T-mesh, so that the patches tile into quads. This is where the integer structure of the
mesh, and with it its loops, is decided.

**Balance** (*équilibre*). For each patch, the two opposite sides must have the same total
length. Each arc appears in exactly two balance rows, with a sign each, which makes the
balance matrix the incidence matrix of a bidirected graph.

**Target lengths, deviation** (*longueurs visées, écart*). The real lengths the field asks
for, and how far the whole lengths chosen are from them. A larger deviation means quads that
are less square.

**Deviation cap** (*plafond d'écart*). The largest deviation accepted when closing loops,
relative to the plain search: `DEVIATION_GROWTH` × plain + `DEVIATION_SLACK`. The simplicity
setting moves it.

**Plain search** (*recherche simple*). The integer search with no loop closing, in the manner
of the minimum-deviation flow of Bi-MDF (Heistermann et al. 2023).

**Projection** (*projection*). The smallest change of the real lengths that closes the
chosen turns, solved by least squares.

**Integer search** (*recherche entière*). The search for whole lengths near the projected
ones that keep the balances.

**Way** (*variante*). One of the eight ways of choosing which turns to close (`CLOSE_WAYS`).
They are all tried, and the one that leaves the fewest spirals is kept.

**Repair walk, aimed repair** (*marche de réparation, réparation visée*). Small changes of the
whole lengths that try to close a remaining helix.

**Cut round** (*tour de coupe*). After a search, every strip of the whole lengths is walked,
the helices the field would have closed are added to the turns to close, and the search is
run again.

**Active set** (*ensemble actif*). In the projection, arcs that would come out with a negative
length are held at zero and the system is solved again.

**Candidate** (*candidat*). An admissible set of whole lengths, ranked by its spirals. When
the chosen one folds at meshing, the next ones are tried before the plain lengths.

**Fold guard** (*garde contre les plis*). The check that meshes other lengths when the chosen
ones fold a quad.

**Simplicity** (*simplicité*). The setting from 0 (most regular quads) to 1 (fewest poles and
loops). The presets sit at 0.25. It drives the singularity reach and the deviation cap.

## 6. Turns: the theory of the research note

**Turn** (*tour*). A straight walk across the rectangles of the T-mesh that comes back to
the side it started from, going the same way. The strip of quads it follows is either a ring
or a helix.

**Shift** (*décalage*). How far along its starting side a turn comes back: 0 for a ring, the
pitch of the helix otherwise. It is a linear form in the arc lengths (Proposition 1).

**Pitch** (*pas*). The number of rows a helix advances per turn: its shift at whole lengths.

**Around, along length** (*longueur du tour*). The length of a turn's walk, its component
along the walk.

**Lean** (*inclinaison*). The ratio of a turn's real shift to its length. A turn that the field
nearly closes (lean ≤ 0.12) is a ring of the field.

**Ring of the field** (*anneau du champ*). A turn the field itself would nearly close: it
should end up a ring.

**Undue helix** (*hélice indue*). A ring of the field that the whole lengths left as a helix:
the rounding made it, not the field.

**Station, half-integer start** (*station, départ à mi-entier*). The midpoints of quad edges
along a patch side (k + ½). Walking from all of them finds every strip of the mesh
(Proposition 2, completeness).

**Lattice walk, complete walk** (*marche complète*). Walking from every station of every
patch side at the whole lengths, which lists all the strips of the final mesh.

**Burgers vector** (*vecteur de Burgers*). Borrowed from the dislocations of crystals: the
translation that a loop round some singular vertices fails to close by. The shift of a turn
is the across component of the Burgers vector of the singular vertices on one side of it.

**Dislocation, 3/5 pair** (*dislocation, paire 3/5*). A valence-3 and a valence-5 vertex close
to each other. Between two parallel turns, such a pair makes their shifts differ by ±d₁±d₂,
where d is the separation of the pair.

**Staircase** (*escalier*). A path of arcs running in two directions a quarter turn apart
between the two vertices of a pair. The difference of the shifts of the two turns round it is
the staircase's length, so both can be rings only if the pair merges.

**Exchange of two vertices** (*échange de deux sommets*). Two singular vertices of the same
valence lying on opposite sides of two crossing turns. It acts on the shifts like a
dislocation.

**Cap** (*calotte*). A disc closed off by a turn, such as a fingertip, holding singular
vertices whose indices add up to 4, typically four of valence 3.

**Winding separatrix** (*séparatrice qui s'enroule*). A separatrix that goes once round a
limb and lands just beside its own vertex. The turn round the limb is then held open by
exactly that gap, as at the wrist of the male body.

**Gauss–Bonnet for turns** (*Gauss–Bonnet pour les tours*). A simple turn bounding a disc
encloses singularities of total index 4. Two disjoint turns bounding an annulus enclose a
total of 0.

**Parity of a turn** (*parité d'un tour*). On a closed sphere, the length plus the shift of a
turn is even at whole lengths. Every ring therefore has an even number of quads, and a helix
of odd pitch can only close if its length changes. It is false on surfaces with borders.

**Checkerboard rule** (*règle du damier*). Colour the whole points of the plane in a
checkerboard. Two parallel turns can both be rings only if an even number of odd-valence
singular vertices between them sit on white points.

**Twisted winding, Fox–Burgers formula** (*enroulement tordu, formule de Fox–Burgers*). Any
combination of shifts is a sum, over the singular vertices, of a two-component winding times
their Burgers term. The windings come from Fox's free differential calculus. Plain winding
numbers do not suffice.

**Sign-definite turn** (*tour de signe fixe*). A turn whose shift keeps one sign whatever the
lengths. It can never close without collapsing an arc, so it must not be held.

**Farkas certificate** (*certificat de Farkas*). A combination of shifts that proves a set of
turns cannot all close: it equals a non-negative sum of lengths containing an arc held at
length 1 or more.

**Irreducible infeasible set** (*ensemble infaisable irréductible*). A set of turns that
cannot all close, although every smaller subset can.

**Implicit zero** (*zéro implicite*). An arc that every real solution of a closing system
sets to zero. Collapsing it shrinks the mesh.

**Lattice of closed quantizations** (*réseau des quantifications fermées*). All whole lengths
that satisfy the balances and close a set of turns: an integer lattice, ker_ℤ.

**Hermite normal form, LLL, Babai** (*forme normale de Hermite, LLL, Babai*). Tools of the
geometry of numbers: an exact basis of a lattice, a basis made of short vectors, and rounding
to a nearby lattice point.

**Forced helices** (*hélices forcées*). The fewest turns that any quantization must leave
shifted, given the singularities and their separation. On the male body there are 1 to 9 per
way, where the engine leaves 13 to 123.

**Chain of dislocations** (*chaîne de dislocations*). Nested parallel turns separated by
dislocations of the same orientation. m + 1 such turns force at least m helices.

**Jenkins–Strebel differential, cylinder twist** (*différentielle de Jenkins–Strebel,
torsion d'un cylindre*). The classical picture of a surface cut into flat cylinders. A helix
is a cylinder whose twist is not a whole number.

## 7. Measures of the bench

**Spiral loop edges** (*arêtes en spirale*). The share of edges that lie on spiral loops.
Lower is better. A single model's value is chaotic, so changes are judged over three budgets.

**Closed loop edges** (*arêtes en boucle fermée*). The share of edges on closed loops. Higher
is better.

**Hausdorff distance** (*distance de Hausdorff*). The largest distance between the result and
the source, in thousandths of the diagonal. It is chaotic for a single model.

**Mean distance** (*distance moyenne*). The average distance between the result and the
source.

**Thin-part distance** (*distance des parties fines*). The distance on the thin parts
(fingers, ears, sheets), in quad edges.

**Aspect** (*rapport d'aspect*). How elongated the quads are: 1 is square.

**Angle deviation** (*écart d'angle*). How far the quads' corners are from 90°.

**Alignment** (*alignement*). How far the edges are from the principal curvature directions.

**Budget error** (*écart au budget*). How far the number of quads made is from the number
asked for.

**Validity** (*validité*). Closed where the source is closed, manifold, with no inverted or
degenerate face.

**QuadSphere** (*QuadSphere*). The academic answer for a sphere: the layout of a cube, with
eight poles of valence 3, six patches and every loop closed.

**Three-budget batch** (*lot sur trois budgets*). The 12 models of the quick comparison, each
remeshed at 2,970, 3,000 and 3,030 quads, so that a change is not judged on one chaotic run.
