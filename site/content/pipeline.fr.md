# Le pipeline de retopologie dans Blender

<p class="subtitle">D'une sculpture ou d'un scan dense à un maillage léger en quads, avec le détail recuit dessus : les étapes, et la page du manuel de Blender pour chaque outil.</p>

Une retopologie reconstruit un maillage : la source garde le détail, le nouveau maillage porte la topologie dont l'animation, les UV et la subdivision ont besoin. Les étapes ci-dessous se suivent ; chaque outil renvoie à sa page dans le [manuel de Blender](https://docs.blender.org/manual/fr/latest/). Les noms de l'interface sont laissés en anglais. Les moteurs que Blender livre pour la voie automatique sont décrits dans [Le remaillage intégré à Blender](blender.fr.md).

## 1. Préparer la source

Une source avec des sommets doublés, des trous ou des pièces qui se croisent trompe tous les outils qui suivent.

- [Merge by Distance](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/mesh/merge.html) et le menu [Clean Up](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/mesh/cleanup.html) : sommets doublés, géométrie isolée, faces dégénérées, trous.
- [Voxel Remesh](https://docs.blender.org/manual/fr/latest/modeling/meshes/retopology.html) : soude les pièces qui se chevauchent en une seule coque fermée.
- [Decimate](https://docs.blender.org/manual/fr/latest/modeling/modifiers/generate/decimate.html) : allège un scan de millions de triangles, pour que la vue et l'accrochage restent rapides.
- [Snap to Symmetry](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/mesh/snap_symmetry.html) : rend exactement symétrique une source qui l'est presque, pour pouvoir n'en faire qu'une moitié.

## 2. Prévoir le flux

Décider où passent les boucles avant de les faire : autour des yeux et de la bouche, le long des membres, aux articulations.

- [Face Sets](https://docs.blender.org/manual/fr/latest/sculpt_paint/sculpting/editing/face_sets.html) : peindre les régions de la source en Sculpt Mode, une par partie.
- [Mark Seam](https://docs.blender.org/manual/fr/latest/modeling/meshes/uv/unwrapping/seams.html) : les coupes des futures UV, quand elles décident où les boucles doivent passer.

## 3. Faire les quads

**En automatique.** [QuadriFlow Remesh](https://docs.blender.org/manual/fr/latest/modeling/meshes/retopology.html) est le seul moteur de Blender dont les quads suivent la forme ; des greffons, dont openclare, en lancent d'autres : voir [les outils](retopology.fr.md). Le résultat est un départ, à corriger à la main à l'étape 4.

**À la main**, par-dessus la source :

- la surcouche [Retopology](https://docs.blender.org/manual/fr/latest/editors/3dview/display/overlays.html), qui montre le nouveau maillage à travers la source ;
- l'[accrochage](https://docs.blender.org/manual/fr/latest/editors/3dview/controls/snapping.html) aux faces (Face Project, Face Nearest), pour que chaque nouveau sommet tombe sur la surface ;
- le modificateur [Mirror](https://docs.blender.org/manual/fr/latest/modeling/modifiers/generate/mirror.html) avec Clipping, pour ne faire qu'une moitié ;
- [Poly Build](https://docs.blender.org/manual/fr/latest/modeling/meshes/tools/poly_build.html), pour poser sommets et quads un à un ;
- [Extrude](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/vertex/extrude_vertices.html), [Loop Cut](https://docs.blender.org/manual/fr/latest/modeling/meshes/tools/loop.html) et le [Knife](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/mesh/knife_topology_tool.html), pour faire pousser et couper des bandes ;
- [Grid Fill](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/face/grid_fill.html) et [Bridge Edge Loops](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/edge/bridge_edge_loops.html), pour remplir un trou ou relier deux boucles en quads ;
- [Triangles to Quads](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/face/triangles_quads.html), pour apparier les triangles restants.

## 4. Ajuster et détendre

- le modificateur [Shrinkwrap](https://docs.blender.org/manual/fr/latest/modeling/modifiers/deform/shrinkwrap.html) : remet chaque sommet sur la source après les retouches ;
- [Smooth Vertices](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/vertex/smooth_vertices.html) : régularise les quads ; Shrinkwrap les ramène sur la surface ;
- [Edge Slide](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/edge/edge_slide.html) : déplace une boucle le long de la surface sans changer sa forme.

## 5. Vérifier

- [Select All by Trait](https://docs.blender.org/manual/fr/latest/modeling/meshes/selecting/all_by_trait.html) : Non Manifold pour les trous et les arêtes doublées, Faces by Sides pour les triangles et n-gones restants ;
- [Select Loops](https://docs.blender.org/manual/fr/latest/modeling/meshes/selecting/loops.html) : un Alt-clic sur une boucle montre où elle va, un tour ou une spirale ;
- [Mesh Analysis](https://docs.blender.org/manual/fr/latest/modeling/meshes/mesh_analysis.html) : distorsion, angles vifs, intersections ;
- le modificateur [Subdivision Surface](https://docs.blender.org/manual/fr/latest/modeling/modifiers/generate/subdivision_surface.html) : le maillage lissé, et les pôles qui le pincent.

## 6. Déplier

- [Mark Seam](https://docs.blender.org/manual/fr/latest/modeling/meshes/uv/unwrapping/seams.html) puis [Unwrap](https://docs.blender.org/manual/fr/latest/modeling/meshes/editing/uv.html) : les UV du nouveau maillage, sur lesquelles le détail est recuit.

## 7. Reporter le détail

- [Bake](https://docs.blender.org/manual/fr/latest/render/cycles/baking.html) dans Cycles, Selected to Active : le détail de la source dans une normal map du nouveau maillage ;
- le modificateur [Multiresolution](https://docs.blender.org/manual/fr/latest/modeling/modifiers/generate/multiresolution.html) : subdiviser le nouveau maillage et le Reshape sur la source, pour continuer à sculpter ;
- le modificateur [Data Transfer](https://docs.blender.org/manual/fr/latest/modeling/modifiers/modify/data_transfer.html) : reporte couleurs de sommets, poids ou UV de la source sur le nouveau maillage.

Un fait est faux ou un lien cassé : ouvrez une [issue](https://github.com/marcgardent/openclare/issues).
