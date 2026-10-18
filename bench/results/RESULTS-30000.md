# Bench results

Requested budget: 30000 quads. Median over 5 seeds. Lower is better for every metric; "—" means it could not be measured.

## ogre (organic)

| | quadriflow | instant-meshes |
|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 |
| Irregular vertices (S) | 214 | 969 |
| … of which above the minimum | 206 | 953 |
| Base complex patches (C) | 26760 | 29218 |
| Alignment, degrees (A) | 16.16 | 16.30 |
| Mean angle deviation, degrees (F) | 8.62 | 4.53 |
| Angle deviation, 95th percentile (F) | 25.86 | 14.96 |
| Mean aspect ratio (F) | 1.285 | 1.141 |
| Area variation (F) | 0.322 | 0.146 |
| Hausdorff, ‰ of the diagonal (H) | 16.25 | 11.27 |
| Mean distance, ‰ (H) | 0.180 | 0.152 |
| Off budget, % (N) | 9.5 | 1.0 |
| Time, seconds (T) | 35.31 | 2.54 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (1 models)

| Metric | quadriflow | instant-meshes |
|---|---|---|
| Irregular vertices (S) | 1 | 0 |
| Base complex patches (C) | 1 | 0 |
| Alignment, degrees (A) | 1 | 0 |
| Mean angle deviation, degrees (F) | 0 | 1 |
| Mean aspect ratio (F) | 0 | 1 |
| Valid meshes (V) | 0/5 | 0/5 |
