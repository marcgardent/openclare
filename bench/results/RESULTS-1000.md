# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 36 | 76 | 24 | 24 | 24 | 26 | 26 |
| … of which above the minimum | 28 | 68 | 16 | 16 | 16 | 18 | 18 |
| Base complex patches (C) | 941 | 1036 | 601 | 664 | 790 | 520 | 434 |
| Alignment, degrees (A) | 18.04 | 17.88 | 7.53 | 5.27 | 4.43 | 3.42 | 4.94 |
| Mean angle deviation, degrees (F) | 9.26 | 7.67 | 9.62 | 6.39 | 5.85 | 6.86 | 6.31 |
| Angle deviation, 95th percentile (F) | 25.27 | 23.42 | 24.97 | 16.66 | 14.88 | 20.12 | 18.61 |
| Mean aspect ratio (F) | 1.221 | 1.235 | 1.436 | 1.373 | 1.350 | 2.760 | 1.379 |
| Area variation (F) | 0.218 | 0.218 | 0.385 | 0.453 | 0.463 | 0.625 | 0.353 |
| Hausdorff, ‰ of the diagonal (H) | 14.52 | 15.11 | 17.75 | 10.26 | 6.34 | 7.74 | 19.23 |
| Mean distance, ‰ (H) | 1.325 | 1.340 | 0.706 | 0.540 | 0.596 | 1.172 | 1.534 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.58 | 98.90 | 100.06 | 100.05 | 100.07 | 98.08 | 98.43 |
| Off budget, % (N) | 5.9 | 3.6 | 13.3 | 2.6 | 1.0 | 36.2 | 12.2 |
| Time, seconds (T) | 1.95 | 1.70 | 4.48 | 2.42 | 3.29 | 14.01 | 13.25 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 40 | 26 | 26 | 26 | 90 | 84 |
| … of which above the minimum | 0 | 32 | 18 | 18 | 18 | 82 | 76 |
| Base complex patches (C) | 941 | 912 | 898 | 767 | 981 | 1290 | 837 |
| Alignment, degrees (A) | 23.14 | 23.62 | 16.22 | 17.02 | 16.85 | 13.52 | 14.33 |
| Mean angle deviation, degrees (F) | 6.16 | 6.09 | 9.43 | 5.76 | 4.95 | 11.01 | 11.93 |
| Angle deviation, 95th percentile (F) | 16.14 | 18.18 | 23.72 | 16.05 | 14.03 | 27.61 | 30.02 |
| Mean aspect ratio (F) | 1.162 | 1.188 | 1.321 | 1.257 | 1.228 | 1.631 | 1.472 |
| Area variation (F) | 0.179 | 0.178 | 0.313 | 0.354 | 0.330 | 0.354 | 0.299 |
| Hausdorff, ‰ of the diagonal (H) | 9.38 | 10.32 | 11.07 | 8.83 | 7.73 | 9.88 | 12.74 |
| Mean distance, ‰ (H) | 1.512 | 1.641 | 1.335 | 1.242 | 1.241 | 1.291 | 1.761 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.30 | 99.04 | 99.94 | 100.02 | 99.99 | 99.46 | 98.87 |
| Off budget, % (N) | 5.3 | 3.4 | 10.2 | 2.2 | 1.9 | 29.0 | 16.3 |
| Time, seconds (T) | 1.04 | 1.20 | 4.29 | 1.55 | 2.54 | 28.00 | 16.00 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 16 | 51 | 24 | 26 | 40 | 42 | 39 |
| … of which above the minimum | 8 | 43 | — | 18 | 32 | 34 | 31 |
| Base complex patches (C) | 812 | 822 | 666 | 773 | 355 | 588 | 346 |
| Alignment, degrees (A) | 14.79 | 11.93 | 8.02 | 8.03 | 5.67 | 4.65 | 4.83 |
| Mean angle deviation, degrees (F) | 12.55 | 6.23 | 7.31 | 5.01 | 5.53 | 6.49 | 6.99 |
| Angle deviation, 95th percentile (F) | 30.83 | 18.64 | 22.70 | 15.21 | 15.71 | 18.32 | 21.32 |
| Mean aspect ratio (F) | 1.391 | 1.185 | 1.250 | 1.246 | 1.347 | 1.473 | 1.394 |
| Area variation (F) | 0.298 | 0.183 | 0.288 | 0.352 | 0.481 | 0.461 | 0.380 |
| Hausdorff, ‰ of the diagonal (H) | 40.53 | 18.99 | 12.89 | 5.03 | 3.25 | 3.74 | 6.58 |
| Mean distance, ‰ (H) | 1.486 | 0.917 | 0.516 | 0.465 | 0.499 | 0.604 | 0.932 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 97.34 | 98.23 | — | 100.08 | 100.07 | 98.75 | 98.01 |
| Off budget, % (N) | 18.8 | 1.4 | 8.0 | 3.5 | 0.1 | 20.8 | 7.2 |
| Time, seconds (T) | 1.11 | 1.03 | 3.88 | 1.79 | 2.20 | 11.95 | 8.72 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 64 | 136 | 40 | 48 | 48 | 61 | 49 |
| … of which above the minimum | 32 | 104 | 8 | 16 | 16 | 29 | 17 |
| Base complex patches (C) | 716 | 779 | 96 | 263 | 208 | 741 | 211 |
| Alignment, degrees (A) | 7.66 | 10.19 | 3.01 | 4.06 | 3.17 | 2.70 | 3.17 |
| Mean angle deviation, degrees (F) | 6.96 | 9.36 | 4.97 | 4.72 | 5.00 | 4.46 | 6.04 |
| Angle deviation, 95th percentile (F) | 21.11 | 30.34 | 16.26 | 14.08 | 15.30 | 15.38 | 19.11 |
| Mean aspect ratio (F) | 1.250 | 1.402 | 1.236 | 1.251 | 1.243 | 2.906 | 1.395 |
| Area variation (F) | 0.155 | 0.279 | 0.284 | 0.242 | 0.345 | 0.384 | 0.250 |
| Hausdorff, ‰ of the diagonal (H) | 21.45 | 28.51 | 17.39 | 25.69 | 10.70 | 7.11 | 31.90 |
| Mean distance, ‰ (H) | 1.546 | 2.710 | 1.750 | 2.627 | 1.276 | 0.849 | 2.892 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.12 | 95.62 | 99.67 | 99.47 | 99.93 | 99.30 | 95.23 |
| Off budget, % (N) | 4.3 | 2.6 | 3.0 | 0.1 | 0.1 | 222.4 | 6.6 |
| Time, seconds (T) | 0.39 | 0.57 | 4.72 | 0.98 | 1.64 | 5.08 | 4.00 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 67 | 114 | 51 | 44 | 48 | 68 | 68 |
| … of which above the minimum | — | 98 | — | — | — | — | — |
| Base complex patches (C) | 995 | 930 | 733 | 601 | 554 | 1538 | 802 |
| Alignment, degrees (A) | 15.73 | 17.84 | 11.65 | 11.38 | 10.36 | 8.58 | 9.64 |
| Mean angle deviation, degrees (F) | 9.65 | 10.18 | 9.03 | 4.85 | 5.61 | 8.74 | 8.64 |
| Angle deviation, 95th percentile (F) | 29.23 | 34.26 | 26.04 | 17.33 | 18.44 | 23.66 | 23.37 |
| Mean aspect ratio (F) | 1.325 | 1.423 | 1.311 | 1.231 | 1.303 | 1.823 | 1.345 |
| Area variation (F) | 0.506 | 0.395 | 0.357 | 0.287 | 0.433 | 0.551 | 0.247 |
| Hausdorff, ‰ of the diagonal (H) | 44.79 | 50.97 | 36.86 | 16.89 | 12.92 | 10.72 | 17.45 |
| Mean distance, ‰ (H) | 2.245 | 3.552 | 2.048 | 1.672 | 1.473 | 1.112 | 2.422 |
| Thin parts: mean distance, % of a quad edge (V) | 10.6 | 23.9 | 12.5 | 9.7 | 5.1 | 4.8 | 12.7 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 1.0 | 4.2 | 2.4 | 0.2 | 0.6 | 79.0 | 7.8 |
| Time, seconds (T) | 0.76 | 0.30 | 4.33 | 1.41 | 2.06 | 4.46 | 3.77 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 73 | 114 | 48 | 46 | 102 | 245 | 142 |
| … of which above the minimum | 64 | 99 | 40 | 38 | 94 | 237 | — |
| Base complex patches (C) | 891 | 860 | 555 | 562 | 534 | 2186 | — |
| Alignment, degrees (A) | 20.86 | 18.87 | 11.63 | 10.54 | 11.16 | 7.89 | 10.49 |
| Mean angle deviation, degrees (F) | 14.18 | 9.68 | 11.01 | 7.22 | 9.32 | 9.34 | 12.33 |
| Angle deviation, 95th percentile (F) | 39.23 | 29.97 | 29.05 | 20.66 | 27.54 | 26.48 | 34.04 |
| Mean aspect ratio (F) | 1.552 | 1.419 | 1.646 | 1.693 | 1.726 | 2.393 | 1.743 |
| Area variation (F) | 0.348 | 0.282 | 0.408 | 0.423 | 0.801 | 0.728 | 0.444 |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 85.71 | 37.45 | 29.99 | 29.81 | 27.76 | 51.64 |
| Mean distance, ‰ (H) | 4.418 | 3.773 | 2.195 | 1.614 | 1.802 | 0.968 | 3.165 |
| Thin parts: mean distance, % of a quad edge (V) | 115.0 | 107.2 | 35.3 | 20.5 | 5.8 | 4.5 | 50.1 |
| Volume kept, % of the source (V) | 94.57 | 95.94 | 99.48 | 99.75 | 99.78 | 98.71 | — |
| Off budget, % (N) | 10.9 | 6.8 | 21.7 | 0.2 | 3.7 | 189.0 | 20.1 |
| Time, seconds (T) | 3.33 | 2.71 | 5.13 | 3.44 | 3.09 | 18.93 | 16.67 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 62 | 95 | 59 | 64 | 68 | 78 | 76 |
| … of which above the minimum | 54 | 87 | 51 | 56 | 60 | 70 | 68 |
| Base complex patches (C) | 965 | 982 | 747 | 770 | 721 | 1079 | 630 |
| Alignment, degrees (A) | 18.23 | 17.84 | 11.04 | 9.52 | 7.69 | 6.34 | 7.17 |
| Mean angle deviation, degrees (F) | 7.55 | 7.36 | 11.19 | 7.28 | 6.98 | 8.24 | 8.98 |
| Angle deviation, 95th percentile (F) | 22.42 | 25.61 | 29.29 | 19.78 | 20.30 | 22.62 | 24.90 |
| Mean aspect ratio (F) | 1.252 | 1.267 | 1.447 | 1.390 | 1.397 | 1.689 | 1.455 |
| Area variation (F) | 0.209 | 0.211 | 0.344 | 0.396 | 0.445 | 0.555 | 0.343 |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 45.10 | 22.32 | 12.55 | 15.09 | 8.32 | 15.39 |
| Mean distance, ‰ (H) | 1.330 | 1.556 | 0.824 | 0.659 | 0.616 | 0.810 | 1.403 |
| Thin parts: mean distance, % of a quad edge (V) | 29.6 | 37.4 | 23.2 | 13.4 | 18.7 | 12.2 | 14.9 |
| Volume kept, % of the source (V) | 98.16 | 97.96 | 99.82 | 100.04 | 100.01 | 98.52 | 97.64 |
| Off budget, % (N) | 2.8 | 1.6 | 9.3 | 1.0 | 0.9 | 49.7 | 11.8 |
| Time, seconds (T) | 1.54 | 2.26 | 6.57 | 2.20 | 2.13 | 17.76 | 13.95 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 28 | 69 | 18 | 18 | 20 | 21 | 21 |
| … of which above the minimum | 28 | 69 | 18 | 18 | 20 | 21 | 21 |
| Base complex patches (C) | 958 | 1006 | 177 | 237 | 185 | 398 | 235 |
| Alignment, degrees (A) | 20.00 | 20.88 | 4.19 | 4.29 | 3.60 | 3.16 | 3.69 |
| Mean angle deviation, degrees (F) | 7.93 | 7.12 | 5.39 | 4.49 | 4.49 | 5.39 | 6.25 |
| Angle deviation, 95th percentile (F) | 22.21 | 22.89 | 16.57 | 11.74 | 13.09 | 15.59 | 19.36 |
| Mean aspect ratio (F) | 1.201 | 1.225 | 1.445 | 1.380 | 1.393 | 1.530 | 1.451 |
| Area variation (F) | 0.159 | 0.197 | 0.357 | 0.435 | 0.456 | 0.413 | 0.359 |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 28.68 | 19.08 | 4.86 | 4.33 | 7.54 | 7.25 |
| Mean distance, ‰ (H) | 0.966 | 1.021 | 0.442 | 0.384 | 0.397 | 1.012 | 1.114 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.35 | 98.23 | 100.01 | 100.05 | 100.05 | 97.96 | 97.78 |
| Off budget, % (N) | 2.8 | 2.4 | 13.9 | 0.7 | 2.8 | 4.6 | 10.7 |
| Time, seconds (T) | 1.43 | 1.10 | 3.35 | 3.16 | 2.80 | 12.87 | 16.31 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 44 | 82 | 41 | 40 | 50 | 60 | 53 |
| … of which above the minimum | 32 | 74 | — | 32 | 42 | 52 | 45 |
| Base complex patches (C) | 785 | 980 | — | 321 | 523 | 836 | 440 |
| Alignment, degrees (A) | 16.32 | 17.18 | 9.20 | 8.49 | 9.18 | 4.88 | 6.96 |
| Mean angle deviation, degrees (F) | 10.04 | 7.61 | 8.42 | 4.85 | 5.71 | 6.85 | 8.39 |
| Angle deviation, 95th percentile (F) | 28.85 | 26.75 | 22.60 | 13.95 | 17.12 | 19.46 | 22.44 |
| Mean aspect ratio (F) | 1.336 | 1.285 | 1.301 | 1.309 | 1.353 | 2.117 | 1.414 |
| Area variation (F) | 0.267 | 0.235 | 0.272 | 0.357 | 0.454 | 0.568 | 0.310 |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 37.01 | 37.76 | 22.64 | 17.27 | 8.09 | 29.46 |
| Mean distance, ‰ (H) | 1.731 | 1.524 | 1.075 | 0.839 | 0.608 | 0.521 | 1.283 |
| Thin parts: mean distance, % of a quad edge (V) | 30.4 | 26.6 | 26.5 | 17.9 | 6.6 | 4.0 | 21.3 |
| Volume kept, % of the source (V) | 96.59 | 97.11 | 99.08 | 99.46 | 99.84 | 98.99 | 97.39 |
| Off budget, % (N) | 15.3 | 1.5 | 8.8 | 1.7 | 1.1 | 63.8 | 10.8 |
| Time, seconds (T) | 1.50 | 1.14 | 5.30 | 1.54 | 1.59 | 14.40 | 12.45 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 92 | 94 | 59 | 60 | 92 | 95 | 85 |
| … of which above the minimum | 76 | 75 | — | 52 | 84 | 87 | — |
| Base complex patches (C) | 848 | 861 | — | 541 | 603 | 1149 | — |
| Alignment, degrees (A) | 17.33 | 16.11 | 13.07 | 12.05 | 9.26 | 7.39 | 10.36 |
| Mean angle deviation, degrees (F) | 15.46 | 8.44 | 8.62 | 6.26 | 7.28 | 8.27 | 9.33 |
| Angle deviation, 95th percentile (F) | 46.32 | 30.16 | 25.91 | 20.19 | 22.36 | 23.65 | 27.06 |
| Mean aspect ratio (F) | 1.533 | 1.339 | 1.324 | 1.347 | 1.463 | 2.045 | 1.425 |
| Area variation (F) | 0.437 | 0.272 | 0.276 | 0.318 | 0.877 | 0.753 | 0.323 |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 97.30 | 34.21 | 22.16 | 18.64 | 11.45 | 85.93 |
| Mean distance, ‰ (H) | 4.074 | 5.299 | 1.712 | 1.089 | 1.048 | 0.786 | 2.893 |
| Thin parts: mean distance, % of a quad edge (V) | 84.1 | 119.6 | 24.5 | 13.9 | 4.7 | 3.5 | 50.0 |
| Volume kept, % of the source (V) | 96.89 | 96.53 | 98.61 | 99.33 | 99.73 | 98.97 | 97.29 |
| Off budget, % (N) | 13.9 | 9.0 | 9.7 | 0.9 | 0.9 | 97.5 | 14.7 |
| Time, seconds (T) | 1.55 | 1.07 | 5.45 | 3.03 | 2.23 | 17.10 | 15.43 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 3/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 69 | 108 | 52 | 51 | 83 | 182 | 150 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 724 | 849 | 595 | 605 | 886 | — | — |
| Alignment, degrees (A) | 18.48 | 19.26 | 16.56 | 14.75 | 15.27 | 12.26 | 14.41 |
| Mean angle deviation, degrees (F) | 10.23 | 11.03 | 10.72 | 7.85 | 9.04 | 10.70 | 12.37 |
| Angle deviation, 95th percentile (F) | 28.90 | 36.55 | 28.05 | 21.89 | 27.57 | 30.61 | 34.62 |
| Mean aspect ratio (F) | 1.359 | 1.449 | 1.405 | 1.424 | 1.456 | 2.192 | 1.561 |
| Area variation (F) | 0.306 | 0.338 | 0.288 | 0.323 | 0.766 | 0.752 | 0.389 |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 84.04 | 69.27 | 36.45 | 25.10 | 7.66 | 49.73 |
| Mean distance, ‰ (H) | 4.435 | 6.117 | 4.835 | 2.971 | 2.774 | 0.950 | 3.575 |
| Thin parts: mean distance, % of a quad edge (V) | 28.0 | 42.2 | 38.8 | 22.1 | 14.3 | 4.8 | 18.4 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 10.0 | 11.4 | 17.0 | 0.0 | 0.5 | 209.3 | 12.4 |
| Time, seconds (T) | 0.74 | 0.37 | 36.70 | 1.84 | 1.62 | 6.67 | 10.59 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 164 | 181 | 188 | 715 | 731 | 1682 | 572 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 453 | 500 | — | — | — | — | — |
| Alignment, degrees (A) | 18.41 | 18.59 | 16.86 | 17.39 | 17.37 | 9.46 | 14.37 |
| Mean angle deviation, degrees (F) | 18.81 | 20.14 | 16.04 | 25.38 | 28.76 | 9.03 | 16.83 |
| Angle deviation, 95th percentile (F) | 52.35 | 58.52 | 48.76 | 68.68 | 74.82 | 26.22 | 49.51 |
| Mean aspect ratio (F) | 1.970 | 2.108 | 1.656 | 4.845 | 6.191 | 1.662 | 1.788 |
| Area variation (F) | 0.599 | 0.707 | 0.550 | 1.969 | 2.184 | 1.972 | 1.056 |
| Hausdorff, ‰ of the diagonal (H) | 76.78 | 97.09 | 82.01 | 75.59 | 73.25 | 74.63 | 78.15 |
| Mean distance, ‰ (H) | 13.479 | 14.528 | 8.862 | 6.237 | 8.429 | 1.049 | 5.982 |
| Thin parts: mean distance, % of a quad edge (V) | 64.1 | 71.0 | 43.1 | 29.9 | 40.7 | 5.8 | 28.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 50.6 | 47.0 | 40.4 | 77.3 | 34.3 | 2533.9 | 27.1 |
| Time, seconds (T) | 2.12 | 1.63 | 6.88 | 9.25 | 11.40 | 14.03 | 9.27 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | — | 83 | 27 | 36 | 36 | 76 | 54 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 532 | 171 | 279 | 451 | 1346 | — |
| Alignment, degrees (A) | — | 13.44 | 8.10 | 7.04 | 6.65 | 6.49 | 9.01 |
| Mean angle deviation, degrees (F) | — | 8.67 | 7.81 | 4.24 | 5.58 | 8.08 | 9.87 |
| Angle deviation, 95th percentile (F) | — | 30.33 | 22.68 | 13.37 | 17.08 | 22.80 | 25.01 |
| Mean aspect ratio (F) | — | 1.363 | 1.346 | 1.276 | 1.332 | 2.085 | 1.391 |
| Area variation (F) | — | 0.276 | 0.286 | 0.278 | 0.534 | 0.486 | 0.271 |
| Hausdorff, ‰ of the diagonal (H) | — | 53.13 | 18.46 | 17.46 | 9.08 | 5.71 | 13.65 |
| Mean distance, ‰ (H) | — | 2.255 | 1.134 | 0.887 | 0.817 | 0.640 | 1.473 |
| Thin parts: mean distance, % of a quad edge (V) | — | 48.5 | 8.2 | 4.7 | 2.8 | 3.4 | 5.6 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 15.5 | 8.8 | 0.3 | 0.8 | 110.5 | 6.4 |
| Time, seconds (T) | — | 1.83 | 6.12 | 2.70 | 2.30 | 13.07 | 11.79 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 30 | 115 | 24 | 24 | 24 | 26 | 24 |
| … of which above the minimum | 22 | 107 | 16 | 16 | 16 | 18 | 16 |
| Base complex patches (C) | 227 | 630 | 388 | 86 | 68 | 367 | 293 |
| Alignment, degrees (A) | 13.41 | 14.60 | 12.82 | 12.46 | 10.87 | 11.10 | 10.93 |
| Mean angle deviation, degrees (F) | 3.78 | 8.58 | 4.85 | 2.07 | 2.16 | 4.39 | 4.53 |
| Angle deviation, 95th percentile (F) | 15.04 | 35.19 | 15.54 | 9.04 | 10.18 | 14.60 | 15.17 |
| Mean aspect ratio (F) | 1.191 | 1.407 | 1.208 | 1.219 | 1.177 | 1.323 | 1.314 |
| Area variation (F) | 0.176 | 0.263 | 0.143 | 0.175 | 0.381 | 0.296 | 0.273 |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 20.39 | 17.19 | 14.36 | 1.37 | 1.64 | 2.29 |
| Mean distance, ‰ (H) | 0.335 | 0.238 | 0.679 | 0.259 | 0.165 | 0.147 | 0.173 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.43 | 100.43 | 99.90 | 100.01 | 100.01 | 100.26 | 100.31 |
| Off budget, % (N) | 5.7 | 3.6 | 4.1 | 2.4 | 1.8 | 1.3 | 7.6 |
| Time, seconds (T) | 0.70 | 0.15 | 9.34 | 1.15 | 1.27 | 3.06 | 2.65 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 16 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 8 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 174 | 999 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.17 | 1.29 | 1.55 | 0.11 | 0.25 | 0.11 | 0.11 |
| Mean angle deviation, degrees (F) | 0.08 | 2.07 | 2.64 | 0.00 | 0.33 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 0.23 | 7.03 | 6.80 | 0.01 | 0.76 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.003 | 1.093 | 1.086 | 1.000 | 1.071 | 1.000 | 1.000 |
| Area variation (F) | 0.002 | 0.103 | 0.074 | 0.000 | 0.082 | 0.000 | 0.000 |
| Hausdorff, ‰ of the diagonal (H) | 2.31 | 0.00 | 25.45 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.015 | 0.000 | 0.399 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 | 99.93 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 1.4 | 10.0 | 0.1 | 1.4 | 1.4 | 1.4 | 1.4 |
| Time, seconds (T) | 0.33 | 0.06 | 3.86 | 0.67 | 0.72 | 2.35 | 3.01 |

## lblock (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 12 | 16 | 12 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 4 | 8 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 85 | 84 | 26 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.36 | 1.05 | 1.80 | 0.56 | 0.64 | 0.56 | 0.56 |
| Mean angle deviation, degrees (F) | 1.26 | 1.39 | 2.01 | 0.10 | 0.37 | 0.30 | 0.30 |
| Angle deviation, 95th percentile (F) | 4.73 | 3.73 | 6.27 | 0.33 | 0.90 | 1.26 | 1.26 |
| Mean aspect ratio (F) | 1.064 | 1.041 | 1.120 | 1.072 | 1.099 | 1.090 | 1.090 |
| Area variation (F) | 0.059 | 0.057 | 0.080 | 0.070 | 0.096 | 0.053 | 0.053 |
| Hausdorff, ‰ of the diagonal (H) | 9.75 | 0.00 | 13.72 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.123 | 0.000 | 0.591 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.99 | 100.00 | 99.86 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 7.0 | 9.8 | 0.8 | 0.6 | 0.6 | 1.0 | 1.0 |
| Time, seconds (T) | 0.23 | 0.05 | 3.04 | 1.31 | 0.83 | 2.79 | 2.67 |

## cylinder (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 40 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 32 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 576 | 178 | 6 | 186 | 138 | 138 |
| Alignment, degrees (A) | 3.95 | 4.86 | 1.95 | 1.24 | 1.81 | 1.55 | 1.55 |
| Mean angle deviation, degrees (F) | 4.50 | 5.03 | 4.42 | 2.65 | 3.74 | 4.01 | 4.01 |
| Angle deviation, 95th percentile (F) | 17.64 | 20.26 | 12.92 | 7.69 | 8.53 | 11.66 | 11.66 |
| Mean aspect ratio (F) | 1.124 | 1.170 | 1.309 | 1.286 | 1.510 | 1.528 | 1.528 |
| Area variation (F) | 0.112 | 0.174 | 0.330 | 0.334 | 0.206 | 0.416 | 0.416 |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 15.50 | 1.55 | 1.48 | 1.70 | 2.59 | 2.59 |
| Mean distance, ‰ (H) | 0.373 | 0.376 | 0.341 | 0.311 | 0.356 | 0.938 | 0.938 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.73 | 99.66 | 100.00 | 99.99 | 100.02 | 99.00 | 99.00 |
| Off budget, % (N) | 1.8 | 1.0 | 10.8 | 1.0 | 3.2 | 21.8 | 21.8 |
| Time, seconds (T) | 0.33 | 0.05 | 1.84 | 0.98 | 1.02 | 2.92 | 2.88 |

## terrain (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 44 | 40 | 33 | 36 | 38 | 152 | 88 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 307 | 394 | 262 | 647 | 639 | 1446 | — |
| Alignment, degrees (A) | 19.25 | 18.99 | 17.85 | 17.69 | 17.56 | 18.91 | 20.03 |
| Mean angle deviation, degrees (F) | 6.74 | 8.62 | 9.98 | 5.70 | 6.06 | 11.26 | 10.74 |
| Angle deviation, 95th percentile (F) | 23.83 | 29.38 | 28.26 | 18.94 | 18.44 | 32.77 | 31.81 |
| Mean aspect ratio (F) | 1.223 | 1.338 | 1.384 | 1.288 | 1.328 | 1.859 | 1.521 |
| Area variation (F) | 0.195 | 0.224 | 0.321 | 0.368 | 0.458 | 0.545 | 0.321 |
| Hausdorff, ‰ of the diagonal (H) | 30.58 | 146.80 | 28.90 | 23.68 | 26.86 | 18.41 | 31.35 |
| Mean distance, ‰ (H) | 3.158 | 4.386 | 2.668 | 2.165 | 1.994 | 1.506 | 2.787 |
| Thin parts: mean distance, % of a quad edge (V) | 29.7 | 44.9 | 29.2 | 22.3 | 19.4 | 11.3 | 27.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 12.8 | 42.6 | 11.5 | 1.7 | 0.7 | 54.1 | 12.2 |
| Time, seconds (T) | 19.49 | 8.90 | 11.19 | 6.89 | 7.34 | 19.65 | 21.87 |

## mahaveer (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 124 | 45 | 421 | 421 | 3109 | 1013 |
| … of which above the minimum | — | 13 | — | — | — | — | — |
| Base complex patches (C) | — | 423 | 162 | — | — | — | — |
| Alignment, degrees (A) | — | 11.52 | 7.39 | 17.39 | 17.74 | 15.01 | 14.68 |
| Mean angle deviation, degrees (F) | — | 15.07 | 9.90 | 33.35 | 34.25 | 13.01 | 16.98 |
| Angle deviation, 95th percentile (F) | — | 53.29 | 30.38 | 85.62 | 88.24 | 40.35 | 58.74 |
| Mean aspect ratio (F) | — | 2.321 | 1.531 | 119.626 | 184.152 | 1.768 | 2.072 |
| Area variation (F) | — | 0.516 | 0.416 | 8.169 | 8.177 | 3.743 | 1.645 |
| Hausdorff, ‰ of the diagonal (H) | — | 69.46 | 48.19 | 325.50 | 325.41 | 13.85 | 49.37 |
| Mean distance, ‰ (H) | — | 10.323 | 5.432 | 40.078 | 40.082 | 1.081 | 4.277 |
| Thin parts: mean distance, % of a quad edge (V) | — | 50.2 | 28.2 | 153.5 | 153.5 | 7.4 | 22.4 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 43.8 | 33.8 | 105.3 | 102.1 | 4082.1 | 168.8 |
| Time, seconds (T) | — | 5.58 | 16.18 | 8.14 | 13.72 | 16.61 | 15.33 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 139 | — | 5577 | 5538 | 10625 | 4798 |
| … of which above the minimum | — | -56 | — | — | — | — | — |
| Base complex patches (C) | — | 204 | — | — | — | — | — |
| Alignment, degrees (A) | — | 19.32 | — | — | — | 10.09 | — |
| Mean angle deviation, degrees (F) | — | 30.35 | — | 25.76 | 25.86 | 10.19 | 17.58 |
| Angle deviation, 95th percentile (F) | — | 69.83 | — | 74.07 | 73.42 | 30.61 | 52.92 |
| Mean aspect ratio (F) | — | 3.003 | — | 7.161 | 14.969 | 1.591 | 1.958 |
| Area variation (F) | — | 0.795 | — | 2.571 | 2.396 | 3.347 | 3.276 |
| Hausdorff, ‰ of the diagonal (H) | — | 10.79 | — | 6.31 | 6.31 | 2.06 | 5.15 |
| Mean distance, ‰ (H) | — | 3.973 | — | 0.534 | 0.551 | 0.152 | 0.565 |
| Thin parts: mean distance, % of a quad edge (V) | — | 70.3 | — | 9.5 | 9.7 | 5.2 | 11.1 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 79.6 | — | 548.5 | 536.4 | 20418.5 | 962.3 |
| Time, seconds (T) | — | 3.43 | — | 41.94 | 54.31 | 14.01 | 8.85 |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 545 non-manifold or inconsistently oriented edges.

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 44 | 8 | 8 | 8 | 10 | 10 |
| … of which above the minimum | 0 | 36 | 0 | 0 | 0 | 2 | 2 |
| Base complex patches (C) | 937 | 1066 | 923 | 1010 | 1010 | 699 | 699 |
| Alignment, degrees (A) | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.18 | 8.36 | 4.61 | 4.61 | 6.03 | 6.03 |
| Angle deviation, 95th percentile (F) | 12.31 | 17.01 | 22.46 | 11.13 | 11.13 | 14.57 | 14.57 |
| Mean aspect ratio (F) | 1.148 | 1.153 | 1.171 | 1.180 | 1.180 | 1.213 | 1.213 |
| Area variation (F) | 0.136 | 0.179 | 0.152 | 0.225 | 0.225 | 0.318 | 0.318 |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 1.68 | 1.11 | 0.99 | 0.99 | 1.91 | 1.91 |
| Mean distance, ‰ (H) | 0.670 | 0.632 | 0.356 | 0.332 | 0.332 | 0.762 | 0.762 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.38 | 99.38 | 100.02 | 100.02 | 100.02 | 99.24 | 99.24 |
| Off budget, % (N) | 5.0 | 6.6 | 7.7 | 1.0 | 1.0 | 8.5 | 8.5 |
| Time, seconds (T) | 0.41 | 0.19 | 2.04 | 0.76 | 1.41 | 4.77 | 3.75 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 20 | 90 | 0 | 0 | 0 | 0 | 0 |
| … of which above the minimum | 20 | 90 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1016 | 932 | 1 | 1 | 1 | 1 | 1 |
| Alignment, degrees (A) | 13.64 | 8.78 | 0.63 | 0.16 | 0.16 | 0.63 | 0.63 |
| Mean angle deviation, degrees (F) | 7.73 | 8.51 | 2.51 | 2.40 | 2.40 | 2.49 | 2.49 |
| Angle deviation, 95th percentile (F) | 18.19 | 26.44 | 3.93 | 3.73 | 3.73 | 3.93 | 3.93 |
| Mean aspect ratio (F) | 1.178 | 1.273 | 1.362 | 1.295 | 1.295 | 1.372 | 1.372 |
| Area variation (F) | 0.180 | 0.237 | 0.294 | 0.350 | 0.351 | 0.280 | 0.280 |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 3.96 | 1.51 | 1.35 | 1.35 | 2.60 | 2.60 |
| Mean distance, ‰ (H) | 0.775 | 0.849 | 0.447 | 0.424 | 0.424 | 1.127 | 1.127 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.57 | 98.28 | 100.06 | 100.04 | 100.04 | 97.62 | 97.62 |
| Off budget, % (N) | 2.8 | 5.4 | 8.0 | 0.8 | 0.8 | 8.0 | 8.0 |
| Time, seconds (T) | 0.31 | 0.27 | 0.99 | 0.92 | 1.15 | 4.41 | 2.61 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 6 | 6 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 4 | 3 | 2 | 0 | 3 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 13 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 8 | 3 | 2 | 0 |
| Mean aspect ratio (F) | 5 | 3 | 3 | 2 | 0 | 0 | 0 |
| Valid meshes (V) | 38/65 | 44/65 | 8/13 | 12/13 | 12/13 | 11/13 | 8/13 |

### mechanical (4 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 4 | 4 | 4 | 3 | 4 |
| Base complex patches (C) | 1 | 0 | 0 | 3 | 3 | 2 | 2 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 1 | 2 | 2 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 3 | 0 | 1 | 1 |
| Mean aspect ratio (F) | 1 | 1 | 0 | 0 | 1 | 1 | 1 |
| Valid meshes (V) | 20/20 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
