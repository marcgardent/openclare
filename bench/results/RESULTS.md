# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 24 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 2361 | 2045 |
| Alignment, degrees (A) | 5.36 | 4.90 |
| Mean angle deviation, degrees (F) | 6.16 | 5.68 |
| Angle deviation, 95th percentile (F) | 16.82 | 15.30 |
| Mean aspect ratio (F) | 1.414 | 1.370 |
| Area variation (F) | 0.376 | 0.396 |
| Hausdorff, ‰ of the diagonal (H) | 4.40 | 3.50 |
| Mean distance, ‰ (H) | 0.459 | 0.456 |
| Off budget, % (N) | 0.7 | 0.2 |
| Time, seconds (T) | 2.90 | 1.96 |

## rock (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 50 | 66 |
| … of which above the minimum | 42 | 58 |
| Base complex patches (C) | 2968 | 2984 |
| Alignment, degrees (A) | 16.38 | 15.74 |
| Mean angle deviation, degrees (F) | 5.30 | 5.00 |
| Angle deviation, 95th percentile (F) | 13.73 | 13.17 |
| Mean aspect ratio (F) | 1.211 | 1.211 |
| Area variation (F) | 0.269 | 0.290 |
| Hausdorff, ‰ of the diagonal (H) | 5.70 | 5.60 |
| Mean distance, ‰ (H) | 0.791 | 0.787 |
| Off budget, % (N) | 1.1 | 0.5 |
| Time, seconds (T) | 1.85 | 2.21 |

## creature (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 40 | 40 |
| … of which above the minimum | 32 | 32 |
| Base complex patches (C) | 1301 | 2839 |
| Alignment, degrees (A) | 6.95 | 5.75 |
| Mean angle deviation, degrees (F) | 5.70 | 5.07 |
| Angle deviation, 95th percentile (F) | 14.93 | 13.48 |
| Mean aspect ratio (F) | 1.277 | 1.293 |
| Area variation (F) | 0.340 | 0.498 |
| Hausdorff, ‰ of the diagonal (H) | 2.38 | 2.45 |
| Mean distance, ‰ (H) | 0.295 | 0.302 |
| Off budget, % (N) | 1.4 | 1.5 |
| Time, seconds (T) | 2.31 | 1.66 |

## cage (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 48 | 48 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 689 | 304 |
| Alignment, degrees (A) | 3.15 | 2.31 |
| Mean angle deviation, degrees (F) | 2.90 | 3.20 |
| Angle deviation, 95th percentile (F) | 8.99 | 10.08 |
| Mean aspect ratio (F) | 1.183 | 1.230 |
| Area variation (F) | 0.205 | 0.332 |
| Hausdorff, ‰ of the diagonal (H) | 13.74 | 14.20 |
| Mean distance, ‰ (H) | 0.924 | 0.681 |
| Off budget, % (N) | 1.3 | 1.1 |
| Time, seconds (T) | 1.49 | 1.69 |

## suzanne (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 53 | 56 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 2472 | 1895 |
| Alignment, degrees (A) | 10.26 | 9.85 |
| Mean angle deviation, degrees (F) | 4.28 | 4.41 |
| Angle deviation, 95th percentile (F) | 11.60 | 12.38 |
| Mean aspect ratio (F) | 1.213 | 1.210 |
| Area variation (F) | 0.271 | 0.368 |
| Hausdorff, ‰ of the diagonal (H) | 9.13 | 8.95 |
| Mean distance, ‰ (H) | 0.756 | 0.660 |
| Off budget, % (N) | 0.7 | 1.3 |
| Time, seconds (T) | 2.71 | 1.91 |

## ogre (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 76 | 88 |
| … of which above the minimum | 68 | 80 |
| Base complex patches (C) | 2727 | 2109 |
| Alignment, degrees (A) | 12.22 | 12.29 |
| Mean angle deviation, degrees (F) | 6.46 | 5.92 |
| Angle deviation, 95th percentile (F) | 18.16 | 16.68 |
| Mean aspect ratio (F) | 1.558 | 1.404 |
| Area variation (F) | 0.384 | 0.435 |
| Hausdorff, ‰ of the diagonal (H) | 27.60 | 25.27 |
| Mean distance, ‰ (H) | 0.991 | 0.962 |
| Off budget, % (N) | 2.0 | 0.5 |
| Time, seconds (T) | 4.80 | 4.01 |

## spot (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 70 | 70 |
| … of which above the minimum | 62 | 62 |
| Base complex patches (C) | 1730 | 1663 |
| Alignment, degrees (A) | 8.70 | 7.87 |
| Mean angle deviation, degrees (F) | 5.59 | 5.60 |
| Angle deviation, 95th percentile (F) | 14.97 | 14.87 |
| Mean aspect ratio (F) | 1.341 | 1.340 |
| Area variation (F) | 0.324 | 0.427 |
| Hausdorff, ‰ of the diagonal (H) | 6.29 | 7.46 |
| Mean distance, ‰ (H) | 0.416 | 0.422 |
| Off budget, % (N) | 1.6 | 0.7 |
| Time, seconds (T) | 3.62 | 3.51 |

## bob (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 20 | 20 |
| … of which above the minimum | 20 | 20 |
| Base complex patches (C) | 1290 | 237 |
| Alignment, degrees (A) | 4.00 | 3.61 |
| Mean angle deviation, degrees (F) | 3.28 | 3.24 |
| Angle deviation, 95th percentile (F) | 8.81 | 9.07 |
| Mean aspect ratio (F) | 1.385 | 1.370 |
| Area variation (F) | 0.373 | 0.432 |
| Hausdorff, ‰ of the diagonal (H) | 2.49 | 2.10 |
| Mean distance, ‰ (H) | 0.328 | 0.336 |
| Off budget, % (N) | 1.4 | 1.7 |
| Time, seconds (T) | 1.69 | 1.75 |

## blub (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 44 | 52 |
| … of which above the minimum | 36 | 44 |
| Base complex patches (C) | 1287 | 1036 |
| Alignment, degrees (A) | 7.86 | 7.09 |
| Mean angle deviation, degrees (F) | 5.15 | 4.84 |
| Angle deviation, 95th percentile (F) | 15.61 | 15.36 |
| Mean aspect ratio (F) | 1.296 | 1.295 |
| Area variation (F) | 0.296 | 0.398 |
| Hausdorff, ‰ of the diagonal (H) | 10.36 | 8.65 |
| Mean distance, ‰ (H) | 0.343 | 0.330 |
| Off budget, % (N) | 0.2 | 0.3 |
| Time, seconds (T) | 2.06 | 3.26 |

## pig (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 78 | 80 |
| … of which above the minimum | 70 | 72 |
| Base complex patches (C) | 2401 | 1666 |
| Alignment, degrees (A) | 10.56 | 9.01 |
| Mean angle deviation, degrees (F) | 5.87 | 5.44 |
| Angle deviation, 95th percentile (F) | 16.42 | 15.81 |
| Mean aspect ratio (F) | 1.239 | 1.257 |
| Area variation (F) | 0.293 | 0.444 |
| Hausdorff, ‰ of the diagonal (H) | 18.14 | 22.22 |
| Mean distance, ‰ (H) | 0.607 | 0.583 |
| Off budget, % (N) | 1.6 | 1.8 |
| Time, seconds (T) | 2.77 | 4.43 |

## jerry (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 86 | 102 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 2043 | 2384 |
| Alignment, degrees (A) | 14.30 | 13.83 |
| Mean angle deviation, degrees (F) | 6.30 | 5.96 |
| Angle deviation, 95th percentile (F) | 16.07 | 18.39 |
| Mean aspect ratio (F) | 1.268 | 1.292 |
| Area variation (F) | 0.309 | 0.452 |
| Hausdorff, ‰ of the diagonal (H) | 31.62 | 22.32 |
| Mean distance, ‰ (H) | 1.372 | 1.221 |
| Off budget, % (N) | 0.0 | 1.1 |
| Time, seconds (T) | 1.78 | 1.62 |

## rose (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 911 | 900 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 14.64 | 14.18 |
| Mean angle deviation, degrees (F) | 19.74 | 19.62 |
| Angle deviation, 95th percentile (F) | 63.27 | 62.71 |
| Mean aspect ratio (F) | 3.866 | 4.127 |
| Area variation (F) | 0.895 | 0.949 |
| Hausdorff, ‰ of the diagonal (H) | 35.29 | 35.29 |
| Mean distance, ‰ (H) | 2.950 | 2.915 |
| Off budget, % (N) | 8.1 | 15.8 |
| Time, seconds (T) | 7.57 | 7.66 |

## bridge (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 50 | 46 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 1066 | 764 |
| Alignment, degrees (A) | 8.02 | 7.42 |
| Mean angle deviation, degrees (F) | 3.77 | 3.90 |
| Angle deviation, 95th percentile (F) | 11.08 | 11.54 |
| Mean aspect ratio (F) | 1.244 | 1.248 |
| Area variation (F) | 0.294 | 0.436 |
| Hausdorff, ‰ of the diagonal (H) | 6.89 | 6.65 |
| Mean distance, ‰ (H) | 0.597 | 0.577 |
| Off budget, % (N) | 1.6 | 1.4 |
| Time, seconds (T) | 2.69 | 2.51 |

## part (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 24 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 78 | 285 |
| Alignment, degrees (A) | 11.96 | 10.48 |
| Mean angle deviation, degrees (F) | 1.35 | 2.25 |
| Angle deviation, 95th percentile (F) | 4.82 | 6.23 |
| Mean aspect ratio (F) | 1.159 | 1.162 |
| Area variation (F) | 0.113 | 0.365 |
| Hausdorff, ‰ of the diagonal (H) | 1.07 | 0.59 |
| Mean distance, ‰ (H) | 0.052 | 0.056 |
| Off budget, % (N) | 1.6 | 0.7 |
| Time, seconds (T) | 1.64 | 1.60 |

## cube (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 8 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 6 | 6 |
| Alignment, degrees (A) | 0.14 | 0.36 |
| Mean angle deviation, degrees (F) | 0.01 | 0.36 |
| Angle deviation, 95th percentile (F) | 0.02 | 0.85 |
| Mean aspect ratio (F) | 1.031 | 1.088 |
| Area variation (F) | 0.021 | 0.098 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 |
| Off budget, % (N) | 0.3 | 0.3 |
| Time, seconds (T) | 1.34 | 1.22 |

## lblock (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 12 | 12 |
| … of which above the minimum | 4 | 4 |
| Base complex patches (C) | 14 | 14 |
| Alignment, degrees (A) | 0.52 | 0.86 |
| Mean angle deviation, degrees (F) | 0.01 | 0.53 |
| Angle deviation, 95th percentile (F) | 0.05 | 1.43 |
| Mean aspect ratio (F) | 1.030 | 1.097 |
| Area variation (F) | 0.033 | 0.106 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 |
| Off budget, % (N) | 2.0 | 0.9 |
| Time, seconds (T) | 1.24 | 1.27 |

## cylinder (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 8 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 722 | 463 |
| Alignment, degrees (A) | 1.50 | 1.63 |
| Mean angle deviation, degrees (F) | 3.59 | 2.70 |
| Angle deviation, 95th percentile (F) | 13.76 | 8.52 |
| Mean aspect ratio (F) | 1.388 | 1.532 |
| Area variation (F) | 0.375 | 0.233 |
| Hausdorff, ‰ of the diagonal (H) | 0.94 | 0.93 |
| Mean distance, ‰ (H) | 0.176 | 0.165 |
| Off budget, % (N) | 0.5 | 1.3 |
| Time, seconds (T) | 1.37 | 1.38 |

## terrain (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 56 | 68 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 1681 | 1952 |
| Alignment, degrees (A) | 18.56 | 18.78 |
| Mean angle deviation, degrees (F) | 6.16 | 6.39 |
| Angle deviation, 95th percentile (F) | 17.46 | 18.13 |
| Mean aspect ratio (F) | 1.292 | 1.325 |
| Area variation (F) | 0.317 | 0.460 |
| Hausdorff, ‰ of the diagonal (H) | 21.40 | 18.85 |
| Mean distance, ‰ (H) | 1.537 | 1.317 |
| Off budget, % (N) | 1.1 | 0.5 |
| Time, seconds (T) | 7.16 | 6.58 |

## mahaveer (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 453 | 453 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 714 | 707 |
| Alignment, degrees (A) | 14.13 | 13.90 |
| Mean angle deviation, degrees (F) | 26.19 | 26.24 |
| Angle deviation, 95th percentile (F) | 79.74 | 80.61 |
| Mean aspect ratio (F) | 88.317 | 80.036 |
| Area variation (F) | 1.575 | 1.573 |
| Hausdorff, ‰ of the diagonal (H) | 41.14 | 41.14 |
| Mean distance, ‰ (H) | 3.418 | 3.416 |
| Off budget, % (N) | 0.4 | 0.1 |
| Time, seconds (T) | 9.76 | 11.32 |

## marine (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 5577 | 5562 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 12.17 | 12.17 |
| Mean angle deviation, degrees (F) | 26.27 | 26.30 |
| Angle deviation, 95th percentile (F) | 74.74 | 74.83 |
| Mean aspect ratio (F) | 7.329 | 7.716 |
| Area variation (F) | 2.632 | 2.643 |
| Hausdorff, ‰ of the diagonal (H) | 6.26 | 6.26 |
| Mean distance, ‰ (H) | 0.581 | 0.583 |
| Off budget, % (N) | 116.2 | 115.2 |
| Time, seconds (T) | 37.26 | 39.11 |

## sphere (calibration)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 32 | 32 |
| … of which above the minimum | 24 | 24 |
| Base complex patches (C) | 30 | 30 |
| Alignment, degrees (A) | — | — |
| Mean angle deviation, degrees (F) | 3.93 | 3.94 |
| Angle deviation, 95th percentile (F) | 7.43 | 7.44 |
| Mean aspect ratio (F) | 1.083 | 1.083 |
| Area variation (F) | 0.302 | 0.302 |
| Hausdorff, ‰ of the diagonal (H) | 0.86 | 0.86 |
| Mean distance, ‰ (H) | 0.223 | 0.223 |
| Off budget, % (N) | 0.0 | 0.0 |
| Time, seconds (T) | 0.77 | 0.79 |

## torus (calibration)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 0 | 0 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 1 | 1 |
| Alignment, degrees (A) | 0.62 | 0.62 |
| Mean angle deviation, degrees (F) | 1.44 | 1.44 |
| Angle deviation, 95th percentile (F) | 2.36 | 2.36 |
| Mean aspect ratio (F) | 1.323 | 1.323 |
| Area variation (F) | 0.307 | 0.307 |
| Hausdorff, ‰ of the diagonal (H) | 1.00 | 1.00 |
| Mean distance, ‰ (H) | 0.304 | 0.304 |
| Off budget, % (N) | 1.6 | 1.6 |
| Time, seconds (T) | 1.11 | 1.14 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 11 | 7 |
| Base complex patches (C) | 3 | 9 |
| Alignment, degrees (A) | 1 | 12 |
| Mean angle deviation, degrees (F) | 4 | 9 |
| Mean aspect ratio (F) | 7 | 6 |
| Valid meshes (V) | 12/13 | 12/13 |

### mechanical (4 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 4 | 4 |
| Base complex patches (C) | 3 | 3 |
| Alignment, degrees (A) | 3 | 1 |
| Mean angle deviation, degrees (F) | 3 | 1 |
| Mean aspect ratio (F) | 4 | 0 |
| Valid meshes (V) | 4/4 | 4/4 |
