# References kept

Outputs of tools that cannot be run again at will. They are tracked so that the bench can go
on comparing with them once the licence has expired.

## quadremesher/

Quad Remesher 1.4.1 (Exoside), trial licence, run on 6, 7 and 8 October 2026 from Blender 5.2.2
on the models of the bench (`bench/scenes/`). Non-commercial use.

One file per model, budget and setting: `<model>__<quads asked for>__<setting>.obj`, with a
`.seconds` file beside it that keeps the computing time measured.

| Setting | Meaning |
|---|---|
| `default` | settings as shipped: adaptive size at 50 %, quad count adjusted |
| `uniform` | constant size and exact quad count, to compare on equal terms |

Budgets: 1000, 3000 and 8000 quads. Detection of hard edges on the mechanical models only;
symmetry off.

These meshes only stand against the exact models of `bench/scenes/`, tracked too: making the
models again with another version of Blender would give slightly different ones.

To make more while a licence is available:

```bash
python3 bench/make_references.py --targets 1000,3000,8000
```

References already there are kept; `--refresh` computes them again.
