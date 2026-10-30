# Retopology tools for Blender, 2026

One file per tool that makes or helps make the topology of a mesh in Blender, checked against its distribution page:
automatic remeshers, semi-automatic and semi-manual tools, manual retopology tools, and helpers.
Each tool carries tags from [taxonomy.toml](taxonomy.toml): as many as fit, so a tool can be both automatic and semi-automatic.
`python3 scripts/retopology.py` checks every file against the schema below and rewrites the indexes.
The facts are what each publisher shows on the date of `checked`; nothing here is a measure of quality.
Marks on the bench exist only for the tools that `bench` names: see [the models](../models/README.md).

## Tools

<!-- tools:start -->
| Tool | Mode | Output | Engine | Price | Where | Blender | Status |
|---|---|---|---|---|---|---|---|
| [AutoRemesher](tools/autoremesher.toml) | automatic | quad-dominant | own | free | GitHub | — | active |
| [AutoRemesher Bridge](tools/autoremesher-bridge.toml) | automatic | quad-dominant | autoremesher | free | GitHub | 4.2–5.2 | active |
| [Autoremesher NOW! \| Automatic Remeshing](tools/autoremesher-now.toml) | automatic | quad-dominant, triangles | geogram | paid | Superhive (formerly Blender Market) | 5.1 | active |
| [AutoRemesher Quad: One-Click Quad Remesh](tools/autoremesher-quad.toml) | automatic | quad-dominant | autoremesher | paid | Superhive (formerly Blender Market) | 3.3–5.2 | active |
| [Blender CWF Remesher](tools/cwf-remesher.toml) | automatic | triangles | cwf | free | GitHub | 3.6+ | abandoned |
| [Blender Instant Mesh Bridge](tools/instant-mesh-bridge.toml) | automatic | quads, quad-dominant, triangles | instant-meshes | free | GitHub | 2.90–4.1 | stale |
| [Blender's retopology aids: Retopology overlay, face snapping, Shrinkwrap](tools/blender-retopology-aids.toml) | helper | — | — | free | Blender | — | active |
| [Boundary Aligned Remesh [Bookyakuno Edit]](tools/boundary-aligned-remesh.toml) | automatic | quads, triangles | own | paid, pay-what-you-want | Superhive (formerly Blender Market), Gumroad, GitHub | 2.81–5.2 | active |
| [Bsurfaces GPL Edition](tools/bsurfaces.toml) | semi-manual | quad-dominant | own | free | Blender Extensions | 4.3+ | stale |
| [Draw Xray](tools/draw-xray.toml) | manual, helper | — | — | freemium | Gumroad | 4.2+ | stale |
| [DynRemesh/AutoFlow - Remeshing Toolset](tools/dynremesh.toml) | automatic, semi-automatic | quads | quadriflow | paid | Superhive (formerly Blender Market), Gumroad | 2.80–2.91 | stale |
| [Easy Patch 2 - The Smart Retopology Patching Tool](tools/easy-patch.toml) | semi-manual | quads | own | paid | Superhive (formerly Blender Market) | 3.3–5.2 | active |
| [F2](tools/f2.toml) | manual | — | — | free | Blender Extensions | 4.2+ | stale |
| [Fast Remesher](tools/fast-remesher.toml) | automatic | quad-dominant | instant-meshes, quadriflow, voxel | paid | Superhive (formerly Blender Market), Publisher's own site | 4.2–5.2 | active |
| [Geometry Nodes: Mesh to Volume, Volume to Mesh](tools/geometry-nodes-remesh.toml) | automatic | quads, triangles, n-gons | voxel | free | Blender | 3.3+ | active |
| [HeadWrap: Automatic Head Retopology With Eyes and Teeth](tools/headwrap.toml) | automatic, semi-automatic | template | own, ml | paid | Superhive (formerly Blender Market) | 4.2–5.2 | active |
| [Instant Meshes](tools/instant-meshes.toml) | automatic | quads, quad-dominant, triangles | own | free | GitHub | — | stale |
| [Instant Meshes Remesh](tools/instant-meshes-remesh.toml) | automatic | quads, quad-dominant, triangles | instant-meshes | free | GitHub | — | stale |
| [JRemesh Tools](tools/jremesh-tools.toml) | automatic | quads, quad-dominant | instant-meshes, quadriflow | paid | Superhive (formerly Blender Market) | 2.8–5.2 | active |
| [JRetopo Tools](tools/jretopo-tools.toml) | manual, semi-manual, helper | quads, triangles | — | paid | Superhive (formerly Blender Market) | 5.2 | active |
| [Livemesher - Arrange your parts before sculpting](tools/livemesher.toml) | automatic | quads | voxel | paid | Superhive (formerly Blender Market) | 3.0–5.2 | unknown |
| [MMGpy - Mesh Remeshing](tools/mmgpy.toml) | automatic, semi-automatic | triangles | mmg | free | Blender Extensions | 4.2+ | active |
| [Model Optimizer Pro: Quad Remesh & Auto Retopology for Blender](tools/model-optimizer-pro.toml) | automatic | quad-dominant | autoremesher | paid | Superhive (formerly Blender Market) | 3.2–5.2 | active |
| [Mutamesh \| 4 Remeshers in One \| Instant Meshes, Quadwild, CWF, AdapTropic](tools/mutamesh.toml) | automatic | quads, quad-dominant, triangles | instant-meshes, quadwild, cwf | paid | Superhive (formerly Blender Market) | 5.1 | active |
| [Nitro Retopo](tools/nitro-retopo.toml) | semi-manual | quad-dominant | own | paid | Superhive (formerly Blender Market) | 5.2+ | active |
| [OkTopo Remesher for Blender](tools/oktopo.toml) | semi-automatic | template | own | paid | Superhive (formerly Blender Market), Gumroad | — | active |
| [openclare](tools/openclare.toml) | automatic, semi-automatic | quads | own | free | GitHub | 4.2+ | active |
| [Petik - Retopology Patch Maker](tools/petik.toml) | semi-manual | quads | own | paid | Superhive (formerly Blender Market), Gumroad | 3.6–5.0 | active |
| [Poly Build](tools/poly-build.toml) | manual | — | — | free | Blender | 2.80+ | active |
| [PolyQuilt Retopology Tool](tools/polyquilt.toml) | manual | — | — | free | Blender Extensions, GitHub | 4.5+ | active |
| [QRemeshify](tools/qremeshify.toml) | automatic, semi-automatic | quads | quadwild | free, pay-what-you-want, paid | GitHub, Gumroad, Superhive (formerly Blender Market) | 4.2+ | stale |
| [Quad Remesher](tools/quadremesher.toml) | automatic, semi-automatic | quads | proprietary | paid | Publisher's own site | 2.80+ | active |
| [QuadAnneal Remeshing — QuadRemesh & Retopology for Blender](tools/quadanneal-remeshing.toml) | automatic | quads | instant-meshes | paid | Superhive (formerly Blender Market) | 4.2–5.2 | active |
| [Quadify](tools/quadify.toml) | automatic | quads, quad-dominant | own, quadwild, quadriflow, voxel, ml | freemium | Publisher's own site, Superhive (formerly Blender Market), Gumroad | — | active |
| [QuadMend - Quad Remesher for AI & Scanned Models](tools/quadmend.toml) | automatic | quads | instant-meshes, quadriflow | paid | Superhive (formerly Blender Market) | 4.2–5.2 | active |
| [QuadriFlow Remesh](tools/quadriflow.toml) | automatic | quads | quadriflow | free | Blender | 2.81+ | active |
| [QuadSketch – Interactive Retopology Add-on for Blender](tools/quadsketch.toml) | manual, semi-manual | quads | own | paid | Superhive (formerly Blender Market) | 4.2–5.2 | active |
| [Quadwrap - Retopology Tool](tools/quadwrap.toml) | semi-manual | quads | own | paid | Superhive (formerly Blender Market), Gumroad | 5.0–5.1 | active |
| [Relief Rebake](tools/relief-rebake.toml) | automatic | quads | own | free | Blender Extensions, GitHub | 4.2+ | active |
| [Remesh Modifier](tools/remesh-modifier.toml) | automatic | quads | octree, voxel | free | Blender | 2.80+ | active |
| [Remi](tools/remi.toml) | automatic | quads, triangles | voxel, instant-meshes, autoremesher | freemium | GitHub | 5.1–5.2 | active |
| [Retopo_MAT](tools/retopo-mat.toml) | helper | — | — | paid | Superhive (formerly Blender Market), Gumroad | 2.83–3.0 | stale |
| [RetopoFlow 4](tools/retopoflow.toml) | manual, semi-manual | quads | — | paid | Superhive (formerly Blender Market), GitHub | 4.2–5.2 | active |
| [RetopoGen - Game-Ready Auto-Retopology, LODs & Baking for Blender](tools/retopogen.toml) | automatic | quads | quadriflow, instant-meshes, voxel | paid | Superhive (formerly Blender Market) | 4.5–5.2 | active |
| [RetopoMagic](tools/retopomagic.toml) | manual | — | — | paid | Superhive (formerly Blender Market) | 4.1–5.1 | active |
| [Smart Remesh](tools/smart-remesh.toml) | automatic | quads, n-gons | instant-meshes, own | paid | Superhive (formerly Blender Market), ArtStation Marketplace | 4.2–5.2 | active |
| [Smart Topology Pro](tools/smart-topology-pro.toml) | automatic, semi-automatic | quads | instant-meshes, quadriflow | paid | Superhive (formerly Blender Market) | 4.2–5.2 | active |
| [SoftWrap - Dynamics for Retopology](tools/softwrap.toml) | semi-automatic | template | simulation | paid | Superhive (formerly Blender Market) | 2.83–5.2 | active |
| [SpeedRetopo](tools/speedretopo.toml) | helper | — | — | paid | Superhive (formerly Blender Market), Gumroad | 2.9–5.2 | unknown |
| [Tesselator - Quadrilateral Remeshing](tools/tesselator.toml) | automatic, semi-automatic | quads, quad-dominant, triangles | particles | paid | Superhive (formerly Blender Market) | 2.79–3.2 | stale |
| [TopoPen](tools/topopen.toml) | manual, semi-manual | quads | — | paid | Superhive (formerly Blender Market) | 4.5–5.2 | active |
| [TopoPot - Retopology Tool for Blender](tools/topopot.toml) | manual, semi-manual | quads, triangles | — | free | Blender Extensions | 5.0+ | active |
| [Voxel Remesh](tools/voxel-remesh.toml) | automatic | quads | voxel | free | Blender | 2.81+ | active |
| [Wrap for Blender - Automatic Retopology of Any Model](tools/wrap-for-blender.toml) | semi-automatic | template | own | paid | Superhive (formerly Blender Market) | 5.0–5.2 | active |
<!-- tools:end -->

## Engines

The solvers inside the tools, those that more than one tool runs or that are published apart: [engines.toml](engines.toml).
Which tool runs which engine is in its `engine:` tags; a tool with a solver of its own carries `engine:own`.

<!-- engines:start -->
| Engine | Kind | Licence | Tools |
|---|---|---|---|
| Instant Meshes | paper | BSD-3-Clause | 12 |
| QuadriFlow | paper | BSD-3-Clause | 8 |
| QuadWild | paper | GPL-3.0-or-later | 3 |
| AutoRemesher | program | MIT | 5 |
| OpenVDB | library | Apache-2.0 | 8 |
| Dual Contouring (Blender's dualcon) | paper | GPL-2.0-or-later | 1 |
| mmg | library | LGPL-3.0-or-later | 1 |
| CWF | paper | AGPL-3.0 | 2 |
| Geogram | library | BSD-3-Clause | 1 |
<!-- engines:end -->

## Platforms

Where the tools are distributed: [platforms.toml](platforms.toml).

<!-- platforms:start -->
| Platform | Kind | Paid add-ons | Tools here |
|---|---|---|---|
| [Blender](https://www.blender.org/download/) | built-in | no | 6 |
| [Blender Extensions](https://extensions.blender.org) | official | no | 6 |
| [Superhive (formerly Blender Market)](https://superhivemarket.com) | marketplace | yes | 32 |
| [Gumroad](https://gumroad.com) | creator-store | yes | 10 |
| [GitHub](https://github.com) | code-host | no | 13 |
| Publisher's own site | vendor | yes | 3 |
| [Third-party extension repository](https://docs.blender.org/manual/en/latest/advanced/extensions/creating_repository/index.html) | vendor | yes | 0 |
| [Blendkit (formerly BlenderKit)](https://www.blendkit.com/get-blender-add-ons/) | marketplace | yes | 0 |
| [itch.io](https://itch.io/tools/tag-blender) | marketplace | yes | 0 |
| [Ko-fi (shop)](https://ko-fi.com) | creator-store | yes | 0 |
| [Patreon](https://www.patreon.com) | donation | yes | 0 |
| [ArtStation Marketplace](https://www.artstation.com/marketplace/game-dev/resources/dev-tools/plugins?softwares=blender) | marketplace | yes | 1 |
| [Fab](https://www.fab.com) | marketplace | yes | 0 |
| [Payhip](https://payhip.com) | creator-store | yes | 0 |
| [Lemon Squeezy](https://www.lemonsqueezy.com) | creator-store | yes | 0 |
<!-- platforms:end -->

## Tags

<!-- tags:start -->
| Facet | Tag | Meaning | Tools |
|---|---|---|---|
| mode | `mode:automatic` | global solve: the user sets parameters, the tool makes the whole mesh | 33 |
| mode | `mode:semi-automatic` | global solve steered by the user: marked edges, painted density, strokes or points | 11 |
| mode | `mode:semi-manual` | local solve in Edit Mode: the user bounds a region (strokes, a loop, a patch) and the tool fills it | 10 |
| mode | `mode:manual` | the user places the vertices or the faces; the tool snaps and speeds up | 10 |
| mode | `mode:helper` | makes no faces: sets up the scene, the shading or the snapping | 5 |
| scope | `scope:global` | on the whole object | 36 |
| scope | `scope:local` | on a region or a selection | 19 |
| guide | `guide:parameters` | settings only: face count, size, smoothing | 32 |
| guide | `guide:density` | a painted density map, a vertex group or a size map | 5 |
| guide | `guide:marked-edges` | edges marked sharp or seam, material or face set boundaries | 3 |
| guide | `guide:strokes` | strokes, curves or lines drawn over the source | 12 |
| guide | `guide:points` | points or markers placed on the source | 6 |
| guide | `guide:template` | a base mesh whose topology is kept | 4 |
| guide | `guide:boundary` | a closed loop of edges to fill | 5 |
| guide | `guide:selection` | only the selected faces | 1 |
| output | `output:quads` | only quads | 32 |
| output | `output:quad-dominant` | mostly quads, a few triangles | 15 |
| output | `output:triangles` | triangles | 13 |
| output | `output:n-gons` | faces of more than four sides | 2 |
| output | `output:template` | the topology of the base mesh given by the user | 4 |
| engine | `engine:own` | its own algorithm | 16 |
| engine | `engine:proprietary` | a closed algorithm | 1 |
| engine | `engine:instant-meshes` | Instant Meshes (Jakob et al. 2015) | 11 |
| engine | `engine:quadriflow` | QuadriFlow (Huang et al. 2018) | 8 |
| engine | `engine:quadwild` | QuadWild (Pietroni et al. 2021), with or without Bi-MDF | 3 |
| engine | `engine:autoremesher` | AutoRemesher (Jeremy Hu) | 4 |
| engine | `engine:voxel` | a voxel grid (OpenVDB) | 8 |
| engine | `engine:octree` | dual contouring on an octree (Ju et al. 2002) | 1 |
| engine | `engine:mmg` | the MMG library | 1 |
| engine | `engine:cwf` | CWF | 2 |
| engine | `engine:geogram` | Geogram | 1 |
| engine | `engine:simulation` | a softbody simulation | 1 |
| engine | `engine:particles` | particles relaxed on the surface | 1 |
| engine | `engine:ml` | a machine-learning model somewhere in the chain | 2 |
| use | `use:sculpt` | dense sculpts | 7 |
| use | `use:scan` | 3D scans | 8 |
| use | `use:ai-generated` | AI-generated meshes | 5 |
| use | `use:cad` | tessellated CAD | 1 |
| use | `use:hard-surface` | hard-surface models | 3 |
| use | `use:organic` | organic models | 2 |
| use | `use:head` | heads and faces | 2 |
| use | `use:cylinder` | limbs and other cylindrical parts | 1 |
| use | `use:relief` | reliefs and height fields | 1 |
| use | `use:parts` | several parts joined into one | 1 |
| use | `use:open-boundaries` | meshes with open boundaries to keep | 1 |
| use | `use:game-asset` | game assets | 1 |
| extra | `extra:uv` | unwraps UVs | 4 |
| extra | `extra:lod` | makes levels of detail | 1 |
| extra | `extra:repair` | repairs the input | 2 |
| extra | `extra:keeps-attributes` | carries UVs, materials or weights over to the result | 6 |
| extra | `extra:cloud` | runs part of the work on a server | 1 |
| density | `density:uniform` | faces of one size | 16 |
| density | `density:adaptive` | smaller faces where the detail or the curvature asks for them, on its own | 16 |
| edge-flow | `edge-flow:feature-lines` | keeps or detects sharp edges and creases | 25 |
| edge-flow | `edge-flow:curvature` | aligns the loops with the directions of curvature | 4 |
| edge-flow | `edge-flow:symmetry` | keeps a symmetry | 24 |
| edge-flow | `edge-flow:pole-control` | lets the user place or limit the poles | 3 |
| edge-flow | `edge-flow:loops` | lets the user impose edge loops or rings | 11 |
| input | `input:manifold` | closed and manifold | 3 |
| input | `input:triangles` | triangles only | 2 |
| input | `input:single-object` | one object at a time | 5 |
| input | `input:any` | any mesh, as the publisher states | 3 |
| os | `os:windows` | Windows | 41 |
| os | `os:linux` | Linux | 32 |
| os | `os:macos` | macOS | 35 |
| requires | `requires:external-program` | a program installed apart | 4 |
| requires | `requires:account` | a user account | 1 |
| requires | `requires:internet` | a connection to a server | 0 |
| requires | `requires:api-key` | the user's own key for an online service | 0 |
| requires | `requires:gpu` | a specific GPU, such as NVIDIA with CUDA | 0 |
| blender | `blender:5-x` | Blender 5.0 or later, stated or built in | 35 |
| blender | `blender:4-x` | newest stated is in 4.2 to 4.5 | 0 |
| blender | `blender:legacy` | newest stated is before 4.2 | 4 |
| blender | `blender:unstated` | no newest version stated | 15 |
| install | `install:built-in` | ships with Blender | 6 |
| install | `install:get-extensions` | from Get Extensions in Blender, with updates | 6 |
| install | `install:zip` | a ZIP installed from disk | 40 |
| install | `install:external` | a program of its own | 2 |
| bake | `bake:color` | colour: albedo, diffuse or the source's image texture | 5 |
| bake | `bake:normal` | normal map | 5 |
| bake | `bake:height` | height or displacement map | 0 |
| bake | `bake:ao` | ambient occlusion | 2 |
| bake | `bake:material` | other material channels: roughness, metallic | 3 |
| licence | `licence:open-source` | an open-source licence (GPL, MIT, BSD...), sold or not | 51 |
| licence | `licence:proprietary` | a closed licence | 1 |
| licence | `licence:not-stated` | no licence stated | 2 |
| source | `source:public` | readable online by anyone | 22 |
| source | `source:with-download` | in the add-on one downloads or buys, not published apart | 30 |
| source | `source:closed` | not given | 2 |
| price | `price:free` | free | 20 |
| price | `price:pay-what-you-want` | the buyer chooses the price | 2 |
| price | `price:freemium` | a free edition and paid ones | 3 |
| price | `price:paid` | paid | 32 |
<!-- tags:end -->

## Schema of a tool

Each tool is `tools/<slug>.toml`, read with Python's `tomllib`.
A field that no source confirms is left out, never guessed.

| Field | Required | Meaning |
|---|---|---|
| `name` | yes | the name the publisher uses |
| `slug` | yes | the file name without `.toml`: lower case, digits and `-` |
| `author` | yes | the person or company that publishes it |
| `tags` | yes | list of `"facet:value"` from `taxonomy.toml`; at least one `mode:`, `scope:` and `price:`, and one `output:` unless the tool is only manual or a helper |
| `kind` | yes | `built-in` (ships with Blender), `add-on` (runs inside Blender), `bridge` (an add-on that calls a separate program), `standalone` (a program of its own) |
| `method` | yes | one line: the algorithm or the way of working, with the paper in `papers` when there is one |
| `purpose` | no | what the publisher says it is for, in one line |
| `status` | yes | `active` (a release or commit in the last 12 months), `stale` (none for 12 months or more), `abandoned` (said so by its author, or broken on current Blender), `unknown` |
| `checked` | yes | the date the sources were read (TOML date) |
| `bench` | no | its name on the bench of `bench/results/SCORES.md`, when it is benchmarked |
| `distribution.url` | yes | the main page where one gets it |
| `distribution.channel` | yes | the `id` of its platform in `platforms.toml` |
| `distribution.source` | no | the source code, when it is published |
| `distribution.licence` | yes | an SPDX identifier, `proprietary`, or the words of the store when they give no version: `GPL (version not stated)` |
| `distribution.price` | yes | on the main page: `free`, or the prices as shown, with the currency: `USD 59.90 (Indie), USD 109.90 (Pro)` |
| `distribution.also` | no | list of `{ channel, url, price }`: the other places that distribute it |
| `blender.min` | no | the oldest Blender version the publisher supports |
| `blender.max` | no | the newest Blender version the publisher states as supported |
| `blender.release` | no | the latest release of the tool: `version (YYYY-MM-DD)` |
| `papers` | no | list of `{ title, authors, venue, year, doi }`; `doi` may be left out |
| `sources` | yes | list of `{ url, what }`: each page read, and what it confirmed |
| `notes` | no | anything a reader must know that fits no field |
| `brief.contradicted` | no | claims of the starting tables (2026-10-29) that the sources contradict, with the true value |

## Schema of a platform

`platforms.toml` holds one `[[platform]]` table per platform.

| Field | Required | Meaning |
|---|---|---|
| `id` | yes | what `distribution.channel` names |
| `name` | yes | the name of the platform |
| `url` | yes, but `vendor` | its home page |
| `operator` | yes | who runs it |
| `kind` | yes | `built-in` (Blender itself), `official` (run by the Blender project), `marketplace` (sells for many creators), `creator-store` (a creator's own shop hosted by a service), `code-host` (source and releases), `donation` (gifts, sometimes downloads), `vendor` (a publisher's own site) |
| `paid` | yes | `true` if add-ons can be sold there |
| `licence_rule` | no | what licence the platform requires of add-ons |
| `seller_share` | no | what a creator keeps of a sale, as the platform states it |
| `install` | no | how a user installs an add-on from it |
| `updates` | no | how a user gets updates |
| `blender` | no | Blender versions it serves, when it depends on them |
| `notes` | no | anything else a reader must know |
| `sources` | yes | list of `{ url, what }` |

## Schema of an engine

`engines.toml` holds one `[[engine]]` table per engine.

| Field | Required | Meaning |
|---|---|---|
| `id` | yes | the value of its `engine:` tag in `taxonomy.toml` |
| `name` | yes | the name its authors use |
| `authors` | yes | the people or the organisation behind it |
| `kind` | yes | `paper` (an algorithm published with research code), `library` (meant to be built into other software), `program` (an application) |
| `method` | yes | one line: what it does |
| `year` | no | the year it was first published or released |
| `licence` | no | an SPDX identifier, from the licence of its reference code |
| `source` | no | its reference code |
| `origin` | no | the slug of the tool in `tools/` that is the engine's own program |
| `papers` | no | as for a tool |
| `elsewhere` | no | list of `{ name, url, what }`: software outside Blender that runs it, when a page read says so |
| `sources` | yes | list of `{ url, what }` |
| `notes` | no | anything else a reader must know |
| `checked` | yes | the date the sources were read (TOML date) |

## Schema of the taxonomy

`taxonomy.toml` holds one `[facet.<name>]` table per facet: `title`, `required = true` when every tool needs a tag of it, and one `value = "meaning"` line per tag.
