# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 1.95 | 1.70 | 4.13 | 1.47 | 2.54 | 14.01 | 13.25 | 14.01 | 2.56 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 1.04 | 1.20 | 5.26 | 1.29 | 1.40 | 28.00 | 16.00 | 28.00 | 1.56 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 1.11 | 1.03 | 4.64 | 1.94 | 2.92 | 11.95 | 8.72 | 11.95 | 1.66 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 0.39 | 0.57 | 3.70 | 0.76 | 1.64 | 5.08 | 4.00 | 5.08 | 2.55 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 0.76 | 0.30 | 5.95 | 1.13 | 1.69 | 4.46 | 3.77 | 4.46 | 2.01 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 891 | 932 | 802 | 972 | 1019 | 2890 | 799 | 2890 | 2891 |
| Irregular vertices (S) | 73 | 114 | 47 | 46 | 102 | 245 | 142 | 245 | 153 |
| … of which above the minimum | 64 | 99 | — | 38 | 94 | 237 | — | 237 | 145 |
| Base complex patches (C) | 891 | 860 | — | 492 | 689 | 2186 | — | 2186 | 2583 |
| Alignment, degrees (A) | 20.86 | 18.87 | 11.86 | 10.49 | 11.09 | 7.89 | 10.49 | 9.35 | 11.69 |
| Mean angle deviation, degrees (F) | 14.18 | 9.68 | 11.53 | 7.32 | 9.05 | 9.34 | 12.33 | 9.34 | 6.46 |
| Angle deviation, 95th percentile (F) | 39.23 | 29.97 | 29.24 | 20.26 | 26.58 | 26.48 | 34.04 | 26.48 | 18.43 |
| Mean aspect ratio (F) | 1.552 | 1.419 | 1.628 | 1.713 | 1.659 | 2.393 | 1.743 | 2.393 | 1.416 |
| Area variation (F) | 0.348 | 0.282 | 0.388 | 0.407 | 0.805 | 0.728 | 0.444 | 0.728 | 0.509 |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 85.71 | 33.77 | 28.58 | 30.14 | 27.76 | 51.64 | 27.76 | 29.77 |
| Mean distance, ‰ (H) | 4.418 | 3.773 | 2.160 | 1.640 | 1.795 | 0.968 | 3.165 | 0.968 | 0.768 |
| Thin parts: mean distance, % of a quad edge (V) | 115.0 | 107.2 | 35.4 | 20.3 | 5.9 | 4.5 | 50.1 | 8.9 | 7.6 |
| Volume kept, % of the source (V) | 94.57 | 95.94 | 99.36 | 99.70 | 99.75 | 98.71 | — | 98.71 | 100.02 |
| Off budget, % (N) | 10.9 | 6.8 | 19.8 | 2.8 | 1.9 | 189.0 | 20.1 | 0.0 | 0.0 |
| Time, seconds (T) | 3.33 | 2.71 | 4.70 | 3.25 | 2.79 | 18.93 | 16.67 | 18.93 | 5.08 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1024 | 1016 | 910 | 990 | 986 | 1497 | 882 | 1497 | 1514 |
| Irregular vertices (S) | 62 | 95 | 63 | 64 | 68 | 78 | 76 | 78 | 68 |
| … of which above the minimum | 54 | 87 | 55 | 56 | 60 | 70 | 68 | 70 | 60 |
| Base complex patches (C) | 965 | 982 | 690 | 770 | 754 | 1079 | 630 | 1079 | 968 |
| Alignment, degrees (A) | 18.23 | 17.84 | 10.63 | 9.51 | 7.53 | 6.34 | 7.17 | 6.45 | 7.80 |
| Mean angle deviation, degrees (F) | 7.55 | 7.36 | 11.20 | 7.26 | 6.93 | 8.24 | 8.98 | 8.24 | 6.13 |
| Angle deviation, 95th percentile (F) | 22.42 | 25.61 | 29.66 | 19.69 | 20.12 | 22.62 | 24.90 | 22.62 | 17.28 |
| Mean aspect ratio (F) | 1.252 | 1.267 | 1.448 | 1.389 | 1.378 | 1.689 | 1.455 | 1.689 | 1.364 |
| Area variation (F) | 0.209 | 0.211 | 0.350 | 0.397 | 0.460 | 0.555 | 0.343 | 0.555 | 0.427 |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 45.10 | 19.66 | 12.54 | 14.44 | 8.32 | 15.39 | 8.32 | 14.02 |
| Mean distance, ‰ (H) | 1.330 | 1.556 | 0.811 | 0.659 | 0.606 | 0.810 | 1.403 | 0.810 | 0.411 |
| Thin parts: mean distance, % of a quad edge (V) | 29.6 | 37.4 | 23.1 | 13.4 | 16.2 | 12.2 | 14.9 | 14.7 | 17.7 |
| Volume kept, % of the source (V) | 98.16 | 97.96 | 99.85 | 100.04 | 100.01 | 98.52 | 97.64 | 98.52 | 100.00 |
| Off budget, % (N) | 2.8 | 1.6 | 9.0 | 1.0 | 1.4 | 49.7 | 11.8 | 0.0 | 1.1 |
| Time, seconds (T) | 1.54 | 2.26 | 4.55 | 1.67 | 1.93 | 17.76 | 13.95 | 17.76 | 3.06 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 972 | 1006 | 861 | 993 | 991 | 954 | 893 | 954 | 951 |
| Irregular vertices (S) | 28 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 |
| … of which above the minimum | 28 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 |
| Base complex patches (C) | 958 | 1006 | 177 | 237 | 326 | 398 | 235 | 398 | 185 |
| Alignment, degrees (A) | 20.00 | 20.88 | 4.19 | 4.29 | 4.18 | 3.16 | 3.69 | 3.14 | 3.79 |
| Mean angle deviation, degrees (F) | 7.93 | 7.12 | 5.39 | 4.49 | 5.06 | 5.39 | 6.25 | 5.39 | 4.80 |
| Angle deviation, 95th percentile (F) | 22.21 | 22.89 | 16.58 | 11.75 | 13.49 | 15.59 | 19.36 | 15.59 | 13.43 |
| Mean aspect ratio (F) | 1.201 | 1.225 | 1.445 | 1.380 | 1.415 | 1.530 | 1.451 | 1.530 | 1.419 |
| Area variation (F) | 0.159 | 0.197 | 0.357 | 0.435 | 0.433 | 0.413 | 0.359 | 0.413 | 0.449 |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 28.68 | 19.09 | 4.85 | 5.18 | 7.54 | 7.25 | 7.54 | 4.39 |
| Mean distance, ‰ (H) | 0.966 | 1.021 | 0.443 | 0.384 | 0.390 | 1.012 | 1.114 | 1.012 | 0.398 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.35 | 98.23 | 100.00 | 100.05 | 100.03 | 97.96 | 97.78 | 97.96 | 100.05 |
| Off budget, % (N) | 2.8 | 2.4 | 13.9 | 0.7 | 0.9 | 4.6 | 10.7 | 0.0 | 0.3 |
| Time, seconds (T) | 1.43 | 1.10 | 2.90 | 1.27 | 1.53 | 12.87 | 16.31 | 12.87 | 3.07 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 847 | 996 | 888 | 1027 | 984 | 1638 | 892 | 1638 | 1663 |
| Irregular vertices (S) | 44 | 82 | 35 | 40 | 50 | 60 | 53 | 60 | 50 |
| … of which above the minimum | 32 | 74 | — | 32 | 42 | 52 | 45 | 52 | 42 |
| Base complex patches (C) | 785 | 980 | — | 463 | 532 | 836 | 440 | 836 | 1069 |
| Alignment, degrees (A) | 16.32 | 17.18 | 9.17 | 8.33 | 9.03 | 4.88 | 6.96 | 5.29 | 8.06 |
| Mean angle deviation, degrees (F) | 10.04 | 7.61 | 8.65 | 4.92 | 5.70 | 6.85 | 8.39 | 6.85 | 5.03 |
| Angle deviation, 95th percentile (F) | 28.85 | 26.75 | 23.66 | 14.45 | 17.17 | 19.46 | 22.44 | 19.46 | 15.73 |
| Mean aspect ratio (F) | 1.336 | 1.285 | 1.331 | 1.283 | 1.313 | 2.117 | 1.414 | 2.117 | 1.322 |
| Area variation (F) | 0.267 | 0.235 | 0.263 | 0.350 | 0.464 | 0.568 | 0.310 | 0.568 | 0.413 |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 37.01 | 27.91 | 22.61 | 15.87 | 8.09 | 29.46 | 8.09 | 13.14 |
| Mean distance, ‰ (H) | 1.731 | 1.524 | 1.011 | 0.655 | 0.653 | 0.521 | 1.283 | 0.521 | 0.327 |
| Thin parts: mean distance, % of a quad edge (V) | 30.4 | 26.6 | 23.7 | 12.1 | 8.2 | 4.0 | 21.3 | 6.8 | 7.1 |
| Volume kept, % of the source (V) | 96.59 | 97.11 | 99.24 | 99.68 | 99.74 | 98.99 | 97.39 | 98.99 | 99.90 |
| Off budget, % (N) | 15.3 | 1.5 | 11.2 | 2.7 | 1.6 | 63.8 | 10.8 | 0.0 | 1.5 |
| Time, seconds (T) | 1.50 | 1.14 | 3.05 | 1.62 | 2.58 | 14.40 | 12.45 | 14.40 | 2.58 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 861 | 910 | 877 | 989 | 995 | 1975 | 853 | 1975 | 1953 |
| Irregular vertices (S) | 92 | 94 | 53 | 60 | 92 | 95 | 85 | 95 | 94 |
| … of which above the minimum | 76 | 75 | 45 | 52 | 84 | 87 | — | 87 | 86 |
| Base complex patches (C) | 848 | 861 | 455 | 587 | 599 | 1149 | — | 1149 | 1181 |
| Alignment, degrees (A) | 17.33 | 16.11 | 13.09 | 12.40 | 9.42 | 7.39 | 10.36 | 7.52 | 10.16 |
| Mean angle deviation, degrees (F) | 15.46 | 8.44 | 8.76 | 7.11 | 7.29 | 8.27 | 9.33 | 8.27 | 6.09 |
| Angle deviation, 95th percentile (F) | 46.32 | 30.16 | 25.74 | 23.49 | 22.01 | 23.65 | 27.06 | 23.65 | 18.37 |
| Mean aspect ratio (F) | 1.533 | 1.339 | 1.359 | 1.332 | 1.457 | 2.045 | 1.425 | 2.045 | 1.297 |
| Area variation (F) | 0.437 | 0.272 | 0.274 | 0.325 | 0.865 | 0.753 | 0.323 | 0.753 | 0.514 |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 97.30 | 33.69 | 27.73 | 18.24 | 11.45 | 85.93 | 11.45 | 21.90 |
| Mean distance, ‰ (H) | 4.074 | 5.299 | 1.631 | 1.047 | 1.040 | 0.786 | 2.893 | 0.786 | 0.474 |
| Thin parts: mean distance, % of a quad edge (V) | 84.1 | 119.6 | 22.6 | 13.1 | 4.7 | 3.5 | 50.0 | 6.0 | 7.3 |
| Volume kept, % of the source (V) | 96.89 | 96.53 | 98.78 | 99.45 | 99.75 | 98.97 | 97.29 | 98.97 | 99.93 |
| Off budget, % (N) | 13.9 | 9.0 | 12.3 | 1.1 | 0.5 | 97.5 | 14.7 | 0.0 | 1.1 |
| Time, seconds (T) | 1.55 | 1.07 | 4.82 | 2.25 | 2.46 | 17.10 | 15.43 | 17.10 | 2.65 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 3/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 |
| Quads made | 900 | 886 | 867 | 1004 | 999 | 3093 | 876 | 3093 | 3050 |
| Irregular vertices (S) | 69 | 108 | 50 | 51 | 83 | 182 | 150 | 182 | 172 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 724 | 849 | 661 | 609 | 909 | — | — | — | 2620 |
| Alignment, degrees (A) | 18.48 | 19.26 | 16.55 | 14.39 | 15.54 | 12.26 | 14.41 | 12.79 | 14.08 |
| Mean angle deviation, degrees (F) | 10.23 | 11.03 | 11.24 | 7.86 | 9.19 | 10.70 | 12.37 | 10.70 | 7.00 |
| Angle deviation, 95th percentile (F) | 28.90 | 36.55 | 30.16 | 21.83 | 28.00 | 30.61 | 34.62 | 30.61 | 21.68 |
| Mean aspect ratio (F) | 1.359 | 1.449 | 1.397 | 1.436 | 1.461 | 2.192 | 1.561 | 2.192 | 1.347 |
| Area variation (F) | 0.306 | 0.338 | 0.295 | 0.324 | 0.735 | 0.752 | 0.389 | 0.752 | 0.525 |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 84.04 | 69.02 | 36.46 | 25.80 | 7.66 | 49.73 | 7.66 | 26.17 |
| Mean distance, ‰ (H) | 4.435 | 6.117 | 4.705 | 3.005 | 2.875 | 0.950 | 3.575 | 0.950 | 0.839 |
| Thin parts: mean distance, % of a quad edge (V) | 28.0 | 42.2 | 40.7 | 21.9 | 13.4 | 4.8 | 18.4 | 9.8 | 16.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 10.0 | 11.4 | 13.3 | 0.4 | 0.1 | 209.3 | 12.4 | 0.0 | 1.4 |
| Time, seconds (T) | 0.74 | 0.37 | 3.56 | 1.08 | 2.09 | 6.67 | 10.59 | 6.67 | 2.60 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 494 | 530 | 511 | 2617 | 3667 | 26339 | 1271 | 26339 | 28483 |
| Irregular vertices (S) | 164 | 181 | 165 | 797 | 851 | 1682 | 572 | 1682 | 2308 |
| … of which above the minimum | — | — | — | — | — | — | — | — | 1692 |
| Base complex patches (C) | 453 | 500 | — | — | — | — | — | — | 22726 |
| Alignment, degrees (A) | 18.41 | 18.59 | 15.78 | 15.65 | 13.28 | 9.46 | 14.37 | 12.49 | 9.77 |
| Mean angle deviation, degrees (F) | 18.81 | 20.14 | 17.11 | 24.90 | 18.03 | 9.03 | 16.83 | 9.03 | 7.62 |
| Angle deviation, 95th percentile (F) | 52.35 | 58.52 | 51.04 | 68.54 | 56.91 | 26.22 | 49.51 | 26.22 | 24.04 |
| Mean aspect ratio (F) | 1.970 | 2.108 | 1.882 | 8.610 | 3.737 | 1.662 | 1.788 | 1.662 | 1.515 |
| Area variation (F) | 0.599 | 0.707 | 0.540 | 2.773 | 4.004 | 1.972 | 1.056 | 1.972 | 0.570 |
| Hausdorff, ‰ of the diagonal (H) | 76.78 | 97.09 | 113.33 | 43.69 | 51.05 | 74.63 | 78.15 | 74.63 | 7.95 |
| Mean distance, ‰ (H) | 13.479 | 14.528 | 10.097 | 6.600 | 8.156 | 1.049 | 5.982 | 1.049 | 0.200 |
| Thin parts: mean distance, % of a quad edge (V) | 64.1 | 71.0 | 47.9 | 30.0 | 40.9 | 5.8 | 28.8 | 91.4 | 8.6 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | 99.57 |
| Off budget, % (N) | 50.6 | 47.0 | 48.9 | 161.7 | 266.7 | 2533.9 | 27.1 | 0.0 | 8.1 |
| Time, seconds (T) | 2.12 | 1.63 | 5.94 | 4.58 | 6.23 | 14.03 | 9.27 | 14.03 | 20.22 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | — | 845 | 919 | 1008 | 1008 | 2105 | 936 | 2105 | 2120 |
| Irregular vertices (S) | — | 83 | 28 | 36 | 36 | 76 | 54 | 76 | 36 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 532 | 177 | 254 | 451 | 1346 | — | 1346 | 603 |
| Alignment, degrees (A) | — | 13.44 | 7.94 | 6.96 | 6.65 | 6.49 | 9.01 | 7.14 | 7.40 |
| Mean angle deviation, degrees (F) | — | 8.67 | 7.16 | 4.11 | 5.57 | 8.08 | 9.87 | 8.08 | 4.40 |
| Angle deviation, 95th percentile (F) | — | 30.33 | 20.15 | 12.89 | 17.02 | 22.80 | 25.01 | 22.80 | 12.30 |
| Mean aspect ratio (F) | — | 1.363 | 1.329 | 1.284 | 1.330 | 2.085 | 1.391 | 2.085 | 1.265 |
| Area variation (F) | — | 0.276 | 0.284 | 0.270 | 0.535 | 0.486 | 0.271 | 0.486 | 0.430 |
| Hausdorff, ‰ of the diagonal (H) | — | 53.13 | 13.94 | 17.46 | 9.08 | 5.71 | 13.65 | 5.71 | 7.75 |
| Mean distance, ‰ (H) | — | 2.255 | 1.095 | 0.891 | 0.814 | 0.640 | 1.473 | 0.640 | 0.555 |
| Thin parts: mean distance, % of a quad edge (V) | — | 48.5 | 6.1 | 4.6 | 2.8 | 3.4 | 5.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 15.5 | 8.1 | 0.8 | 0.8 | 110.5 | 6.4 | 0.0 | 0.7 |
| Time, seconds (T) | — | 1.83 | 5.17 | 2.20 | 3.40 | 13.07 | 11.79 | 13.07 | 3.38 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 943 | 1036 | 959 | 976 | 1018 | 1013 | 924 | 1013 | 1018 |
| Irregular vertices (S) | 30 | 115 | 24 | 24 | 24 | 26 | 24 | 26 | 24 |
| … of which above the minimum | 22 | 107 | 16 | 16 | 16 | 18 | 16 | 18 | 16 |
| Base complex patches (C) | 227 | 630 | 388 | 86 | 68 | 367 | 293 | 367 | 68 |
| Alignment, degrees (A) | 13.41 | 14.60 | 12.82 | 12.46 | 10.80 | 11.10 | 10.93 | 11.10 | 10.82 |
| Mean angle deviation, degrees (F) | 3.78 | 8.58 | 4.85 | 2.07 | 2.16 | 4.39 | 4.53 | 4.39 | 2.17 |
| Angle deviation, 95th percentile (F) | 15.04 | 35.19 | 15.54 | 9.04 | 10.18 | 14.60 | 15.17 | 14.60 | 10.14 |
| Mean aspect ratio (F) | 1.191 | 1.407 | 1.208 | 1.219 | 1.177 | 1.323 | 1.314 | 1.323 | 1.177 |
| Area variation (F) | 0.176 | 0.263 | 0.143 | 0.175 | 0.381 | 0.296 | 0.273 | 0.296 | 0.381 |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 20.39 | 17.19 | 14.36 | 1.37 | 1.64 | 2.29 | 1.64 | 1.37 |
| Mean distance, ‰ (H) | 0.335 | 0.238 | 0.679 | 0.259 | 0.165 | 0.147 | 0.173 | 0.147 | 0.165 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.43 | 100.43 | 99.90 | 100.01 | 100.01 | 100.26 | 100.31 | 100.26 | 100.01 |
| Off budget, % (N) | 5.7 | 3.6 | 4.1 | 2.4 | 1.8 | 1.3 | 7.6 | 0.0 | 0.5 |
| Time, seconds (T) | 0.70 | 0.15 | 8.06 | 1.48 | 1.54 | 3.06 | 2.65 | 3.06 | 1.82 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 0.33 | 0.06 | 2.94 | 0.70 | 1.06 | 2.35 | 3.01 | 2.35 | 0.85 |

## lblock (mechanical)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 0.23 | 0.05 | 3.81 | 0.64 | 0.69 | 2.79 | 2.67 | 2.79 | 0.70 |

## cylinder (mechanical)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 995 | 994 | 900 | 990 | 1002 | 782 | 782 | 782 | 764 |
| Irregular vertices (S) | 8 | 40 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 32 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 576 | 46 | 6 | 186 | 138 | 138 | 138 | 56 |
| Alignment, degrees (A) | 3.95 | 4.86 | 1.52 | 1.23 | 1.85 | 1.55 | 1.55 | 1.67 | 1.37 |
| Mean angle deviation, degrees (F) | 4.50 | 5.03 | 3.70 | 2.64 | 3.73 | 4.01 | 4.01 | 4.01 | 2.54 |
| Angle deviation, 95th percentile (F) | 17.64 | 20.26 | 12.51 | 7.63 | 8.67 | 11.66 | 11.66 | 11.66 | 9.04 |
| Mean aspect ratio (F) | 1.124 | 1.170 | 1.306 | 1.286 | 1.504 | 1.528 | 1.528 | 1.528 | 1.509 |
| Area variation (F) | 0.112 | 0.174 | 0.337 | 0.334 | 0.218 | 0.416 | 0.416 | 0.416 | 0.205 |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 15.50 | 1.57 | 1.47 | 1.70 | 2.59 | 2.59 | 2.59 | 1.93 |
| Mean distance, ‰ (H) | 0.373 | 0.376 | 0.312 | 0.311 | 0.349 | 0.938 | 0.938 | 0.938 | 0.457 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.73 | 99.66 | 99.99 | 99.99 | 100.02 | 99.00 | 99.00 | 99.00 | 99.99 |
| Off budget, % (N) | 1.8 | 1.0 | 10.0 | 1.0 | 0.2 | 21.8 | 21.8 | 0.0 | 2.3 |
| Time, seconds (T) | 0.33 | 0.05 | 15.50 | 1.66 | 0.77 | 2.92 | 2.88 | 2.92 | 0.74 |

## venturi (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 639 | 742 | 757 | 836 | 12157 | 8690 | 12157 | 12482 |
| Irregular vertices (S) | — | 184 | 113 | 150 | 176 | 606 | 581 | 606 | 874 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 460 | — | — | — | — | — | — | 10508 |
| Alignment, degrees (A) | — | 12.88 | 14.40 | 11.32 | 13.71 | 6.94 | 6.56 | 8.37 | 11.12 |
| Mean angle deviation, degrees (F) | — | 17.66 | 9.85 | 11.89 | 14.03 | 10.01 | 10.58 | 10.01 | 7.75 |
| Angle deviation, 95th percentile (F) | — | 59.41 | 30.81 | 37.21 | 45.64 | 45.68 | 51.58 | 45.68 | 29.08 |
| Mean aspect ratio (F) | — | 2.099 | 1.393 | 1.553 | 3.065 | 2.423 | 2.658 | 2.423 | 1.535 |
| Area variation (F) | — | 0.480 | 0.363 | 0.486 | 0.595 | 2.342 | 2.356 | 2.342 | 0.384 |
| Hausdorff, ‰ of the diagonal (H) | — | 68.00 | 81.94 | 82.83 | 82.83 | 14.79 | 27.48 | 14.79 | 21.31 |
| Mean distance, ‰ (H) | — | 12.935 | 11.384 | 7.832 | 8.266 | 0.450 | 0.882 | 0.450 | 1.281 |
| Thin parts: mean distance, % of a quad edge (V) | — | 39.2 | 34.9 | 21.1 | 22.9 | 1.4 | 2.2 | 3.4 | 22.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 36.1 | 25.8 | 24.3 | 16.4 | 1115.7 | 769.0 | 0.0 | 2.7 |
| Time, seconds (T) | — | 0.60 | 7.70 | 8.77 | 11.32 | 6.97 | 3.89 | 6.97 | 92.32 |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | — | 940 | 743 | 1019 | 1011 | 2921 | 812 | 2921 | 2887 |
| Irregular vertices (S) | — | 121 | 44 | 46 | 118 | 229 | 126 | 229 | 209 |
| … of which above the minimum | — | 112 | 36 | 38 | 110 | 221 | — | 221 | 201 |
| Base complex patches (C) | — | 910 | 363 | 707 | 733 | 2159 | — | 2159 | 2357 |
| Alignment, degrees (A) | — | 19.57 | 14.37 | 14.59 | 15.15 | 10.54 | 14.09 | 10.54 | 13.02 |
| Mean angle deviation, degrees (F) | — | 10.46 | 10.54 | 8.13 | 9.38 | 9.73 | 11.98 | 9.73 | 6.98 |
| Angle deviation, 95th percentile (F) | — | 32.04 | 28.35 | 22.79 | 26.32 | 26.74 | 31.12 | 26.74 | 20.40 |
| Mean aspect ratio (F) | — | 1.433 | 1.615 | 1.668 | 1.560 | 2.436 | 1.706 | 2.436 | 1.431 |
| Area variation (F) | — | 0.296 | 0.399 | 0.375 | 0.892 | 0.740 | 0.435 | 0.740 | 0.776 |
| Hausdorff, ‰ of the diagonal (H) | — | 81.49 | 38.88 | 31.77 | 30.52 | 29.33 | 57.04 | 29.33 | 24.12 |
| Mean distance, ‰ (H) | — | 3.775 | 2.299 | 1.647 | 1.906 | 0.976 | 3.129 | 0.976 | 0.786 |
| Thin parts: mean distance, % of a quad edge (V) | — | 99.1 | 37.7 | 22.8 | 5.9 | 3.8 | 47.2 | 7.0 | 3.8 |
| Volume kept, % of the source (V) | — | 95.96 | 99.19 | 99.58 | 99.83 | 98.60 | — | 98.60 | 100.06 |
| Off budget, % (N) | — | 6.0 | 25.7 | 1.9 | 1.1 | 192.1 | 18.8 | 0.0 | 1.2 |
| Time, seconds (T) | — | 99.29 | 66.59 | 59.54 | 66.02 | 36.93 | 32.56 | 36.93 | 58.30 |

Failures: quadriflow: cancelled.

## terrain (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 872 | 574 | 896 | 980 | 1000 | 1541 | 878 | 1541 | 1512 |
| Irregular vertices (S) | 44 | 40 | 34 | 34 | 38 | 152 | 88 | 152 | 42 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 307 | 394 | 326 | 500 | 646 | 1446 | — | 1446 | 1026 |
| Alignment, degrees (A) | 19.25 | 18.99 | 17.91 | 18.03 | 17.66 | 18.91 | 20.03 | 18.89 | 18.51 |
| Mean angle deviation, degrees (F) | 6.74 | 8.62 | 10.03 | 5.77 | 6.04 | 11.26 | 10.74 | 11.26 | 5.65 |
| Angle deviation, 95th percentile (F) | 23.83 | 29.38 | 28.12 | 17.77 | 18.76 | 32.77 | 31.81 | 32.77 | 16.71 |
| Mean aspect ratio (F) | 1.223 | 1.338 | 1.361 | 1.319 | 1.321 | 1.859 | 1.521 | 1.859 | 1.293 |
| Area variation (F) | 0.195 | 0.224 | 0.321 | 0.353 | 0.460 | 0.545 | 0.321 | 0.545 | 0.454 |
| Hausdorff, ‰ of the diagonal (H) | 30.58 | 146.80 | 30.18 | 23.57 | 26.88 | 18.41 | 31.35 | 18.41 | 31.13 |
| Mean distance, ‰ (H) | 3.158 | 4.386 | 2.651 | 2.302 | 1.984 | 1.506 | 2.787 | 1.506 | 1.615 |
| Thin parts: mean distance, % of a quad edge (V) | 29.7 | 44.9 | 29.2 | 23.5 | 19.4 | 11.3 | 27.0 | 14.3 | 19.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 12.8 | 42.6 | 10.4 | 2.0 | 0.0 | 54.1 | 12.2 | 0.0 | 1.9 |
| Time, seconds (T) | 19.49 | 8.90 | 8.59 | 7.51 | 6.50 | 19.65 | 21.87 | 19.65 | 9.49 |

## mahaveer (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 562 | 532 | 2852 | 2549 | 41821 | 2688 | 41821 | 50888 |
| Irregular vertices (S) | — | 124 | 45 | 421 | 412 | 3109 | 1013 | 3109 | 1168 |
| … of which above the minimum | — | 13 | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 423 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 11.52 | 8.55 | 13.96 | 18.43 | 15.01 | 14.68 | 14.20 | 6.10 |
| Mean angle deviation, degrees (F) | — | 15.07 | 11.18 | 33.85 | 32.03 | 13.01 | 16.98 | 13.01 | 5.75 |
| Angle deviation, 95th percentile (F) | — | 53.29 | 35.63 | 80.40 | 85.66 | 40.35 | 58.74 | 40.35 | 22.22 |
| Mean aspect ratio (F) | — | 2.321 | 1.724 | 81.719 | 142.939 | 1.768 | 2.072 | 1.768 | 8.081 |
| Area variation (F) | — | 0.516 | 0.635 | 9.307 | 9.147 | 3.743 | 1.645 | 3.743 | 0.776 |
| Hausdorff, ‰ of the diagonal (H) | — | 69.46 | 47.45 | 325.50 | 325.41 | 13.85 | 49.37 | 13.85 | 13.25 |
| Mean distance, ‰ (H) | — | 10.323 | 7.114 | 39.894 | 40.004 | 1.081 | 4.277 | 1.081 | 0.593 |
| Thin parts: mean distance, % of a quad edge (V) | — | 50.2 | 35.7 | 153.8 | 153.8 | 7.4 | 22.4 | 50.7 | 34.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 43.8 | 46.8 | 185.2 | 154.9 | 4082.1 | 168.8 | 0.0 | 21.7 |
| Time, seconds (T) | — | 5.58 | 12.95 | 9.25 | 11.20 | 16.61 | 15.33 | 16.61 | 122.19 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 204 | — | 9809 | 10461 | 205185 | 10623 | 205185 | 218256 |
| Irregular vertices (S) | — | 139 | — | 5951 | 5883 | 10625 | 4798 | 10625 | 11766 |
| … of which above the minimum | — | -56 | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 204 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 19.32 | — | — | — | 10.09 | — | 13.06 | 8.37 |
| Mean angle deviation, degrees (F) | — | 30.35 | — | 19.90 | 17.98 | 10.19 | 17.58 | 10.19 | 5.02 |
| Angle deviation, 95th percentile (F) | — | 69.83 | — | 68.45 | 65.68 | 30.61 | 52.92 | 30.61 | 18.65 |
| Mean aspect ratio (F) | — | 3.003 | — | 4.452 | 3.478 | 1.591 | 1.958 | 1.591 | 1.376 |
| Area variation (F) | — | 0.795 | — | 3.184 | 3.405 | 3.347 | 3.276 | 3.347 | 0.541 |
| Hausdorff, ‰ of the diagonal (H) | — | 10.79 | — | 7.06 | 6.60 | 2.06 | 5.15 | 2.06 | 3.01 |
| Mean distance, ‰ (H) | — | 3.973 | — | 0.439 | 0.457 | 0.152 | 0.565 | 0.152 | 0.035 |
| Thin parts: mean distance, % of a quad edge (V) | — | 70.3 | — | 8.1 | 8.5 | 5.2 | 11.1 | 66.6 | 17.6 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 79.6 | — | 880.9 | 946.1 | 20418.5 | 962.3 | 0.0 | 6.4 |
| Time, seconds (T) | — | 3.43 | — | 12.81 | 18.74 | 14.01 | 8.85 | 14.01 | 51.02 |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 8 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 3/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 465 | 603 | 1724 | 1660 | 30430 | 4601 | 30430 | 33408 |
| Irregular vertices (S) | — | 163 | 181 | 511 | 334 | 2729 | 1219 | 2729 | 850 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 442 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 15.03 | 10.78 | 4.81 | 4.76 | 5.04 | 8.21 | 5.05 | 3.47 |
| Mean angle deviation, degrees (F) | — | 23.16 | 17.86 | 10.50 | 10.42 | 11.44 | 15.59 | 11.44 | 5.41 |
| Angle deviation, 95th percentile (F) | — | 66.13 | 54.34 | 42.43 | 42.24 | 42.16 | 46.05 | 42.16 | 19.67 |
| Mean aspect ratio (F) | — | 3.120 | 2.286 | 180.679 | 299.438 | 2.473 | 3.146 | 2.473 | 18.763 |
| Area variation (F) | — | 0.584 | 0.550 | 5.948 | 5.992 | 2.700 | 2.263 | 2.700 | 0.765 |
| Hausdorff, ‰ of the diagonal (H) | — | 144.73 | 94.72 | 117.05 | 117.05 | 10.84 | 83.07 | 10.84 | 29.55 |
| Mean distance, ‰ (H) | — | 32.571 | 17.454 | 8.116 | 8.146 | 0.478 | 4.559 | 0.478 | 1.109 |
| Thin parts: mean distance, % of a quad edge (V) | — | 129.3 | 70.7 | 30.4 | 30.5 | 2.5 | 18.5 | 13.8 | 40.4 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 53.5 | 39.7 | 72.4 | 66.0 | 2943.0 | 360.1 | 0.0 | 9.8 |
| Time, seconds (T) | — | 4.02 | 56.49 | 17.50 | 19.37 | 11.27 | 10.16 | 11.27 | 108.60 |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 508 | 843 | 1301 | 1293 | 9385 | 3134 | 9385 | 8526 |
| Irregular vertices (S) | — | 126 | 82 | 107 | 83 | 368 | 289 | 368 | 210 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 428 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.38 | 8.25 | 4.89 | 4.91 | 4.16 | 4.27 | 3.37 | 4.57 |
| Mean angle deviation, degrees (F) | — | 18.32 | 9.97 | 9.25 | 9.22 | 7.93 | 11.16 | 7.93 | 5.36 |
| Angle deviation, 95th percentile (F) | — | 55.68 | 36.03 | 37.18 | 37.06 | 28.21 | 42.77 | 28.21 | 20.50 |
| Mean aspect ratio (F) | — | 2.053 | 1.774 | 5.218 | 5.605 | 10.123 | 26.996 | 10.123 | 2.231 |
| Area variation (F) | — | 0.710 | 0.410 | 1.154 | 1.138 | 2.280 | 1.420 | 2.280 | 0.562 |
| Hausdorff, ‰ of the diagonal (H) | — | 81.73 | 35.86 | 44.99 | 44.99 | 21.81 | 36.33 | 21.81 | 29.03 |
| Mean distance, ‰ (H) | — | 10.964 | 4.688 | 5.227 | 5.227 | 1.030 | 2.230 | 1.030 | 1.140 |
| Thin parts: mean distance, % of a quad edge (V) | — | 20.7 | 10.1 | 12.0 | 12.0 | 2.4 | 5.4 | 7.1 | 13.3 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 49.2 | 15.7 | 30.1 | 29.3 | 838.5 | 213.4 | 0.0 | 9.2 |
| Time, seconds (T) | — | 0.51 | 21.84 | 5.80 | 5.93 | 5.98 | 5.10 | 5.98 | 37.39 |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 866 | 744 | 759 | 752 | 7098 | 803 | 7098 | 5955 |
| Irregular vertices (S) | — | 181 | 74 | 126 | 126 | 725 | 192 | 725 | 418 |
| … of which above the minimum | — | 172 | — | 62 | 62 | — | — | — | — |
| Base complex patches (C) | — | 810 | — | 465 | 458 | — | — | — | — |
| Alignment, degrees (A) | — | 14.10 | 10.16 | 9.99 | 10.11 | 9.10 | 10.26 | 10.66 | 10.58 |
| Mean angle deviation, degrees (F) | — | 14.78 | 12.69 | 13.62 | 13.11 | 9.97 | 13.02 | 9.97 | 9.51 |
| Angle deviation, 95th percentile (F) | — | 45.64 | 33.98 | 39.94 | 36.87 | 29.98 | 40.40 | 29.98 | 28.14 |
| Mean aspect ratio (F) | — | 1.759 | 1.600 | 1.995 | 1.981 | 1.773 | 1.662 | 1.773 | 1.475 |
| Area variation (F) | — | 0.411 | 0.449 | 0.494 | 0.485 | 1.130 | 0.547 | 1.130 | 0.342 |
| Hausdorff, ‰ of the diagonal (H) | — | 52.06 | 48.50 | 40.53 | 40.53 | 28.38 | 45.02 | 28.38 | 20.49 |
| Mean distance, ‰ (H) | — | 12.293 | 6.257 | 5.461 | 5.421 | 1.253 | 6.755 | 1.253 | 0.843 |
| Thin parts: mean distance, % of a quad edge (V) | — | 121.8 | 59.0 | 50.6 | 50.0 | 17.0 | 61.2 | 50.3 | 39.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 13.4 | 25.6 | 24.1 | 24.8 | 609.8 | 19.7 | 0.0 | 16.1 |
| Time, seconds (T) | — | 0.19 | 5.77 | 6.60 | 7.90 | 27.33 | 36.82 | 27.33 | 79.48 |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 950 | 1066 | 923 | 1010 | 1010 | 915 | 915 | 915 | 900 |
| Irregular vertices (S) | 8 | 44 | 8 | 8 | 8 | 10 | 10 | 10 | 10 |
| … of which above the minimum | 0 | 36 | 0 | 0 | 0 | 2 | 2 | 2 | 2 |
| Base complex patches (C) | 937 | 1066 | 923 | 1010 | 1010 | 699 | 699 | 699 | 846 |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.18 | 8.36 | 4.61 | 4.61 | 6.03 | 6.03 | 6.03 | 4.08 |
| Angle deviation, 95th percentile (F) | 12.31 | 17.01 | 22.46 | 11.13 | 11.13 | 14.57 | 14.57 | 14.57 | 10.24 |
| Mean aspect ratio (F) | 1.148 | 1.153 | 1.171 | 1.180 | 1.180 | 1.213 | 1.213 | 1.213 | 1.153 |
| Area variation (F) | 0.136 | 0.179 | 0.152 | 0.225 | 0.225 | 0.318 | 0.318 | 0.318 | 0.240 |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 1.68 | 1.11 | 0.99 | 0.99 | 1.91 | 1.91 | 1.91 | 2.07 |
| Mean distance, ‰ (H) | 0.670 | 0.632 | 0.356 | 0.332 | 0.332 | 0.762 | 0.762 | 0.762 | 0.357 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.38 | 99.38 | 100.02 | 100.02 | 100.02 | 99.24 | 99.24 | 99.24 | 100.02 |
| Off budget, % (N) | 5.0 | 6.6 | 7.7 | 1.0 | 1.0 | 8.5 | 8.5 | 0.0 | 1.6 |
| Time, seconds (T) | 0.41 | 0.19 | 1.75 | 1.01 | 0.93 | 4.77 | 3.75 | 4.77 | 1.03 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
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
| Time, seconds (T) | 0.31 | 0.27 | 1.02 | 0.98 | 1.10 | 4.41 | 2.61 | 4.41 | 0.78 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 4 | 0 | 7 | 4 | 1 | 0 | 0 | 0 | 1 |
| Base complex patches (C) | 1 | 0 | 4 | 3 | 1 | 0 | 4 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 1 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 4 | 2 | 0 | 0 | 0 | 7 |
| Mean aspect ratio (F) | 4 | 1 | 0 | 2 | 0 | 0 | 0 | 0 | 6 |
| Valid meshes (V) | 38/65 | 44/65 | 7/13 | 12/13 | 12/13 | 11/13 | 8/13 | 11/13 | 12/13 |

### mechanical (4 models)

| Metric | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 4 | 4 | 4 | 3 | 4 | 3 | 4 |
| Base complex patches (C) | 1 | 0 | 0 | 3 | 3 | 2 | 2 | 2 | 3 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 1 | 2 | 2 | 2 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 2 | 0 | 1 | 1 | 1 | 1 |
| Mean aspect ratio (F) | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 0 |
| Valid meshes (V) | 20/20 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
