# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 941 | 400 | 1036 | 867 | 1002 | 1010 | 1362 | 878 | 1362 | 1377 | — | — |
| Irregular vertices (S) | 36 | 24 | 76 | 24 | 24 | 24 | 26 | 26 | 26 | 24 | — | — |
| … of which above the minimum | 28 | 16 | 68 | 16 | 16 | 16 | 18 | 18 | 18 | 16 | — | — |
| Base complex patches (C) | 941 | 314 | 1036 | 601 | 290 | 392 | 520 | 434 | 520 | 769 | — | — |
| Alignment, degrees (A) | 18.04 | 8.83 | 17.88 | 7.53 | 5.37 | 5.10 | 3.42 | 4.94 | 3.46 | 4.83 | — | — |
| Mean angle deviation, degrees (F) | 9.26 | 12.46 | 7.67 | 9.62 | 6.87 | 7.07 | 6.86 | 6.31 | 6.86 | 6.35 | — | — |
| Angle deviation, 95th percentile (F) | 25.27 | 33.98 | 23.42 | 24.97 | 17.86 | 17.94 | 20.12 | 18.61 | 20.12 | 16.86 | — | — |
| Mean aspect ratio (F) | 1.221 | 1.652 | 1.235 | 1.436 | 1.434 | 1.421 | 2.760 | 1.379 | 2.760 | 1.436 | — | — |
| Area variation (F) | 0.218 | 0.577 | 0.218 | 0.385 | 0.476 | 0.465 | 0.625 | 0.353 | 0.625 | 0.471 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 14.52 | 16.95 | 15.11 | 17.75 | 10.47 | 8.04 | 7.74 | 19.23 | 7.74 | 4.04 | — | — |
| Mean distance, ‰ (H) | 1.325 | 5.131 | 1.340 | 0.706 | 0.540 | 0.575 | 1.172 | 1.534 | 1.172 | 0.379 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.58 | 92.19 | 98.90 | 100.06 | 100.06 | 100.06 | 98.08 | 98.43 | 98.08 | 100.05 | — | — |
| Edges on closed loops, % (L) | 0.0 | 10.8 | — | — | 40.6 | 31.0 | 33.7 | — | — | 22.1 | — | — |
| Edges on spiralling loops, % (L) | 25.3 | 35.2 | — | — | 6.2 | 7.0 | 18.2 | — | — | 38.5 | — | — |
| Spiralling loops (L) | 4 | 7 | — | — | 2 | 1 | 4 | — | — | 7 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.7 | 54.0 | — | — | 53.1 | 62.0 | 48.1 | — | — | 39.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 108 | 44 | — | — | 100 | 100 | 113 | — | — | 217 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.9 | 60.0 | 3.6 | 13.3 | 0.2 | 1.0 | 36.2 | 12.2 | 0.0 | 1.1 | — | — |
| Time, seconds (T) | 1.82 | 0.43 | 1.70 | 4.13 | 2.40 | 2.58 | 14.01 | 13.25 | 14.01 | 2.63 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 947 | 778 | 966 | 898 | 1011 | 988 | 1290 | 837 | 1290 | 1279 | — | — |
| Irregular vertices (S) | 8 | 10 | 40 | 26 | 28 | 28 | 90 | 84 | 90 | 36 | — | — |
| … of which above the minimum | 0 | 2 | 32 | 18 | 20 | 20 | 82 | 76 | 82 | 28 | — | — |
| Base complex patches (C) | 941 | 506 | 912 | 898 | 1011 | 883 | 1290 | 837 | 1290 | 1279 | — | — |
| Alignment, degrees (A) | 23.14 | 19.26 | 23.62 | 16.22 | 16.67 | 16.75 | 13.52 | 14.33 | 14.01 | 16.51 | — | — |
| Mean angle deviation, degrees (F) | 6.16 | 7.46 | 6.09 | 9.43 | 6.33 | 5.93 | 11.01 | 11.93 | 11.01 | 6.05 | — | — |
| Angle deviation, 95th percentile (F) | 16.14 | 18.13 | 18.18 | 23.72 | 16.00 | 15.47 | 27.61 | 30.02 | 27.61 | 15.88 | — | — |
| Mean aspect ratio (F) | 1.162 | 1.494 | 1.188 | 1.321 | 1.326 | 1.301 | 1.631 | 1.472 | 1.631 | 1.293 | — | — |
| Area variation (F) | 0.179 | 0.473 | 0.178 | 0.313 | 0.335 | 0.343 | 0.354 | 0.299 | 0.354 | 0.323 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 9.38 | 14.09 | 10.32 | 11.07 | 7.00 | 7.86 | 9.88 | 12.74 | 9.88 | 6.14 | — | — |
| Mean distance, ‰ (H) | 1.512 | 2.480 | 1.641 | 1.335 | 1.206 | 1.209 | 1.291 | 1.761 | 1.291 | 1.065 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.30 | 98.22 | 99.04 | 99.94 | 100.02 | 100.02 | 99.46 | 98.87 | 99.46 | 100.04 | — | — |
| Edges on closed loops, % (L) | 0.0 | 3.7 | — | — | 0.0 | 5.4 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 60.3 | 69.2 | — | — | 0.0 | 11.6 | 3.8 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 3 | 6 | — | — | 0 | 2 | 1 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 35.9 | 27.1 | — | — | 100.0 | 83.0 | 96.2 | — | — | 100.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 133 | 82 | — | — | 170 | 106 | 75 | — | — | 190 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.3 | 22.2 | 3.4 | 10.2 | 1.1 | 1.2 | 29.0 | 16.3 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | 1.10 | 0.33 | 1.20 | 5.26 | 2.09 | 2.93 | 28.00 | 16.00 | 28.00 | 2.50 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 812 | 604 | 994 | 920 | 1009 | 1011 | 1208 | 928 | 1208 | 1198 | — | — |
| Irregular vertices (S) | 16 | 12 | 51 | 24 | 26 | 38 | 42 | 39 | 42 | 40 | — | — |
| … of which above the minimum | 8 | 4 | 43 | — | 18 | 30 | 34 | 31 | 34 | 32 | — | — |
| Base complex patches (C) | 812 | 308 | 822 | 666 | 313 | 381 | 588 | 346 | 588 | 420 | — | — |
| Alignment, degrees (A) | 14.79 | 11.08 | 11.93 | 8.02 | 7.95 | 6.13 | 4.65 | 4.83 | 4.72 | 5.74 | — | — |
| Mean angle deviation, degrees (F) | 12.55 | 6.78 | 6.23 | 7.31 | 4.98 | 5.47 | 6.49 | 6.99 | 6.49 | 4.92 | — | — |
| Angle deviation, 95th percentile (F) | 30.83 | 16.26 | 18.64 | 22.70 | 13.66 | 17.12 | 18.32 | 21.32 | 18.32 | 14.94 | — | — |
| Mean aspect ratio (F) | 1.391 | 1.697 | 1.185 | 1.250 | 1.237 | 1.306 | 1.473 | 1.394 | 1.473 | 1.309 | — | — |
| Area variation (F) | 0.298 | 0.418 | 0.183 | 0.288 | 0.387 | 0.478 | 0.461 | 0.380 | 0.461 | 0.474 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 40.53 | 8.78 | 18.99 | 12.89 | 4.41 | 4.19 | 3.74 | 6.58 | 3.74 | 3.43 | — | — |
| Mean distance, ‰ (H) | 1.486 | 1.732 | 0.917 | 0.516 | 0.434 | 0.454 | 0.604 | 0.932 | 0.604 | 0.406 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 97.34 | 96.54 | 98.23 | — | 100.08 | 100.07 | 98.75 | 98.01 | 98.75 | 100.04 | — | — |
| Edges on closed loops, % (L) | 0.0 | 24.5 | — | — | 36.0 | 31.2 | 25.7 | — | — | 32.5 | — | — |
| Edges on spiralling loops, % (L) | 43.6 | 54.3 | — | — | 4.3 | 0.0 | 17.5 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 6 | 4 | — | — | 1 | 0 | 7 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 56.4 | 21.2 | — | — | 59.7 | 68.8 | 56.8 | — | — | 67.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 141 | 52 | — | — | 91 | 80 | 84 | — | — | 94 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 18.8 | 39.6 | 1.4 | 8.0 | 0.9 | 1.1 | 20.8 | 7.2 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 1.18 | 0.45 | 1.03 | 4.64 | 3.23 | 2.56 | 11.95 | 8.72 | 11.95 | 2.59 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1043 | 1720 | 974 | 970 | 1019 | 1008 | 3224 | 934 | 3224 | 3197 | — | — |
| Irregular vertices (S) | 64 | 52 | 136 | 40 | 48 | 48 | 61 | 49 | 61 | 48 | — | — |
| … of which above the minimum | 32 | — | 104 | 8 | 16 | 16 | 29 | 17 | 29 | 16 | — | — |
| Base complex patches (C) | 716 | — | 779 | 96 | 173 | 152 | 741 | 211 | 741 | 425 | — | — |
| Alignment, degrees (A) | 7.66 | 8.80 | 10.19 | 3.01 | 4.03 | 2.89 | 2.70 | 3.17 | 2.73 | 2.22 | — | — |
| Mean angle deviation, degrees (F) | 6.96 | 10.60 | 9.36 | 4.97 | 4.45 | 4.62 | 4.46 | 6.04 | 4.46 | 3.15 | — | — |
| Angle deviation, 95th percentile (F) | 21.11 | 30.15 | 30.34 | 16.26 | 14.11 | 14.77 | 15.38 | 19.11 | 15.38 | 9.81 | — | — |
| Mean aspect ratio (F) | 1.250 | 2.067 | 1.402 | 1.236 | 1.236 | 1.241 | 2.906 | 1.395 | 2.906 | 1.231 | — | — |
| Area variation (F) | 0.155 | 0.381 | 0.279 | 0.284 | 0.233 | 0.344 | 0.384 | 0.250 | 0.384 | 0.343 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 21.45 | 52.83 | 28.51 | 17.39 | 17.54 | 11.01 | 7.11 | 31.90 | 7.11 | 10.08 | — | — |
| Mean distance, ‰ (H) | 1.546 | 2.978 | 2.710 | 1.750 | 2.201 | 1.309 | 0.849 | 2.892 | 0.849 | 0.509 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.12 | 96.35 | 95.62 | 99.67 | 99.64 | 99.93 | 99.30 | 95.23 | 99.30 | 99.97 | — | — |
| Edges on closed loops, % (L) | 17.2 | 16.0 | — | — | 58.4 | 61.0 | 48.0 | — | — | 61.6 | — | — |
| Edges on spiralling loops, % (L) | 23.4 | 36.5 | — | — | 8.9 | 3.0 | 28.6 | — | — | 18.2 | — | — |
| Spiralling loops (L) | 10 | 20 | — | — | 3 | 1 | 12 | — | — | 5 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 58.8 | 47.5 | — | — | 32.7 | 36.1 | 23.4 | — | — | 20.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 47 | 74 | — | — | 18 | 24 | 64 | — | — | 69 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 4.3 | 72.0 | 2.6 | 3.0 | 1.9 | 0.8 | 222.4 | 6.6 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 0.38 | 0.75 | 0.57 | 3.70 | 1.10 | 1.41 | 5.08 | 4.00 | 5.08 | 3.88 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1005 | 357 | 958 | 976 | 1005 | 985 | 1790 | 922 | 1790 | 1797 | — | — |
| Irregular vertices (S) | 67 | 48 | 114 | 51 | 44 | 48 | 68 | 68 | 68 | 50 | — | — |
| … of which above the minimum | — | 24 | 98 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 995 | 357 | 930 | 733 | 587 | 582 | 1538 | 802 | 1538 | 855 | — | — |
| Alignment, degrees (A) | 15.73 | 18.27 | 17.84 | 11.65 | 11.62 | 11.07 | 8.58 | 9.64 | 8.91 | 11.05 | — | — |
| Mean angle deviation, degrees (F) | 9.65 | 15.83 | 10.18 | 9.03 | 4.75 | 5.08 | 8.74 | 8.64 | 8.74 | 4.83 | — | — |
| Angle deviation, 95th percentile (F) | 29.23 | 49.29 | 34.26 | 26.04 | 16.41 | 16.90 | 23.66 | 23.37 | 23.66 | 14.75 | — | — |
| Mean aspect ratio (F) | 1.325 | 2.035 | 1.423 | 1.311 | 1.262 | 1.253 | 1.823 | 1.345 | 1.823 | 1.261 | — | — |
| Area variation (F) | 0.506 | 0.760 | 0.395 | 0.357 | 0.249 | 0.383 | 0.551 | 0.247 | 0.551 | 0.383 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 44.79 | 137.08 | 50.97 | 36.86 | 21.06 | 15.63 | 10.72 | 17.45 | 10.72 | 11.32 | — | — |
| Mean distance, ‰ (H) | 2.245 | 13.357 | 3.552 | 2.048 | 1.913 | 1.387 | 1.112 | 2.422 | 1.112 | 0.810 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 10.6 | 112.8 | 23.9 | 12.5 | 12.9 | 4.7 | 4.8 | 12.7 | 6.4 | 4.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.0 | — | — | 16.3 | 16.7 | 4.2 | — | — | 22.4 | — | — |
| Edges on spiralling loops, % (L) | 7.9 | 5.7 | — | — | 0.0 | 3.2 | 15.1 | — | — | 3.4 | — | — |
| Spiralling loops (L) | 5 | 2 | — | — | 0 | 1 | 7 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.6 | 94.3 | — | — | 75.4 | 74.4 | 77.2 | — | — | 67.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 97 | 43 | — | — | 54 | 57 | 112 | — | — | 136 | — | — |
| Edges on loops from a border to a border, % (L) | 0.6 | 0.0 | — | — | 8.3 | 5.7 | 3.5 | — | — | 6.8 | — | — |
| Off budget, % (N) | 1.0 | 64.3 | 4.2 | 2.4 | 0.5 | 1.5 | 79.0 | 7.8 | 0.0 | 0.4 | — | — |
| Time, seconds (T) | 0.66 | 0.43 | 0.30 | 5.95 | 2.40 | 2.18 | 4.46 | 3.77 | 4.46 | 2.52 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 891 | 311 | 932 | 802 | 1019 | 1009 | 2890 | 799 | 2890 | 2901 | — | — |
| Irregular vertices (S) | 73 | 36 | 114 | 47 | 46 | 108 | 245 | 142 | 245 | 149 | — | — |
| … of which above the minimum | 64 | — | 99 | — | 38 | 100 | 237 | — | 237 | 141 | — | — |
| Base complex patches (C) | 891 | — | 860 | — | 385 | 537 | 2186 | — | 2186 | 2567 | — | — |
| Alignment, degrees (A) | 20.86 | 17.10 | 18.87 | 11.86 | 10.58 | 11.41 | 7.89 | 10.49 | 9.35 | 11.70 | — | — |
| Mean angle deviation, degrees (F) | 14.18 | 13.19 | 9.68 | 11.53 | 7.41 | 9.57 | 9.34 | 12.33 | 9.34 | 6.51 | — | — |
| Angle deviation, 95th percentile (F) | 39.23 | 35.34 | 29.97 | 29.24 | 20.66 | 27.42 | 26.48 | 34.04 | 26.48 | 18.44 | — | — |
| Mean aspect ratio (F) | 1.552 | 1.883 | 1.419 | 1.628 | 1.658 | 3.184 | 2.393 | 1.743 | 2.393 | 1.443 | — | — |
| Area variation (F) | 0.348 | 0.671 | 0.282 | 0.388 | 0.444 | 0.833 | 0.728 | 0.444 | 0.728 | 0.508 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 126.19 | 85.71 | 33.77 | 28.38 | 29.60 | 27.76 | 51.64 | 27.76 | 30.56 | — | — |
| Mean distance, ‰ (H) | 4.418 | 13.988 | 3.773 | 2.160 | 1.666 | 1.815 | 0.968 | 3.165 | 0.968 | 0.729 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 115.0 | 180.3 | 107.2 | 35.4 | 19.9 | 5.8 | 4.5 | 50.1 | 8.9 | 8.1 | — | — |
| Volume kept, % of the source (V) | 94.57 | 87.06 | 95.94 | 99.36 | 99.55 | 99.72 | 98.71 | — | 98.71 | 99.99 | — | — |
| Edges on closed loops, % (L) | 0.3 | 7.8 | — | — | 31.1 | 23.4 | 12.2 | — | — | 5.8 | — | — |
| Edges on spiralling loops, % (L) | 10.9 | 25.0 | — | — | 6.8 | 3.7 | 9.3 | — | — | 26.5 | — | — |
| Spiralling loops (L) | 6 | 4 | — | — | 5 | 6 | 9 | — | — | 29 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.1 | 67.2 | — | — | 62.1 | 72.9 | 78.4 | — | — | 67.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 102 | 17 | — | — | 116 | 59 | 124 | — | — | 129 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 10.9 | 68.9 | 6.8 | 19.8 | 1.9 | 0.9 | 189.0 | 20.1 | 0.0 | 0.4 | — | — |
| Time, seconds (T) | 3.32 | 0.64 | 2.71 | 4.70 | 5.80 | 4.96 | 18.93 | 16.67 | 18.93 | 9.84 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 176 | 868 | 613 | 594 | 984 | 2672 | 920 | 2672 | 2667 | 10590 | 9426 |
| Irregular vertices (S) | — | 38 | 96 | 46 | 45 | 112 | 212 | 86 | 212 | 147 | 476 | 141 |
| … of which above the minimum | — | — | 76 | 38 | 37 | 104 | — | — | — | 139 | — | 133 |
| Base complex patches (C) | — | — | 578 | 301 | 242 | 474 | — | — | — | 1035 | — | 5382 |
| Alignment, degrees (A) | — | 11.90 | 11.10 | 7.73 | 8.20 | 9.17 | 5.77 | 6.19 | 6.29 | 7.80 | 9.88 | 8.45 |
| Mean angle deviation, degrees (F) | — | 16.80 | 9.77 | 9.59 | 10.35 | 7.81 | 6.83 | 8.06 | 6.83 | 5.63 | 9.24 | 6.33 |
| Angle deviation, 95th percentile (F) | — | 55.64 | 35.19 | 30.11 | 30.04 | 24.26 | 23.37 | 27.56 | 23.37 | 19.61 | 26.27 | 18.39 |
| Mean aspect ratio (F) | — | 3.300 | 1.497 | 1.661 | 1.607 | 1.556 | 2.345 | 1.522 | 2.345 | 1.387 | 1.616 | 1.333 |
| Area variation (F) | — | 0.883 | 0.327 | 0.631 | 0.551 | 1.022 | 0.665 | 0.385 | 0.665 | 0.655 | 1.417 | 0.294 |
| Hausdorff, ‰ of the diagonal (H) | — | 160.81 | 101.17 | 35.69 | 35.34 | 20.24 | 9.99 | 47.40 | 9.99 | 13.61 | 6.47 | 8.89 |
| Mean distance, ‰ (H) | — | 58.173 | 13.524 | 4.905 | 5.078 | 2.136 | 1.006 | 5.690 | 1.006 | 1.176 | 0.172 | 0.557 |
| Thin parts: mean distance, % of a quad edge (V) | — | 331.3 | 119.6 | 43.9 | 45.3 | 12.4 | 4.6 | 52.8 | 8.0 | 15.0 | 1.5 | 25.0 |
| Volume kept, % of the source (V) | — | 51.85 | 90.43 | 98.89 | 99.15 | 99.54 | 98.23 | — | 98.23 | 100.01 | 98.80 | 99.93 |
| Edges on closed loops, % (L) | — | 21.5 | 15.7 | 25.4 | 29.6 | 25.9 | 27.1 | — | 27.1 | 30.6 | 31.6 | 22.1 |
| Edges on spiralling loops, % (L) | — | 35.4 | 7.9 | 23.2 | 18.4 | 15.3 | 13.9 | — | 13.9 | 5.3 | 2.5 | 32.6 |
| Spiralling loops (L) | — | 18 | 13 | 9 | 7 | 32 | 52 | — | 52 | 13 | 3 | 44 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 43.1 | 78.4 | 51.3 | 51.9 | 58.7 | 59.0 | — | 59.0 | 64.1 | 65.8 | 45.2 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 16 | 40 | 50 | 32 | 64 | 107 | — | 107 | 237 | 274 | 467 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 82.4 | 13.2 | 38.7 | 40.6 | 1.6 | 167.2 | 8.0 | 0.0 | 0.2 | 0.0 | 11.0 |
| Time, seconds (T) | — | 4.03 | 19.84 | 13.59 | 29.56 | 16.45 | 22.46 | 24.37 | 22.46 | 16.10 | — | 74.91 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 172 | 877 | 869 | 1003 | 982 | 2809 | 845 | 2809 | 2753 | 10590 | 10602 |
| Irregular vertices (S) | — | 38 | 105 | 47 | 52 | 108 | 223 | 98 | 223 | 158 | 476 | 280 |
| … of which above the minimum | — | — | 72 | 39 | 44 | 100 | — | — | — | 150 | — | 272 |
| Base complex patches (C) | — | — | 639 | 279 | 305 | 552 | — | — | — | 1135 | — | 3904 |
| Alignment, degrees (A) | — | 12.34 | 11.11 | 6.25 | 6.25 | 9.16 | 5.39 | 5.49 | 5.91 | 7.54 | 9.43 | 7.20 |
| Mean angle deviation, degrees (F) | — | 30.39 | 9.85 | 7.99 | 5.62 | 8.43 | 6.64 | 7.61 | 6.64 | 5.34 | 8.82 | 4.44 |
| Angle deviation, 95th percentile (F) | — | 82.98 | 32.30 | 26.53 | 19.16 | 27.90 | 22.69 | 25.67 | 22.69 | 18.41 | 25.47 | 14.29 |
| Mean aspect ratio (F) | — | 3.326 | 1.445 | 1.445 | 1.430 | 1.552 | 2.429 | 1.466 | 2.429 | 1.335 | 1.657 | 1.281 |
| Area variation (F) | — | 1.040 | 0.326 | 0.428 | 0.446 | 1.137 | 0.650 | 0.347 | 0.650 | 0.652 | 1.432 | 0.482 |
| Hausdorff, ‰ of the diagonal (H) | — | 100.86 | 70.01 | 40.87 | 28.99 | 18.19 | 12.40 | 79.14 | 12.40 | 11.01 | 4.44 | 4.92 |
| Mean distance, ‰ (H) | — | 33.458 | 11.587 | 6.423 | 2.762 | 2.457 | 0.888 | 7.145 | 0.888 | 0.961 | 0.156 | 0.181 |
| Thin parts: mean distance, % of a quad edge (V) | — | 191.1 | 92.0 | 64.6 | 25.0 | 16.2 | 5.5 | 71.9 | 10.1 | 16.7 | 1.4 | 6.1 |
| Volume kept, % of the source (V) | — | 35.72 | 90.12 | 98.59 | 99.88 | 99.41 | 98.29 | 90.99 | 98.29 | 99.94 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | 16.2 | 14.7 | 33.9 | 34.8 | 21.9 | 26.7 | 30.1 | 26.7 | 29.5 | 31.6 | 31.6 |
| Edges on spiralling loops, % (L) | — | 46.8 | 7.6 | 10.7 | 0.9 | 17.3 | 9.8 | 8.5 | 9.8 | 6.1 | 2.5 | 3.5 |
| Spiralling loops (L) | — | 18 | 10 | 11 | 1 | 24 | 48 | 10 | 48 | 13 | 3 | 8 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 37.0 | 79.2 | 55.4 | 64.3 | 60.8 | 63.6 | 61.5 | 63.6 | 64.4 | 65.8 | 64.9 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 11 | 42 | 137 | 128 | 61 | 144 | 101 | 144 | 161 | 274 | 325 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 82.8 | 12.3 | 13.1 | 0.3 | 1.8 | 180.9 | 15.5 | 0.0 | 2.0 | 0.0 | 0.1 |
| Time, seconds (T) | — | 3.40 | 20.81 | 14.45 | 13.46 | 16.84 | 23.55 | 22.00 | 23.55 | 17.18 | — | 33.21 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1024 | 404 | 1016 | 910 | 1008 | 985 | 1497 | 882 | 1497 | 1488 | — | — |
| Irregular vertices (S) | 62 | 39 | 95 | 63 | 62 | 68 | 78 | 76 | 78 | 68 | — | — |
| … of which above the minimum | 54 | — | 87 | 55 | 54 | 60 | 70 | 68 | 70 | 60 | — | — |
| Base complex patches (C) | 965 | — | 982 | 690 | 726 | 537 | 1079 | 630 | 1079 | 762 | — | — |
| Alignment, degrees (A) | 18.23 | 16.98 | 17.84 | 10.63 | 9.78 | 8.27 | 6.34 | 7.17 | 6.45 | 8.34 | — | — |
| Mean angle deviation, degrees (F) | 7.55 | 9.29 | 7.36 | 11.20 | 7.26 | 7.72 | 8.24 | 8.98 | 8.24 | 7.09 | — | — |
| Angle deviation, 95th percentile (F) | 22.42 | 25.28 | 25.61 | 29.66 | 20.10 | 21.86 | 22.62 | 24.90 | 22.62 | 18.89 | — | — |
| Mean aspect ratio (F) | 1.252 | 1.483 | 1.267 | 1.448 | 1.475 | 1.519 | 1.689 | 1.455 | 1.689 | 1.399 | — | — |
| Area variation (F) | 0.209 | 0.498 | 0.211 | 0.350 | 0.397 | 0.566 | 0.555 | 0.343 | 0.555 | 0.471 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 66.11 | 45.10 | 19.66 | 14.22 | 15.02 | 8.32 | 15.39 | 8.32 | 14.00 | — | — |
| Mean distance, ‰ (H) | 1.330 | 5.964 | 1.556 | 0.811 | 0.662 | 0.726 | 0.810 | 1.403 | 0.810 | 0.441 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 29.6 | 41.7 | 37.4 | 23.1 | 11.8 | 15.6 | 12.2 | 14.9 | 14.7 | 20.1 | — | — |
| Volume kept, % of the source (V) | 98.16 | 91.73 | 97.96 | 99.85 | 100.00 | 100.02 | 98.52 | 97.64 | 98.52 | 100.01 | — | — |
| Edges on closed loops, % (L) | 1.8 | 2.1 | — | — | 14.0 | 22.7 | 14.0 | — | — | 25.2 | — | — |
| Edges on spiralling loops, % (L) | 13.9 | 4.8 | — | — | 3.7 | 0.0 | 5.8 | — | — | 0.5 | — | — |
| Spiralling loops (L) | 5 | 2 | — | — | 3 | 0 | 3 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.6 | 93.0 | — | — | 82.3 | 77.3 | 80.2 | — | — | 74.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 48 | — | — | 114 | 52 | 77 | — | — | 96 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 59.6 | 1.6 | 9.0 | 0.8 | 1.5 | 49.7 | 11.8 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | 1.53 | 0.40 | 2.26 | 4.55 | 2.45 | 3.36 | 17.76 | 13.95 | 17.76 | 4.09 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 972 | 576 | 1006 | 861 | 1019 | 1009 | 954 | 893 | 954 | 936 | — | — |
| Irregular vertices (S) | 28 | 12 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 | — | — |
| … of which above the minimum | 28 | 12 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 | — | — |
| Base complex patches (C) | 958 | 125 | 1006 | 177 | 145 | 179 | 398 | 235 | 398 | 242 | — | — |
| Alignment, degrees (A) | 20.00 | 7.87 | 20.88 | 4.19 | 4.13 | 3.87 | 3.16 | 3.69 | 3.14 | 3.92 | — | — |
| Mean angle deviation, degrees (F) | 7.93 | 9.54 | 7.12 | 5.39 | 4.29 | 4.45 | 5.39 | 6.25 | 5.39 | 4.63 | — | — |
| Angle deviation, 95th percentile (F) | 22.21 | 23.72 | 22.89 | 16.58 | 11.71 | 12.96 | 15.59 | 19.36 | 15.59 | 12.95 | — | — |
| Mean aspect ratio (F) | 1.201 | 2.466 | 1.225 | 1.445 | 1.381 | 1.383 | 1.530 | 1.451 | 1.530 | 1.435 | — | — |
| Area variation (F) | 0.159 | 0.448 | 0.197 | 0.357 | 0.463 | 0.447 | 0.413 | 0.359 | 0.413 | 0.435 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 11.15 | 28.68 | 19.09 | 5.64 | 3.90 | 7.54 | 7.25 | 7.54 | 3.50 | — | — |
| Mean distance, ‰ (H) | 0.966 | 2.793 | 1.021 | 0.443 | 0.382 | 0.387 | 1.012 | 1.114 | 1.012 | 0.400 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.35 | 94.76 | 98.23 | 100.00 | 100.05 | 100.04 | 97.96 | 97.78 | 97.96 | 100.04 | — | — |
| Edges on closed loops, % (L) | 0.0 | 49.1 | — | — | 65.4 | 59.4 | 36.2 | — | — | 53.1 | — | — |
| Edges on spiralling loops, % (L) | 27.7 | 17.7 | — | — | 0.0 | 0.0 | 32.4 | — | — | 3.3 | — | — |
| Spiralling loops (L) | 5 | 2 | — | — | 0 | 0 | 5 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 71.2 | 33.2 | — | — | 34.6 | 40.6 | 31.4 | — | — | 43.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 76 | — | — | 71 | 67 | 71 | — | — | 64 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 42.4 | 2.4 | 13.9 | 1.9 | 0.9 | 4.6 | 10.7 | 0.0 | 1.9 | — | — |
| Time, seconds (T) | 1.49 | 0.45 | 1.10 | 2.90 | 2.75 | 2.04 | 12.87 | 16.31 | 12.87 | 3.07 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 847 | 604 | 996 | 888 | 990 | 989 | 1638 | 892 | 1638 | 1626 | — | — |
| Irregular vertices (S) | 44 | 58 | 82 | 35 | 42 | 48 | 60 | 53 | 60 | 48 | — | — |
| … of which above the minimum | 32 | — | 74 | — | 34 | 40 | 52 | 45 | 52 | 40 | — | — |
| Base complex patches (C) | 785 | — | 980 | — | 324 | 357 | 836 | 440 | 836 | 444 | — | — |
| Alignment, degrees (A) | 16.32 | 14.60 | 17.18 | 9.17 | 8.91 | 8.59 | 4.88 | 6.96 | 5.29 | 7.93 | — | — |
| Mean angle deviation, degrees (F) | 10.04 | 10.41 | 7.61 | 8.65 | 5.66 | 6.10 | 6.85 | 8.39 | 6.85 | 4.98 | — | — |
| Angle deviation, 95th percentile (F) | 28.85 | 32.33 | 26.75 | 23.66 | 15.57 | 18.18 | 19.46 | 22.44 | 19.46 | 15.32 | — | — |
| Mean aspect ratio (F) | 1.336 | 1.771 | 1.285 | 1.331 | 1.297 | 1.421 | 2.117 | 1.414 | 2.117 | 1.300 | — | — |
| Area variation (F) | 0.267 | 0.634 | 0.235 | 0.263 | 0.314 | 0.459 | 0.568 | 0.310 | 0.568 | 0.426 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 70.95 | 37.01 | 27.91 | 22.78 | 13.23 | 8.09 | 29.46 | 8.09 | 9.48 | — | — |
| Mean distance, ‰ (H) | 1.731 | 4.041 | 1.524 | 1.011 | 0.673 | 0.605 | 0.521 | 1.283 | 0.521 | 0.288 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 30.4 | 31.8 | 26.6 | 23.7 | 13.1 | 5.7 | 4.0 | 21.3 | 6.8 | 5.5 | — | — |
| Volume kept, % of the source (V) | 96.59 | — | 97.11 | 99.24 | 99.65 | 99.80 | 98.99 | 97.39 | 98.99 | 99.96 | — | — |
| Edges on closed loops, % (L) | 4.2 | 6.4 | — | — | 38.9 | 33.9 | 24.5 | — | — | 42.4 | — | — |
| Edges on spiralling loops, % (L) | 19.3 | 7.7 | — | — | 0.0 | 0.0 | 18.7 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 7 | 3 | — | — | 0 | 0 | 6 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.3 | 85.9 | — | — | 61.1 | 66.1 | 56.8 | — | — | 57.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 87 | 42 | — | — | 106 | 97 | 84 | — | — | 116 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 15.3 | 39.6 | 1.5 | 11.2 | 1.0 | 1.1 | 63.8 | 10.8 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 1.97 | 1.39 | 1.14 | 3.05 | 2.58 | 3.21 | 14.40 | 12.45 | 14.40 | 3.43 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 861 | 548 | 910 | 877 | 980 | 1007 | 1975 | 853 | 1975 | 2002 | — | — |
| Irregular vertices (S) | 92 | 69 | 94 | 53 | 56 | 92 | 95 | 85 | 95 | 94 | — | — |
| … of which above the minimum | 76 | — | 75 | 45 | 48 | 84 | 87 | — | 87 | 86 | — | — |
| Base complex patches (C) | 848 | — | 861 | 455 | 552 | 541 | 1149 | — | 1149 | 1176 | — | — |
| Alignment, degrees (A) | 17.33 | 14.59 | 16.11 | 13.09 | 12.04 | 9.25 | 7.39 | 10.36 | 7.52 | 10.01 | — | — |
| Mean angle deviation, degrees (F) | 15.46 | 12.13 | 8.44 | 8.76 | 8.17 | 7.28 | 8.27 | 9.33 | 8.27 | 6.39 | — | — |
| Angle deviation, 95th percentile (F) | 46.32 | 38.98 | 30.16 | 25.74 | 23.44 | 22.18 | 23.65 | 27.06 | 23.65 | 18.59 | — | — |
| Mean aspect ratio (F) | 1.533 | 1.947 | 1.339 | 1.359 | 1.541 | 1.381 | 2.045 | 1.425 | 2.045 | 1.309 | — | — |
| Area variation (F) | 0.437 | 0.836 | 0.272 | 0.274 | 0.403 | 0.913 | 0.753 | 0.323 | 0.753 | 0.502 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 63.50 | 97.30 | 33.69 | 30.05 | 13.78 | 11.45 | 85.93 | 11.45 | 12.13 | — | — |
| Mean distance, ‰ (H) | 4.074 | 8.335 | 5.299 | 1.631 | 1.205 | 1.034 | 0.786 | 2.893 | 0.786 | 0.441 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 84.1 | 44.9 | 119.6 | 22.6 | 13.6 | 3.8 | 3.5 | 50.0 | 6.0 | 4.7 | — | — |
| Volume kept, % of the source (V) | 96.89 | 90.89 | 96.53 | 98.78 | 99.40 | 99.80 | 98.97 | 97.29 | 98.97 | 99.91 | — | — |
| Edges on closed loops, % (L) | 0.9 | 13.1 | — | — | 23.3 | 24.1 | 20.9 | — | — | 22.1 | — | — |
| Edges on spiralling loops, % (L) | 14.4 | 4.5 | — | — | 3.3 | 3.1 | 7.0 | — | — | 3.3 | — | — |
| Spiralling loops (L) | 9 | 4 | — | — | 4 | 1 | 4 | — | — | 7 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 83.9 | 82.4 | — | — | 73.4 | 72.7 | 72.1 | — | — | 74.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 94 | 24 | — | — | 104 | 74 | 130 | — | — | 160 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 13.9 | 45.2 | 9.0 | 12.3 | 2.0 | 0.7 | 97.5 | 14.7 | 0.0 | 1.4 | — | — |
| Time, seconds (T) | 2.16 | 0.55 | 1.07 | 4.82 | 2.90 | 4.63 | 17.10 | 15.43 | 17.10 | 4.39 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 3/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 900 | 305 | 886 | 867 | 1020 | 1007 | 3093 | 876 | 3093 | 3035 | — | — |
| Irregular vertices (S) | 69 | 33 | 108 | 50 | 42 | 94 | 182 | 150 | 182 | 184 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 724 | — | 849 | 661 | 489 | 985 | — | — | — | 2754 | — | — |
| Alignment, degrees (A) | 18.49 | 20.52 | 19.26 | 16.55 | 14.99 | 15.13 | 12.26 | 14.41 | 12.79 | 14.69 | — | — |
| Mean angle deviation, degrees (F) | 10.23 | 18.53 | 11.03 | 11.24 | 6.74 | 9.73 | 10.70 | 12.37 | 10.70 | 7.12 | — | — |
| Angle deviation, 95th percentile (F) | 28.90 | 57.15 | 36.55 | 30.16 | 20.16 | 27.84 | 30.61 | 34.62 | 30.61 | 22.82 | — | — |
| Mean aspect ratio (F) | 1.359 | 1.887 | 1.449 | 1.397 | 1.353 | 1.471 | 2.192 | 1.561 | 2.192 | 1.356 | — | — |
| Area variation (F) | 0.306 | 1.686 | 0.338 | 0.295 | 0.339 | 0.726 | 0.752 | 0.389 | 0.752 | 0.499 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 201.66 | 84.04 | 69.02 | 37.04 | 27.77 | 7.66 | 49.73 | 7.66 | 22.81 | — | — |
| Mean distance, ‰ (H) | 4.435 | 12.497 | 6.117 | 4.705 | 3.379 | 2.893 | 0.950 | 3.575 | 0.950 | 0.820 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 28.0 | 71.5 | 42.2 | 40.7 | 21.9 | 14.2 | 4.8 | 18.4 | 9.8 | 16.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 5.0 | 0.4 | 1.1 | — | — | 1.6 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 19.9 | — | — | 0.0 | 1.3 | 6.9 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 0 | 3 | — | — | 0 | 1 | 8 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.7 | 80.1 | — | — | 68.7 | 97.6 | 91.4 | — | — | 95.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | 24 | — | — | 66 | 47 | 129 | — | — | 117 | — | — |
| Edges on loops from a border to a border, % (L) | 14.3 | 0.0 | — | — | 26.3 | 0.7 | 0.7 | — | — | 3.1 | — | — |
| Off budget, % (N) | 10.0 | 69.5 | 11.4 | 13.3 | 2.0 | 0.7 | 209.3 | 12.4 | 0.0 | 1.9 | — | — |
| Time, seconds (T) | 0.81 | 0.36 | 0.37 | 3.56 | 1.70 | 2.99 | 6.67 | 10.59 | 6.67 | 5.41 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 494 | 157 | 530 | 511 | 1562 | 1463 | 26339 | 1271 | 26339 | 26365 | — | — |
| Irregular vertices (S) | 164 | 122 | 181 | 165 | 682 | 690 | 1682 | 572 | 1682 | 2270 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | 1654 | — | — |
| Base complex patches (C) | 453 | — | 500 | — | — | — | — | — | — | 18489 | — | — |
| Alignment, degrees (A) | 18.87 | 18.25 | 18.59 | 15.78 | 13.44 | — | 9.47 | 14.37 | 12.49 | 10.01 | — | — |
| Mean angle deviation, degrees (F) | 18.64 | 49.67 | 20.14 | 17.11 | 28.57 | 30.91 | 9.03 | 16.83 | 9.03 | 8.08 | — | — |
| Angle deviation, 95th percentile (F) | 52.24 | 87.99 | 58.52 | 51.04 | 74.43 | 76.52 | 26.22 | 49.51 | 26.22 | 25.17 | — | — |
| Mean aspect ratio (F) | 2.033 | 8.023 | 2.108 | 1.882 | 15.780 | 163.708 | 1.662 | 1.788 | 1.662 | 1.764 | — | — |
| Area variation (F) | 0.598 | 1.952 | 0.707 | 0.540 | 2.518 | 2.873 | 1.972 | 1.056 | 1.972 | 0.598 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 76.78 | 195.94 | 97.09 | 113.33 | 68.09 | 69.78 | 74.63 | 78.15 | 74.63 | 7.86 | — | — |
| Mean distance, ‰ (H) | 13.492 | 46.413 | 14.528 | 10.097 | 16.740 | 18.453 | 1.049 | 5.982 | 1.049 | 0.223 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 64.1 | 184.7 | 71.0 | 47.9 | 73.8 | 78.2 | 5.8 | 28.8 | 91.4 | 9.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 16.90 | — | — | — | — | 99.48 | — | — |
| Edges on closed loops, % (L) | 3.9 | — | — | — | 2.8 | 3.7 | — | — | — | 15.1 | — | — |
| Edges on spiralling loops, % (L) | 19.8 | — | — | — | 31.2 | 34.8 | — | — | — | 9.3 | — | — |
| Spiralling loops (L) | 46 | — | — | — | 386 | 389 | — | — | — | 234 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.7 | — | — | — | 66.0 | 61.5 | — | — | — | 75.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 16 | — | — | — | 19 | 26 | — | — | — | 166 | — | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | 50.6 | 84.3 | 47.0 | 48.9 | 56.2 | 46.3 | 2533.9 | 27.1 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | 2.21 | 0.41 | 1.63 | 5.94 | 4.31 | 4.29 | 14.03 | 9.27 | 14.03 | 47.82 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 748 | 845 | 919 | 1011 | 984 | 2105 | 936 | 2105 | 2115 | — | — |
| Irregular vertices (S) | — | 54 | 83 | 28 | 34 | 34 | 76 | 54 | 76 | 37 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 532 | 177 | 238 | 259 | 1346 | — | 1346 | 542 | — | — |
| Alignment, degrees (A) | — | 11.10 | 13.44 | 7.94 | 7.17 | 6.66 | 6.48 | 9.01 | 7.14 | 7.28 | — | — |
| Mean angle deviation, degrees (F) | — | 16.94 | 8.67 | 7.16 | 3.64 | 4.95 | 8.08 | 9.87 | 8.08 | 4.00 | — | — |
| Angle deviation, 95th percentile (F) | — | 65.68 | 30.33 | 20.15 | 12.02 | 15.09 | 22.80 | 25.01 | 22.80 | 12.22 | — | — |
| Mean aspect ratio (F) | — | 2.304 | 1.363 | 1.329 | 1.254 | 1.276 | 2.085 | 1.391 | 2.085 | 1.254 | — | — |
| Area variation (F) | — | 1.107 | 0.276 | 0.284 | 0.306 | 0.452 | 0.486 | 0.271 | 0.486 | 0.386 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 98.43 | 53.13 | 13.94 | 17.46 | 17.46 | 5.71 | 13.65 | 5.71 | 7.02 | — | — |
| Mean distance, ‰ (H) | — | 7.488 | 2.255 | 1.095 | 0.950 | 0.889 | 0.640 | 1.473 | 0.640 | 0.555 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 15.2 | 48.5 | 6.1 | 4.5 | 2.6 | 3.4 | 5.6 | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.1 | — | — | 17.5 | 24.7 | 10.9 | — | — | 24.9 | — | — |
| Edges on spiralling loops, % (L) | — | 9.4 | — | — | 0.0 | 0.0 | 16.9 | — | — | 1.8 | — | — |
| Spiralling loops (L) | — | 4 | — | — | 0 | 0 | 7 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 71.5 | — | — | 50.8 | 54.1 | 62.9 | — | — | 51.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 41 | — | — | 46 | 48 | 108 | — | — | 78 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 31.7 | 21.2 | 9.3 | — | — | 21.5 | — | — |
| Off budget, % (N) | — | 25.2 | 15.5 | 8.1 | 1.1 | 1.6 | 110.5 | 6.4 | 0.0 | 0.5 | — | — |
| Time, seconds (T) | — | 0.55 | 1.83 | 5.17 | 3.69 | 4.16 | 13.07 | 11.79 | 13.07 | 4.77 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 943 | 348 | 1036 | 959 | 988 | 992 | 1013 | 924 | 1013 | 1008 | — | — |
| Irregular vertices (S) | 30 | 69 | 115 | 24 | 24 | 24 | 26 | 24 | 26 | 24 | — | — |
| … of which above the minimum | 22 | — | 107 | 16 | 16 | 16 | 18 | 16 | 18 | 16 | — | — |
| Base complex patches (C) | 227 | — | 630 | 388 | 66 | 68 | 367 | 293 | 367 | 68 | — | — |
| Alignment, degrees (A) | 13.41 | 19.45 | 14.60 | 12.82 | 13.91 | 10.88 | 11.10 | 10.93 | 11.10 | 10.84 | — | — |
| Mean angle deviation, degrees (F) | 3.78 | 19.61 | 8.58 | 4.85 | 2.31 | 2.15 | 4.39 | 4.53 | 4.39 | 2.15 | — | — |
| Angle deviation, 95th percentile (F) | 15.04 | 60.84 | 35.19 | 15.54 | 10.46 | 10.45 | 14.60 | 15.17 | 14.60 | 10.48 | — | — |
| Mean aspect ratio (F) | 1.191 | 2.662 | 1.407 | 1.208 | 1.259 | 1.175 | 1.323 | 1.314 | 1.323 | 1.164 | — | — |
| Area variation (F) | 0.176 | 0.647 | 0.263 | 0.143 | 0.211 | 0.388 | 0.296 | 0.273 | 0.296 | 0.406 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 93.30 | 20.39 | 17.19 | 15.37 | 1.49 | 1.64 | 2.29 | 1.64 | 1.48 | — | — |
| Mean distance, ‰ (H) | 0.335 | 8.243 | 0.238 | 0.679 | 0.360 | 0.173 | 0.147 | 0.173 | 0.147 | 0.181 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.43 | — | 100.43 | 99.90 | 99.97 | 100.01 | 100.26 | 100.31 | 100.26 | 100.01 | — | — |
| Edges on closed loops, % (L) | 47.9 | — | — | — | 73.6 | 74.2 | 35.9 | — | — | 74.4 | — | — |
| Edges on spiralling loops, % (L) | 13.6 | — | — | — | 0.0 | 0.0 | 28.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 4 | — | — | — | 0 | 0 | 7 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 36.6 | — | — | — | 26.4 | 25.8 | 36.0 | — | — | 25.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 36 | — | — | — | 32 | 24 | 50 | — | — | 24 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.7 | 65.2 | 3.6 | 4.1 | 1.2 | 0.8 | 1.3 | 7.6 | 0.0 | 0.5 | — | — |
| Time, seconds (T) | 0.97 | 0.32 | 0.15 | 8.06 | 1.41 | 1.81 | 3.06 | 2.65 | 3.06 | 1.87 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1014 | 573 | 900 | 999 | 1014 | 1014 | 1014 | 1014 | 1014 | 1014 | — | — |
| Irregular vertices (S) | 8 | 13 | 16 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | — | 174 | 999 | 6 | 6 | 6 | 6 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.17 | 3.25 | 1.29 | 1.55 | 0.11 | 0.25 | 0.11 | 0.11 | 0.11 | 0.25 | — | — |
| Mean angle deviation, degrees (F) | 0.08 | 4.61 | 2.07 | 2.64 | 0.00 | 0.33 | 0.00 | 0.00 | 0.00 | 0.33 | — | — |
| Angle deviation, 95th percentile (F) | 0.23 | 14.18 | 7.03 | 6.80 | 0.01 | 0.76 | 0.00 | 0.00 | 0.00 | 0.76 | — | — |
| Mean aspect ratio (F) | 1.003 | 1.707 | 1.093 | 1.086 | 1.000 | 1.071 | 1.000 | 1.000 | 1.000 | 1.071 | — | — |
| Area variation (F) | 0.002 | 0.500 | 0.103 | 0.074 | 0.000 | 0.081 | 0.000 | 0.000 | 0.000 | 0.081 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 2.31 | 23.35 | 0.00 | 25.45 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.015 | 1.194 | 0.000 | 0.399 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.43 | 100.00 | 99.93 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 92.3 | 44.9 | — | — | 92.3 | 92.3 | 92.3 | — | — | 92.3 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 7.7 | 55.1 | — | — | 7.7 | 7.7 | 7.7 | — | — | 7.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 13 | 40 | — | — | 13 | 13 | 13 | — | — | 13 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.4 | 42.7 | 10.0 | 0.1 | 1.4 | 1.4 | 1.4 | 1.4 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 0.30 | 0.24 | 0.06 | 2.94 | 1.36 | 1.49 | 2.35 | 3.01 | 2.35 | 1.57 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 930 | 588 | 902 | 992 | 994 | 994 | 1010 | 1010 | 1010 | 1028 | — | — |
| Irregular vertices (S) | 12 | 14 | 16 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | 6 | 8 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 85 | 132 | 84 | 26 | 14 | 14 | 14 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 1.36 | 4.35 | 1.05 | 1.80 | 0.56 | 0.64 | 0.56 | 0.56 | 0.56 | 0.64 | — | — |
| Mean angle deviation, degrees (F) | 1.26 | 4.89 | 1.39 | 2.01 | 0.10 | 0.37 | 0.30 | 0.30 | 0.30 | 0.39 | — | — |
| Angle deviation, 95th percentile (F) | 4.73 | 14.86 | 3.73 | 6.27 | 0.33 | 0.90 | 1.26 | 1.26 | 1.26 | 1.01 | — | — |
| Mean aspect ratio (F) | 1.064 | 1.571 | 1.041 | 1.120 | 1.072 | 1.099 | 1.090 | 1.090 | 1.090 | 1.098 | — | — |
| Area variation (F) | 0.059 | 0.415 | 0.057 | 0.080 | 0.070 | 0.096 | 0.053 | 0.053 | 0.053 | 0.096 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 9.75 | 29.11 | 0.00 | 13.72 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.123 | 1.880 | 0.000 | 0.591 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.99 | 99.10 | 100.00 | 99.86 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 68.3 | 52.4 | — | — | 88.1 | 88.1 | 88.2 | — | — | 88.3 | — | — |
| Edges on spiralling loops, % (L) | 13.6 | 11.2 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 1 | 2 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 16.6 | 36.4 | — | — | 11.9 | 11.9 | 11.8 | — | — | 11.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 34 | 50 | — | — | 26 | 25 | 27 | — | — | 25 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 7.0 | 41.2 | 9.8 | 0.8 | 0.6 | 0.6 | 1.0 | 1.0 | 0.0 | 1.8 | — | — |
| Time, seconds (T) | 0.21 | 0.17 | 0.05 | 3.81 | 1.00 | 1.13 | 2.79 | 2.67 | 2.79 | 1.12 | — | — |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 995 | 612 | 994 | 900 | 1000 | 1000 | 782 | 782 | 782 | 781 | — | — |
| Irregular vertices (S) | 8 | 8 | 40 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 32 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 376 | 164 | 576 | 46 | 6 | 6 | 138 | 138 | 138 | 63 | — | — |
| Alignment, degrees (A) | 3.95 | 5.40 | 4.86 | 1.52 | 1.14 | 1.74 | 1.55 | 1.55 | 1.67 | 1.67 | — | — |
| Mean angle deviation, degrees (F) | 4.50 | 8.90 | 5.03 | 3.70 | 1.83 | 2.85 | 4.01 | 4.01 | 4.01 | 2.70 | — | — |
| Angle deviation, 95th percentile (F) | 17.64 | 26.84 | 20.26 | 12.51 | 7.85 | 9.06 | 11.66 | 11.66 | 11.66 | 11.32 | — | — |
| Mean aspect ratio (F) | 1.124 | 1.965 | 1.170 | 1.306 | 1.187 | 1.228 | 1.528 | 1.528 | 1.528 | 1.352 | — | — |
| Area variation (F) | 0.112 | 0.576 | 0.174 | 0.337 | 0.206 | 0.143 | 0.416 | 0.416 | 0.416 | 0.144 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 14.27 | 15.50 | 1.57 | 1.29 | 1.28 | 2.59 | 2.59 | 2.59 | 1.59 | — | — |
| Mean distance, ‰ (H) | 0.373 | 2.062 | 0.376 | 0.312 | 0.264 | 0.278 | 0.938 | 0.938 | 0.938 | 0.364 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.73 | 98.02 | 99.66 | 99.99 | 100.00 | 99.99 | 99.00 | 99.00 | 99.00 | 99.99 | — | — |
| Edges on closed loops, % (L) | 30.7 | 36.6 | — | — | 92.0 | 92.0 | 41.2 | — | — | 61.2 | — | — |
| Edges on spiralling loops, % (L) | 45.4 | 50.2 | — | — | 0.0 | 0.0 | 54.9 | — | — | 26.4 | — | — |
| Spiralling loops (L) | 3 | 4 | — | — | 0 | 0 | 5 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 24.2 | 13.2 | — | — | 8.0 | 8.0 | 3.9 | — | — | 12.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 134 | 30 | — | — | 20 | 20 | 21 | — | — | 54 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.8 | 38.8 | 1.0 | 10.0 | 0.0 | 0.0 | 21.8 | 21.8 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | 0.30 | 0.16 | 0.05 | 15.50 | 1.65 | 1.50 | 2.92 | 2.88 | 2.92 | 1.60 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2390 | 639 | 742 | 589 | 758 | 12157 | 8690 | 12157 | 12459 | — | — |
| Irregular vertices (S) | — | 1182 | 184 | 113 | 154 | 219 | 606 | 581 | 606 | 1011 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 460 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.64 | 12.88 | 14.40 | 13.19 | 14.92 | 6.94 | 6.56 | 8.37 | 11.38 | — | — |
| Mean angle deviation, degrees (F) | — | 53.69 | 17.66 | 9.85 | 16.24 | 18.88 | 10.01 | 10.58 | 10.01 | 7.23 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.78 | 59.41 | 30.81 | 45.22 | 51.72 | 45.68 | 51.58 | 45.68 | 27.02 | — | — |
| Mean aspect ratio (F) | — | 7.059 | 2.099 | 1.393 | 1.861 | 2.274 | 2.423 | 2.658 | 2.423 | 1.462 | — | — |
| Area variation (F) | — | 5.029 | 0.480 | 0.363 | 0.743 | 0.963 | 2.342 | 2.356 | 2.342 | 0.361 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 174.38 | 68.00 | 81.94 | 94.88 | 94.88 | 14.79 | 27.48 | 14.79 | 24.93 | — | — |
| Mean distance, ‰ (H) | — | 17.134 | 12.935 | 11.384 | 8.239 | 7.940 | 0.450 | 0.882 | 0.450 | 1.301 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 31.9 | 39.2 | 34.9 | 21.4 | 20.4 | 1.4 | 2.2 | 3.4 | 23.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 14.4 | 8.1 | — | — | — | 17.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 14.3 | 10.2 | — | — | — | 25.3 | — | — |
| Spiralling loops (L) | — | — | — | — | 75 | 63 | — | — | — | 31 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 71.3 | 81.7 | — | — | — | 57.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 34 | 34 | — | — | — | 332 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 139.0 | 36.1 | 25.8 | 41.1 | 24.2 | 1115.7 | 769.0 | 0.0 | 2.5 | — | — |
| Time, seconds (T) | — | 0.36 | 0.60 | 7.70 | 8.80 | 10.98 | 6.97 | 3.89 | 6.97 | 116.26 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 334 | 940 | 743 | 1017 | 999 | 2921 | 812 | 2921 | 2867 | — | — |
| Irregular vertices (S) | — | 38 | 121 | 44 | 46 | 122 | 229 | 126 | 229 | 197 | — | — |
| … of which above the minimum | — | — | 112 | 36 | 38 | 114 | 221 | — | 221 | 189 | — | — |
| Base complex patches (C) | — | — | 910 | 363 | 349 | 683 | 2159 | — | 2159 | 1897 | — | — |
| Alignment, degrees (A) | — | 18.93 | 19.57 | 14.37 | 14.43 | 16.19 | 10.54 | 14.09 | 10.54 | 13.35 | — | — |
| Mean angle deviation, degrees (F) | — | 14.98 | 10.46 | 10.54 | 8.59 | 9.84 | 9.73 | 11.98 | 9.73 | 7.10 | — | — |
| Angle deviation, 95th percentile (F) | — | 43.87 | 32.04 | 28.35 | 24.08 | 27.80 | 26.74 | 31.12 | 26.74 | 21.20 | — | — |
| Mean aspect ratio (F) | — | 1.799 | 1.433 | 1.615 | 1.822 | 1.523 | 2.436 | 1.706 | 2.436 | 1.484 | — | — |
| Area variation (F) | — | 0.758 | 0.296 | 0.399 | 0.472 | 1.046 | 0.740 | 0.435 | 0.740 | 0.779 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 124.75 | 81.49 | 38.88 | 32.56 | 31.65 | 29.33 | 57.04 | 29.33 | 23.01 | — | — |
| Mean distance, ‰ (H) | — | 11.455 | 3.775 | 2.299 | 1.815 | 1.959 | 0.976 | 3.129 | 0.976 | 0.807 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 89.7 | 99.1 | 37.7 | 23.6 | 5.7 | 3.8 | 47.2 | 7.0 | 3.9 | — | — |
| Volume kept, % of the source (V) | — | 86.48 | 95.96 | 99.19 | 99.39 | 99.80 | 98.60 | — | 98.60 | 100.07 | — | — |
| Edges on closed loops, % (L) | — | 5.8 | — | — | 36.8 | 16.6 | 13.4 | — | — | 17.8 | — | — |
| Edges on spiralling loops, % (L) | — | 18.0 | — | — | 2.0 | 9.0 | 10.5 | — | — | 3.8 | — | — |
| Spiralling loops (L) | — | 9 | — | — | 4 | 16 | 14 | — | — | 7 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.2 | — | — | 61.3 | 74.4 | 76.1 | — | — | 78.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 28 | — | — | 100 | 47 | 85 | — | — | 74 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 66.6 | 6.0 | 25.7 | 1.7 | 0.1 | 192.1 | 18.8 | 0.0 | 1.8 | — | — |
| Time, seconds (T) | — | 17.00 | 99.29 | 66.59 | 60.93 | 67.66 | 36.93 | 32.56 | 36.93 | 72.45 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 763 | 225 | — | — | 390 | 390 | 9441 | — | — | — | — | — |
| Irregular vertices (S) | 130 | 79 | — | — | 93 | 93 | 881 | — | — | — | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 751 | — | — | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 16.26 | 19.32 | — | — | 17.80 | 17.83 | 11.80 | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 15.96 | 26.39 | — | — | 20.48 | 20.46 | 10.11 | — | — | — | — | — |
| Angle deviation, 95th percentile (F) | 53.61 | 82.13 | — | — | 63.41 | 63.41 | 31.55 | — | — | — | — | — |
| Mean aspect ratio (F) | 1.692 | 4.170 | — | — | 2.344 | 2.334 | 1.746 | — | — | — | — | — |
| Area variation (F) | 0.466 | 1.055 | — | — | 0.823 | 0.823 | 0.926 | — | — | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 130.87 | 298.17 | — | — | 120.93 | 120.93 | 38.62 | — | — | — | — | — |
| Mean distance, ‰ (H) | 17.928 | 82.620 | — | — | 20.034 | 20.033 | 2.325 | — | — | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 47.7 | 197.5 | — | — | 49.2 | 49.2 | 9.8 | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.7 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.5 | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | 14 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 86.8 | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 35 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 5.2 | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 23.7 | 77.5 | — | — | 61.0 | 61.0 | 844.1 | — | — | — | — | — |
| Time, seconds (T) | 0.84 | 1.57 | — | — | 10.75 | 12.56 | 526.55 | — | — | — | — | — |

Failures: openclare-matched: timed out after 1800 s.

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 1/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 866 | 405 | — | — | 1930 | 3818 | 6083 | — | — | 5071 | — | — |
| Irregular vertices (S) | 165 | 121 | — | — | 313 | 1194 | 505 | — | — | 232 | — | — |
| … of which above the minimum | — | — | — | — | 9 | 890 | — | — | — | — | — | — |
| Base complex patches (C) | 846 | — | — | — | 1930 | 3818 | — | — | — | — | — | — |
| Alignment, degrees (A) | 13.39 | 18.17 | — | — | 16.92 | 18.73 | 9.77 | — | — | 11.16 | — | — |
| Mean angle deviation, degrees (F) | 14.47 | 27.63 | — | — | 24.60 | 20.75 | 10.07 | — | — | 7.45 | — | — |
| Angle deviation, 95th percentile (F) | 46.45 | 75.88 | — | — | 63.64 | 52.88 | 32.00 | — | — | 25.96 | — | — |
| Mean aspect ratio (F) | 1.621 | 16.899 | — | — | 5.426 | 2.785 | 2.374 | — | — | 1.366 | — | — |
| Area variation (F) | 0.403 | 0.903 | — | — | 1.748 | 1.355 | 0.688 | — | — | 0.273 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 102.61 | 138.78 | — | — | 76.87 | 56.49 | 28.19 | — | — | 56.38 | — | — |
| Mean distance, ‰ (H) | 9.005 | 32.456 | — | — | 5.769 | 3.557 | 2.452 | — | — | 2.748 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.9 | 75.3 | — | — | 16.0 | 12.7 | 8.2 | — | — | 28.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 83.61 | 89.77 | 102.25 | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.6 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 8.1 | — | — | — | 5.6 | 3.1 | 15.2 | — | — | — | — | — |
| Spiralling loops (L) | 7 | — | — | — | 39 | 34 | 51 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.2 | — | — | — | 94.4 | 96.9 | 84.3 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 23 | — | — | — | 42 | 28 | 100 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | — | — | — |
| Off budget, % (N) | 13.4 | 59.5 | — | — | 93.0 | 281.8 | 508.3 | — | — | 16.6 | — | — |
| Time, seconds (T) | 0.64 | 1.20 | — | — | 63.16 | 1077.00 | 79.69 | — | — | 52.21 | — | — |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 1/1 | 0/1 | 0/1 | 0/0 | 0/0 | 1/1 | 0/0 | 0/0 |
| Quads made | 938 | 703 | — | — | 1000 | 987 | 5437 | — | — | 5447 | — | — |
| Irregular vertices (S) | 132 | 73 | — | — | 92 | 161 | 428 | — | — | 239 | — | — |
| … of which above the minimum | — | — | — | — | 28 | 97 | 376 | — | — | 175 | — | — |
| Base complex patches (C) | 926 | — | — | — | 884 | 965 | 4943 | — | — | 4685 | — | — |
| Alignment, degrees (A) | 15.11 | 17.16 | — | — | 13.65 | 15.09 | 11.05 | — | — | 12.04 | — | — |
| Mean angle deviation, degrees (F) | 12.56 | 11.97 | — | — | 9.91 | 14.31 | 9.53 | — | — | 6.19 | — | — |
| Angle deviation, 95th percentile (F) | 35.62 | 36.94 | — | — | 34.35 | 40.54 | 29.45 | — | — | 19.03 | — | — |
| Mean aspect ratio (F) | 1.391 | 1.994 | — | — | 1.969 | 2.111 | 1.908 | — | — | 1.345 | — | — |
| Area variation (F) | 0.275 | 0.656 | — | — | 0.475 | 1.100 | 0.903 | — | — | 0.392 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 98.74 | 186.96 | — | — | 35.63 | 28.29 | 20.43 | — | — | 14.25 | — | — |
| Mean distance, ‰ (H) | 4.506 | 9.114 | — | — | 3.680 | 3.302 | 2.028 | — | — | 1.318 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 38.2 | 76.4 | — | — | 20.7 | 13.3 | 12.4 | — | — | 23.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 99.50 | 99.56 | 98.75 | — | — | 100.05 | — | — |
| Edges on closed loops, % (L) | 0.3 | — | — | — | 6.0 | 1.2 | 4.7 | — | — | 7.6 | — | — |
| Edges on spiralling loops, % (L) | 4.3 | — | — | — | 12.2 | 7.3 | 6.6 | — | — | 6.5 | — | — |
| Spiralling loops (L) | 14 | — | — | — | 10 | 18 | 14 | — | — | 18 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 95.4 | — | — | — | 81.8 | 91.5 | 88.7 | — | — | 85.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 50 | — | — | — | 57 | 30 | 137 | — | — | 208 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 6.2 | 29.7 | — | — | 0.0 | 1.3 | 443.7 | — | — | 0.2 | — | — |
| Time, seconds (T) | 0.87 | 0.85 | — | — | 3.37 | 11.52 | 31.11 | — | — | 10.48 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 1/1 | 0/1 | 0/0 | 0/0 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 370 | — | — | 136 | 207 | 6933 | — | — | 595 | — | — |
| Irregular vertices (S) | — | 75 | — | — | 28 | 46 | 599 | — | — | 173 | — | — |
| … of which above the minimum | — | — | — | — | 4 | 22 | — | — | — | 149 | — | — |
| Base complex patches (C) | — | — | — | — | 136 | 207 | — | — | — | 579 | — | — |
| Alignment, degrees (A) | — | 14.45 | — | — | 17.82 | 20.70 | 9.68 | — | — | 20.48 | — | — |
| Mean angle deviation, degrees (F) | — | 28.12 | — | — | 32.05 | 24.99 | 9.49 | — | — | 19.91 | — | — |
| Angle deviation, 95th percentile (F) | — | 83.45 | — | — | 73.49 | 66.19 | 31.62 | — | — | 51.43 | — | — |
| Mean aspect ratio (F) | — | 3.970 | — | — | 8.936 | 3.123 | 1.954 | — | — | 2.640 | — | — |
| Area variation (F) | — | 0.923 | — | — | 1.180 | 1.246 | 0.818 | — | — | 1.262 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 203.64 | — | — | 538.35 | 536.93 | 23.21 | — | — | 529.26 | — | — |
| Mean distance, ‰ (H) | — | 36.911 | — | — | 256.390 | 249.218 | 1.584 | — | — | 224.422 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 110.1 | — | — | 713.1 | 705.9 | 7.5 | — | — | 1737.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 1.14 | 0.68 | — | — | — | 1.31 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 1.3 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 11.8 | 6.5 | — | — | — | 1.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 2 | 2 | — | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 88.2 | 93.5 | — | — | — | 97.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 18 | 23 | — | — | — | 17 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 63.0 | — | — | 86.4 | 79.3 | 593.3 | — | — | 91.4 | — | — |
| Time, seconds (T) | — | 1.40 | — | — | 0.88 | 0.65 | 101.96 | — | — | 10.76 | — | — |

Failures: quadriflow: cancelled.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 872 | 308 | 574 | 896 | 1004 | 1011 | 1541 | 878 | 1541 | 1563 | — | — |
| Irregular vertices (S) | 44 | 30 | 40 | 34 | 32 | 34 | 152 | 88 | 152 | 40 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 307 | — | 394 | 326 | 457 | 594 | 1446 | — | 1446 | 907 | — | — |
| Alignment, degrees (A) | 19.22 | 22.42 | 18.99 | 17.91 | 18.03 | 18.52 | 18.90 | 20.03 | 18.89 | 18.62 | — | — |
| Mean angle deviation, degrees (F) | 6.74 | 26.16 | 8.62 | 10.03 | 5.42 | 6.56 | 11.26 | 10.74 | 11.26 | 5.93 | — | — |
| Angle deviation, 95th percentile (F) | 23.83 | 73.60 | 29.38 | 28.12 | 16.64 | 19.12 | 32.77 | 31.81 | 32.77 | 18.02 | — | — |
| Mean aspect ratio (F) | 1.223 | 1.911 | 1.338 | 1.361 | 1.282 | 1.301 | 1.859 | 1.521 | 1.859 | 1.293 | — | — |
| Area variation (F) | 0.195 | 0.775 | 0.224 | 0.321 | 0.342 | 0.512 | 0.545 | 0.321 | 0.545 | 0.467 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 30.58 | 234.03 | 146.80 | 30.18 | 27.35 | 30.92 | 18.41 | 31.35 | 18.41 | 27.47 | — | — |
| Mean distance, ‰ (H) | 3.158 | 9.103 | 4.386 | 2.651 | 2.234 | 1.897 | 1.506 | 2.787 | 1.506 | 1.600 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 29.7 | 54.8 | 44.9 | 29.2 | 22.8 | 18.3 | 11.3 | 27.0 | 14.3 | 20.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 3.5 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 7.8 | — | — | 0.0 | 3.8 | 0.0 | — | — | 5.8 | — | — |
| Spiralling loops (L) | 0 | 1 | — | — | 0 | 1 | 0 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 57.4 | 92.2 | — | — | 66.3 | 76.0 | 96.8 | — | — | 69.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 25 | 27 | — | — | 36 | 50 | 54 | — | — | 61 | — | — |
| Edges on loops from a border to a border, % (L) | 42.6 | 0.0 | — | — | 33.7 | 16.6 | 3.2 | — | — | 24.5 | — | — |
| Off budget, % (N) | 12.8 | 69.2 | 42.6 | 10.4 | 0.4 | 1.1 | 54.1 | 12.2 | 0.0 | 1.4 | — | — |
| Time, seconds (T) | 19.91 | 1.73 | 8.90 | 8.59 | 7.80 | 8.42 | 19.65 | 21.87 | 19.65 | 8.77 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 636 | 562 | 532 | 2115 | 1958 | 41821 | 2688 | 41821 | 41444 | — | — |
| Irregular vertices (S) | — | 343 | 124 | 45 | 485 | 476 | 3109 | 1013 | 3109 | 1379 | — | — |
| … of which above the minimum | — | — | 13 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 423 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.88 | 11.52 | 8.55 | 12.30 | 19.32 | 15.01 | 14.68 | 14.20 | 5.77 | — | — |
| Mean angle deviation, degrees (F) | — | 38.85 | 15.07 | 11.18 | 35.77 | 34.18 | 13.01 | 16.98 | 13.01 | 5.50 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.58 | 53.29 | 35.63 | 83.86 | 85.00 | 40.35 | 58.74 | 40.35 | 21.91 | — | — |
| Mean aspect ratio (F) | — | 7.492 | 2.321 | 1.724 | 111.495 | 211.925 | 1.768 | 2.072 | 1.768 | 11.018 | — | — |
| Area variation (F) | — | 1.913 | 0.516 | 0.635 | 12.630 | 12.109 | 3.743 | 1.645 | 3.743 | 0.974 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 88.93 | 69.46 | 47.45 | 325.51 | 325.53 | 13.85 | 49.37 | 13.85 | 13.28 | — | — |
| Mean distance, ‰ (H) | — | 14.421 | 10.323 | 7.114 | 40.256 | 40.313 | 1.081 | 4.277 | 1.081 | 0.677 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 65.0 | 50.2 | 35.7 | 154.2 | 154.2 | 7.4 | 22.4 | 50.7 | 36.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 0.8 | 0.8 | — | — | — | 23.2 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 80.1 | 76.3 | — | — | — | 23.0 | — | — |
| Spiralling loops (L) | — | — | — | — | 484 | 457 | — | — | — | 343 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 19.2 | 22.8 | — | — | — | 44.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 5 | 3 | — | — | — | 740 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.1 | — | — | — | 9.6 | — | — |
| Off budget, % (N) | — | 36.4 | 43.8 | 46.8 | 111.5 | 95.8 | 4082.1 | 168.8 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | — | 1.37 | 5.58 | 12.95 | 12.39 | 10.99 | 16.61 | 15.33 | 16.61 | 33.75 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1110 | 204 | — | 8052 | 8094 | 205185 | 10623 | 205185 | 194806 | — | — |
| Irregular vertices (S) | — | 905 | 139 | — | 6194 | 6148 | 10625 | 4798 | 10625 | 12006 | — | — |
| … of which above the minimum | — | — | -56 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 204 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | — | 19.32 | — | — | — | 10.09 | — | 13.06 | 8.59 | — | — |
| Mean angle deviation, degrees (F) | — | 40.06 | 30.35 | — | 24.06 | 22.73 | 10.19 | 17.58 | 10.19 | 5.28 | — | — |
| Angle deviation, 95th percentile (F) | — | 88.92 | 69.83 | — | 70.91 | 69.71 | 30.61 | 52.92 | 30.61 | 20.71 | — | — |
| Mean aspect ratio (F) | — | 4.992 | 3.003 | — | 6.769 | 4.165 | 1.591 | 1.958 | 1.591 | 1.387 | — | — |
| Area variation (F) | — | 5.647 | 0.795 | — | 2.780 | 2.799 | 3.347 | 3.276 | 3.347 | 0.578 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 11.21 | 10.79 | — | 7.04 | 6.04 | 2.06 | 5.15 | 2.06 | 3.01 | — | — |
| Mean distance, ‰ (H) | — | 3.155 | 3.973 | — | 0.440 | 0.443 | 0.152 | 0.565 | 0.152 | 0.038 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 56.2 | 70.3 | — | 7.9 | 8.0 | 5.2 | 11.1 | 66.6 | 18.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | 36.7 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | 11.6 | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | 1215 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | 45.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | 447 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | 6.5 | — | — |
| Off budget, % (N) | — | 11.0 | 79.6 | — | 705.2 | 709.4 | 20418.5 | 962.3 | 0.0 | 5.1 | — | — |
| Time, seconds (T) | — | 1.28 | 3.43 | — | 14.97 | 21.60 | 14.01 | 8.85 | 14.01 | 216.88 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 8 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 3146 | 465 | 603 | 2598 | 2651 | 30430 | 4601 | 30430 | 28167 | — | — |
| Irregular vertices (S) | — | 1983 | 163 | 181 | 1750 | 1669 | 2729 | 1219 | 2729 | 1937 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 442 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 17.00 | 15.03 | 10.78 | 4.97 | 5.24 | 5.04 | 8.21 | 5.05 | 4.01 | — | — |
| Mean angle deviation, degrees (F) | — | 45.13 | 23.16 | 17.86 | 21.62 | 21.51 | 11.44 | 15.59 | 11.44 | 6.48 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.51 | 66.13 | 54.34 | 76.01 | 75.27 | 42.16 | 46.05 | 42.16 | 27.97 | — | — |
| Mean aspect ratio (F) | — | 8.830 | 3.120 | 2.286 | 177.284 | 197.299 | 2.473 | 3.146 | 2.473 | 17.493 | — | — |
| Area variation (F) | — | 5.401 | 0.584 | 0.550 | 7.605 | 6.772 | 2.700 | 2.263 | 2.700 | 1.063 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 128.15 | 144.73 | 94.72 | 83.29 | 68.45 | 10.84 | 83.07 | 10.84 | 13.12 | — | — |
| Mean distance, ‰ (H) | — | 13.795 | 32.571 | 17.454 | 4.820 | 4.179 | 0.478 | 4.559 | 0.478 | 0.355 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.9 | 129.3 | 70.7 | 15.8 | 13.6 | 2.5 | 18.5 | 13.8 | 8.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 5.4 | 6.6 | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 33.2 | 32.0 | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | 1316 | 1286 | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 52.8 | 53.1 | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 14 | 25 | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 8.6 | 8.4 | — | — | — | — | — | — |
| Off budget, % (N) | — | 214.6 | 53.5 | 39.7 | 159.8 | 165.1 | 2943.0 | 360.1 | 0.0 | 7.4 | — | — |
| Time, seconds (T) | — | 1.26 | 4.02 | 56.49 | 10.32 | 13.11 | 11.27 | 10.16 | 11.27 | 77.31 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 988 | 508 | 843 | 1626 | 1671 | 9385 | 3134 | 9385 | 8818 | — | — |
| Irregular vertices (S) | — | 272 | 126 | 82 | 164 | 173 | 368 | 289 | 368 | 284 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 428 | — | 405 | 429 | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.41 | 9.38 | 8.25 | 5.14 | 5.83 | 4.16 | 4.27 | 3.37 | 4.34 | — | — |
| Mean angle deviation, degrees (F) | — | 25.43 | 18.32 | 9.97 | 22.05 | 21.12 | 7.93 | 11.16 | 7.93 | 8.32 | — | — |
| Angle deviation, 95th percentile (F) | — | 86.02 | 55.68 | 36.03 | 74.84 | 72.24 | 28.21 | 42.77 | 28.21 | 41.56 | — | — |
| Mean aspect ratio (F) | — | 3.381 | 2.053 | 1.774 | 39.590 | 78.292 | 10.123 | 26.996 | 10.123 | 15.749 | — | — |
| Area variation (F) | — | 1.855 | 0.710 | 0.410 | 2.163 | 1.980 | 2.280 | 1.420 | 2.280 | 0.670 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 72.47 | 81.73 | 35.86 | 52.85 | 41.28 | 21.81 | 36.33 | 21.81 | 19.79 | — | — |
| Mean distance, ‰ (H) | — | 8.475 | 10.964 | 4.688 | 6.030 | 5.834 | 1.030 | 2.230 | 1.030 | 0.946 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 17.9 | 20.7 | 10.1 | 13.4 | 13.0 | 2.4 | 5.4 | 7.1 | 11.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 56.6 | 56.1 | 33.6 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 13.9 | 15.2 | 5.0 | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | 60 | 67 | 19 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 25.9 | 25.6 | 38.1 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 18 | 17 | 76 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 3.5 | 3.0 | 23.4 | — | — | — | — | — |
| Off budget, % (N) | — | 1.2 | 49.2 | 15.7 | 62.6 | 67.1 | 838.5 | 213.4 | 0.0 | 6.0 | — | — |
| Time, seconds (T) | — | 0.39 | 0.51 | 21.84 | 2.53 | 2.54 | 5.98 | 5.10 | 5.98 | 30.74 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 180 | 866 | 744 | 738 | 738 | 7098 | 803 | 7098 | 6016 | — | — |
| Irregular vertices (S) | — | 56 | 181 | 74 | 119 | 119 | 725 | 192 | 725 | 421 | — | — |
| … of which above the minimum | — | — | 172 | — | 55 | 55 | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 810 | — | 440 | 440 | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.42 | 14.10 | 10.16 | 10.33 | 10.34 | 9.10 | 10.26 | 10.66 | 10.77 | — | — |
| Mean angle deviation, degrees (F) | — | 24.14 | 14.78 | 12.69 | 12.29 | 12.30 | 9.97 | 13.02 | 9.97 | 9.66 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.70 | 45.64 | 33.98 | 33.95 | 34.18 | 29.98 | 40.40 | 29.98 | 28.66 | — | — |
| Mean aspect ratio (F) | — | 3.965 | 1.759 | 1.600 | 1.985 | 2.054 | 1.773 | 1.662 | 1.773 | 1.473 | — | — |
| Area variation (F) | — | 0.945 | 0.411 | 0.449 | 0.486 | 0.486 | 1.130 | 0.547 | 1.130 | 0.341 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 123.45 | 52.06 | 48.50 | 40.57 | 40.57 | 28.38 | 45.02 | 28.38 | 20.49 | — | — |
| Mean distance, ‰ (H) | — | 18.850 | 12.293 | 6.257 | 5.933 | 5.906 | 1.253 | 6.755 | 1.253 | 0.797 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 123.8 | 121.8 | 59.0 | 55.4 | 55.1 | 17.0 | 61.2 | 50.3 | 36.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 7.5 | — | — | 20.2 | 20.2 | 16.7 | — | — | 19.8 | — | — |
| Edges on spiralling loops, % (L) | — | 16.4 | — | — | 8.4 | 8.4 | 8.8 | — | — | 17.8 | — | — |
| Spiralling loops (L) | — | 14 | — | — | 43 | 43 | 55 | — | — | 105 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.1 | — | — | 71.4 | 71.4 | 74.4 | — | — | 62.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 11 | — | — | 36 | 36 | 59 | — | — | 137 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | 0.1 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 82.0 | 13.4 | 25.6 | 26.2 | 26.2 | 609.8 | 19.7 | 0.0 | 15.2 | — | — |
| Time, seconds (T) | — | 0.58 | 0.19 | 5.77 | 6.09 | 5.43 | 27.33 | 36.82 | 27.33 | 81.43 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 950 | 768 | 1066 | 923 | 1012 | 1014 | 915 | 915 | 915 | 912 | — | — |
| Irregular vertices (S) | 8 | 8 | 44 | 8 | 8 | 8 | 10 | 10 | 10 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 36 | 0 | 0 | 0 | 2 | 2 | 2 | 0 | — | — |
| Base complex patches (C) | 937 | 768 | 1066 | 923 | 6 | 6 | 699 | 699 | 699 | 6 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.13 | 5.18 | 8.36 | 1.70 | 1.79 | 6.03 | 6.03 | 6.03 | 1.87 | — | — |
| Angle deviation, 95th percentile (F) | 12.31 | 13.45 | 17.01 | 22.46 | 5.53 | 5.86 | 14.57 | 14.57 | 14.57 | 6.32 | — | — |
| Mean aspect ratio (F) | 1.148 | 1.336 | 1.153 | 1.171 | 1.114 | 1.113 | 1.213 | 1.213 | 1.213 | 1.119 | — | — |
| Area variation (F) | 0.136 | 0.229 | 0.179 | 0.152 | 0.165 | 0.160 | 0.318 | 0.318 | 0.318 | 0.162 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 3.06 | 1.68 | 1.11 | 0.77 | 0.82 | 1.91 | 1.91 | 1.91 | 0.89 | — | — |
| Mean distance, ‰ (H) | 0.670 | 1.508 | 0.632 | 0.356 | 0.328 | 0.328 | 0.762 | 0.762 | 0.762 | 0.350 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.38 | 98.48 | 99.38 | 100.02 | 100.02 | 100.02 | 99.24 | 99.24 | 99.24 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 92.3 | 92.3 | 12.0 | — | — | 91.9 | — | — |
| Edges on spiralling loops, % (L) | 69.5 | 76.7 | — | — | 0.0 | 0.0 | 64.4 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 4 | 4 | — | — | 0 | 0 | 5 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 25.1 | 23.3 | — | — | 7.7 | 7.7 | 23.6 | — | — | 8.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 126 | — | — | 14 | 13 | 110 | — | — | 13 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.0 | 23.2 | 6.6 | 7.7 | 1.2 | 1.4 | 8.5 | 8.5 | 0.0 | 0.3 | — | — |
| Time, seconds (T) | 0.45 | 0.18 | 0.19 | 1.75 | 2.96 | 17.38 | 4.77 | 3.75 | 4.77 | 19.06 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1028 | 728 | 1054 | 920 | 1008 | 1008 | 920 | 920 | 920 | 920 | — | — |
| Irregular vertices (S) | 20 | 0 | 90 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| … of which above the minimum | 20 | 0 | 90 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 1016 | 1 | 932 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | — | — |
| Alignment, degrees (A) | 13.64 | 1.94 | 8.78 | 0.63 | 0.16 | 0.16 | 0.63 | 0.63 | 0.64 | 0.64 | — | — |
| Mean angle deviation, degrees (F) | 7.73 | 4.57 | 8.51 | 2.51 | 2.40 | 2.40 | 2.49 | 2.49 | 2.49 | 2.52 | — | — |
| Angle deviation, 95th percentile (F) | 18.19 | 9.20 | 26.44 | 3.93 | 3.73 | 3.73 | 3.93 | 3.93 | 3.93 | 3.91 | — | — |
| Mean aspect ratio (F) | 1.178 | 3.376 | 1.273 | 1.362 | 1.295 | 1.295 | 1.372 | 1.372 | 1.372 | 1.293 | — | — |
| Area variation (F) | 0.180 | 0.121 | 0.237 | 0.294 | 0.351 | 0.351 | 0.280 | 0.280 | 0.280 | 0.356 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 5.42 | 3.96 | 1.51 | 1.35 | 1.35 | 2.60 | 2.60 | 2.60 | 1.67 | — | — |
| Mean distance, ‰ (H) | 0.775 | 1.726 | 0.849 | 0.447 | 0.424 | 0.424 | 1.127 | 1.127 | 1.127 | 0.449 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.57 | 96.27 | 98.28 | 100.06 | 100.04 | 100.04 | 97.62 | 97.62 | 97.62 | 100.06 | — | — |
| Edges on closed loops, % (L) | 0.0 | 100.0 | — | — | 100.0 | 100.0 | 100.0 | — | — | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 52.8 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 11 | 0 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 44.7 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 109 | 0 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 27.2 | 5.4 | 8.0 | 0.8 | 0.8 | 8.0 | 8.0 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 0.33 | 0.44 | 0.27 | 1.02 | 1.16 | 1.56 | 4.41 | 2.61 | 4.41 | 0.87 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 1 | 9 | 0 | 5 | 2 | 1 | 0 | 0 | 0 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 4 | 0 | 4 | 5 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 0 | 13 | 0 | 1 | 1 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 4 | 1 | 0 | 1 | 0 | 8 | 0 | 1 |
| Mean aspect ratio (F) | 4 | 0 | 3 | 0 | 2 | 1 | 1 | 0 | 1 | 2 | 0 | 2 |
| Valid meshes (V) | 38/75 | 4/15 | 45/75 | 9/15 | 13/15 | 14/15 | 11/15 | 8/15 | 11/15 | 15/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 1 | 0 | 4 | 4 | 4 | 3 | 4 | 3 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 0 | 4 | 3 | 2 | 2 | 2 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 2 | 2 | 1 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 2 | 1 | 1 | 1 | 1 | 0 | 0 | 0 |
| Mean aspect ratio (F) | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| Valid meshes (V) | 20/20 | 2/4 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
