# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher has no seed and runs once. Lower is better for every metric; "—" means it could not be measured.

## ogre (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 1/1 | 0/1 |
| Irregular vertices (S) | 162 | 455 | 338 | 322 |
| … of which above the minimum | 154 | 429 | 330 | — |
| Base complex patches (C) | 7352 | 7656 | 9903 | — |
| Alignment, degrees (A) | 17.78 | 17.72 | 8.50 | 9.79 |
| Mean angle deviation, degrees (F) | 11.59 | 6.18 | 7.95 | 8.39 |
| Angle deviation, 95th percentile (F) | 33.33 | 20.84 | 22.55 | 22.69 |
| Mean aspect ratio (F) | 1.339 | 1.225 | 2.025 | 1.482 |
| Area variation (F) | 0.312 | 0.196 | 0.643 | 0.364 |
| Hausdorff, ‰ of the diagonal (H) | 27.28 | 30.74 | 5.23 | 25.57 |
| Mean distance, ‰ (H) | 0.521 | 0.601 | 0.346 | 0.548 |
| Off budget, % (N) | 3.0 | 2.1 | 59.0 | 15.5 |
| Time, seconds (T) | 4.25 | 2.80 | 23.50 | 24.03 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (1 models)

| Metric | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| Irregular vertices (S) | 1 | 0 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 1 | 0 |
| Mean angle deviation, degrees (F) | 0 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 0 | 1 | 0 | 0 |
| Valid meshes (V) | 0/5 | 0/5 | 1/1 | 0/1 |
