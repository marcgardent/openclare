# Bench results

Requested budget: 30000 quads. Median over 5 seeds; Quad Remesher has no seed and runs once. Lower is better for every metric; "—" means it could not be measured.

## ogre (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 |
| Irregular vertices (S) | 214 | 969 | 351 | 345 |
| … of which above the minimum | 206 | 953 | 343 | 337 |
| Base complex patches (C) | 26760 | 29218 | 21045 | 17385 |
| Alignment, degrees (A) | 16.16 | 16.30 | 9.80 | 10.52 |
| Mean angle deviation, degrees (F) | 8.62 | 4.53 | 7.42 | 7.72 |
| Angle deviation, 95th percentile (F) | 25.86 | 14.96 | 20.07 | 20.27 |
| Mean aspect ratio (F) | 1.285 | 1.141 | 1.560 | 1.407 |
| Area variation (F) | 0.322 | 0.146 | 0.463 | 0.345 |
| Hausdorff, ‰ of the diagonal (H) | 16.25 | 11.27 | 4.22 | 7.45 |
| Mean distance, ‰ (H) | 0.180 | 0.152 | 0.224 | 0.238 |
| Off budget, % (N) | 9.5 | 1.0 | 5.6 | 15.5 |
| Time, seconds (T) | 35.31 | 2.54 | 31.85 | 22.31 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (1 models)

| Metric | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| Irregular vertices (S) | 1 | 0 | 0 | 0 |
| Base complex patches (C) | 0 | 0 | 0 | 1 |
| Alignment, degrees (A) | 0 | 0 | 1 | 0 |
| Mean angle deviation, degrees (F) | 0 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 0 | 1 | 0 | 0 |
| Valid meshes (V) | 0/5 | 0/5 | 0/1 | 1/1 |
