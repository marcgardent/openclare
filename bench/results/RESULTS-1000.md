# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `quad-layout-matched` is `quad-layout-adaptive` asked for that same number.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 941 | 1036 | 867 | 1026 | 990 | 1362 | 878 | 1362 | 1340 |
| Irregular vertices (S) | 36 | 76 | 24 | 24 | 24 | 26 | 26 | 26 | 24 |
| … of which above the minimum | 28 | 68 | 16 | 16 | 16 | 18 | 18 | 18 | 16 |
| Base complex patches (C) | 941 | 1036 | 601 | 664 | 790 | 520 | 434 | 520 | 1148 |
| Alignment, degrees (A) | 18.04 | 17.88 | 7.53 | 5.27 | 4.43 | 3.42 | 4.94 | 3.46 | 4.87 |
| Mean angle deviation, degrees (F) | 9.26 | 7.67 | 9.62 | 6.39 | 5.85 | 6.86 | 6.31 | 6.86 | 5.86 |
| Angle deviation, 95th percentile (F) | 25.27 | 23.42 | 24.97 | 16.66 | 14.88 | 20.12 | 18.61 | 20.12 | 15.76 |
| Mean aspect ratio (F) | 1.221 | 1.235 | 1.436 | 1.373 | 1.350 | 2.760 | 1.379 | 2.760 | 1.346 |
| Area variation (F) | 0.218 | 0.218 | 0.385 | 0.453 | 0.463 | 0.625 | 0.353 | 0.625 | 0.434 |
| Hausdorff, ‰ of the diagonal (H) | 14.52 | 15.11 | 17.75 | 10.26 | 6.34 | 7.74 | 19.23 | 7.74 | 4.33 |
| Mean distance, ‰ (H) | 1.325 | 1.340 | 0.706 | 0.540 | 0.596 | 1.172 | 1.534 | 1.172 | 0.396 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.58 | 98.90 | 100.06 | 100.05 | 100.07 | 98.08 | 98.43 | 98.08 | 100.04 |
| Off budget, % (N) | 5.9 | 3.6 | 13.3 | 2.6 | 1.0 | 36.2 | 12.2 | 0.0 | 1.6 |
| Time, seconds (T) | 1.95 | 1.70 | 4.48 | 2.42 | 2.03 | 14.01 | 13.25 | 14.01 | 2.35 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 947 | 966 | 898 | 978 | 981 | 1290 | 837 | 1290 | 1298 |
| Irregular vertices (S) | 8 | 40 | 26 | 26 | 26 | 90 | 84 | 90 | 34 |
| … of which above the minimum | 0 | 32 | 18 | 18 | 18 | 82 | 76 | 82 | 26 |
| Base complex patches (C) | 941 | 912 | 898 | 767 | 981 | 1290 | 837 | 1290 | 1298 |
| Alignment, degrees (A) | 23.14 | 23.62 | 16.22 | 17.02 | 16.85 | 13.52 | 14.33 | 14.01 | 16.65 |
| Mean angle deviation, degrees (F) | 6.16 | 6.09 | 9.43 | 5.76 | 4.95 | 11.01 | 11.93 | 11.01 | 5.62 |
| Angle deviation, 95th percentile (F) | 16.14 | 18.18 | 23.72 | 16.05 | 14.03 | 27.61 | 30.02 | 27.61 | 15.70 |
| Mean aspect ratio (F) | 1.162 | 1.188 | 1.321 | 1.257 | 1.228 | 1.631 | 1.472 | 1.631 | 1.227 |
| Area variation (F) | 0.179 | 0.178 | 0.313 | 0.354 | 0.330 | 0.354 | 0.299 | 0.354 | 0.293 |
| Hausdorff, ‰ of the diagonal (H) | 9.38 | 10.32 | 11.07 | 8.83 | 7.73 | 9.88 | 12.74 | 9.88 | 7.55 |
| Mean distance, ‰ (H) | 1.512 | 1.641 | 1.335 | 1.242 | 1.241 | 1.291 | 1.761 | 1.291 | 1.059 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.30 | 99.04 | 99.94 | 100.02 | 99.99 | 99.46 | 98.87 | 99.46 | 100.02 |
| Off budget, % (N) | 5.3 | 3.4 | 10.2 | 2.2 | 1.9 | 29.0 | 16.3 | 0.0 | 0.6 |
| Time, seconds (T) | 1.04 | 1.20 | 4.29 | 1.55 | 2.03 | 28.00 | 16.00 | 28.00 | 1.52 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 812 | 994 | 920 | 965 | 1001 | 1208 | 928 | 1208 | 1207 |
| Irregular vertices (S) | 16 | 51 | 24 | 26 | 40 | 42 | 39 | 42 | 40 |
| … of which above the minimum | 8 | 43 | — | 18 | 32 | 34 | 31 | 34 | 32 |
| Base complex patches (C) | 812 | 822 | 666 | 773 | 355 | 588 | 346 | 588 | 465 |
| Alignment, degrees (A) | 14.79 | 11.93 | 8.02 | 8.03 | 5.67 | 4.65 | 4.83 | 4.72 | 5.92 |
| Mean angle deviation, degrees (F) | 12.55 | 6.23 | 7.31 | 5.01 | 5.53 | 6.49 | 6.99 | 6.49 | 5.24 |
| Angle deviation, 95th percentile (F) | 30.83 | 18.64 | 22.70 | 15.21 | 15.71 | 18.32 | 21.32 | 18.32 | 14.86 |
| Mean aspect ratio (F) | 1.391 | 1.185 | 1.250 | 1.246 | 1.347 | 1.473 | 1.394 | 1.473 | 1.299 |
| Area variation (F) | 0.298 | 0.183 | 0.288 | 0.352 | 0.481 | 0.461 | 0.380 | 0.461 | 0.490 |
| Hausdorff, ‰ of the diagonal (H) | 40.53 | 18.99 | 12.89 | 5.03 | 3.25 | 3.74 | 6.58 | 3.74 | 3.30 |
| Mean distance, ‰ (H) | 1.486 | 0.917 | 0.516 | 0.465 | 0.499 | 0.604 | 0.932 | 0.604 | 0.401 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 97.34 | 98.23 | — | 100.08 | 100.07 | 98.75 | 98.01 | 98.75 | 100.05 |
| Off budget, % (N) | 18.8 | 1.4 | 8.0 | 3.5 | 0.1 | 20.8 | 7.2 | 0.0 | 0.1 |
| Time, seconds (T) | 1.11 | 1.03 | 3.88 | 1.79 | 1.66 | 11.95 | 8.72 | 11.95 | 1.76 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1043 | 974 | 970 | 1001 | 1001 | 3224 | 934 | 3224 | 3168 |
| Irregular vertices (S) | 64 | 136 | 40 | 48 | 48 | 61 | 49 | 61 | 48 |
| … of which above the minimum | 32 | 104 | 8 | 16 | 16 | 29 | 17 | 29 | 16 |
| Base complex patches (C) | 716 | 779 | 96 | 263 | 208 | 741 | 211 | 741 | 438 |
| Alignment, degrees (A) | 7.66 | 10.19 | 3.01 | 4.06 | 3.17 | 2.70 | 3.17 | 2.73 | 2.27 |
| Mean angle deviation, degrees (F) | 6.96 | 9.36 | 4.97 | 4.72 | 5.00 | 4.46 | 6.04 | 4.46 | 3.14 |
| Angle deviation, 95th percentile (F) | 21.11 | 30.34 | 16.26 | 14.08 | 15.30 | 15.38 | 19.11 | 15.38 | 9.61 |
| Mean aspect ratio (F) | 1.250 | 1.402 | 1.236 | 1.251 | 1.243 | 2.906 | 1.395 | 2.906 | 1.205 |
| Area variation (F) | 0.155 | 0.279 | 0.284 | 0.242 | 0.345 | 0.384 | 0.250 | 0.384 | 0.365 |
| Hausdorff, ‰ of the diagonal (H) | 21.45 | 28.51 | 17.39 | 25.69 | 10.70 | 7.11 | 31.90 | 7.11 | 10.68 |
| Mean distance, ‰ (H) | 1.546 | 2.710 | 1.750 | 2.627 | 1.276 | 0.849 | 2.892 | 0.849 | 0.492 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.12 | 95.62 | 99.67 | 99.47 | 99.93 | 99.30 | 95.23 | 99.30 | 99.97 |
| Off budget, % (N) | 4.3 | 2.6 | 3.0 | 0.1 | 0.1 | 222.4 | 6.6 | 0.0 | 1.7 |
| Time, seconds (T) | 0.39 | 0.57 | 4.72 | 0.98 | 1.19 | 5.08 | 4.00 | 5.08 | 3.22 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1005 | 958 | 976 | 1002 | 994 | 1790 | 922 | 1790 | 1771 |
| Irregular vertices (S) | 67 | 114 | 51 | 44 | 48 | 68 | 68 | 68 | 50 |
| … of which above the minimum | — | 98 | — | — | — | — | — | — | — |
| Base complex patches (C) | 995 | 930 | 733 | 601 | 554 | 1538 | 802 | 1538 | 848 |
| Alignment, degrees (A) | 15.73 | 17.84 | 11.65 | 11.38 | 10.36 | 8.58 | 9.64 | 8.91 | 10.68 |
| Mean angle deviation, degrees (F) | 9.65 | 10.18 | 9.03 | 4.85 | 5.61 | 8.74 | 8.64 | 8.74 | 4.71 |
| Angle deviation, 95th percentile (F) | 29.23 | 34.26 | 26.04 | 17.33 | 18.44 | 23.66 | 23.37 | 23.66 | 14.65 |
| Mean aspect ratio (F) | 1.325 | 1.423 | 1.311 | 1.231 | 1.303 | 1.823 | 1.345 | 1.823 | 1.235 |
| Area variation (F) | 0.506 | 0.395 | 0.357 | 0.287 | 0.433 | 0.551 | 0.247 | 0.551 | 0.389 |
| Hausdorff, ‰ of the diagonal (H) | 44.79 | 50.97 | 36.86 | 16.89 | 12.92 | 10.72 | 17.45 | 10.72 | 9.24 |
| Mean distance, ‰ (H) | 2.245 | 3.552 | 2.048 | 1.672 | 1.473 | 1.112 | 2.422 | 1.112 | 0.799 |
| Thin parts: mean distance, % of a quad edge (V) | 10.6 | 23.9 | 12.5 | 9.7 | 5.1 | 4.8 | 12.7 | 6.4 | 3.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 1.0 | 4.2 | 2.4 | 0.2 | 0.6 | 79.0 | 7.8 | 0.0 | 1.1 |
| Time, seconds (T) | 0.76 | 0.30 | 4.33 | 1.41 | 1.43 | 4.46 | 3.77 | 4.46 | 1.84 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 891 | 932 | 783 | 1002 | 963 | 2890 | 799 | 2890 | 2887 |
| Irregular vertices (S) | 73 | 114 | 48 | 46 | 102 | 245 | 142 | 245 | 153 |
| … of which above the minimum | 64 | 99 | 40 | 38 | 94 | 237 | — | 237 | 145 |
| Base complex patches (C) | 891 | 860 | 555 | 562 | 534 | 2186 | — | 2186 | 2539 |
| Alignment, degrees (A) | 20.86 | 18.87 | 11.63 | 10.54 | 11.16 | 7.89 | 10.49 | 9.35 | 11.87 |
| Mean angle deviation, degrees (F) | 14.18 | 9.68 | 11.01 | 7.22 | 9.32 | 9.34 | 12.33 | 9.34 | 6.38 |
| Angle deviation, 95th percentile (F) | 39.23 | 29.97 | 29.05 | 20.66 | 27.54 | 26.48 | 34.04 | 26.48 | 18.54 |
| Mean aspect ratio (F) | 1.552 | 1.419 | 1.646 | 1.693 | 1.726 | 2.393 | 1.743 | 2.393 | 1.472 |
| Area variation (F) | 0.348 | 0.282 | 0.408 | 0.423 | 0.801 | 0.728 | 0.444 | 0.728 | 0.524 |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 85.71 | 37.45 | 29.99 | 29.81 | 27.76 | 51.64 | 27.76 | 29.81 |
| Mean distance, ‰ (H) | 4.418 | 3.773 | 2.195 | 1.614 | 1.802 | 0.968 | 3.165 | 0.968 | 0.747 |
| Thin parts: mean distance, % of a quad edge (V) | 115.0 | 107.2 | 35.3 | 20.5 | 5.8 | 4.5 | 50.1 | 8.9 | 7.7 |
| Volume kept, % of the source (V) | 94.57 | 95.94 | 99.48 | 99.75 | 99.78 | 98.71 | — | 98.71 | 100.00 |
| Off budget, % (N) | 10.9 | 6.8 | 21.7 | 0.2 | 3.7 | 189.0 | 20.1 | 0.0 | 0.1 |
| Time, seconds (T) | 3.33 | 2.71 | 5.13 | 3.44 | 4.40 | 18.93 | 16.67 | 18.93 | 4.74 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1024 | 1016 | 907 | 990 | 991 | 1497 | 882 | 1497 | 1514 |
| Irregular vertices (S) | 62 | 95 | 59 | 64 | 68 | 78 | 76 | 78 | 68 |
| … of which above the minimum | 54 | 87 | 51 | 56 | 60 | 70 | 68 | 70 | 60 |
| Base complex patches (C) | 965 | 982 | 747 | 770 | 721 | 1079 | 630 | 1079 | 968 |
| Alignment, degrees (A) | 18.23 | 17.84 | 11.04 | 9.52 | 7.69 | 6.34 | 7.17 | 6.45 | 7.79 |
| Mean angle deviation, degrees (F) | 7.55 | 7.36 | 11.19 | 7.28 | 6.98 | 8.24 | 8.98 | 8.24 | 6.14 |
| Angle deviation, 95th percentile (F) | 22.42 | 25.61 | 29.29 | 19.78 | 20.30 | 22.62 | 24.90 | 22.62 | 17.21 |
| Mean aspect ratio (F) | 1.252 | 1.267 | 1.447 | 1.390 | 1.397 | 1.689 | 1.455 | 1.689 | 1.364 |
| Area variation (F) | 0.209 | 0.211 | 0.344 | 0.396 | 0.445 | 0.555 | 0.343 | 0.555 | 0.428 |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 45.10 | 22.32 | 12.55 | 15.09 | 8.32 | 15.39 | 8.32 | 14.03 |
| Mean distance, ‰ (H) | 1.330 | 1.556 | 0.824 | 0.659 | 0.616 | 0.810 | 1.403 | 0.810 | 0.410 |
| Thin parts: mean distance, % of a quad edge (V) | 29.6 | 37.4 | 23.2 | 13.4 | 18.7 | 12.2 | 14.9 | 14.7 | 17.7 |
| Volume kept, % of the source (V) | 98.16 | 97.96 | 99.82 | 100.04 | 100.01 | 98.52 | 97.64 | 98.52 | 100.01 |
| Off budget, % (N) | 2.8 | 1.6 | 9.3 | 1.0 | 0.9 | 49.7 | 11.8 | 0.0 | 1.1 |
| Time, seconds (T) | 1.54 | 2.26 | 6.57 | 2.20 | 2.50 | 17.76 | 13.95 | 17.76 | 2.09 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 972 | 1006 | 861 | 993 | 972 | 954 | 893 | 954 | 951 |
| Irregular vertices (S) | 28 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 |
| … of which above the minimum | 28 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 |
| Base complex patches (C) | 958 | 1006 | 177 | 237 | 185 | 398 | 235 | 398 | 185 |
| Alignment, degrees (A) | 20.00 | 20.88 | 4.19 | 4.29 | 3.60 | 3.16 | 3.69 | 3.14 | 3.80 |
| Mean angle deviation, degrees (F) | 7.93 | 7.12 | 5.39 | 4.49 | 4.49 | 5.39 | 6.25 | 5.39 | 4.80 |
| Angle deviation, 95th percentile (F) | 22.21 | 22.89 | 16.57 | 11.74 | 13.09 | 15.59 | 19.36 | 15.59 | 13.43 |
| Mean aspect ratio (F) | 1.201 | 1.225 | 1.445 | 1.380 | 1.393 | 1.530 | 1.451 | 1.530 | 1.418 |
| Area variation (F) | 0.159 | 0.197 | 0.357 | 0.435 | 0.456 | 0.413 | 0.359 | 0.413 | 0.449 |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 28.68 | 19.08 | 4.86 | 4.33 | 7.54 | 7.25 | 7.54 | 4.39 |
| Mean distance, ‰ (H) | 0.966 | 1.021 | 0.442 | 0.384 | 0.397 | 1.012 | 1.114 | 1.012 | 0.398 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.35 | 98.23 | 100.01 | 100.05 | 100.05 | 97.96 | 97.78 | 97.96 | 100.05 |
| Off budget, % (N) | 2.8 | 2.4 | 13.9 | 0.7 | 2.8 | 4.6 | 10.7 | 0.0 | 0.3 |
| Time, seconds (T) | 1.43 | 1.10 | 3.35 | 3.16 | 2.84 | 12.87 | 16.31 | 12.87 | 2.06 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 847 | 996 | 912 | 983 | 989 | 1638 | 892 | 1638 | 1647 |
| Irregular vertices (S) | 44 | 82 | 41 | 40 | 50 | 60 | 53 | 60 | 50 |
| … of which above the minimum | 32 | 74 | — | 32 | 42 | 52 | 45 | 52 | 42 |
| Base complex patches (C) | 785 | 980 | — | 321 | 523 | 836 | 440 | 836 | 1063 |
| Alignment, degrees (A) | 16.32 | 17.18 | 9.20 | 8.49 | 9.18 | 4.88 | 6.96 | 5.29 | 8.18 |
| Mean angle deviation, degrees (F) | 10.04 | 7.61 | 8.42 | 4.85 | 5.71 | 6.85 | 8.39 | 6.85 | 5.06 |
| Angle deviation, 95th percentile (F) | 28.85 | 26.75 | 22.60 | 13.95 | 17.12 | 19.46 | 22.44 | 19.46 | 15.79 |
| Mean aspect ratio (F) | 1.336 | 1.285 | 1.301 | 1.309 | 1.353 | 2.117 | 1.414 | 2.117 | 1.320 |
| Area variation (F) | 0.267 | 0.235 | 0.272 | 0.357 | 0.454 | 0.568 | 0.310 | 0.568 | 0.404 |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 37.01 | 37.76 | 22.64 | 17.27 | 8.09 | 29.46 | 8.09 | 12.58 |
| Mean distance, ‰ (H) | 1.731 | 1.524 | 1.075 | 0.839 | 0.608 | 0.521 | 1.283 | 0.521 | 0.367 |
| Thin parts: mean distance, % of a quad edge (V) | 30.4 | 26.6 | 26.5 | 17.9 | 6.6 | 4.0 | 21.3 | 6.8 | 9.8 |
| Volume kept, % of the source (V) | 96.59 | 97.11 | 99.08 | 99.46 | 99.84 | 98.99 | 97.39 | 98.99 | 99.85 |
| Off budget, % (N) | 15.3 | 1.5 | 8.8 | 1.7 | 1.1 | 63.8 | 10.8 | 0.0 | 0.5 |
| Time, seconds (T) | 1.50 | 1.14 | 5.30 | 1.54 | 2.80 | 14.40 | 12.45 | 14.40 | 1.98 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 861 | 910 | 903 | 1009 | 1009 | 1975 | 853 | 1975 | 1949 |
| Irregular vertices (S) | 92 | 94 | 59 | 60 | 92 | 95 | 85 | 95 | 94 |
| … of which above the minimum | 76 | 75 | — | 52 | 84 | 87 | — | 87 | 86 |
| Base complex patches (C) | 848 | 861 | — | 541 | 603 | 1149 | — | 1149 | 1097 |
| Alignment, degrees (A) | 17.33 | 16.11 | 13.07 | 12.05 | 9.26 | 7.39 | 10.36 | 7.52 | 10.07 |
| Mean angle deviation, degrees (F) | 15.46 | 8.44 | 8.62 | 6.26 | 7.28 | 8.27 | 9.33 | 8.27 | 6.37 |
| Angle deviation, 95th percentile (F) | 46.32 | 30.16 | 25.91 | 20.19 | 22.36 | 23.65 | 27.06 | 23.65 | 19.23 |
| Mean aspect ratio (F) | 1.533 | 1.339 | 1.324 | 1.347 | 1.463 | 2.045 | 1.425 | 2.045 | 1.280 |
| Area variation (F) | 0.437 | 0.272 | 0.276 | 0.318 | 0.877 | 0.753 | 0.323 | 0.753 | 0.514 |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 97.30 | 34.21 | 22.16 | 18.64 | 11.45 | 85.93 | 11.45 | 11.55 |
| Mean distance, ‰ (H) | 4.074 | 5.299 | 1.712 | 1.089 | 1.048 | 0.786 | 2.893 | 0.786 | 0.465 |
| Thin parts: mean distance, % of a quad edge (V) | 84.1 | 119.6 | 24.5 | 13.9 | 4.7 | 3.5 | 50.0 | 6.0 | 4.8 |
| Volume kept, % of the source (V) | 96.89 | 96.53 | 98.61 | 99.33 | 99.73 | 98.97 | 97.29 | 98.97 | 99.97 |
| Off budget, % (N) | 13.9 | 9.0 | 9.7 | 0.9 | 0.9 | 97.5 | 14.7 | 0.0 | 1.3 |
| Time, seconds (T) | 1.55 | 1.07 | 5.45 | 3.03 | 2.53 | 17.10 | 15.43 | 17.10 | 2.65 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 3/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 |
| Quads made | 900 | 886 | 830 | 1000 | 1005 | 3093 | 876 | 3093 | 3071 |
| Irregular vertices (S) | 69 | 108 | 52 | 51 | 83 | 182 | 150 | 182 | 172 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 724 | 849 | 595 | 605 | 886 | — | — | — | 2664 |
| Alignment, degrees (A) | 18.48 | 19.26 | 16.56 | 14.75 | 15.27 | 12.26 | 14.41 | 12.79 | 14.08 |
| Mean angle deviation, degrees (F) | 10.23 | 11.03 | 10.72 | 7.85 | 9.04 | 10.70 | 12.37 | 10.70 | 6.97 |
| Angle deviation, 95th percentile (F) | 28.90 | 36.55 | 28.05 | 21.89 | 27.57 | 30.61 | 34.62 | 30.61 | 21.43 |
| Mean aspect ratio (F) | 1.359 | 1.449 | 1.405 | 1.424 | 1.456 | 2.192 | 1.561 | 2.192 | 1.333 |
| Area variation (F) | 0.306 | 0.338 | 0.288 | 0.323 | 0.766 | 0.752 | 0.389 | 0.752 | 0.514 |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 84.04 | 69.27 | 36.45 | 25.10 | 7.66 | 49.73 | 7.66 | 26.16 |
| Mean distance, ‰ (H) | 4.435 | 6.117 | 4.835 | 2.971 | 2.774 | 0.950 | 3.575 | 0.950 | 0.833 |
| Thin parts: mean distance, % of a quad edge (V) | 28.0 | 42.2 | 38.8 | 22.1 | 14.3 | 4.8 | 18.4 | 9.8 | 16.5 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 10.0 | 11.4 | 17.0 | 0.0 | 0.5 | 209.3 | 12.4 | 0.0 | 0.7 |
| Time, seconds (T) | 0.74 | 0.37 | 36.70 | 1.84 | 2.16 | 6.67 | 10.59 | 6.67 | 3.19 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 494 | 530 | 596 | 1773 | 1343 | 26339 | 1271 | 26339 | 26548 |
| Irregular vertices (S) | 164 | 181 | 188 | 715 | 731 | 1682 | 572 | 1682 | 2181 |
| … of which above the minimum | — | — | — | — | — | — | — | — | 1565 |
| Base complex patches (C) | 453 | 500 | — | — | — | — | — | — | 21468 |
| Alignment, degrees (A) | 18.41 | 18.59 | 16.86 | 17.39 | 17.37 | 9.46 | 14.37 | 12.49 | 9.87 |
| Mean angle deviation, degrees (F) | 18.81 | 20.14 | 16.04 | 25.38 | 28.76 | 9.03 | 16.83 | 9.03 | 7.56 |
| Angle deviation, 95th percentile (F) | 52.35 | 58.52 | 48.76 | 68.68 | 74.82 | 26.22 | 49.51 | 26.22 | 23.53 |
| Mean aspect ratio (F) | 1.970 | 2.108 | 1.656 | 4.845 | 6.191 | 1.662 | 1.788 | 1.662 | 1.857 |
| Area variation (F) | 0.599 | 0.707 | 0.550 | 1.969 | 2.184 | 1.972 | 1.056 | 1.972 | 0.481 |
| Hausdorff, ‰ of the diagonal (H) | 76.78 | 97.09 | 82.01 | 75.59 | 73.25 | 74.63 | 78.15 | 74.63 | 12.02 |
| Mean distance, ‰ (H) | 13.479 | 14.528 | 8.862 | 6.237 | 8.429 | 1.049 | 5.982 | 1.049 | 0.213 |
| Thin parts: mean distance, % of a quad edge (V) | 64.1 | 71.0 | 43.1 | 29.9 | 40.7 | 5.8 | 28.8 | 91.4 | 10.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | 99.44 |
| Off budget, % (N) | 50.6 | 47.0 | 40.4 | 77.3 | 34.3 | 2533.9 | 27.1 | 0.0 | 0.8 |
| Time, seconds (T) | 2.12 | 1.63 | 6.88 | 9.25 | 11.36 | 14.03 | 9.27 | 14.03 | 30.43 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | — | 845 | 912 | 1003 | 1008 | 2105 | 936 | 2105 | 2120 |
| Irregular vertices (S) | — | 83 | 27 | 36 | 36 | 76 | 54 | 76 | 36 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 532 | 171 | 279 | 451 | 1346 | — | 1346 | 603 |
| Alignment, degrees (A) | — | 13.44 | 8.10 | 7.04 | 6.65 | 6.49 | 9.01 | 7.14 | 7.40 |
| Mean angle deviation, degrees (F) | — | 8.67 | 7.81 | 4.24 | 5.58 | 8.08 | 9.87 | 8.08 | 4.39 |
| Angle deviation, 95th percentile (F) | — | 30.33 | 22.68 | 13.37 | 17.08 | 22.80 | 25.01 | 22.80 | 12.26 |
| Mean aspect ratio (F) | — | 1.363 | 1.346 | 1.276 | 1.332 | 2.085 | 1.391 | 2.085 | 1.265 |
| Area variation (F) | — | 0.276 | 0.286 | 0.278 | 0.534 | 0.486 | 0.271 | 0.486 | 0.430 |
| Hausdorff, ‰ of the diagonal (H) | — | 53.13 | 18.46 | 17.46 | 9.08 | 5.71 | 13.65 | 5.71 | 7.75 |
| Mean distance, ‰ (H) | — | 2.255 | 1.134 | 0.887 | 0.817 | 0.640 | 1.473 | 0.640 | 0.555 |
| Thin parts: mean distance, % of a quad edge (V) | — | 48.5 | 8.2 | 4.7 | 2.8 | 3.4 | 5.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 15.5 | 8.8 | 0.3 | 0.8 | 110.5 | 6.4 | 0.0 | 0.7 |
| Time, seconds (T) | — | 1.83 | 6.12 | 2.70 | 3.68 | 13.07 | 11.79 | 13.07 | 2.72 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 943 | 1036 | 959 | 976 | 1018 | 1013 | 924 | 1013 | 1018 |
| Irregular vertices (S) | 30 | 115 | 24 | 24 | 24 | 26 | 24 | 26 | 24 |
| … of which above the minimum | 22 | 107 | 16 | 16 | 16 | 18 | 16 | 18 | 16 |
| Base complex patches (C) | 227 | 630 | 388 | 86 | 68 | 367 | 293 | 367 | 68 |
| Alignment, degrees (A) | 13.41 | 14.60 | 12.82 | 12.46 | 10.87 | 11.10 | 10.93 | 11.10 | 10.87 |
| Mean angle deviation, degrees (F) | 3.78 | 8.58 | 4.85 | 2.07 | 2.16 | 4.39 | 4.53 | 4.39 | 2.17 |
| Angle deviation, 95th percentile (F) | 15.04 | 35.19 | 15.54 | 9.04 | 10.18 | 14.60 | 15.17 | 14.60 | 10.14 |
| Mean aspect ratio (F) | 1.191 | 1.407 | 1.208 | 1.219 | 1.177 | 1.323 | 1.314 | 1.323 | 1.177 |
| Area variation (F) | 0.176 | 0.263 | 0.143 | 0.175 | 0.381 | 0.296 | 0.273 | 0.296 | 0.381 |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 20.39 | 17.19 | 14.36 | 1.37 | 1.64 | 2.29 | 1.64 | 1.37 |
| Mean distance, ‰ (H) | 0.335 | 0.238 | 0.679 | 0.259 | 0.165 | 0.147 | 0.173 | 0.147 | 0.165 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.43 | 100.43 | 99.90 | 100.01 | 100.01 | 100.26 | 100.31 | 100.26 | 100.01 |
| Off budget, % (N) | 5.7 | 3.6 | 4.1 | 2.4 | 1.8 | 1.3 | 7.6 | 0.0 | 0.5 |
| Time, seconds (T) | 0.70 | 0.15 | 9.34 | 1.15 | 1.87 | 3.06 | 2.65 | 3.06 | 0.87 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1014 | 900 | 999 | 1014 | 1014 | 1014 | 1014 | 1014 | 1014 |
| Irregular vertices (S) | 8 | 16 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 174 | 999 | 6 | 6 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.17 | 1.29 | 1.55 | 0.11 | 0.25 | 0.11 | 0.11 | 0.11 | 0.25 |
| Mean angle deviation, degrees (F) | 0.08 | 2.07 | 2.64 | 0.00 | 0.33 | 0.00 | 0.00 | 0.00 | 0.33 |
| Angle deviation, 95th percentile (F) | 0.23 | 7.03 | 6.80 | 0.01 | 0.76 | 0.00 | 0.00 | 0.00 | 0.76 |
| Mean aspect ratio (F) | 1.003 | 1.093 | 1.086 | 1.000 | 1.071 | 1.000 | 1.000 | 1.000 | 1.071 |
| Area variation (F) | 0.002 | 0.103 | 0.074 | 0.000 | 0.082 | 0.000 | 0.000 | 0.000 | 0.081 |
| Hausdorff, ‰ of the diagonal (H) | 2.31 | 0.00 | 25.45 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.015 | 0.000 | 0.399 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 | 99.93 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 1.4 | 10.0 | 0.1 | 1.4 | 1.4 | 1.4 | 1.4 | 0.0 | 0.0 |
| Time, seconds (T) | 0.33 | 0.06 | 3.86 | 0.67 | 1.07 | 2.35 | 3.01 | 2.35 | 0.94 |

## lblock (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 930 | 902 | 992 | 994 | 994 | 1010 | 1010 | 1010 | 1028 |
| Irregular vertices (S) | 12 | 16 | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 4 | 8 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 85 | 84 | 26 | 14 | 14 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.36 | 1.05 | 1.80 | 0.56 | 0.64 | 0.56 | 0.56 | 0.56 | 0.64 |
| Mean angle deviation, degrees (F) | 1.26 | 1.39 | 2.01 | 0.10 | 0.37 | 0.30 | 0.30 | 0.30 | 0.39 |
| Angle deviation, 95th percentile (F) | 4.73 | 3.73 | 6.27 | 0.33 | 0.90 | 1.26 | 1.26 | 1.26 | 1.01 |
| Mean aspect ratio (F) | 1.064 | 1.041 | 1.120 | 1.072 | 1.099 | 1.090 | 1.090 | 1.090 | 1.098 |
| Area variation (F) | 0.059 | 0.057 | 0.080 | 0.070 | 0.096 | 0.053 | 0.053 | 0.053 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 9.75 | 0.00 | 13.72 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.123 | 0.000 | 0.591 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.99 | 100.00 | 99.86 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 7.0 | 9.8 | 0.8 | 0.6 | 0.6 | 1.0 | 1.0 | 0.0 | 1.8 |
| Time, seconds (T) | 0.23 | 0.05 | 3.04 | 1.31 | 1.16 | 2.79 | 2.67 | 2.79 | 1.13 |

## cylinder (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 995 | 994 | 892 | 990 | 968 | 782 | 782 | 782 | 764 |
| Irregular vertices (S) | 8 | 40 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 32 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 576 | 178 | 6 | 186 | 138 | 138 | 138 | 56 |
| Alignment, degrees (A) | 3.95 | 4.86 | 1.95 | 1.24 | 1.81 | 1.55 | 1.55 | 1.67 | 1.39 |
| Mean angle deviation, degrees (F) | 4.50 | 5.03 | 4.42 | 2.65 | 3.74 | 4.01 | 4.01 | 4.01 | 2.64 |
| Angle deviation, 95th percentile (F) | 17.64 | 20.26 | 12.92 | 7.69 | 8.53 | 11.66 | 11.66 | 11.66 | 9.35 |
| Mean aspect ratio (F) | 1.124 | 1.170 | 1.309 | 1.286 | 1.510 | 1.528 | 1.528 | 1.528 | 1.513 |
| Area variation (F) | 0.112 | 0.174 | 0.330 | 0.334 | 0.206 | 0.416 | 0.416 | 0.416 | 0.196 |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 15.50 | 1.55 | 1.48 | 1.70 | 2.59 | 2.59 | 2.59 | 1.95 |
| Mean distance, ‰ (H) | 0.373 | 0.376 | 0.341 | 0.311 | 0.356 | 0.938 | 0.938 | 0.938 | 0.459 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.73 | 99.66 | 100.00 | 99.99 | 100.02 | 99.00 | 99.00 | 99.00 | 100.00 |
| Off budget, % (N) | 1.8 | 1.0 | 10.8 | 1.0 | 3.2 | 21.8 | 21.8 | 0.0 | 2.3 |
| Time, seconds (T) | 0.33 | 0.05 | 1.84 | 0.98 | 1.39 | 2.92 | 2.88 | 2.92 | 0.91 |

## terrain (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 872 | 574 | 885 | 1017 | 993 | 1541 | 878 | 1541 | 1569 |
| Irregular vertices (S) | 44 | 40 | 33 | 36 | 38 | 152 | 88 | 152 | 42 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 307 | 394 | 262 | 647 | 639 | 1446 | — | 1446 | 1011 |
| Alignment, degrees (A) | 19.25 | 18.99 | 17.85 | 17.69 | 17.56 | 18.91 | 20.03 | 18.89 | 18.68 |
| Mean angle deviation, degrees (F) | 6.74 | 8.62 | 9.98 | 5.70 | 6.06 | 11.26 | 10.74 | 11.26 | 5.83 |
| Angle deviation, 95th percentile (F) | 23.83 | 29.38 | 28.26 | 18.94 | 18.44 | 32.77 | 31.81 | 32.77 | 16.81 |
| Mean aspect ratio (F) | 1.223 | 1.338 | 1.384 | 1.288 | 1.328 | 1.859 | 1.521 | 1.859 | 1.311 |
| Area variation (F) | 0.195 | 0.224 | 0.321 | 0.368 | 0.458 | 0.545 | 0.321 | 0.545 | 0.446 |
| Hausdorff, ‰ of the diagonal (H) | 30.58 | 146.80 | 28.90 | 23.68 | 26.86 | 18.41 | 31.35 | 18.41 | 29.15 |
| Mean distance, ‰ (H) | 3.158 | 4.386 | 2.668 | 2.165 | 1.994 | 1.506 | 2.787 | 1.506 | 1.599 |
| Thin parts: mean distance, % of a quad edge (V) | 29.7 | 44.9 | 29.2 | 22.3 | 19.4 | 11.3 | 27.0 | 14.3 | 20.6 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 12.8 | 42.6 | 11.5 | 1.7 | 0.7 | 54.1 | 12.2 | 0.0 | 1.8 |
| Time, seconds (T) | 19.49 | 8.90 | 11.19 | 6.89 | 7.39 | 19.65 | 21.87 | 19.65 | 10.63 |

## mahaveer (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 562 | 662 | 2053 | 2021 | 41821 | 2688 | 41821 | 40442 |
| Irregular vertices (S) | — | 124 | 45 | 421 | 421 | 3109 | 1013 | 3109 | 1238 |
| … of which above the minimum | — | 13 | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 423 | 162 | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 11.52 | 7.39 | 17.39 | 17.74 | 15.01 | 14.68 | 14.20 | 9.51 |
| Mean angle deviation, degrees (F) | — | 15.07 | 9.90 | 33.35 | 34.25 | 13.01 | 16.98 | 13.01 | 7.26 |
| Angle deviation, 95th percentile (F) | — | 53.29 | 30.38 | 85.62 | 88.24 | 40.35 | 58.74 | 40.35 | 25.61 |
| Mean aspect ratio (F) | — | 2.321 | 1.531 | 119.626 | 184.152 | 1.768 | 2.072 | 1.768 | 10.606 |
| Area variation (F) | — | 0.516 | 0.416 | 8.169 | 8.177 | 3.743 | 1.645 | 3.743 | 0.536 |
| Hausdorff, ‰ of the diagonal (H) | — | 69.46 | 48.19 | 325.50 | 325.41 | 13.85 | 49.37 | 13.85 | 13.24 |
| Mean distance, ‰ (H) | — | 10.323 | 5.432 | 40.078 | 40.082 | 1.081 | 4.277 | 1.081 | 0.551 |
| Thin parts: mean distance, % of a quad edge (V) | — | 50.2 | 28.2 | 153.5 | 153.5 | 7.4 | 22.4 | 50.7 | 29.1 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 43.8 | 33.8 | 105.3 | 102.1 | 4082.1 | 168.8 | 0.0 | 3.3 |
| Time, seconds (T) | — | 5.58 | 16.18 | 8.14 | 11.44 | 16.61 | 15.33 | 16.61 | 74.02 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 204 | — | 6485 | 6364 | 205185 | 10623 | 205185 | 188593 |
| Irregular vertices (S) | — | 139 | — | 5577 | 5538 | 10625 | 4798 | 10625 | 11041 |
| … of which above the minimum | — | -56 | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 204 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 19.32 | — | — | — | 10.09 | — | 13.06 | 8.24 |
| Mean angle deviation, degrees (F) | — | 30.35 | — | 25.76 | 25.86 | 10.19 | 17.58 | 10.19 | 5.29 |
| Angle deviation, 95th percentile (F) | — | 69.83 | — | 74.07 | 73.42 | 30.61 | 52.92 | 30.61 | 19.41 |
| Mean aspect ratio (F) | — | 3.003 | — | 7.161 | 14.969 | 1.591 | 1.958 | 1.591 | 1.401 |
| Area variation (F) | — | 0.795 | — | 2.571 | 2.396 | 3.347 | 3.276 | 3.347 | 0.357 |
| Hausdorff, ‰ of the diagonal (H) | — | 10.79 | — | 6.31 | 6.31 | 2.06 | 5.15 | 2.06 | 2.99 |
| Mean distance, ‰ (H) | — | 3.973 | — | 0.534 | 0.551 | 0.152 | 0.565 | 0.152 | 0.047 |
| Thin parts: mean distance, % of a quad edge (V) | — | 70.3 | — | 9.5 | 9.7 | 5.2 | 11.1 | 66.6 | 30.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 79.6 | — | 548.5 | 536.4 | 20418.5 | 962.3 | 0.0 | 8.1 |
| Time, seconds (T) | — | 3.43 | — | 41.94 | 57.95 | 14.01 | 8.85 | 14.01 | 172.30 |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 545 non-manifold or inconsistently oriented edges.

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 950 | 1066 | 923 | 1010 | 1010 | 915 | 915 | 915 | 900 |
| Irregular vertices (S) | 8 | 44 | 8 | 8 | 8 | 10 | 10 | 10 | 10 |
| … of which above the minimum | 0 | 36 | 0 | 0 | 0 | 2 | 2 | 2 | 2 |
| Base complex patches (C) | 937 | 1066 | 923 | 1010 | 1010 | 699 | 699 | 699 | 846 |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.18 | 8.36 | 4.61 | 4.61 | 6.03 | 6.03 | 6.03 | 4.08 |
| Angle deviation, 95th percentile (F) | 12.31 | 17.01 | 22.46 | 11.13 | 11.13 | 14.57 | 14.57 | 14.57 | 10.25 |
| Mean aspect ratio (F) | 1.148 | 1.153 | 1.171 | 1.180 | 1.180 | 1.213 | 1.213 | 1.213 | 1.153 |
| Area variation (F) | 0.136 | 0.179 | 0.152 | 0.225 | 0.225 | 0.318 | 0.318 | 0.318 | 0.240 |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 1.68 | 1.11 | 0.99 | 0.99 | 1.91 | 1.91 | 1.91 | 2.08 |
| Mean distance, ‰ (H) | 0.670 | 0.632 | 0.356 | 0.332 | 0.332 | 0.762 | 0.762 | 0.762 | 0.357 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.38 | 99.38 | 100.02 | 100.02 | 100.02 | 99.24 | 99.24 | 99.24 | 100.02 |
| Off budget, % (N) | 5.0 | 6.6 | 7.7 | 1.0 | 1.0 | 8.5 | 8.5 | 0.0 | 1.6 |
| Time, seconds (T) | 0.41 | 0.19 | 2.04 | 0.76 | 1.28 | 4.77 | 3.75 | 4.77 | 1.50 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1028 | 1054 | 920 | 1008 | 1008 | 920 | 920 | 920 | 920 |
| Irregular vertices (S) | 20 | 90 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| … of which above the minimum | 20 | 90 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1016 | 932 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| Alignment, degrees (A) | 13.64 | 8.78 | 0.63 | 0.16 | 0.16 | 0.63 | 0.63 | 0.64 | 0.64 |
| Mean angle deviation, degrees (F) | 7.73 | 8.51 | 2.51 | 2.40 | 2.40 | 2.49 | 2.49 | 2.49 | 2.52 |
| Angle deviation, 95th percentile (F) | 18.19 | 26.44 | 3.93 | 3.73 | 3.73 | 3.93 | 3.93 | 3.93 | 3.91 |
| Mean aspect ratio (F) | 1.178 | 1.273 | 1.362 | 1.295 | 1.295 | 1.372 | 1.372 | 1.372 | 1.293 |
| Area variation (F) | 0.180 | 0.237 | 0.294 | 0.350 | 0.351 | 0.280 | 0.280 | 0.280 | 0.356 |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 3.96 | 1.51 | 1.35 | 1.35 | 2.60 | 2.60 | 2.60 | 1.67 |
| Mean distance, ‰ (H) | 0.775 | 0.849 | 0.447 | 0.424 | 0.424 | 1.127 | 1.127 | 1.127 | 0.449 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.57 | 98.28 | 100.06 | 100.04 | 100.04 | 97.62 | 97.62 | 97.62 | 100.06 |
| Off budget, % (N) | 2.8 | 5.4 | 8.0 | 0.8 | 0.8 | 8.0 | 8.0 | 0.0 | 0.0 |
| Time, seconds (T) | 0.31 | 0.27 | 0.99 | 0.92 | 1.04 | 4.41 | 2.61 | 4.41 | 1.39 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 6 | 6 | 1 | 0 | 0 | 0 | 1 |
| Base complex patches (C) | 1 | 0 | 4 | 3 | 2 | 0 | 3 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 1 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 5 | 2 | 0 | 0 | 0 | 6 |
| Mean aspect ratio (F) | 4 | 3 | 1 | 1 | 0 | 0 | 0 | 0 | 4 |
| Valid meshes (V) | 38/65 | 44/65 | 8/13 | 12/13 | 12/13 | 11/13 | 8/13 | 11/13 | 12/13 |

### mechanical (4 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform | quadremesher-as-made | quad-layout-matched |
|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 4 | 4 | 4 | 3 | 4 | 3 | 4 |
| Base complex patches (C) | 1 | 0 | 0 | 3 | 3 | 2 | 2 | 2 | 3 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 0 | 2 | 2 | 2 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 2 | 0 | 1 | 1 | 1 | 1 |
| Mean aspect ratio (F) | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 0 |
| Valid meshes (V) | 20/20 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
