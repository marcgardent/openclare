# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 24 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 664 | 802 |
| Alignment, degrees (A) | 5.25 | 4.72 |
| Mean angle deviation, degrees (F) | 6.36 | 5.67 |
| Angle deviation, 95th percentile (F) | 16.63 | 14.46 |
| Mean aspect ratio (F) | 1.372 | 1.356 |
| Area variation (F) | 0.454 | 0.469 |
| Hausdorff, ‰ of the diagonal (H) | 13.79 | 12.28 |
| Mean distance, ‰ (H) | 1.294 | 1.363 |
| Off budget, % (N) | 2.6 | 0.2 |
| Time, seconds (T) | 1.45 | 1.59 |

## rock (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 26 | 26 |
| … of which above the minimum | 18 | 18 |
| Base complex patches (C) | 767 | 981 |
| Alignment, degrees (A) | 16.99 | 16.76 |
| Mean angle deviation, degrees (F) | 5.63 | 4.82 |
| Angle deviation, 95th percentile (F) | 15.95 | 13.84 |
| Mean aspect ratio (F) | 1.253 | 1.226 |
| Area variation (F) | 0.355 | 0.331 |
| Hausdorff, ‰ of the diagonal (H) | 9.57 | 9.06 |
| Mean distance, ‰ (H) | 1.510 | 1.522 |
| Off budget, % (N) | 2.2 | 1.9 |
| Time, seconds (T) | 1.10 | 1.29 |

## creature (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 26 | 40 |
| … of which above the minimum | 18 | 32 |
| Base complex patches (C) | 773 | 355 |
| Alignment, degrees (A) | 8.01 | 5.68 |
| Mean angle deviation, degrees (F) | 4.90 | 5.45 |
| Angle deviation, 95th percentile (F) | 14.91 | 15.55 |
| Mean aspect ratio (F) | 1.240 | 1.335 |
| Area variation (F) | 0.354 | 0.487 |
| Hausdorff, ‰ of the diagonal (H) | 8.03 | 5.03 |
| Mean distance, ‰ (H) | 0.855 | 0.928 |
| Off budget, % (N) | 3.5 | 0.1 |
| Time, seconds (T) | 1.74 | 1.69 |

## cage (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 48 | 48 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 263 | 208 |
| Alignment, degrees (A) | 4.09 | 3.16 |
| Mean angle deviation, degrees (F) | 4.64 | 4.99 |
| Angle deviation, 95th percentile (F) | 13.90 | 15.32 |
| Mean aspect ratio (F) | 1.252 | 1.248 |
| Area variation (F) | 0.236 | 0.342 |
| Hausdorff, ‰ of the diagonal (H) | 29.70 | 15.07 |
| Mean distance, ‰ (H) | 3.002 | 1.726 |
| Off budget, % (N) | 0.1 | 0.1 |
| Time, seconds (T) | 0.76 | 1.38 |

## suzanne (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 44 | 48 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 601 | 648 |
| Alignment, degrees (A) | 11.14 | 10.38 |
| Mean angle deviation, degrees (F) | 4.77 | 5.24 |
| Angle deviation, 95th percentile (F) | 16.96 | 18.03 |
| Mean aspect ratio (F) | 1.230 | 1.249 |
| Area variation (F) | 0.290 | 0.425 |
| Hausdorff, ‰ of the diagonal (H) | 18.81 | 13.07 |
| Mean distance, ‰ (H) | 2.213 | 1.975 |
| Off budget, % (N) | 0.2 | 0.6 |
| Time, seconds (T) | 1.18 | 1.11 |

## ogre (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 46 | 44 |
| … of which above the minimum | 38 | 36 |
| Base complex patches (C) | 562 | 436 |
| Alignment, degrees (A) | 10.54 | 9.98 |
| Mean angle deviation, degrees (F) | 6.95 | 7.53 |
| Angle deviation, 95th percentile (F) | 19.86 | 20.80 |
| Mean aspect ratio (F) | 1.666 | 1.664 |
| Area variation (F) | 0.424 | 0.551 |
| Hausdorff, ‰ of the diagonal (H) | 28.57 | 31.45 |
| Mean distance, ‰ (H) | 2.188 | 2.240 |
| Off budget, % (N) | 0.2 | 0.4 |
| Time, seconds (T) | 3.75 | 2.51 |

## spot (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 64 | 66 |
| … of which above the minimum | 56 | 58 |
| Base complex patches (C) | 770 | 681 |
| Alignment, degrees (A) | 9.52 | 8.16 |
| Mean angle deviation, degrees (F) | 7.21 | 7.54 |
| Angle deviation, 95th percentile (F) | 19.51 | 21.69 |
| Mean aspect ratio (F) | 1.378 | 1.367 |
| Area variation (F) | 0.406 | 0.519 |
| Hausdorff, ‰ of the diagonal (H) | 12.51 | 10.17 |
| Mean distance, ‰ (H) | 1.287 | 1.249 |
| Off budget, % (N) | 1.0 | 0.3 |
| Time, seconds (T) | 1.65 | 1.67 |

## bob (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 18 | 20 |
| … of which above the minimum | 18 | 20 |
| Base complex patches (C) | 237 | 185 |
| Alignment, degrees (A) | 4.27 | 3.58 |
| Mean angle deviation, degrees (F) | 4.47 | 4.46 |
| Angle deviation, 95th percentile (F) | 11.60 | 12.91 |
| Mean aspect ratio (F) | 1.382 | 1.395 |
| Area variation (F) | 0.431 | 0.452 |
| Hausdorff, ‰ of the diagonal (H) | 8.11 | 5.48 |
| Mean distance, ‰ (H) | 1.002 | 1.046 |
| Off budget, % (N) | 0.7 | 2.8 |
| Time, seconds (T) | 1.59 | 1.47 |

## blub (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 40 | 40 |
| … of which above the minimum | 32 | 32 |
| Base complex patches (C) | 321 | 406 |
| Alignment, degrees (A) | 8.65 | 8.58 |
| Mean angle deviation, degrees (F) | 4.73 | 4.97 |
| Angle deviation, 95th percentile (F) | 13.69 | 14.59 |
| Mean aspect ratio (F) | 1.312 | 1.345 |
| Area variation (F) | 0.349 | 0.426 |
| Hausdorff, ‰ of the diagonal (H) | 21.55 | 19.38 |
| Mean distance, ‰ (H) | 1.274 | 1.098 |
| Off budget, % (N) | 1.7 | 0.2 |
| Time, seconds (T) | 1.42 | 1.31 |

## pig (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 |
| Irregular vertices (S) | 60 | 56 |
| … of which above the minimum | 52 | 48 |
| Base complex patches (C) | 541 | 625 |
| Alignment, degrees (A) | 11.72 | 10.94 |
| Mean angle deviation, degrees (F) | 5.95 | 5.90 |
| Angle deviation, 95th percentile (F) | 18.82 | 19.14 |
| Mean aspect ratio (F) | 1.342 | 1.344 |
| Area variation (F) | 0.325 | 0.500 |
| Hausdorff, ‰ of the diagonal (H) | 23.62 | 33.26 |
| Mean distance, ‰ (H) | 1.664 | 1.723 |
| Off budget, % (N) | 0.9 | 2.5 |
| Time, seconds (T) | 2.32 | 2.96 |

## jerry (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 51 | 50 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 605 | 714 |
| Alignment, degrees (A) | 14.76 | 15.37 |
| Mean angle deviation, degrees (F) | 7.50 | 7.46 |
| Angle deviation, 95th percentile (F) | 20.89 | 21.35 |
| Mean aspect ratio (F) | 1.419 | 1.368 |
| Area variation (F) | 0.328 | 0.532 |
| Hausdorff, ‰ of the diagonal (H) | 35.79 | 33.89 |
| Mean distance, ‰ (H) | 3.338 | 3.082 |
| Off budget, % (N) | 0.0 | 1.1 |
| Time, seconds (T) | 1.28 | 2.55 |

## rose (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 715 | 707 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 17.39 | 17.13 |
| Mean angle deviation, degrees (F) | 25.75 | 26.84 |
| Angle deviation, 95th percentile (F) | 69.17 | 70.28 |
| Mean aspect ratio (F) | 4.884 | 4.714 |
| Area variation (F) | 2.023 | 2.024 |
| Hausdorff, ‰ of the diagonal (H) | 77.32 | 77.44 |
| Mean distance, ‰ (H) | 6.886 | 7.306 |
| Off budget, % (N) | 77.3 | 64.0 |
| Time, seconds (T) | 8.30 | 7.39 |

## bridge (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 36 | 36 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 279 | 303 |
| Alignment, degrees (A) | 7.06 | 6.59 |
| Mean angle deviation, degrees (F) | 4.07 | 4.72 |
| Angle deviation, 95th percentile (F) | 12.92 | 14.56 |
| Mean aspect ratio (F) | 1.267 | 1.299 |
| Area variation (F) | 0.275 | 0.494 |
| Hausdorff, ‰ of the diagonal (H) | 17.46 | 17.47 |
| Mean distance, ‰ (H) | 1.366 | 1.256 |
| Off budget, % (N) | 0.3 | 1.8 |
| Time, seconds (T) | 3.93 | 2.40 |

## part (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 24 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 86 | 68 |
| Alignment, degrees (A) | 12.41 | 10.87 |
| Mean angle deviation, degrees (F) | 2.08 | 2.16 |
| Angle deviation, 95th percentile (F) | 9.09 | 10.18 |
| Mean aspect ratio (F) | 1.221 | 1.178 |
| Area variation (F) | 0.176 | 0.384 |
| Hausdorff, ‰ of the diagonal (H) | 15.02 | 1.66 |
| Mean distance, ‰ (H) | 0.246 | 0.164 |
| Off budget, % (N) | 2.4 | 1.8 |
| Time, seconds (T) | 1.53 | 0.81 |

## cube (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 8 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 6 | 6 |
| Alignment, degrees (A) | 0.11 | 0.25 |
| Mean angle deviation, degrees (F) | 0.00 | 0.33 |
| Angle deviation, 95th percentile (F) | 0.01 | 0.76 |
| Mean aspect ratio (F) | 1.000 | 1.071 |
| Area variation (F) | 0.000 | 0.082 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 |
| Off budget, % (N) | 1.4 | 1.4 |
| Time, seconds (T) | 0.64 | 0.84 |

## lblock (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 12 | 12 |
| … of which above the minimum | 4 | 4 |
| Base complex patches (C) | 14 | 14 |
| Alignment, degrees (A) | 0.56 | 0.64 |
| Mean angle deviation, degrees (F) | 0.10 | 0.37 |
| Angle deviation, 95th percentile (F) | 0.33 | 0.90 |
| Mean aspect ratio (F) | 1.072 | 1.099 |
| Area variation (F) | 0.070 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 |
| Off budget, % (N) | 0.6 | 0.6 |
| Time, seconds (T) | 0.98 | 0.63 |

## cylinder (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 8 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 6 | 186 |
| Alignment, degrees (A) | 1.24 | 1.81 |
| Mean angle deviation, degrees (F) | 2.65 | 3.73 |
| Angle deviation, 95th percentile (F) | 7.69 | 8.53 |
| Mean aspect ratio (F) | 1.284 | 1.507 |
| Area variation (F) | 0.334 | 0.205 |
| Hausdorff, ‰ of the diagonal (H) | 1.49 | 1.71 |
| Mean distance, ‰ (H) | 0.485 | 0.582 |
| Off budget, % (N) | 1.0 | 3.2 |
| Time, seconds (T) | 0.96 | 0.99 |

## terrain (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 36 | 32 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 647 | 580 |
| Alignment, degrees (A) | 17.95 | 17.84 |
| Mean angle deviation, degrees (F) | 5.66 | 6.01 |
| Angle deviation, 95th percentile (F) | 18.16 | 17.88 |
| Mean aspect ratio (F) | 1.290 | 1.317 |
| Area variation (F) | 0.373 | 0.482 |
| Hausdorff, ‰ of the diagonal (H) | 24.30 | 33.07 |
| Mean distance, ‰ (H) | 2.485 | 2.172 |
| Off budget, % (N) | 1.7 | 1.1 |
| Time, seconds (T) | 8.37 | 5.70 |

## mahaveer (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 421 | 421 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 16.19 | 16.05 |
| Mean angle deviation, degrees (F) | 35.08 | 35.15 |
| Angle deviation, 95th percentile (F) | 86.45 | 86.48 |
| Mean aspect ratio (F) | 129.120 | 117.052 |
| Area variation (F) | 8.454 | 8.545 |
| Hausdorff, ‰ of the diagonal (H) | 325.73 | 325.73 |
| Mean distance, ‰ (H) | 40.224 | 40.364 |
| Off budget, % (N) | 105.3 | 104.6 |
| Time, seconds (T) | 6.72 | 7.89 |

## marine (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 5577 | 5562 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | — | — |
| Mean angle deviation, degrees (F) | 26.27 | 26.30 |
| Angle deviation, 95th percentile (F) | 74.74 | 74.83 |
| Mean aspect ratio (F) | 7.329 | 7.716 |
| Area variation (F) | 2.632 | 2.643 |
| Hausdorff, ‰ of the diagonal (H) | 6.26 | 6.26 |
| Mean distance, ‰ (H) | 0.581 | 0.583 |
| Off budget, % (N) | 548.5 | 545.5 |
| Time, seconds (T) | 38.56 | 42.11 |

## sphere (calibration)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 8 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 1010 | 1010 |
| Alignment, degrees (A) | — | — |
| Mean angle deviation, degrees (F) | 4.61 | 4.61 |
| Angle deviation, 95th percentile (F) | 11.14 | 11.12 |
| Mean aspect ratio (F) | 1.180 | 1.180 |
| Area variation (F) | 0.224 | 0.224 |
| Hausdorff, ‰ of the diagonal (H) | 1.67 | 1.67 |
| Mean distance, ‰ (H) | 0.670 | 0.670 |
| Off budget, % (N) | 1.0 | 1.0 |
| Time, seconds (T) | 1.91 | 1.18 |

## torus (calibration)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 0 | 0 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 1 | 1 |
| Alignment, degrees (A) | 0.16 | 0.16 |
| Mean angle deviation, degrees (F) | 2.40 | 2.40 |
| Angle deviation, 95th percentile (F) | 3.72 | 3.72 |
| Mean aspect ratio (F) | 1.297 | 1.297 |
| Area variation (F) | 0.345 | 0.346 |
| Hausdorff, ‰ of the diagonal (H) | 2.13 | 2.13 |
| Mean distance, ‰ (H) | 0.833 | 0.833 |
| Off budget, % (N) | 0.8 | 0.8 |
| Time, seconds (T) | 1.25 | 0.50 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 9 | 9 |
| Base complex patches (C) | 7 | 5 |
| Alignment, degrees (A) | 1 | 12 |
| Mean angle deviation, degrees (F) | 8 | 5 |
| Mean aspect ratio (F) | 6 | 7 |
| Valid meshes (V) | 11/13 | 12/13 |

### mechanical (4 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 4 | 4 |
| Base complex patches (C) | 3 | 3 |
| Alignment, degrees (A) | 3 | 1 |
| Mean angle deviation, degrees (F) | 4 | 0 |
| Mean aspect ratio (F) | 3 | 1 |
| Valid meshes (V) | 4/4 | 4/4 |
