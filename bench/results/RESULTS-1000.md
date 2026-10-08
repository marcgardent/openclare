# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 24 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 664 | 790 |
| Alignment, degrees (A) | 5.25 | 4.44 |
| Mean angle deviation, degrees (F) | 6.36 | 5.76 |
| Angle deviation, 95th percentile (F) | 16.63 | 14.54 |
| Mean aspect ratio (F) | 1.372 | 1.349 |
| Area variation (F) | 0.454 | 0.464 |
| Hausdorff, ‰ of the diagonal (H) | 13.79 | 8.28 |
| Mean distance, ‰ (H) | 1.294 | 1.373 |
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 98.35 | 98.33 |
| Off budget, % (N) | 2.6 | 1.0 |
| Time, seconds (T) | 1.86 | 1.97 |

## rock (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 26 | 26 |
| … of which above the minimum | 18 | 18 |
| Base complex patches (C) | 767 | 981 |
| Alignment, degrees (A) | 16.99 | 16.76 |
| Mean angle deviation, degrees (F) | 5.63 | 4.82 |
| Angle deviation, 95th percentile (F) | 15.95 | 13.83 |
| Mean aspect ratio (F) | 1.253 | 1.226 |
| Area variation (F) | 0.355 | 0.331 |
| Hausdorff, ‰ of the diagonal (H) | 9.57 | 9.06 |
| Mean distance, ‰ (H) | 1.510 | 1.522 |
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.29 | 99.19 |
| Off budget, % (N) | 2.2 | 1.9 |
| Time, seconds (T) | 1.12 | 1.31 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 98.25 | 98.03 |
| Off budget, % (N) | 3.5 | 0.1 |
| Time, seconds (T) | 2.25 | 1.68 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 95.71 | 97.28 |
| Off budget, % (N) | 0.1 | 0.1 |
| Time, seconds (T) | 0.79 | 1.48 |

## suzanne (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 44 | 48 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 601 | 554 |
| Alignment, degrees (A) | 11.14 | 10.08 |
| Mean angle deviation, degrees (F) | 4.77 | 5.53 |
| Angle deviation, 95th percentile (F) | 16.96 | 18.15 |
| Mean aspect ratio (F) | 1.230 | 1.306 |
| Area variation (F) | 0.290 | 0.449 |
| Hausdorff, ‰ of the diagonal (H) | 18.81 | 12.09 |
| Mean distance, ‰ (H) | 2.213 | 1.939 |
| Thin parts: mean distance, % of a quad edge (V) | 12.3 | 7.3 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 0.2 | 0.6 |
| Time, seconds (T) | 0.87 | 1.29 |

## ogre (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 46 | 106 |
| … of which above the minimum | 38 | 98 |
| Base complex patches (C) | 562 | 611 |
| Alignment, degrees (A) | 10.54 | 10.80 |
| Mean angle deviation, degrees (F) | 6.95 | 9.21 |
| Angle deviation, 95th percentile (F) | 19.86 | 27.40 |
| Mean aspect ratio (F) | 1.666 | 1.669 |
| Area variation (F) | 0.424 | 0.831 |
| Hausdorff, ‰ of the diagonal (H) | 28.57 | 29.26 |
| Mean distance, ‰ (H) | 2.188 | 2.521 |
| Thin parts: mean distance, % of a quad edge (V) | 20.5 | 7.9 |
| Volume kept, % of the source (V) | 96.91 | 96.37 |
| Off budget, % (N) | 0.2 | 3.1 |
| Time, seconds (T) | 3.41 | 3.37 |

## spot (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 64 | 68 |
| … of which above the minimum | 56 | 60 |
| Base complex patches (C) | 770 | 721 |
| Alignment, degrees (A) | 9.52 | 7.66 |
| Mean angle deviation, degrees (F) | 7.21 | 6.94 |
| Angle deviation, 95th percentile (F) | 19.51 | 20.13 |
| Mean aspect ratio (F) | 1.378 | 1.396 |
| Area variation (F) | 0.406 | 0.456 |
| Hausdorff, ‰ of the diagonal (H) | 12.51 | 16.18 |
| Mean distance, ‰ (H) | 1.287 | 1.256 |
| Thin parts: mean distance, % of a quad edge (V) | 13.9 | 21.1 |
| Volume kept, % of the source (V) | 97.91 | 97.89 |
| Off budget, % (N) | 1.0 | 0.9 |
| Time, seconds (T) | 2.72 | 2.05 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 98.04 | 97.91 |
| Off budget, % (N) | 0.7 | 2.8 |
| Time, seconds (T) | 1.30 | 1.95 |

## blub (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 40 | 50 |
| … of which above the minimum | 32 | 42 |
| Base complex patches (C) | 321 | 523 |
| Alignment, degrees (A) | 8.65 | 9.15 |
| Mean angle deviation, degrees (F) | 4.73 | 5.64 |
| Angle deviation, 95th percentile (F) | 13.69 | 17.16 |
| Mean aspect ratio (F) | 1.312 | 1.355 |
| Area variation (F) | 0.349 | 0.470 |
| Hausdorff, ‰ of the diagonal (H) | 21.55 | 19.65 |
| Mean distance, ‰ (H) | 1.274 | 1.064 |
| Thin parts: mean distance, % of a quad edge (V) | 22.2 | 9.1 |
| Volume kept, % of the source (V) | 97.41 | 97.82 |
| Off budget, % (N) | 1.7 | 1.1 |
| Time, seconds (T) | 2.11 | 1.62 |

## pig (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 |
| Irregular vertices (S) | 60 | 92 |
| … of which above the minimum | 52 | 84 |
| Base complex patches (C) | 541 | 622 |
| Alignment, degrees (A) | 11.72 | 9.30 |
| Mean angle deviation, degrees (F) | 5.95 | 7.10 |
| Angle deviation, 95th percentile (F) | 18.82 | 21.70 |
| Mean aspect ratio (F) | 1.342 | 1.447 |
| Area variation (F) | 0.325 | 0.906 |
| Hausdorff, ‰ of the diagonal (H) | 23.62 | 19.87 |
| Mean distance, ‰ (H) | 1.664 | 1.863 |
| Thin parts: mean distance, % of a quad edge (V) | 16.7 | 6.7 |
| Volume kept, % of the source (V) | 97.68 | 97.19 |
| Off budget, % (N) | 0.9 | 0.8 |
| Time, seconds (T) | 2.00 | 2.46 |

## jerry (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 0/1 |
| Irregular vertices (S) | 51 | 103 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 605 | 982 |
| Alignment, degrees (A) | 14.76 | 16.17 |
| Mean angle deviation, degrees (F) | 7.50 | 10.34 |
| Angle deviation, 95th percentile (F) | 20.89 | 30.37 |
| Mean aspect ratio (F) | 1.419 | 1.551 |
| Area variation (F) | 0.328 | 1.012 |
| Hausdorff, ‰ of the diagonal (H) | 35.79 | 28.86 |
| Mean distance, ‰ (H) | 3.338 | 3.425 |
| Thin parts: mean distance, % of a quad edge (V) | 21.8 | 16.5 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 0.0 | 1.8 |
| Time, seconds (T) | 1.09 | 1.29 |

## rose (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 715 | 842 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 17.39 | 17.22 |
| Mean angle deviation, degrees (F) | 25.75 | 28.43 |
| Angle deviation, 95th percentile (F) | 69.17 | 73.44 |
| Mean aspect ratio (F) | 4.884 | 4.652 |
| Area variation (F) | 2.023 | 2.048 |
| Hausdorff, ‰ of the diagonal (H) | 77.32 | 76.71 |
| Mean distance, ‰ (H) | 6.886 | 8.013 |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 39.1 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 77.3 | 57.7 |
| Time, seconds (T) | 8.50 | 10.69 |

## bridge (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 36 | 36 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 279 | 451 |
| Alignment, degrees (A) | 7.06 | 6.67 |
| Mean angle deviation, degrees (F) | 4.07 | 5.50 |
| Angle deviation, 95th percentile (F) | 12.92 | 16.28 |
| Mean aspect ratio (F) | 1.267 | 1.335 |
| Area variation (F) | 0.275 | 0.550 |
| Hausdorff, ‰ of the diagonal (H) | 17.46 | 10.83 |
| Mean distance, ‰ (H) | 1.366 | 1.154 |
| Thin parts: mean distance, % of a quad edge (V) | 6.9 | 3.9 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 0.3 | 0.8 |
| Time, seconds (T) | 2.19 | 2.68 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 100.31 | 100.24 |
| Off budget, % (N) | 2.4 | 1.8 |
| Time, seconds (T) | 1.33 | 1.26 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 |
| Off budget, % (N) | 1.4 | 1.4 |
| Time, seconds (T) | 0.56 | 0.93 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 |
| Off budget, % (N) | 0.6 | 0.6 |
| Time, seconds (T) | 1.00 | 0.89 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.50 | 99.44 |
| Off budget, % (N) | 1.0 | 3.2 |
| Time, seconds (T) | 0.72 | 0.74 |

## terrain (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 36 | 38 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 647 | 661 |
| Alignment, degrees (A) | 17.95 | 18.11 |
| Mean angle deviation, degrees (F) | 5.66 | 5.86 |
| Angle deviation, 95th percentile (F) | 18.16 | 17.01 |
| Mean aspect ratio (F) | 1.290 | 1.303 |
| Area variation (F) | 0.373 | 0.504 |
| Hausdorff, ‰ of the diagonal (H) | 24.30 | 27.60 |
| Mean distance, ‰ (H) | 2.485 | 2.179 |
| Thin parts: mean distance, % of a quad edge (V) | 24.3 | 20.8 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 1.7 | 1.5 |
| Time, seconds (T) | 7.04 | 6.85 |

## mahaveer (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 421 | 427 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 16.19 | 16.98 |
| Mean angle deviation, degrees (F) | 35.08 | 35.59 |
| Angle deviation, 95th percentile (F) | 86.45 | 88.74 |
| Mean aspect ratio (F) | 129.120 | 193.627 |
| Area variation (F) | 8.454 | 11.860 |
| Hausdorff, ‰ of the diagonal (H) | 325.73 | 325.66 |
| Mean distance, ‰ (H) | 40.224 | 40.303 |
| Thin parts: mean distance, % of a quad edge (V) | 154.1 | 154.2 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 105.3 | 100.3 |
| Time, seconds (T) | 8.02 | 9.16 |

## marine (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 5577 | 5589 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | — | — |
| Mean angle deviation, degrees (F) | 26.27 | 26.75 |
| Angle deviation, 95th percentile (F) | 74.74 | 73.36 |
| Mean aspect ratio (F) | 7.329 | 6.246 |
| Area variation (F) | 2.632 | 2.755 |
| Hausdorff, ‰ of the diagonal (H) | 6.26 | 5.58 |
| Mean distance, ‰ (H) | 0.581 | 0.589 |
| Thin parts: mean distance, % of a quad edge (V) | 10.4 | 10.6 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 548.5 | 555.7 |
| Time, seconds (T) | 41.66 | 53.54 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.34 | 99.34 |
| Off budget, % (N) | 1.0 | 1.0 |
| Time, seconds (T) | 0.98 | 2.00 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 98.24 | 98.24 |
| Off budget, % (N) | 0.8 | 0.8 |
| Time, seconds (T) | 1.54 | 1.24 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 13 | 4 |
| Base complex patches (C) | 7 | 5 |
| Alignment, degrees (A) | 3 | 10 |
| Mean angle deviation, degrees (F) | 9 | 4 |
| Mean aspect ratio (F) | 9 | 4 |
| Valid meshes (V) | 11/13 | 11/13 |

### mechanical (4 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 4 | 4 |
| Base complex patches (C) | 3 | 3 |
| Alignment, degrees (A) | 3 | 1 |
| Mean angle deviation, degrees (F) | 4 | 0 |
| Mean aspect ratio (F) | 3 | 1 |
| Valid meshes (V) | 4/4 | 4/4 |
