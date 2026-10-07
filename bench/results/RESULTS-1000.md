# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 36 | 76 | 24 | 24 | 24 | 26 | 26 |
| … of which above the minimum | 28 | 68 | 16 | 16 | 16 | 18 | 18 |
| Base complex patches (C) | 941 | 1036 | 601 | 664 | 802 | 520 | 434 |
| Alignment, degrees (A) | 18.04 | 17.88 | 7.54 | 5.25 | 4.72 | 3.42 | 4.94 |
| Mean angle deviation, degrees (F) | 9.26 | 7.67 | 9.61 | 6.36 | 5.67 | 6.86 | 6.31 |
| Angle deviation, 95th percentile (F) | 25.27 | 23.42 | 24.91 | 16.63 | 14.46 | 20.12 | 18.61 |
| Mean aspect ratio (F) | 1.221 | 1.235 | 1.434 | 1.372 | 1.356 | 2.760 | 1.379 |
| Area variation (F) | 0.218 | 0.218 | 0.385 | 0.454 | 0.469 | 0.625 | 0.353 |
| Hausdorff, ‰ of the diagonal (H) | 14.52 | 15.11 | 21.30 | 13.79 | 12.28 | 7.74 | 19.23 |
| Mean distance, ‰ (H) | 1.325 | 1.340 | 1.564 | 1.294 | 1.363 | 1.172 | 1.534 |
| Off budget, % (N) | 5.9 | 3.6 | 13.3 | 2.6 | 0.2 | 36.2 | 12.2 |
| Time, seconds (T) | 2.00 | 1.40 | 3.77 | 1.42 | 1.72 | 14.01 | 13.25 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 40 | 26 | 26 | 26 | 90 | 84 |
| … of which above the minimum | 0 | 32 | 18 | 18 | 18 | 82 | 76 |
| Base complex patches (C) | 941 | 912 | 898 | 767 | 981 | 1290 | 837 |
| Alignment, degrees (A) | 23.14 | 23.62 | 16.20 | 16.99 | 16.76 | 13.52 | 14.33 |
| Mean angle deviation, degrees (F) | 6.16 | 6.09 | 9.42 | 5.63 | 4.82 | 11.01 | 11.93 |
| Angle deviation, 95th percentile (F) | 16.14 | 18.18 | 23.76 | 15.95 | 13.84 | 27.61 | 30.02 |
| Mean aspect ratio (F) | 1.162 | 1.188 | 1.318 | 1.253 | 1.226 | 1.631 | 1.472 |
| Area variation (F) | 0.179 | 0.178 | 0.313 | 0.355 | 0.331 | 0.354 | 0.299 |
| Hausdorff, ‰ of the diagonal (H) | 9.38 | 10.32 | 10.57 | 9.57 | 9.06 | 9.88 | 12.74 |
| Mean distance, ‰ (H) | 1.512 | 1.641 | 1.705 | 1.510 | 1.522 | 1.291 | 1.761 |
| Off budget, % (N) | 5.3 | 3.4 | 10.2 | 2.2 | 1.9 | 29.0 | 16.3 |
| Time, seconds (T) | 1.15 | 1.00 | 3.75 | 1.80 | 1.49 | 28.00 | 16.00 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 16 | 51 | 24 | 26 | 40 | 42 | 39 |
| … of which above the minimum | 8 | 43 | — | 18 | 32 | 34 | 31 |
| Base complex patches (C) | 812 | 822 | 666 | 773 | 355 | 588 | 346 |
| Alignment, degrees (A) | 14.79 | 11.93 | 8.00 | 8.01 | 5.68 | 4.65 | 4.83 |
| Mean angle deviation, degrees (F) | 12.55 | 6.23 | 7.20 | 4.90 | 5.45 | 6.49 | 6.99 |
| Angle deviation, 95th percentile (F) | 30.83 | 18.64 | 22.31 | 14.91 | 15.55 | 18.32 | 21.32 |
| Mean aspect ratio (F) | 1.391 | 1.185 | 1.245 | 1.240 | 1.335 | 1.473 | 1.394 |
| Area variation (F) | 0.298 | 0.183 | 0.289 | 0.354 | 0.487 | 0.461 | 0.380 |
| Hausdorff, ‰ of the diagonal (H) | 40.53 | 18.99 | 12.89 | 8.03 | 5.03 | 3.74 | 6.58 |
| Mean distance, ‰ (H) | 1.486 | 0.917 | 0.958 | 0.855 | 0.928 | 0.604 | 0.932 |
| Off budget, % (N) | 18.8 | 1.4 | 8.0 | 3.5 | 0.1 | 20.8 | 7.2 |
| Time, seconds (T) | 1.19 | 0.87 | 3.62 | 2.37 | 1.69 | 11.95 | 8.72 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 64 | 136 | 40 | 48 | 48 | 61 | 49 |
| … of which above the minimum | 32 | 104 | 8 | 16 | 16 | 29 | 17 |
| Base complex patches (C) | 716 | 779 | 96 | 263 | 208 | 741 | 211 |
| Alignment, degrees (A) | 7.66 | 10.19 | 3.06 | 4.09 | 3.16 | 2.70 | 3.17 |
| Mean angle deviation, degrees (F) | 6.96 | 9.36 | 4.94 | 4.64 | 4.99 | 4.46 | 6.04 |
| Angle deviation, 95th percentile (F) | 21.11 | 30.34 | 16.31 | 13.90 | 15.32 | 15.38 | 19.11 |
| Mean aspect ratio (F) | 1.250 | 1.402 | 1.249 | 1.252 | 1.248 | 2.906 | 1.395 |
| Area variation (F) | 0.155 | 0.279 | 0.276 | 0.236 | 0.342 | 0.384 | 0.250 |
| Hausdorff, ‰ of the diagonal (H) | 21.45 | 28.51 | 27.15 | 29.70 | 15.07 | 7.11 | 31.90 |
| Mean distance, ‰ (H) | 1.546 | 2.710 | 2.340 | 3.002 | 1.726 | 0.849 | 2.892 |
| Off budget, % (N) | 4.3 | 2.6 | 3.0 | 0.1 | 0.1 | 222.4 | 6.6 |
| Time, seconds (T) | 0.46 | 0.28 | 2.97 | 1.43 | 1.63 | 5.08 | 4.00 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 67 | 114 | 51 | 43 | 48 | 68 | 68 |
| … of which above the minimum | — | 98 | — | — | — | — | — |
| Base complex patches (C) | 995 | 930 | 733 | 645 | 648 | 1538 | 802 |
| Alignment, degrees (A) | 15.73 | 17.84 | 11.98 | 11.13 | 10.35 | 8.58 | 9.64 |
| Mean angle deviation, degrees (F) | 9.65 | 10.18 | 9.12 | 4.74 | 5.24 | 8.74 | 8.64 |
| Angle deviation, 95th percentile (F) | 29.23 | 34.26 | 26.80 | 17.18 | 17.95 | 23.66 | 23.37 |
| Mean aspect ratio (F) | 1.325 | 1.423 | 1.313 | 1.233 | 1.251 | 1.823 | 1.345 |
| Area variation (F) | 0.506 | 0.395 | 0.364 | 0.309 | 0.422 | 0.551 | 0.247 |
| Hausdorff, ‰ of the diagonal (H) | 44.79 | 50.97 | 36.86 | 17.80 | 13.10 | 10.72 | 17.45 |
| Mean distance, ‰ (H) | 2.245 | 3.552 | 2.550 | 2.242 | 1.977 | 1.112 | 2.422 |
| Off budget, % (N) | 1.0 | 4.2 | 2.4 | 0.1 | 0.6 | 79.0 | 7.8 |
| Time, seconds (T) | 0.69 | 0.20 | 4.11 | 2.52 | 1.60 | 4.46 | 3.77 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 73 | 114 | 48 | 46 | 44 | 245 | 142 |
| … of which above the minimum | 64 | 99 | 40 | 38 | 36 | 237 | — |
| Base complex patches (C) | 891 | 860 | 555 | 562 | 436 | 2186 | — |
| Alignment, degrees (A) | 20.86 | 18.87 | 11.68 | 10.54 | 9.98 | 7.89 | 10.49 |
| Mean angle deviation, degrees (F) | 14.18 | 9.68 | 10.93 | 6.95 | 7.53 | 9.34 | 12.33 |
| Angle deviation, 95th percentile (F) | 39.23 | 29.97 | 29.20 | 19.86 | 20.80 | 26.48 | 34.04 |
| Mean aspect ratio (F) | 1.552 | 1.419 | 1.614 | 1.666 | 1.664 | 2.393 | 1.743 |
| Area variation (F) | 0.348 | 0.282 | 0.414 | 0.424 | 0.551 | 0.728 | 0.444 |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 85.71 | 38.90 | 28.57 | 31.45 | 27.76 | 51.64 |
| Mean distance, ‰ (H) | 4.418 | 3.773 | 3.083 | 2.188 | 2.240 | 0.968 | 3.165 |
| Off budget, % (N) | 10.9 | 6.8 | 21.7 | 0.2 | 0.4 | 189.0 | 20.1 |
| Time, seconds (T) | 3.03 | 2.76 | 4.70 | 2.43 | 3.51 | 18.93 | 16.67 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 62 | 95 | 59 | 64 | 66 | 78 | 76 |
| … of which above the minimum | 54 | 87 | 51 | 56 | 58 | 70 | 68 |
| Base complex patches (C) | 965 | 982 | 747 | 770 | 681 | 1079 | 630 |
| Alignment, degrees (A) | 18.23 | 17.84 | 11.11 | 9.52 | 8.16 | 6.34 | 7.17 |
| Mean angle deviation, degrees (F) | 7.55 | 7.36 | 11.16 | 7.21 | 7.54 | 8.24 | 8.98 |
| Angle deviation, 95th percentile (F) | 22.42 | 25.61 | 29.32 | 19.51 | 21.69 | 22.62 | 24.90 |
| Mean aspect ratio (F) | 1.252 | 1.267 | 1.440 | 1.378 | 1.367 | 1.689 | 1.455 |
| Area variation (F) | 0.209 | 0.211 | 0.353 | 0.406 | 0.519 | 0.555 | 0.343 |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 45.10 | 27.21 | 12.51 | 10.17 | 8.32 | 15.39 |
| Mean distance, ‰ (H) | 1.330 | 1.556 | 1.540 | 1.287 | 1.249 | 0.810 | 1.403 |
| Off budget, % (N) | 2.8 | 1.6 | 9.3 | 1.0 | 0.3 | 49.7 | 11.8 |
| Time, seconds (T) | 1.44 | 1.04 | 4.08 | 2.10 | 3.20 | 17.76 | 13.95 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 28 | 69 | 18 | 18 | 20 | 21 | 21 |
| … of which above the minimum | 28 | 69 | 18 | 18 | 20 | 21 | 21 |
| Base complex patches (C) | 958 | 1006 | 177 | 237 | 185 | 398 | 235 |
| Alignment, degrees (A) | 20.00 | 20.88 | 4.17 | 4.27 | 3.58 | 3.16 | 3.69 |
| Mean angle deviation, degrees (F) | 7.93 | 7.12 | 5.36 | 4.47 | 4.46 | 5.39 | 6.25 |
| Angle deviation, 95th percentile (F) | 22.21 | 22.89 | 16.25 | 11.60 | 12.91 | 15.59 | 19.36 |
| Mean aspect ratio (F) | 1.201 | 1.225 | 1.446 | 1.382 | 1.395 | 1.530 | 1.451 |
| Area variation (F) | 0.159 | 0.197 | 0.354 | 0.431 | 0.452 | 0.413 | 0.359 |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 28.68 | 23.70 | 8.11 | 5.48 | 7.54 | 7.25 |
| Mean distance, ‰ (H) | 0.966 | 1.021 | 1.121 | 1.002 | 1.046 | 1.012 | 1.114 |
| Off budget, % (N) | 2.8 | 2.4 | 13.9 | 0.7 | 2.8 | 4.6 | 10.7 |
| Time, seconds (T) | 1.52 | 1.07 | 2.77 | 1.27 | 2.41 | 12.87 | 16.31 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 44 | 82 | 41 | 40 | 40 | 60 | 53 |
| … of which above the minimum | 32 | 74 | — | 32 | 32 | 52 | 45 |
| Base complex patches (C) | 785 | 980 | — | 321 | 406 | 836 | 440 |
| Alignment, degrees (A) | 16.32 | 17.18 | 9.39 | 8.65 | 8.58 | 4.88 | 6.96 |
| Mean angle deviation, degrees (F) | 10.04 | 7.61 | 8.51 | 4.73 | 4.97 | 6.85 | 8.39 |
| Angle deviation, 95th percentile (F) | 28.85 | 26.75 | 22.97 | 13.69 | 14.59 | 19.46 | 22.44 |
| Mean aspect ratio (F) | 1.336 | 1.285 | 1.313 | 1.312 | 1.345 | 2.117 | 1.414 |
| Area variation (F) | 0.267 | 0.235 | 0.273 | 0.349 | 0.426 | 0.568 | 0.310 |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 37.01 | 39.08 | 21.55 | 19.38 | 8.09 | 29.46 |
| Mean distance, ‰ (H) | 1.731 | 1.524 | 1.583 | 1.274 | 1.098 | 0.521 | 1.283 |
| Off budget, % (N) | 15.3 | 1.5 | 8.8 | 1.7 | 0.2 | 63.8 | 10.8 |
| Time, seconds (T) | 1.60 | 1.25 | 2.91 | 3.25 | 2.04 | 14.40 | 12.45 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 92 | 94 | 59 | 60 | 56 | 95 | 85 |
| … of which above the minimum | 76 | 75 | — | 52 | 48 | 87 | — |
| Base complex patches (C) | 848 | 861 | — | 541 | 625 | 1149 | — |
| Alignment, degrees (A) | 17.33 | 16.11 | 12.70 | 11.72 | 10.94 | 7.39 | 10.36 |
| Mean angle deviation, degrees (F) | 15.46 | 8.44 | 8.46 | 5.95 | 5.90 | 8.27 | 9.33 |
| Angle deviation, 95th percentile (F) | 46.32 | 30.16 | 25.59 | 18.82 | 19.14 | 23.65 | 27.06 |
| Mean aspect ratio (F) | 1.533 | 1.339 | 1.336 | 1.342 | 1.344 | 2.045 | 1.425 |
| Area variation (F) | 0.437 | 0.272 | 0.296 | 0.325 | 0.500 | 0.753 | 0.323 |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 97.30 | 38.46 | 23.62 | 33.26 | 11.45 | 85.93 |
| Mean distance, ‰ (H) | 4.074 | 5.299 | 2.422 | 1.664 | 1.723 | 0.786 | 2.893 |
| Off budget, % (N) | 13.9 | 9.0 | 9.7 | 0.9 | 2.5 | 97.5 | 14.7 |
| Time, seconds (T) | 1.64 | 1.14 | 3.95 | 2.37 | 2.06 | 17.10 | 15.43 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 3/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 69 | 108 | 52 | 51 | 50 | 182 | 150 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 724 | 849 | 595 | 605 | 714 | — | — |
| Alignment, degrees (A) | 18.48 | 19.26 | 15.86 | 14.76 | 15.37 | 12.26 | 14.41 |
| Mean angle deviation, degrees (F) | 10.23 | 11.03 | 10.73 | 7.50 | 7.46 | 10.70 | 12.37 |
| Angle deviation, 95th percentile (F) | 28.90 | 36.55 | 28.43 | 20.89 | 21.35 | 30.61 | 34.62 |
| Mean aspect ratio (F) | 1.359 | 1.449 | 1.411 | 1.419 | 1.368 | 2.192 | 1.561 |
| Area variation (F) | 0.306 | 0.338 | 0.305 | 0.328 | 0.532 | 0.752 | 0.389 |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 84.04 | 65.61 | 35.79 | 33.89 | 7.66 | 49.73 |
| Mean distance, ‰ (H) | 4.435 | 6.117 | 5.584 | 3.338 | 3.082 | 0.950 | 3.575 |
| Off budget, % (N) | 10.0 | 11.4 | 17.0 | 0.0 | 1.1 | 209.3 | 12.4 |
| Time, seconds (T) | 0.82 | 0.41 | 34.09 | 1.04 | 1.11 | 6.67 | 10.59 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 164 | 181 | 188 | 188 | 188 | 1682 | 572 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 452 | 500 | — | — | — | — | — |
| Alignment, degrees (A) | 18.48 | 18.59 | 16.17 | 16.17 | 16.17 | 9.46 | 14.37 |
| Mean angle deviation, degrees (F) | 18.62 | 20.14 | 16.33 | 16.33 | 16.33 | 9.03 | 16.83 |
| Angle deviation, 95th percentile (F) | 51.84 | 58.52 | 48.79 | 48.79 | 48.79 | 26.22 | 49.51 |
| Mean aspect ratio (F) | 1.978 | 2.108 | 1.690 | 1.690 | 1.690 | 1.662 | 1.788 |
| Area variation (F) | 0.598 | 0.707 | 0.550 | 0.550 | 0.550 | 1.972 | 1.056 |
| Hausdorff, ‰ of the diagonal (H) | 78.38 | 97.09 | 81.53 | 81.53 | 81.53 | 74.63 | 78.15 |
| Mean distance, ‰ (H) | 13.607 | 14.528 | 9.634 | 9.634 | 9.634 | 1.049 | 5.982 |
| Off budget, % (N) | 50.7 | 47.0 | 40.4 | 40.4 | 40.4 | 2533.9 | 27.1 |
| Time, seconds (T) | 2.29 | 1.66 | 7.48 | 6.47 | 7.08 | 14.03 | 9.27 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | — | 83 | 27 | 36 | 36 | 76 | 54 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 532 | 171 | 237 | 303 | 1346 | — |
| Alignment, degrees (A) | — | 13.44 | 8.19 | 6.89 | 6.59 | 6.49 | 9.01 |
| Mean angle deviation, degrees (F) | — | 8.67 | 7.70 | 3.94 | 4.72 | 8.08 | 9.87 |
| Angle deviation, 95th percentile (F) | — | 30.33 | 22.51 | 12.63 | 14.56 | 22.80 | 25.01 |
| Mean aspect ratio (F) | — | 1.363 | 1.330 | 1.268 | 1.299 | 2.085 | 1.391 |
| Area variation (F) | — | 0.276 | 0.280 | 0.278 | 0.494 | 0.486 | 0.271 |
| Hausdorff, ‰ of the diagonal (H) | — | 53.13 | 22.61 | 17.46 | 17.47 | 5.71 | 13.65 |
| Mean distance, ‰ (H) | — | 2.255 | 1.692 | 1.368 | 1.256 | 0.640 | 1.473 |
| Off budget, % (N) | — | 15.5 | 8.8 | 0.8 | 1.8 | 110.5 | 6.4 |
| Time, seconds (T) | — | 1.96 | 5.93 | 1.99 | 2.52 | 13.07 | 11.79 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 30 | 115 | 24 | 24 | 24 | 26 | 24 |
| … of which above the minimum | 22 | 107 | 16 | 16 | 16 | 18 | 16 |
| Base complex patches (C) | 227 | 630 | 388 | 86 | 68 | 367 | 293 |
| Alignment, degrees (A) | 13.41 | 14.60 | 12.82 | 12.36 | 10.83 | 11.10 | 10.93 |
| Mean angle deviation, degrees (F) | 3.78 | 8.58 | 4.84 | 2.08 | 2.16 | 4.39 | 4.53 |
| Angle deviation, 95th percentile (F) | 15.04 | 35.19 | 15.42 | 9.09 | 10.18 | 14.60 | 15.17 |
| Mean aspect ratio (F) | 1.191 | 1.407 | 1.212 | 1.221 | 1.178 | 1.323 | 1.314 |
| Area variation (F) | 0.176 | 0.263 | 0.147 | 0.176 | 0.384 | 0.296 | 0.273 |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 20.39 | 19.95 | 15.02 | 1.66 | 1.64 | 2.29 |
| Mean distance, ‰ (H) | 0.335 | 0.238 | 0.547 | 0.246 | 0.164 | 0.147 | 0.173 |
| Off budget, % (N) | 5.7 | 3.6 | 4.1 | 2.4 | 1.8 | 1.3 | 7.6 |
| Time, seconds (T) | 0.70 | 0.16 | 10.52 | 0.87 | 0.79 | 3.06 | 2.65 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 16 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 8 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 174 | 999 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.17 | 1.29 | 1.51 | 0.11 | 0.25 | 0.11 | 0.11 |
| Mean angle deviation, degrees (F) | 0.08 | 2.07 | 2.64 | 0.00 | 0.33 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 0.23 | 7.03 | 6.66 | 0.01 | 0.76 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.003 | 1.093 | 1.090 | 1.000 | 1.071 | 1.000 | 1.000 |
| Area variation (F) | 0.002 | 0.103 | 0.079 | 0.000 | 0.082 | 0.000 | 0.000 |
| Hausdorff, ‰ of the diagonal (H) | 2.31 | 0.00 | 26.05 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.015 | 0.000 | 0.268 | 0.000 | 0.000 | 0.000 | 0.000 |
| Off budget, % (N) | 1.4 | 10.0 | 0.1 | 1.4 | 1.4 | 1.4 | 1.4 |
| Time, seconds (T) | 0.35 | 0.05 | 2.42 | 0.74 | 0.55 | 2.35 | 3.01 |

## lblock (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 12 | 16 | 12 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 4 | 8 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 85 | 84 | 26 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.36 | 1.05 | 1.76 | 0.56 | 0.65 | 0.56 | 0.56 |
| Mean angle deviation, degrees (F) | 1.26 | 1.39 | 2.01 | 0.10 | 0.37 | 0.30 | 0.30 |
| Angle deviation, 95th percentile (F) | 4.73 | 3.73 | 6.16 | 0.33 | 0.89 | 1.26 | 1.26 |
| Mean aspect ratio (F) | 1.064 | 1.041 | 1.131 | 1.072 | 1.099 | 1.090 | 1.090 |
| Area variation (F) | 0.059 | 0.057 | 0.087 | 0.070 | 0.096 | 0.053 | 0.053 |
| Hausdorff, ‰ of the diagonal (H) | 9.75 | 0.00 | 16.92 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.123 | 0.000 | 0.396 | 0.000 | 0.000 | 0.000 | 0.000 |
| Off budget, % (N) | 7.0 | 9.8 | 0.8 | 0.6 | 0.6 | 1.0 | 1.0 |
| Time, seconds (T) | 0.38 | 0.05 | 2.89 | 0.68 | 0.59 | 2.79 | 2.67 |

## cylinder (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 40 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 32 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 576 | 178 | 6 | 186 | 138 | 138 |
| Alignment, degrees (A) | 3.95 | 4.86 | 1.95 | 1.24 | 1.81 | 1.55 | 1.55 |
| Mean angle deviation, degrees (F) | 4.50 | 5.03 | 4.41 | 2.65 | 3.73 | 4.01 | 4.01 |
| Angle deviation, 95th percentile (F) | 17.64 | 20.26 | 12.92 | 7.69 | 8.53 | 11.66 | 11.66 |
| Mean aspect ratio (F) | 1.124 | 1.170 | 1.307 | 1.284 | 1.507 | 1.528 | 1.528 |
| Area variation (F) | 0.112 | 0.174 | 0.329 | 0.334 | 0.205 | 0.416 | 0.416 |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 15.50 | 1.60 | 1.49 | 1.71 | 2.59 | 2.59 |
| Mean distance, ‰ (H) | 0.373 | 0.376 | 0.544 | 0.485 | 0.582 | 0.938 | 0.938 |
| Off budget, % (N) | 1.8 | 1.0 | 10.8 | 1.0 | 3.2 | 21.8 | 21.8 |
| Time, seconds (T) | 0.31 | 0.06 | 2.22 | 0.64 | 0.99 | 2.92 | 2.88 |

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 44 | 8 | 8 | 8 | 10 | 10 |
| … of which above the minimum | 0 | 36 | 0 | 0 | 0 | 2 | 2 |
| Base complex patches (C) | 937 | 1066 | 923 | 1010 | 1010 | 699 | 699 |
| Alignment, degrees (A) | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.18 | 8.36 | 4.61 | 4.61 | 6.03 | 6.03 |
| Angle deviation, 95th percentile (F) | 12.31 | 17.01 | 22.47 | 11.14 | 11.12 | 14.57 | 14.57 |
| Mean aspect ratio (F) | 1.148 | 1.153 | 1.171 | 1.180 | 1.180 | 1.213 | 1.213 |
| Area variation (F) | 0.136 | 0.179 | 0.151 | 0.224 | 0.224 | 0.318 | 0.318 |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 1.68 | 2.00 | 1.67 | 1.67 | 1.91 | 1.91 |
| Mean distance, ‰ (H) | 0.670 | 0.632 | 0.726 | 0.670 | 0.670 | 0.762 | 0.762 |
| Off budget, % (N) | 5.0 | 6.6 | 7.7 | 1.0 | 1.0 | 8.5 | 8.5 |
| Time, seconds (T) | 0.48 | 0.05 | 1.23 | 0.62 | 0.93 | 4.77 | 3.75 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 20 | 90 | 0 | 0 | 0 | 0 | 0 |
| … of which above the minimum | 20 | 90 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1016 | 932 | 1 | 1 | 1 | 1 | 1 |
| Alignment, degrees (A) | 13.64 | 8.78 | 0.63 | 0.63 | 0.63 | 0.63 | 0.63 |
| Mean angle deviation, degrees (F) | 7.73 | 8.51 | 2.51 | 2.51 | 2.51 | 2.49 | 2.49 |
| Angle deviation, 95th percentile (F) | 18.19 | 26.44 | 3.93 | 3.93 | 3.93 | 3.93 | 3.93 |
| Mean aspect ratio (F) | 1.178 | 1.273 | 1.363 | 1.363 | 1.363 | 1.372 | 1.372 |
| Area variation (F) | 0.180 | 0.237 | 0.288 | 0.288 | 0.288 | 0.280 | 0.280 |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 3.96 | 2.22 | 2.22 | 2.22 | 2.60 | 2.60 |
| Mean distance, ‰ (H) | 0.775 | 0.849 | 0.946 | 0.946 | 0.946 | 1.127 | 1.127 |
| Off budget, % (N) | 2.8 | 5.4 | 8.0 | 8.0 | 8.0 | 8.0 | 8.0 |
| Time, seconds (T) | 0.35 | 0.07 | 0.89 | 0.95 | 0.60 | 4.41 | 2.61 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 5 | 4 | 5 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 4 | 4 | 1 | 0 | 3 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 13 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 6 | 5 | 2 | 0 |
| Mean aspect ratio (F) | 5 | 3 | 1 | 2 | 1 | 1 | 0 |
| Valid meshes (V) | 38/65 | 44/65 | 8/13 | 11/13 | 12/13 | 11/13 | 8/13 |

### mechanical (4 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 4 | 4 | 4 | 3 | 4 |
| Base complex patches (C) | 1 | 0 | 0 | 3 | 3 | 2 | 2 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 1 | 2 | 2 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 3 | 0 | 1 | 1 |
| Mean aspect ratio (F) | 1 | 1 | 0 | 0 | 1 | 1 | 1 |
| Valid meshes (V) | 20/20 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
