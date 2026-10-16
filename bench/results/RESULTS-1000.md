# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 941 | 400 | 1036 | 867 | 1017 | 999 | 1362 | 878 | 1362 | 1354 | — | — |
| Irregular vertices (S) | 36 | 24 | 76 | 24 | 24 | 24 | 26 | 26 | 26 | 24 | — | — |
| … of which above the minimum | 28 | 16 | 68 | 16 | 16 | 16 | 18 | 18 | 18 | 16 | — | — |
| Base complex patches (C) | 941 | 314 | 1036 | 601 | 553 | 275 | 520 | 434 | 520 | 424 | — | — |
| Alignment, degrees (A) | 18.04 | 8.83 | 17.88 | 7.53 | 5.20 | 4.57 | 3.42 | 4.94 | 3.46 | 4.56 | — | — |
| Mean angle deviation, degrees (F) | 9.26 | 12.46 | 7.67 | 9.62 | 6.48 | 6.25 | 6.86 | 6.31 | 6.86 | 6.12 | — | — |
| Angle deviation, 95th percentile (F) | 25.27 | 33.98 | 23.42 | 24.97 | 15.88 | 15.40 | 20.12 | 18.61 | 20.12 | 15.52 | — | — |
| Mean aspect ratio (F) | 1.221 | 1.652 | 1.235 | 1.436 | 1.406 | 1.383 | 2.760 | 1.379 | 2.760 | 1.364 | — | — |
| Area variation (F) | 0.218 | 0.577 | 0.218 | 0.385 | 0.453 | 0.479 | 0.625 | 0.353 | 0.625 | 0.463 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 14.52 | 16.95 | 15.11 | 17.75 | 10.36 | 6.50 | 7.74 | 19.23 | 7.74 | 5.22 | — | — |
| Mean distance, ‰ (H) | 1.325 | 5.131 | 1.340 | 0.706 | 0.513 | 0.616 | 1.172 | 1.534 | 1.172 | 0.452 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.58 | 92.19 | 98.90 | 100.06 | 100.05 | 100.05 | 98.08 | 98.43 | 98.08 | 100.05 | — | — |
| Edges on closed loops, % (L) | 0.0 | 10.8 | — | — | 23.7 | 41.1 | 33.7 | — | — | 36.3 | — | — |
| Edges on spiralling loops, % (L) | 25.3 | 35.2 | — | — | 18.8 | 15.3 | 18.2 | — | — | 28.4 | — | — |
| Spiralling loops (L) | 4 | 7 | — | — | 3 | 2 | 4 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.7 | 54.0 | — | — | 57.5 | 43.5 | 48.1 | — | — | 35.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 108 | 44 | — | — | 104 | 100 | 113 | — | — | 104 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.9 | 60.0 | 3.6 | 13.3 | 1.7 | 0.1 | 36.2 | 12.2 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | 1.82 | 0.43 | 1.70 | 4.13 | 1.71 | 2.43 | 14.01 | 13.25 | 14.01 | 1.86 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 947 | 778 | 966 | 898 | 978 | 995 | 1290 | 837 | 1290 | 1298 | — | — |
| Irregular vertices (S) | 8 | 10 | 40 | 26 | 26 | 26 | 90 | 84 | 90 | 34 | — | — |
| … of which above the minimum | 0 | 2 | 32 | 18 | 18 | 18 | 82 | 76 | 82 | 26 | — | — |
| Base complex patches (C) | 941 | 506 | 912 | 898 | 767 | 995 | 1290 | 837 | 1290 | 1298 | — | — |
| Alignment, degrees (A) | 23.14 | 19.26 | 23.62 | 16.22 | 17.02 | 16.67 | 13.52 | 14.33 | 14.01 | 16.65 | — | — |
| Mean angle deviation, degrees (F) | 6.16 | 7.46 | 6.09 | 9.43 | 5.76 | 5.63 | 11.01 | 11.93 | 11.01 | 5.62 | — | — |
| Angle deviation, 95th percentile (F) | 16.14 | 18.13 | 18.18 | 23.72 | 16.05 | 14.61 | 27.61 | 30.02 | 27.61 | 15.70 | — | — |
| Mean aspect ratio (F) | 1.162 | 1.494 | 1.188 | 1.321 | 1.257 | 1.245 | 1.631 | 1.472 | 1.631 | 1.227 | — | — |
| Area variation (F) | 0.179 | 0.473 | 0.178 | 0.313 | 0.354 | 0.340 | 0.354 | 0.299 | 0.354 | 0.293 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 9.38 | 14.09 | 10.32 | 11.07 | 8.83 | 7.40 | 9.88 | 12.74 | 9.88 | 7.55 | — | — |
| Mean distance, ‰ (H) | 1.512 | 2.480 | 1.641 | 1.335 | 1.242 | 1.214 | 1.291 | 1.761 | 1.291 | 1.059 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.30 | 98.22 | 99.04 | 99.94 | 100.02 | 99.98 | 99.46 | 98.87 | 99.46 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 3.7 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 60.3 | 69.2 | — | — | 16.3 | 14.7 | 3.8 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 3 | 6 | — | — | 2 | 3 | 1 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 35.9 | 27.1 | — | — | 83.7 | 85.3 | 96.2 | — | — | 100.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 133 | 82 | — | — | 130 | 154 | 75 | — | — | 172 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.3 | 22.2 | 3.4 | 10.2 | 2.2 | 0.5 | 29.0 | 16.3 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | 1.10 | 0.33 | 1.20 | 5.26 | 1.60 | 1.50 | 28.00 | 16.00 | 28.00 | 2.44 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 812 | 604 | 994 | 920 | 1017 | 983 | 1208 | 928 | 1208 | 1205 | — | — |
| Irregular vertices (S) | 16 | 12 | 51 | 24 | 26 | 40 | 42 | 39 | 42 | 40 | — | — |
| … of which above the minimum | 8 | 4 | 43 | — | 18 | 32 | 34 | 31 | 34 | 32 | — | — |
| Base complex patches (C) | 812 | 308 | 822 | 666 | 339 | 361 | 588 | 346 | 588 | 463 | — | — |
| Alignment, degrees (A) | 14.79 | 11.08 | 11.93 | 8.02 | 7.96 | 5.73 | 4.65 | 4.83 | 4.72 | 5.89 | — | — |
| Mean angle deviation, degrees (F) | 12.55 | 6.78 | 6.23 | 7.31 | 5.06 | 5.70 | 6.49 | 6.99 | 6.49 | 5.39 | — | — |
| Angle deviation, 95th percentile (F) | 30.83 | 16.26 | 18.64 | 22.70 | 14.02 | 17.10 | 18.32 | 21.32 | 18.32 | 15.33 | — | — |
| Mean aspect ratio (F) | 1.391 | 1.697 | 1.185 | 1.250 | 1.210 | 1.297 | 1.473 | 1.394 | 1.473 | 1.307 | — | — |
| Area variation (F) | 0.298 | 0.418 | 0.183 | 0.288 | 0.363 | 0.492 | 0.461 | 0.380 | 0.461 | 0.489 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 40.53 | 8.78 | 18.99 | 12.89 | 5.53 | 3.59 | 3.74 | 6.58 | 3.74 | 3.30 | — | — |
| Mean distance, ‰ (H) | 1.486 | 1.732 | 0.917 | 0.516 | 0.439 | 0.467 | 0.604 | 0.932 | 0.604 | 0.404 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 97.34 | 96.54 | 98.23 | — | 100.08 | 100.07 | 98.75 | 98.01 | 98.75 | 100.05 | — | — |
| Edges on closed loops, % (L) | 0.0 | 24.5 | — | — | 34.7 | 31.6 | 25.7 | — | — | 30.8 | — | — |
| Edges on spiralling loops, % (L) | 43.6 | 54.3 | — | — | 9.1 | 7.0 | 17.5 | — | — | 3.7 | — | — |
| Spiralling loops (L) | 6 | 4 | — | — | 2 | 2 | 7 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 56.4 | 21.2 | — | — | 56.1 | 61.4 | 56.8 | — | — | 65.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 141 | 52 | — | — | 109 | 103 | 84 | — | — | 118 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 18.8 | 39.6 | 1.4 | 8.0 | 1.7 | 1.7 | 20.8 | 7.2 | 0.0 | 0.2 | — | — |
| Time, seconds (T) | 1.18 | 0.45 | 1.03 | 4.64 | 1.71 | 1.86 | 11.95 | 8.72 | 11.95 | 1.65 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1043 | 1720 | 974 | 970 | 1001 | 1001 | 3224 | 934 | 3224 | 3205 | — | — |
| Irregular vertices (S) | 64 | 52 | 136 | 40 | 48 | 48 | 61 | 49 | 61 | 48 | — | — |
| … of which above the minimum | 32 | — | 104 | 8 | 16 | 16 | 29 | 17 | 29 | 16 | — | — |
| Base complex patches (C) | 716 | — | 779 | 96 | 263 | 208 | 741 | 211 | 741 | 346 | — | — |
| Alignment, degrees (A) | 7.66 | 8.80 | 10.19 | 3.01 | 4.06 | 3.17 | 2.70 | 3.17 | 2.73 | 2.23 | — | — |
| Mean angle deviation, degrees (F) | 6.96 | 10.60 | 9.36 | 4.97 | 4.72 | 5.00 | 4.46 | 6.04 | 4.46 | 3.15 | — | — |
| Angle deviation, 95th percentile (F) | 21.11 | 30.15 | 30.34 | 16.26 | 14.08 | 15.30 | 15.38 | 19.11 | 15.38 | 9.83 | — | — |
| Mean aspect ratio (F) | 1.250 | 2.067 | 1.402 | 1.236 | 1.251 | 1.243 | 2.906 | 1.395 | 2.906 | 1.222 | — | — |
| Area variation (F) | 0.155 | 0.381 | 0.279 | 0.284 | 0.242 | 0.345 | 0.384 | 0.250 | 0.384 | 0.362 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 21.45 | 52.83 | 28.51 | 17.39 | 25.69 | 10.70 | 7.11 | 31.90 | 7.11 | 5.21 | — | — |
| Mean distance, ‰ (H) | 1.546 | 2.978 | 2.710 | 1.750 | 2.627 | 1.276 | 0.849 | 2.892 | 0.849 | 0.446 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.12 | 96.35 | 95.62 | 99.67 | 99.47 | 99.93 | 99.30 | 95.23 | 99.30 | 99.98 | — | — |
| Edges on closed loops, % (L) | 17.2 | 16.0 | — | — | 49.0 | 53.8 | 48.0 | — | — | 68.4 | — | — |
| Edges on spiralling loops, % (L) | 23.4 | 36.5 | — | — | 12.6 | 12.9 | 28.6 | — | — | 12.4 | — | — |
| Spiralling loops (L) | 10 | 20 | — | — | 4 | 4 | 12 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 58.8 | 47.5 | — | — | 38.5 | 33.3 | 23.4 | — | — | 19.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 47 | 74 | — | — | 43 | 20 | 64 | — | — | 36 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 4.3 | 72.0 | 2.6 | 3.0 | 0.1 | 0.1 | 222.4 | 6.6 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | 0.38 | 0.75 | 0.57 | 3.70 | 1.27 | 1.19 | 5.08 | 4.00 | 5.08 | 3.43 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1005 | 357 | 958 | 976 | 1010 | 994 | 1790 | 922 | 1790 | 1771 | — | — |
| Irregular vertices (S) | 67 | 48 | 114 | 51 | 43 | 48 | 68 | 68 | 68 | 50 | — | — |
| … of which above the minimum | — | 24 | 98 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 995 | 357 | 930 | 733 | 594 | 554 | 1538 | 802 | 1538 | 848 | — | — |
| Alignment, degrees (A) | 15.73 | 18.27 | 17.84 | 11.65 | 11.63 | 10.36 | 8.58 | 9.64 | 8.91 | 10.68 | — | — |
| Mean angle deviation, degrees (F) | 9.65 | 15.83 | 10.18 | 9.03 | 4.99 | 5.61 | 8.74 | 8.64 | 8.74 | 4.71 | — | — |
| Angle deviation, 95th percentile (F) | 29.23 | 49.29 | 34.26 | 26.04 | 17.67 | 18.44 | 23.66 | 23.37 | 23.66 | 14.65 | — | — |
| Mean aspect ratio (F) | 1.325 | 2.035 | 1.423 | 1.311 | 1.240 | 1.303 | 1.823 | 1.345 | 1.823 | 1.235 | — | — |
| Area variation (F) | 0.506 | 0.760 | 0.395 | 0.357 | 0.293 | 0.433 | 0.551 | 0.247 | 0.551 | 0.389 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 44.79 | 137.08 | 50.97 | 36.86 | 18.70 | 12.92 | 10.72 | 17.45 | 10.72 | 9.24 | — | — |
| Mean distance, ‰ (H) | 2.245 | 13.357 | 3.552 | 2.048 | 1.646 | 1.473 | 1.112 | 2.422 | 1.112 | 0.799 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 10.6 | 112.8 | 23.9 | 12.5 | 9.8 | 5.1 | 4.8 | 12.7 | 6.4 | 3.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.0 | — | — | 20.5 | 18.9 | 4.2 | — | — | 22.5 | — | — |
| Edges on spiralling loops, % (L) | 7.9 | 5.7 | — | — | 0.0 | 8.0 | 15.1 | — | — | 6.8 | — | — |
| Spiralling loops (L) | 5 | 2 | — | — | 0 | 4 | 7 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.6 | 94.3 | — | — | 71.7 | 67.9 | 77.2 | — | — | 63.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 97 | 43 | — | — | 56 | 54 | 112 | — | — | 143 | — | — |
| Edges on loops from a border to a border, % (L) | 0.6 | 0.0 | — | — | 7.8 | 5.2 | 3.5 | — | — | 7.4 | — | — |
| Off budget, % (N) | 1.0 | 64.3 | 4.2 | 2.4 | 1.0 | 0.6 | 79.0 | 7.8 | 0.0 | 1.1 | — | — |
| Time, seconds (T) | 0.66 | 0.43 | 0.30 | 5.95 | 1.83 | 1.31 | 4.46 | 3.77 | 4.46 | 1.73 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 891 | 311 | 932 | 802 | 994 | 980 | 2890 | 799 | 2890 | 2864 | — | — |
| Irregular vertices (S) | 73 | 36 | 114 | 47 | 46 | 102 | 245 | 142 | 245 | 153 | — | — |
| … of which above the minimum | 64 | — | 99 | — | 38 | 94 | 237 | — | 237 | 145 | — | — |
| Base complex patches (C) | 891 | — | 860 | — | 504 | 542 | 2186 | — | 2186 | 2218 | — | — |
| Alignment, degrees (A) | 20.86 | 17.10 | 18.87 | 11.86 | 10.53 | 11.12 | 7.89 | 10.49 | 9.35 | 11.72 | — | — |
| Mean angle deviation, degrees (F) | 14.18 | 13.19 | 9.68 | 11.53 | 7.41 | 9.10 | 9.34 | 12.33 | 9.34 | 6.37 | — | — |
| Angle deviation, 95th percentile (F) | 39.23 | 35.34 | 29.97 | 29.24 | 20.41 | 26.94 | 26.48 | 34.04 | 26.48 | 17.84 | — | — |
| Mean aspect ratio (F) | 1.552 | 1.883 | 1.419 | 1.628 | 1.693 | 1.769 | 2.393 | 1.743 | 2.393 | 1.447 | — | — |
| Area variation (F) | 0.348 | 0.671 | 0.282 | 0.388 | 0.431 | 0.756 | 0.728 | 0.444 | 0.728 | 0.503 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 126.19 | 85.71 | 33.77 | 28.57 | 30.07 | 27.76 | 51.64 | 27.76 | 29.78 | — | — |
| Mean distance, ‰ (H) | 4.418 | 13.988 | 3.773 | 2.160 | 1.626 | 1.805 | 0.968 | 3.165 | 0.968 | 0.742 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 115.0 | 180.3 | 107.2 | 35.4 | 20.1 | 6.1 | 4.5 | 50.1 | 8.9 | 7.7 | — | — |
| Volume kept, % of the source (V) | 94.57 | 87.06 | 95.94 | 99.36 | 99.75 | 99.70 | 98.71 | — | 98.71 | 99.99 | — | — |
| Edges on closed loops, % (L) | 0.3 | 7.8 | — | — | 24.6 | 22.3 | 12.2 | — | — | 11.3 | — | — |
| Edges on spiralling loops, % (L) | 10.9 | 25.0 | — | — | 14.8 | 4.4 | 9.3 | — | — | 16.8 | — | — |
| Spiralling loops (L) | 6 | 4 | — | — | 3 | 6 | 9 | — | — | 27 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.1 | 67.2 | — | — | 60.6 | 73.3 | 78.4 | — | — | 72.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 102 | 17 | — | — | 82 | 43 | 124 | — | — | 129 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 10.9 | 68.9 | 6.8 | 19.8 | 0.6 | 2.0 | 189.0 | 20.1 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | 3.32 | 0.64 | 2.71 | 4.70 | 3.01 | 3.79 | 18.93 | 16.67 | 18.93 | 5.31 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 176 | 868 | 613 | 613 | 1039 | 2672 | 920 | 2672 | 2695 | 10590 | 10709 |
| Irregular vertices (S) | — | 38 | 96 | 46 | 46 | 108 | 212 | 86 | 212 | 146 | 476 | 257 |
| … of which above the minimum | — | — | 76 | 38 | 38 | 100 | — | — | — | 138 | — | 249 |
| Base complex patches (C) | — | — | 578 | 301 | 301 | 525 | — | — | — | 1519 | — | 6523 |
| Alignment, degrees (A) | — | 11.90 | 11.10 | 7.73 | 7.73 | 9.61 | 5.77 | 6.19 | 6.29 | 7.96 | 9.88 | 8.32 |
| Mean angle deviation, degrees (F) | — | 16.80 | 9.77 | 9.59 | 9.59 | 8.29 | 6.83 | 8.06 | 6.83 | 5.93 | 9.24 | 4.33 |
| Angle deviation, 95th percentile (F) | — | 55.64 | 35.19 | 30.11 | 30.11 | 26.62 | 23.37 | 27.56 | 23.37 | 19.08 | 26.27 | 13.76 |
| Mean aspect ratio (F) | — | 3.300 | 1.497 | 1.661 | 1.661 | 1.510 | 2.345 | 1.522 | 2.345 | 1.379 | 1.616 | 1.312 |
| Area variation (F) | — | 0.883 | 0.327 | 0.631 | 0.631 | 1.109 | 0.665 | 0.385 | 0.665 | 0.661 | 1.417 | 0.461 |
| Hausdorff, ‰ of the diagonal (H) | — | 160.81 | 101.17 | 35.69 | 35.69 | 18.99 | 9.99 | 47.40 | 9.99 | 12.07 | 6.47 | 4.93 |
| Mean distance, ‰ (H) | — | 58.173 | 13.524 | 4.905 | 4.905 | 2.166 | 1.006 | 5.690 | 1.006 | 1.162 | 0.172 | 0.193 |
| Thin parts: mean distance, % of a quad edge (V) | — | 331.3 | 119.6 | 43.9 | 43.9 | 12.2 | 4.6 | 52.8 | 8.0 | 13.3 | 1.5 | 6.9 |
| Volume kept, % of the source (V) | — | 51.85 | 90.43 | 98.89 | 98.89 | 99.31 | 98.23 | — | 98.23 | 100.05 | 98.80 | 100.01 |
| Edges on closed loops, % (L) | — | 21.5 | 15.7 | 25.4 | 25.4 | 24.7 | 27.1 | — | 27.1 | 21.8 | 31.6 | 19.5 |
| Edges on spiralling loops, % (L) | — | 35.4 | 7.9 | 23.2 | 23.2 | 16.9 | 13.9 | — | 13.9 | 14.0 | 2.5 | 18.1 |
| Spiralling loops (L) | — | 18 | 13 | 9 | 9 | 31 | 52 | — | 52 | 11 | 3 | 18 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 43.1 | 78.4 | 51.3 | 51.3 | 58.3 | 59.0 | — | 59.0 | 64.1 | 65.8 | 62.4 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 16 | 40 | 50 | 50 | 62 | 107 | — | 107 | 241 | 274 | 521 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 82.4 | 13.2 | 38.7 | 38.7 | 3.9 | 167.2 | 8.0 | 0.0 | 0.9 | 0.0 | 1.1 |
| Time, seconds (T) | — | 4.03 | 19.84 | 13.59 | 28.88 | 14.56 | 22.46 | 24.37 | 22.46 | 15.45 | — | 31.44 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 172 | 877 | 869 | 984 | 995 | 2809 | 845 | 2809 | 2798 | 10590 | 10656 |
| Irregular vertices (S) | — | 38 | 105 | 47 | 52 | 103 | 223 | 98 | 223 | 159 | 476 | 278 |
| … of which above the minimum | — | — | 72 | 39 | 44 | 95 | — | — | — | 151 | — | 270 |
| Base complex patches (C) | — | — | 639 | 279 | 358 | 571 | — | — | — | 1142 | — | 4118 |
| Alignment, degrees (A) | — | 12.34 | 11.11 | 6.25 | 6.37 | 8.43 | 5.39 | 5.49 | 5.91 | 7.13 | 9.43 | 7.47 |
| Mean angle deviation, degrees (F) | — | 30.39 | 9.85 | 7.99 | 5.55 | 8.19 | 6.64 | 7.61 | 6.64 | 5.41 | 8.82 | 4.18 |
| Angle deviation, 95th percentile (F) | — | 82.98 | 32.30 | 26.53 | 19.59 | 26.65 | 22.69 | 25.67 | 22.69 | 19.23 | 25.47 | 14.56 |
| Mean aspect ratio (F) | — | 3.326 | 1.445 | 1.445 | 1.408 | 1.531 | 2.429 | 1.466 | 2.429 | 1.337 | 1.657 | 1.277 |
| Area variation (F) | — | 1.040 | 0.326 | 0.428 | 0.444 | 1.002 | 0.650 | 0.347 | 0.650 | 0.675 | 1.432 | 0.488 |
| Hausdorff, ‰ of the diagonal (H) | — | 100.86 | 70.01 | 40.87 | 29.49 | 22.78 | 12.40 | 79.14 | 12.40 | 10.85 | 4.44 | 4.97 |
| Mean distance, ‰ (H) | — | 33.458 | 11.587 | 6.423 | 2.843 | 2.271 | 0.888 | 7.145 | 0.888 | 0.922 | 0.156 | 0.189 |
| Thin parts: mean distance, % of a quad edge (V) | — | 191.1 | 92.0 | 64.6 | 26.2 | 16.5 | 5.5 | 71.9 | 10.1 | 16.1 | 1.4 | 6.8 |
| Volume kept, % of the source (V) | — | 35.72 | 90.12 | 98.59 | 99.66 | 99.63 | 98.29 | 90.99 | 98.29 | 99.95 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | 16.2 | 14.7 | 33.9 | 31.8 | 21.3 | 26.7 | 30.1 | 26.7 | 29.6 | 31.6 | 30.7 |
| Edges on spiralling loops, % (L) | — | 46.8 | 7.6 | 10.7 | 4.0 | 14.8 | 9.8 | 8.5 | 9.8 | 4.1 | 2.5 | 4.0 |
| Spiralling loops (L) | — | 18 | 10 | 11 | 4 | 24 | 48 | 10 | 48 | 11 | 3 | 14 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 37.0 | 79.2 | 55.4 | 64.2 | 63.9 | 63.6 | 61.5 | 63.6 | 66.3 | 65.8 | 65.3 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 11 | 42 | 137 | 85 | 55 | 144 | 101 | 144 | 195 | 274 | 389 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 82.8 | 12.3 | 13.1 | 1.6 | 0.5 | 180.9 | 15.5 | 0.0 | 0.4 | 0.0 | 0.6 |
| Time, seconds (T) | — | 3.40 | 20.81 | 14.45 | 12.04 | 15.90 | 23.55 | 22.00 | 23.55 | 15.67 | — | 29.64 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1024 | 404 | 1016 | 910 | 988 | 1008 | 1497 | 882 | 1497 | 1478 | — | — |
| Irregular vertices (S) | 62 | 39 | 95 | 63 | 64 | 68 | 78 | 76 | 78 | 68 | — | — |
| … of which above the minimum | 54 | — | 87 | 55 | 56 | 60 | 70 | 68 | 70 | 60 | — | — |
| Base complex patches (C) | 965 | — | 982 | 690 | 664 | 576 | 1079 | 630 | 1079 | 852 | — | — |
| Alignment, degrees (A) | 18.23 | 16.98 | 17.84 | 10.63 | 9.28 | 7.87 | 6.34 | 7.17 | 6.45 | 7.71 | — | — |
| Mean angle deviation, degrees (F) | 7.55 | 9.29 | 7.36 | 11.20 | 7.46 | 7.13 | 8.24 | 8.98 | 8.24 | 6.01 | — | — |
| Angle deviation, 95th percentile (F) | 22.42 | 25.28 | 25.61 | 29.66 | 19.76 | 21.23 | 22.62 | 24.90 | 22.62 | 17.21 | — | — |
| Mean aspect ratio (F) | 1.252 | 1.483 | 1.267 | 1.448 | 1.410 | 1.406 | 1.689 | 1.455 | 1.689 | 1.357 | — | — |
| Area variation (F) | 0.209 | 0.498 | 0.211 | 0.350 | 0.410 | 0.458 | 0.555 | 0.343 | 0.555 | 0.455 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 66.11 | 45.10 | 19.66 | 9.83 | 6.60 | 8.32 | 15.39 | 8.32 | 6.72 | — | — |
| Mean distance, ‰ (H) | 1.330 | 5.964 | 1.556 | 0.811 | 0.666 | 0.600 | 0.810 | 1.403 | 0.810 | 0.405 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 29.6 | 41.7 | 37.4 | 23.1 | 9.3 | 4.8 | 12.2 | 14.9 | 14.7 | 5.8 | — | — |
| Volume kept, % of the source (V) | 98.16 | 91.73 | 97.96 | 99.85 | 100.05 | 100.02 | 98.52 | 97.64 | 98.52 | 100.02 | — | — |
| Edges on closed loops, % (L) | 1.8 | 2.1 | — | — | 16.4 | 21.6 | 14.0 | — | — | 21.2 | — | — |
| Edges on spiralling loops, % (L) | 13.9 | 4.8 | — | — | 7.5 | 1.7 | 5.8 | — | — | 3.7 | — | — |
| Spiralling loops (L) | 5 | 2 | — | — | 4 | 2 | 3 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.6 | 93.0 | — | — | 76.1 | 76.6 | 80.2 | — | — | 75.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 48 | — | — | 92 | 96 | 77 | — | — | 160 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 59.6 | 1.6 | 9.0 | 1.2 | 0.8 | 49.7 | 11.8 | 0.0 | 1.3 | — | — |
| Time, seconds (T) | 1.53 | 0.40 | 2.26 | 4.55 | 3.72 | 1.95 | 17.76 | 13.95 | 17.76 | 3.59 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 972 | 576 | 1006 | 861 | 993 | 1003 | 954 | 893 | 954 | 951 | — | — |
| Irregular vertices (S) | 28 | 12 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 | — | — |
| … of which above the minimum | 28 | 12 | 69 | 18 | 18 | 20 | 21 | 21 | 21 | 20 | — | — |
| Base complex patches (C) | 958 | 125 | 1006 | 177 | 237 | 201 | 398 | 235 | 398 | 185 | — | — |
| Alignment, degrees (A) | 20.00 | 7.87 | 20.88 | 4.19 | 4.29 | 4.18 | 3.16 | 3.69 | 3.14 | 3.79 | — | — |
| Mean angle deviation, degrees (F) | 7.93 | 9.54 | 7.12 | 5.39 | 4.49 | 4.87 | 5.39 | 6.25 | 5.39 | 4.80 | — | — |
| Angle deviation, 95th percentile (F) | 22.21 | 23.72 | 22.89 | 16.58 | 11.75 | 13.68 | 15.59 | 19.36 | 15.59 | 13.43 | — | — |
| Mean aspect ratio (F) | 1.201 | 2.466 | 1.225 | 1.445 | 1.380 | 1.386 | 1.530 | 1.451 | 1.530 | 1.419 | — | — |
| Area variation (F) | 0.159 | 0.448 | 0.197 | 0.357 | 0.435 | 0.488 | 0.413 | 0.359 | 0.413 | 0.449 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 11.15 | 28.68 | 19.09 | 4.85 | 4.39 | 7.54 | 7.25 | 7.54 | 4.39 | — | — |
| Mean distance, ‰ (H) | 0.966 | 2.793 | 1.021 | 0.443 | 0.384 | 0.386 | 1.012 | 1.114 | 1.012 | 0.398 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.35 | 94.76 | 98.23 | 100.00 | 100.05 | 100.05 | 97.96 | 97.78 | 97.96 | 100.05 | — | — |
| Edges on closed loops, % (L) | 0.0 | 49.1 | — | — | 50.2 | 55.9 | 36.2 | — | — | 57.1 | — | — |
| Edges on spiralling loops, % (L) | 27.7 | 17.7 | — | — | 17.9 | 3.7 | 32.4 | — | — | 3.8 | — | — |
| Spiralling loops (L) | 5 | 2 | — | — | 1 | 1 | 5 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 71.2 | 33.2 | — | — | 31.9 | 40.4 | 31.4 | — | — | 39.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 76 | — | — | 68 | 94 | 71 | — | — | 76 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 42.4 | 2.4 | 13.9 | 0.7 | 0.3 | 4.6 | 10.7 | 0.0 | 0.3 | — | — |
| Time, seconds (T) | 1.49 | 0.45 | 1.10 | 2.90 | 1.78 | 2.23 | 12.87 | 16.31 | 12.87 | 2.55 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 847 | 604 | 996 | 888 | 1027 | 984 | 1638 | 892 | 1638 | 1668 | — | — |
| Irregular vertices (S) | 44 | 58 | 82 | 35 | 40 | 50 | 60 | 53 | 60 | 50 | — | — |
| … of which above the minimum | 32 | — | 74 | — | 32 | 42 | 52 | 45 | 52 | 42 | — | — |
| Base complex patches (C) | 785 | — | 980 | — | 463 | 532 | 836 | 440 | 836 | 654 | — | — |
| Alignment, degrees (A) | 16.32 | 14.60 | 17.18 | 9.17 | 8.33 | 9.03 | 4.88 | 6.96 | 5.29 | 8.06 | — | — |
| Mean angle deviation, degrees (F) | 10.04 | 10.41 | 7.61 | 8.65 | 4.92 | 5.70 | 6.85 | 8.39 | 6.85 | 5.25 | — | — |
| Angle deviation, 95th percentile (F) | 28.85 | 32.33 | 26.75 | 23.66 | 14.45 | 17.17 | 19.46 | 22.44 | 19.46 | 15.92 | — | — |
| Mean aspect ratio (F) | 1.336 | 1.771 | 1.285 | 1.331 | 1.283 | 1.313 | 2.117 | 1.414 | 2.117 | 1.312 | — | — |
| Area variation (F) | 0.267 | 0.634 | 0.235 | 0.263 | 0.350 | 0.464 | 0.568 | 0.310 | 0.568 | 0.408 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 70.95 | 37.01 | 27.91 | 22.61 | 15.87 | 8.09 | 29.46 | 8.09 | 10.77 | — | — |
| Mean distance, ‰ (H) | 1.731 | 4.041 | 1.524 | 1.011 | 0.655 | 0.653 | 0.521 | 1.283 | 0.521 | 0.305 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 30.4 | 31.8 | 26.6 | 23.7 | 12.1 | 8.2 | 4.0 | 21.3 | 6.8 | 7.2 | — | — |
| Volume kept, % of the source (V) | 96.59 | — | 97.11 | 99.24 | 99.68 | 99.74 | 98.99 | 97.39 | 98.99 | 99.94 | — | — |
| Edges on closed loops, % (L) | 4.2 | 6.4 | — | — | 30.0 | 23.0 | 24.5 | — | — | 32.4 | — | — |
| Edges on spiralling loops, % (L) | 19.3 | 7.7 | — | — | 9.4 | 20.5 | 18.7 | — | — | 1.1 | — | — |
| Spiralling loops (L) | 7 | 3 | — | — | 1 | 6 | 6 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.3 | 85.9 | — | — | 60.6 | 56.5 | 56.8 | — | — | 66.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 87 | 42 | — | — | 81 | 58 | 84 | — | — | 196 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 15.3 | 39.6 | 1.5 | 11.2 | 2.7 | 1.6 | 63.8 | 10.8 | 0.0 | 1.8 | — | — |
| Time, seconds (T) | 1.97 | 1.39 | 1.14 | 3.05 | 1.98 | 2.15 | 14.40 | 12.45 | 14.40 | 2.17 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 861 | 548 | 910 | 877 | 990 | 1008 | 1975 | 853 | 1975 | 1973 | — | — |
| Irregular vertices (S) | 92 | 69 | 94 | 53 | 60 | 90 | 95 | 85 | 95 | 94 | — | — |
| … of which above the minimum | 76 | — | 75 | 45 | 52 | 82 | 87 | — | 87 | 86 | — | — |
| Base complex patches (C) | 848 | — | 861 | 455 | 556 | 588 | 1149 | — | 1149 | 1109 | — | — |
| Alignment, degrees (A) | 17.33 | 14.59 | 16.11 | 13.09 | 12.43 | 9.21 | 7.39 | 10.36 | 7.52 | 10.01 | — | — |
| Mean angle deviation, degrees (F) | 15.46 | 12.13 | 8.44 | 8.76 | 7.23 | 7.11 | 8.27 | 9.33 | 8.27 | 6.51 | — | — |
| Angle deviation, 95th percentile (F) | 46.32 | 38.98 | 30.16 | 25.74 | 24.11 | 21.71 | 23.65 | 27.06 | 23.65 | 19.50 | — | — |
| Mean aspect ratio (F) | 1.533 | 1.947 | 1.339 | 1.359 | 1.332 | 1.445 | 2.045 | 1.425 | 2.045 | 1.300 | — | — |
| Area variation (F) | 0.437 | 0.836 | 0.272 | 0.274 | 0.326 | 0.885 | 0.753 | 0.323 | 0.753 | 0.502 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 63.50 | 97.30 | 33.69 | 21.44 | 14.22 | 11.45 | 85.93 | 11.45 | 21.90 | — | — |
| Mean distance, ‰ (H) | 4.074 | 8.335 | 5.299 | 1.631 | 1.047 | 1.035 | 0.786 | 2.893 | 0.786 | 0.463 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 84.1 | 44.9 | 119.6 | 22.6 | 12.6 | 4.3 | 3.5 | 50.0 | 6.0 | 6.8 | — | — |
| Volume kept, % of the source (V) | 96.89 | 90.89 | 96.53 | 98.78 | 99.44 | 99.77 | 98.97 | 97.29 | 98.97 | 99.93 | — | — |
| Edges on closed loops, % (L) | 0.9 | 13.1 | — | — | 23.6 | 21.6 | 20.9 | — | — | 23.6 | — | — |
| Edges on spiralling loops, % (L) | 14.4 | 4.5 | — | — | 15.0 | 7.0 | 7.0 | — | — | 3.4 | — | — |
| Spiralling loops (L) | 9 | 4 | — | — | 11 | 5 | 4 | — | — | 7 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 83.9 | 82.4 | — | — | 61.4 | 71.3 | 72.1 | — | — | 73.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 94 | 24 | — | — | 107 | 72 | 130 | — | — | 160 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 13.9 | 45.2 | 9.0 | 12.3 | 1.0 | 0.8 | 97.5 | 14.7 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | 2.16 | 0.55 | 1.07 | 4.82 | 2.10 | 2.37 | 17.10 | 15.43 | 17.10 | 3.56 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 3/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 900 | 305 | 886 | 867 | 991 | 987 | 3093 | 876 | 3093 | 3116 | — | — |
| Irregular vertices (S) | 69 | 33 | 108 | 50 | 51 | 83 | 182 | 150 | 182 | 172 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 724 | — | 849 | 661 | 581 | 863 | — | — | — | 2723 | — | — |
| Alignment, degrees (A) | 18.49 | 20.52 | 19.26 | 16.55 | 14.31 | 15.03 | 12.26 | 14.41 | 12.79 | 13.99 | — | — |
| Mean angle deviation, degrees (F) | 10.23 | 18.53 | 11.03 | 11.24 | 7.58 | 9.31 | 10.70 | 12.37 | 10.70 | 6.88 | — | — |
| Angle deviation, 95th percentile (F) | 28.90 | 57.15 | 36.55 | 30.16 | 20.74 | 27.74 | 30.61 | 34.62 | 30.61 | 21.49 | — | — |
| Mean aspect ratio (F) | 1.359 | 1.887 | 1.449 | 1.397 | 1.395 | 1.467 | 2.192 | 1.561 | 2.192 | 1.333 | — | — |
| Area variation (F) | 0.306 | 1.686 | 0.338 | 0.295 | 0.342 | 0.752 | 0.752 | 0.389 | 0.752 | 0.519 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 201.66 | 84.04 | 69.02 | 36.46 | 26.83 | 7.66 | 49.73 | 7.66 | 26.16 | — | — |
| Mean distance, ‰ (H) | 4.435 | 12.497 | 6.117 | 4.705 | 2.848 | 2.933 | 0.950 | 3.575 | 0.950 | 0.818 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 28.0 | 71.5 | 42.2 | 40.7 | 20.9 | 14.0 | 4.8 | 18.4 | 9.8 | 16.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 5.2 | 1.9 | 1.1 | — | — | 1.5 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 19.9 | — | — | 0.0 | 3.3 | 6.9 | — | — | 2.6 | — | — |
| Spiralling loops (L) | 0 | 3 | — | — | 0 | 3 | 8 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.7 | 80.1 | — | — | 76.4 | 90.3 | 91.4 | — | — | 90.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | 24 | — | — | 59 | 57 | 129 | — | — | 116 | — | — |
| Edges on loops from a border to a border, % (L) | 14.3 | 0.0 | — | — | 18.3 | 4.6 | 0.7 | — | — | 5.0 | — | — |
| Off budget, % (N) | 10.0 | 69.5 | 11.4 | 13.3 | 0.9 | 1.3 | 209.3 | 12.4 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 0.81 | 0.36 | 0.37 | 3.56 | 2.02 | 1.66 | 6.67 | 10.59 | 6.67 | 4.13 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 494 | 157 | 530 | 511 | 1571 | 1403 | 26339 | 1271 | 26339 | 26552 | — | — |
| Irregular vertices (S) | 164 | 122 | 181 | 165 | 690 | 689 | 1682 | 572 | 1682 | 2259 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | 1643 | — | — |
| Base complex patches (C) | 453 | — | 500 | — | — | — | — | — | — | 19202 | — | — |
| Alignment, degrees (A) | 18.87 | 18.25 | 18.59 | 15.78 | 13.56 | — | 9.47 | 14.37 | 12.49 | 9.94 | — | — |
| Mean angle deviation, degrees (F) | 18.64 | 49.67 | 20.14 | 17.11 | 29.26 | 29.69 | 9.03 | 16.83 | 9.03 | 7.90 | — | — |
| Angle deviation, 95th percentile (F) | 52.24 | 87.99 | 58.52 | 51.04 | 75.25 | 77.33 | 26.22 | 49.51 | 26.22 | 25.05 | — | — |
| Mean aspect ratio (F) | 2.033 | 8.023 | 2.108 | 1.882 | 16.549 | 11.075 | 1.662 | 1.788 | 1.662 | 1.543 | — | — |
| Area variation (F) | 0.598 | 1.952 | 0.707 | 0.540 | 2.339 | 2.984 | 1.972 | 1.056 | 1.972 | 0.587 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 76.78 | 195.94 | 97.09 | 113.33 | 73.60 | 89.42 | 74.63 | 78.15 | 74.63 | 7.95 | — | — |
| Mean distance, ‰ (H) | 13.492 | 46.413 | 14.528 | 10.097 | 17.915 | 20.570 | 1.049 | 5.982 | 1.049 | 0.216 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 64.1 | 184.7 | 71.0 | 47.9 | 78.8 | 87.6 | 5.8 | 28.8 | 91.4 | 9.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 13.95 | — | — | — | — | 99.49 | — | — |
| Edges on closed loops, % (L) | 3.9 | — | — | — | 2.5 | 5.0 | — | — | — | 14.1 | — | — |
| Edges on spiralling loops, % (L) | 19.8 | — | — | — | 28.1 | 34.0 | — | — | — | 12.6 | — | — |
| Spiralling loops (L) | 46 | — | — | — | 377 | 392 | — | — | — | 290 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.7 | — | — | — | 69.4 | 61.0 | — | — | — | 73.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 16 | — | — | — | 23 | 25 | — | — | — | 168 | — | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | 50.6 | 84.3 | 47.0 | 48.9 | 57.1 | 40.3 | 2533.9 | 27.1 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 2.21 | 0.41 | 1.63 | 5.94 | 3.49 | 4.07 | 14.03 | 9.27 | 14.03 | 20.40 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 748 | 845 | 919 | 1008 | 1008 | 2105 | 936 | 2105 | 2126 | — | — |
| Irregular vertices (S) | — | 54 | 83 | 28 | 36 | 36 | 76 | 54 | 76 | 36 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 532 | 177 | 254 | 451 | 1346 | — | 1346 | 515 | — | — |
| Alignment, degrees (A) | — | 11.10 | 13.44 | 7.94 | 6.96 | 6.66 | 6.48 | 9.01 | 7.14 | 7.07 | — | — |
| Mean angle deviation, degrees (F) | — | 16.94 | 8.67 | 7.16 | 4.11 | 5.57 | 8.08 | 9.87 | 8.08 | 4.12 | — | — |
| Angle deviation, 95th percentile (F) | — | 65.68 | 30.33 | 20.15 | 12.89 | 17.02 | 22.80 | 25.01 | 22.80 | 11.49 | — | — |
| Mean aspect ratio (F) | — | 2.304 | 1.363 | 1.329 | 1.284 | 1.330 | 2.085 | 1.391 | 2.085 | 1.272 | — | — |
| Area variation (F) | — | 1.107 | 0.276 | 0.284 | 0.270 | 0.535 | 0.486 | 0.271 | 0.486 | 0.445 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 98.43 | 53.13 | 13.94 | 17.46 | 9.08 | 5.71 | 13.65 | 5.71 | 7.31 | — | — |
| Mean distance, ‰ (H) | — | 7.488 | 2.255 | 1.095 | 0.891 | 0.814 | 0.640 | 1.473 | 0.640 | 0.551 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 15.2 | 48.5 | 6.1 | 4.6 | 2.8 | 3.4 | 5.6 | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.1 | — | — | 22.1 | 19.5 | 10.9 | — | — | 26.2 | — | — |
| Edges on spiralling loops, % (L) | — | 9.4 | — | — | 0.0 | 15.3 | 16.9 | — | — | 3.8 | — | — |
| Spiralling loops (L) | — | 4 | — | — | 0 | 3 | 7 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 71.5 | — | — | 54.9 | 51.2 | 62.9 | — | — | 48.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 41 | — | — | 47 | 47 | 108 | — | — | 71 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 23.1 | 14.0 | 9.3 | — | — | 21.7 | — | — |
| Off budget, % (N) | — | 25.2 | 15.5 | 8.1 | 0.8 | 0.8 | 110.5 | 6.4 | 0.0 | 1.0 | — | — |
| Time, seconds (T) | — | 0.55 | 1.83 | 5.17 | 2.65 | 2.92 | 13.07 | 11.79 | 13.07 | 3.12 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 943 | 348 | 1036 | 959 | 976 | 1018 | 1013 | 924 | 1013 | 1018 | — | — |
| Irregular vertices (S) | 30 | 69 | 115 | 24 | 24 | 24 | 26 | 24 | 26 | 24 | — | — |
| … of which above the minimum | 22 | — | 107 | 16 | 16 | 16 | 18 | 16 | 18 | 16 | — | — |
| Base complex patches (C) | 227 | — | 630 | 388 | 66 | 68 | 367 | 293 | 367 | 68 | — | — |
| Alignment, degrees (A) | 13.41 | 19.45 | 14.60 | 12.82 | 12.53 | 10.80 | 11.10 | 10.93 | 11.10 | 10.82 | — | — |
| Mean angle deviation, degrees (F) | 3.78 | 19.61 | 8.58 | 4.85 | 1.76 | 2.16 | 4.39 | 4.53 | 4.39 | 2.17 | — | — |
| Angle deviation, 95th percentile (F) | 15.04 | 60.84 | 35.19 | 15.54 | 8.65 | 10.18 | 14.60 | 15.17 | 14.60 | 10.14 | — | — |
| Mean aspect ratio (F) | 1.191 | 2.662 | 1.407 | 1.208 | 1.217 | 1.177 | 1.323 | 1.314 | 1.323 | 1.177 | — | — |
| Area variation (F) | 0.176 | 0.647 | 0.263 | 0.143 | 0.181 | 0.381 | 0.296 | 0.273 | 0.296 | 0.381 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 93.30 | 20.39 | 17.19 | 15.00 | 1.37 | 1.64 | 2.29 | 1.64 | 1.37 | — | — |
| Mean distance, ‰ (H) | 0.335 | 8.243 | 0.238 | 0.679 | 0.271 | 0.165 | 0.147 | 0.173 | 0.147 | 0.165 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.43 | — | 100.43 | 99.90 | 100.00 | 100.01 | 100.26 | 100.31 | 100.26 | 100.01 | — | — |
| Edges on closed loops, % (L) | 47.9 | — | — | — | 73.4 | 74.5 | 35.9 | — | — | 74.5 | — | — |
| Edges on spiralling loops, % (L) | 13.6 | — | — | — | 0.0 | 0.0 | 28.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 4 | — | — | — | 0 | 0 | 7 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 36.6 | — | — | — | 26.6 | 25.5 | 36.0 | — | — | 25.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 36 | — | — | — | 32 | 26 | 50 | — | — | 26 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.7 | 65.2 | 3.6 | 4.1 | 2.4 | 1.8 | 1.3 | 7.6 | 0.0 | 0.5 | — | — |
| Time, seconds (T) | 0.97 | 0.32 | 0.15 | 8.06 | 1.46 | 1.02 | 3.06 | 2.65 | 3.06 | 0.97 | — | — |

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
| Area variation (F) | 0.002 | 0.500 | 0.103 | 0.074 | 0.000 | 0.082 | 0.000 | 0.000 | 0.000 | 0.081 | — | — |
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
| Time, seconds (T) | 0.30 | 0.24 | 0.06 | 2.94 | 0.70 | 0.70 | 2.35 | 3.01 | 2.35 | 1.04 | — | — |

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
| Time, seconds (T) | 0.21 | 0.17 | 0.05 | 3.81 | 0.63 | 0.73 | 2.79 | 2.67 | 2.79 | 0.75 | — | — |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 995 | 612 | 994 | 900 | 990 | 1002 | 782 | 782 | 782 | 764 | — | — |
| Irregular vertices (S) | 8 | 8 | 40 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 32 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 376 | 164 | 576 | 46 | 6 | 186 | 138 | 138 | 138 | 56 | — | — |
| Alignment, degrees (A) | 3.95 | 5.40 | 4.86 | 1.52 | 1.23 | 1.85 | 1.55 | 1.55 | 1.67 | 1.37 | — | — |
| Mean angle deviation, degrees (F) | 4.50 | 8.90 | 5.03 | 3.70 | 2.64 | 3.73 | 4.01 | 4.01 | 4.01 | 2.54 | — | — |
| Angle deviation, 95th percentile (F) | 17.64 | 26.84 | 20.26 | 12.51 | 7.63 | 8.66 | 11.66 | 11.66 | 11.66 | 9.04 | — | — |
| Mean aspect ratio (F) | 1.124 | 1.965 | 1.170 | 1.306 | 1.286 | 1.504 | 1.528 | 1.528 | 1.528 | 1.509 | — | — |
| Area variation (F) | 0.112 | 0.576 | 0.174 | 0.337 | 0.334 | 0.218 | 0.416 | 0.416 | 0.416 | 0.205 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 14.27 | 15.50 | 1.57 | 1.47 | 1.70 | 2.59 | 2.59 | 2.59 | 1.93 | — | — |
| Mean distance, ‰ (H) | 0.373 | 2.062 | 0.376 | 0.312 | 0.311 | 0.349 | 0.938 | 0.938 | 0.938 | 0.457 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.73 | 98.02 | 99.66 | 99.99 | 99.99 | 100.02 | 99.00 | 99.00 | 99.00 | 99.99 | — | — |
| Edges on closed loops, % (L) | 30.7 | 36.6 | — | — | 91.7 | 40.7 | 41.2 | — | — | 62.0 | — | — |
| Edges on spiralling loops, % (L) | 45.4 | 50.2 | — | — | 0.0 | 55.9 | 54.9 | — | — | 25.1 | — | — |
| Spiralling loops (L) | 3 | 4 | — | — | 0 | 5 | 5 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 24.2 | 13.2 | — | — | 8.3 | 3.4 | 3.9 | — | — | 12.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 134 | 30 | — | — | 23 | 16 | 21 | — | — | 58 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.8 | 38.8 | 1.0 | 10.0 | 1.0 | 0.2 | 21.8 | 21.8 | 0.0 | 2.3 | — | — |
| Time, seconds (T) | 0.30 | 0.16 | 0.05 | 15.50 | 0.81 | 0.92 | 2.92 | 2.88 | 2.92 | 1.35 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2390 | 639 | 742 | 757 | 836 | 12157 | 8690 | 12157 | 12464 | — | — |
| Irregular vertices (S) | — | 1182 | 184 | 113 | 150 | 176 | 606 | 581 | 606 | 874 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 460 | — | — | — | — | — | — | 9856 | — | — |
| Alignment, degrees (A) | — | 8.64 | 12.88 | 14.40 | 11.32 | 13.71 | 6.94 | 6.56 | 8.37 | 11.16 | — | — |
| Mean angle deviation, degrees (F) | — | 53.69 | 17.66 | 9.85 | 11.89 | 14.03 | 10.01 | 10.58 | 10.01 | 7.67 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.78 | 59.41 | 30.81 | 37.21 | 45.64 | 45.68 | 51.58 | 45.68 | 28.90 | — | — |
| Mean aspect ratio (F) | — | 7.059 | 2.099 | 1.393 | 1.553 | 3.065 | 2.423 | 2.658 | 2.423 | 1.538 | — | — |
| Area variation (F) | — | 5.029 | 0.480 | 0.363 | 0.486 | 0.595 | 2.342 | 2.356 | 2.342 | 0.376 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 174.38 | 68.00 | 81.94 | 82.83 | 82.83 | 14.79 | 27.48 | 14.79 | 21.31 | — | — |
| Mean distance, ‰ (H) | — | 17.134 | 12.935 | 11.384 | 7.832 | 8.266 | 0.450 | 0.882 | 0.450 | 1.266 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 31.9 | 39.2 | 34.9 | 21.1 | 22.9 | 1.4 | 2.2 | 3.4 | 22.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 19.1 | 12.0 | — | — | — | 10.9 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 25.6 | 25.8 | — | — | — | 31.0 | — | — |
| Spiralling loops (L) | — | — | — | — | 78 | 91 | — | — | — | 43 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 55.3 | 62.2 | — | — | — | 56.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 47 | 47 | — | — | — | 287 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 1.4 | — | — |
| Off budget, % (N) | — | 139.0 | 36.1 | 25.8 | 24.3 | 16.4 | 1115.7 | 769.0 | 0.0 | 2.5 | — | — |
| Time, seconds (T) | — | 0.36 | 0.60 | 7.70 | 6.66 | 8.98 | 6.97 | 3.89 | 6.97 | 80.75 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 334 | 940 | 743 | 1018 | 1011 | 2921 | 812 | 2921 | 2903 | — | — |
| Irregular vertices (S) | — | 38 | 121 | 44 | 46 | 118 | 229 | 126 | 229 | 209 | — | — |
| … of which above the minimum | — | — | 112 | 36 | 38 | 110 | 221 | — | 221 | 201 | — | — |
| Base complex patches (C) | — | — | 910 | 363 | 516 | 733 | 2159 | — | 2159 | 2273 | — | — |
| Alignment, degrees (A) | — | 18.93 | 19.57 | 14.37 | 14.18 | 15.15 | 10.54 | 14.09 | 10.54 | 13.04 | — | — |
| Mean angle deviation, degrees (F) | — | 14.98 | 10.46 | 10.54 | 8.09 | 9.39 | 9.73 | 11.98 | 9.73 | 7.03 | — | — |
| Angle deviation, 95th percentile (F) | — | 43.87 | 32.04 | 28.35 | 21.93 | 26.33 | 26.74 | 31.12 | 26.74 | 20.69 | — | — |
| Mean aspect ratio (F) | — | 1.799 | 1.433 | 1.615 | 16.060 | 1.559 | 2.436 | 1.706 | 2.436 | 1.435 | — | — |
| Area variation (F) | — | 0.758 | 0.296 | 0.399 | 0.416 | 0.892 | 0.740 | 0.435 | 0.740 | 0.762 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 124.75 | 81.49 | 38.88 | 27.59 | 30.52 | 29.33 | 57.04 | 29.33 | 17.36 | — | — |
| Mean distance, ‰ (H) | — | 11.455 | 3.775 | 2.299 | 1.617 | 1.906 | 0.976 | 3.129 | 0.976 | 0.759 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 89.7 | 99.1 | 37.7 | 21.9 | 5.9 | 3.8 | 47.2 | 7.0 | 4.0 | — | — |
| Volume kept, % of the source (V) | — | 86.48 | 95.96 | 99.19 | 99.64 | 99.83 | 98.60 | — | 98.60 | 100.06 | — | — |
| Edges on closed loops, % (L) | — | 5.8 | — | — | 24.7 | 14.1 | 13.4 | — | — | 10.9 | — | — |
| Edges on spiralling loops, % (L) | — | 18.0 | — | — | 6.4 | 11.0 | 10.5 | — | — | 14.5 | — | — |
| Spiralling loops (L) | — | 9 | — | — | 2 | 14 | 14 | — | — | 21 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.2 | — | — | 69.0 | 74.9 | 76.1 | — | — | 74.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 28 | — | — | 88 | 70 | 85 | — | — | 69 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 66.6 | 6.0 | 25.7 | 1.8 | 1.1 | 192.1 | 18.8 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | — | 17.00 | 99.29 | 66.59 | 49.76 | 55.73 | 36.93 | 32.56 | 36.93 | 62.51 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 763 | 225 | — | — | 592 | 592 | 9441 | — | — | — | — | — |
| Irregular vertices (S) | 130 | 79 | — | — | 106 | 106 | 881 | — | — | — | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 751 | — | — | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 16.26 | 19.32 | — | — | 16.52 | 16.52 | 11.80 | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 15.96 | 26.39 | — | — | 16.68 | 16.70 | 10.11 | — | — | — | — | — |
| Angle deviation, 95th percentile (F) | 53.61 | 82.13 | — | — | 62.19 | 62.19 | 31.55 | — | — | — | — | — |
| Mean aspect ratio (F) | 1.692 | 4.170 | — | — | 2.301 | 2.323 | 1.746 | — | — | — | — | — |
| Area variation (F) | 0.466 | 1.055 | — | — | 0.800 | 0.800 | 0.926 | — | — | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 130.87 | 298.17 | — | — | 119.51 | 119.51 | 38.62 | — | — | — | — | — |
| Mean distance, ‰ (H) | 17.928 | 82.620 | — | — | 15.502 | 15.509 | 2.325 | — | — | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 47.7 | 197.5 | — | — | 39.7 | 39.7 | 9.8 | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.7 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.5 | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | 14 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 86.8 | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 35 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 5.2 | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 23.7 | 77.5 | — | — | 40.8 | 40.8 | 844.1 | — | — | — | — | — |
| Time, seconds (T) | 0.84 | 1.57 | — | — | 12.63 | 13.65 | 526.55 | — | — | — | — | — |

Failures: openclare-matched: timed out after 1800 s.

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 866 | 405 | — | — | 1826 | 685 | 6083 | — | — | — | — | — |
| Irregular vertices (S) | 165 | 121 | — | — | 311 | 94 | 505 | — | — | — | — | — |
| … of which above the minimum | — | — | — | — | 7 | — | — | — | — | — | — | — |
| Base complex patches (C) | 846 | — | — | — | 1826 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 13.39 | 18.17 | — | — | 16.89 | 14.91 | 9.77 | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 14.47 | 27.63 | — | — | 25.86 | 15.35 | 10.07 | — | — | — | — | — |
| Angle deviation, 95th percentile (F) | 46.45 | 75.88 | — | — | 68.26 | 50.55 | 32.00 | — | — | — | — | — |
| Mean aspect ratio (F) | 1.621 | 16.899 | — | — | 5.045 | 1.801 | 2.374 | — | — | — | — | — |
| Area variation (F) | 0.403 | 0.903 | — | — | 1.939 | 0.428 | 0.688 | — | — | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 102.61 | 138.78 | — | — | 75.07 | 110.42 | 28.19 | — | — | — | — | — |
| Mean distance, ‰ (H) | 9.005 | 32.456 | — | — | 6.685 | 12.376 | 2.452 | — | — | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.9 | 75.3 | — | — | 19.2 | 30.9 | 8.2 | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 78.09 | — | 102.25 | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 0.0 | — | 0.6 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 8.1 | — | — | — | 6.1 | — | 15.2 | — | — | — | — | — |
| Spiralling loops (L) | 7 | — | — | — | 35 | — | 51 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.2 | — | — | — | 93.9 | — | 84.3 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 23 | — | — | — | 35 | — | 100 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | — | — | 0.0 | — | 0.0 | — | — | — | — | — |
| Off budget, % (N) | 13.4 | 59.5 | — | — | 82.6 | 31.5 | 508.3 | — | — | — | — | — |
| Time, seconds (T) | 0.64 | 1.20 | — | — | 48.31 | 9.79 | 79.69 | — | — | — | — | — |

Failures: openclare-matched: timed out after 1800 s.

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 1/1 | 0/1 | 0/1 | 0/0 | 0/0 | 1/1 | 0/0 | 0/0 |
| Quads made | 938 | 703 | — | — | 1018 | 991 | 5437 | — | — | 5433 | — | — |
| Irregular vertices (S) | 132 | 73 | — | — | 92 | 155 | 428 | — | — | 239 | — | — |
| … of which above the minimum | — | — | — | — | 28 | 91 | 376 | — | — | 175 | — | — |
| Base complex patches (C) | 926 | — | — | — | 894 | 971 | 4943 | — | — | 5086 | — | — |
| Alignment, degrees (A) | 15.11 | 17.16 | — | — | 14.04 | 15.12 | 11.05 | — | — | 11.80 | — | — |
| Mean angle deviation, degrees (F) | 12.56 | 11.97 | — | — | 9.32 | 14.96 | 9.53 | — | — | 5.90 | — | — |
| Angle deviation, 95th percentile (F) | 35.62 | 36.94 | — | — | 30.81 | 43.78 | 29.45 | — | — | 18.61 | — | — |
| Mean aspect ratio (F) | 1.391 | 1.994 | — | — | 1.692 | 2.194 | 1.908 | — | — | 1.290 | — | — |
| Area variation (F) | 0.275 | 0.656 | — | — | 0.429 | 1.002 | 0.903 | — | — | 0.376 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 98.74 | 186.96 | — | — | 33.04 | 30.67 | 20.43 | — | — | 14.64 | — | — |
| Mean distance, ‰ (H) | 4.506 | 9.114 | — | — | 3.546 | 3.454 | 2.028 | — | — | 1.329 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 38.2 | 76.4 | — | — | 20.0 | 13.0 | 12.4 | — | — | 21.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 99.52 | 99.32 | 98.75 | — | — | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.3 | — | — | — | 6.3 | 1.1 | 4.7 | — | — | 3.6 | — | — |
| Edges on spiralling loops, % (L) | 4.3 | — | — | — | 17.7 | 8.8 | 6.6 | — | — | 11.6 | — | — |
| Spiralling loops (L) | 14 | — | — | — | 21 | 17 | 14 | — | — | 20 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 95.4 | — | — | — | 76.0 | 90.1 | 88.7 | — | — | 84.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 50 | — | — | — | 54 | 39 | 137 | — | — | 206 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 6.2 | 29.7 | — | — | 1.8 | 0.9 | 443.7 | — | — | 0.1 | — | — |
| Time, seconds (T) | 0.87 | 0.85 | — | — | 1.71 | 7.29 | 31.11 | — | — | 6.87 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 370 | — | — | 141 | 164 | 6933 | — | — | 405 | — | — |
| Irregular vertices (S) | — | 75 | — | — | 28 | 36 | 599 | — | — | 106 | — | — |
| … of which above the minimum | — | — | — | — | 4 | 12 | — | — | — | 82 | — | — |
| Base complex patches (C) | — | — | — | — | 141 | 164 | — | — | — | 405 | — | — |
| Alignment, degrees (A) | — | 14.45 | — | — | 17.99 | 21.40 | 9.68 | — | — | 19.70 | — | — |
| Mean angle deviation, degrees (F) | — | 28.12 | — | — | 30.30 | 31.43 | 9.49 | — | — | 21.91 | — | — |
| Angle deviation, 95th percentile (F) | — | 83.45 | — | — | 72.51 | 73.77 | 31.62 | — | — | 57.87 | — | — |
| Mean aspect ratio (F) | — | 3.970 | — | — | 4.563 | 4.437 | 1.954 | — | — | 2.844 | — | — |
| Area variation (F) | — | 0.923 | — | — | 1.077 | 1.342 | 0.818 | — | — | 1.089 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 203.64 | — | — | 538.64 | 539.23 | 23.21 | — | — | 530.76 | — | — |
| Mean distance, ‰ (H) | — | 36.911 | — | — | 255.822 | 252.352 | 1.584 | — | — | 235.732 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 110.1 | — | — | 712.3 | 706.9 | 7.5 | — | — | 1743.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 0.89 | 0.69 | — | — | — | 1.22 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 21.6 | 17.7 | — | — | — | 5.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 4 | 6 | — | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 78.4 | 82.3 | — | — | — | 94.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 16 | 16 | — | — | — | 21 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 63.0 | — | — | 85.9 | 83.6 | 593.3 | — | — | 94.2 | — | — |
| Time, seconds (T) | — | 1.40 | — | — | 0.60 | 1.12 | 101.96 | — | — | 3.78 | — | — |

Failures: quadriflow: cancelled.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 872 | 308 | 574 | 896 | 980 | 1000 | 1541 | 878 | 1541 | 1514 | — | — |
| Irregular vertices (S) | 44 | 30 | 40 | 34 | 34 | 38 | 152 | 88 | 152 | 42 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 307 | — | 394 | 326 | 500 | 503 | 1446 | — | 1446 | 973 | — | — |
| Alignment, degrees (A) | 19.22 | 22.42 | 18.99 | 17.91 | 18.03 | 18.11 | 18.90 | 20.03 | 18.89 | 18.60 | — | — |
| Mean angle deviation, degrees (F) | 6.74 | 26.16 | 8.62 | 10.03 | 5.77 | 6.44 | 11.26 | 10.74 | 11.26 | 5.62 | — | — |
| Angle deviation, 95th percentile (F) | 23.83 | 73.60 | 29.38 | 28.12 | 17.77 | 19.08 | 32.77 | 31.81 | 32.77 | 16.65 | — | — |
| Mean aspect ratio (F) | 1.223 | 1.911 | 1.338 | 1.361 | 1.319 | 1.306 | 1.859 | 1.521 | 1.859 | 1.296 | — | — |
| Area variation (F) | 0.195 | 0.775 | 0.224 | 0.321 | 0.353 | 0.497 | 0.545 | 0.321 | 0.545 | 0.447 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 30.58 | 234.03 | 146.80 | 30.18 | 23.57 | 25.86 | 18.41 | 31.35 | 18.41 | 31.13 | — | — |
| Mean distance, ‰ (H) | 3.158 | 9.103 | 4.386 | 2.651 | 2.302 | 1.927 | 1.506 | 2.787 | 1.506 | 1.617 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 29.7 | 54.8 | 44.9 | 29.2 | 23.5 | 18.8 | 11.3 | 27.0 | 14.3 | 19.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 5.2 | 0.0 | — | — | 1.4 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 7.8 | — | — | 0.0 | 3.2 | 0.0 | — | — | 7.7 | — | — |
| Spiralling loops (L) | 0 | 1 | — | — | 0 | 1 | 0 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 57.4 | 92.2 | — | — | 70.3 | 67.0 | 96.8 | — | — | 72.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 25 | 27 | — | — | 53 | 38 | 54 | — | — | 60 | — | — |
| Edges on loops from a border to a border, % (L) | 42.6 | 0.0 | — | — | 29.7 | 24.5 | 3.2 | — | — | 18.8 | — | — |
| Off budget, % (N) | 12.8 | 69.2 | 42.6 | 10.4 | 2.0 | 0.0 | 54.1 | 12.2 | 0.0 | 1.8 | — | — |
| Time, seconds (T) | 19.91 | 1.73 | 8.90 | 8.59 | 8.69 | 7.36 | 19.65 | 21.87 | 19.65 | 8.81 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 636 | 562 | 532 | 2037 | 1869 | 41821 | 2688 | 41821 | 41386 | — | — |
| Irregular vertices (S) | — | 343 | 124 | 45 | 485 | 476 | 3109 | 1013 | 3109 | 1380 | — | — |
| … of which above the minimum | — | — | 13 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 423 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.88 | 11.52 | 8.55 | 12.07 | 17.86 | 15.01 | 14.68 | 14.20 | 5.82 | — | — |
| Mean angle deviation, degrees (F) | — | 38.85 | 15.07 | 11.18 | 38.51 | 32.88 | 13.01 | 16.98 | 13.01 | 5.58 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.58 | 53.29 | 35.63 | 84.45 | 85.94 | 40.35 | 58.74 | 40.35 | 22.14 | — | — |
| Mean aspect ratio (F) | — | 7.492 | 2.321 | 1.724 | 155.937 | 205.386 | 1.768 | 2.072 | 1.768 | 10.281 | — | — |
| Area variation (F) | — | 1.913 | 0.516 | 0.635 | 8.209 | 7.845 | 3.743 | 1.645 | 3.743 | 0.958 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 88.93 | 69.46 | 47.45 | 325.51 | 325.54 | 13.85 | 49.37 | 13.85 | 13.40 | — | — |
| Mean distance, ‰ (H) | — | 14.421 | 10.323 | 7.114 | 40.240 | 40.314 | 1.081 | 4.277 | 1.081 | 0.635 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 65.0 | 50.2 | 35.7 | 154.1 | 154.1 | 7.4 | 22.4 | 50.7 | 34.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 0.8 | 0.9 | — | — | — | 23.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 85.3 | 86.9 | — | — | — | 23.8 | — | — |
| Spiralling loops (L) | — | — | — | — | 576 | 634 | — | — | — | 556 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 13.9 | 12.2 | — | — | — | 43.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 6 | 6 | — | — | — | 673 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 9.5 | — | — |
| Off budget, % (N) | — | 36.4 | 43.8 | 46.8 | 103.7 | 86.9 | 4082.1 | 168.8 | 0.0 | 1.0 | — | — |
| Time, seconds (T) | — | 1.37 | 5.58 | 12.95 | 7.22 | 8.35 | 16.61 | 15.33 | 16.61 | 35.05 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1110 | 204 | — | 8160 | 7930 | 205185 | 10623 | 205185 | 193999 | — | — |
| Irregular vertices (S) | — | 905 | 139 | — | 6210 | 6140 | 10625 | 4798 | 10625 | 11613 | — | — |
| … of which above the minimum | — | — | -56 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 204 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | — | 19.32 | — | — | — | 10.09 | — | 13.06 | 8.58 | — | — |
| Mean angle deviation, degrees (F) | — | 40.06 | 30.35 | — | 23.67 | 22.67 | 10.19 | 17.58 | 10.19 | 5.27 | — | — |
| Angle deviation, 95th percentile (F) | — | 88.92 | 69.83 | — | 70.99 | 69.72 | 30.61 | 52.92 | 30.61 | 20.28 | — | — |
| Mean aspect ratio (F) | — | 4.992 | 3.003 | — | 6.257 | 4.472 | 1.591 | 1.958 | 1.591 | 1.383 | — | — |
| Area variation (F) | — | 5.647 | 0.795 | — | 2.825 | 2.847 | 3.347 | 3.276 | 3.347 | 0.574 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 11.21 | 10.79 | — | 7.06 | 6.26 | 2.06 | 5.15 | 2.06 | 3.02 | — | — |
| Mean distance, ‰ (H) | — | 3.155 | 3.973 | — | 0.435 | 0.452 | 0.152 | 0.565 | 0.152 | 0.038 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 56.2 | 70.3 | — | 7.9 | 8.2 | 5.2 | 11.1 | 66.6 | 18.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | 35.3 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | 14.0 | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | 1432 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | 44.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | 327 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | 6.5 | — | — |
| Off budget, % (N) | — | 11.0 | 79.6 | — | 716.0 | 693.0 | 20418.5 | 962.3 | 0.0 | 5.5 | — | — |
| Time, seconds (T) | — | 1.28 | 3.43 | — | 17.75 | 17.92 | 14.01 | 8.85 | 14.01 | 91.81 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 8 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 3146 | 465 | 603 | 2562 | 2662 | 30430 | 4601 | 30430 | 28157 | — | — |
| Irregular vertices (S) | — | 1983 | 163 | 181 | 1767 | 1662 | 2729 | 1219 | 2729 | 1942 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 442 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 17.00 | 15.03 | 10.78 | 4.70 | 4.94 | 5.04 | 8.21 | 5.05 | 4.12 | — | — |
| Mean angle deviation, degrees (F) | — | 45.13 | 23.16 | 17.86 | 21.40 | 21.77 | 11.44 | 15.59 | 11.44 | 6.43 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.51 | 66.13 | 54.34 | 78.34 | 77.58 | 42.16 | 46.05 | 42.16 | 27.96 | — | — |
| Mean aspect ratio (F) | — | 8.830 | 3.120 | 2.286 | 179.581 | 198.798 | 2.473 | 3.146 | 2.473 | 17.763 | — | — |
| Area variation (F) | — | 5.401 | 0.584 | 0.550 | 6.287 | 5.709 | 2.700 | 2.263 | 2.700 | 1.063 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 128.15 | 144.73 | 94.72 | 112.60 | 107.04 | 10.84 | 83.07 | 10.84 | 13.12 | — | — |
| Mean distance, ‰ (H) | — | 13.795 | 32.571 | 17.454 | 5.348 | 4.618 | 0.478 | 4.559 | 0.478 | 0.351 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.9 | 129.3 | 70.7 | 16.5 | 14.1 | 2.5 | 18.5 | 13.8 | 7.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 5.4 | 6.7 | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 34.2 | 31.6 | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | 1366 | 1288 | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 52.4 | 53.7 | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 14 | 25 | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 8.0 | 8.0 | — | — | — | — | — | — |
| Off budget, % (N) | — | 214.6 | 53.5 | 39.7 | 156.2 | 166.2 | 2943.0 | 360.1 | 0.0 | 7.5 | — | — |
| Time, seconds (T) | — | 1.26 | 4.02 | 56.49 | 10.35 | 13.06 | 11.27 | 10.16 | 11.27 | 88.71 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 988 | 508 | 843 | 1462 | 1670 | 9385 | 3134 | 9385 | 8871 | — | — |
| Irregular vertices (S) | — | 272 | 126 | 82 | 136 | 182 | 368 | 289 | 368 | 314 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 428 | — | — | 484 | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.41 | 9.38 | 8.25 | 4.95 | 6.02 | 4.16 | 4.27 | 3.37 | 3.35 | — | — |
| Mean angle deviation, degrees (F) | — | 25.43 | 18.32 | 9.97 | 19.83 | 20.85 | 7.93 | 11.16 | 7.93 | 9.35 | — | — |
| Angle deviation, 95th percentile (F) | — | 86.02 | 55.68 | 36.03 | 70.71 | 71.98 | 28.21 | 42.77 | 28.21 | 45.02 | — | — |
| Mean aspect ratio (F) | — | 3.381 | 2.053 | 1.774 | 39.958 | 58.806 | 10.123 | 26.996 | 10.123 | 9.426 | — | — |
| Area variation (F) | — | 1.855 | 0.710 | 0.410 | 2.039 | 1.978 | 2.280 | 1.420 | 2.280 | 0.787 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 72.47 | 81.73 | 35.86 | 52.85 | 39.36 | 21.81 | 36.33 | 21.81 | 19.79 | — | — |
| Mean distance, ‰ (H) | — | 8.475 | 10.964 | 4.688 | 6.157 | 5.818 | 1.030 | 2.230 | 1.030 | 0.941 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 17.9 | 20.7 | 10.1 | 13.6 | 13.0 | 2.4 | 5.4 | 7.1 | 11.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 63.3 | 54.1 | 33.6 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 15.7 | 17.2 | 5.0 | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | 64 | 86 | 19 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 17.0 | 25.6 | 38.1 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 14 | 19 | 76 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 4.0 | 3.0 | 23.4 | — | — | — | — | — |
| Off budget, % (N) | — | 1.2 | 49.2 | 15.7 | 46.2 | 67.0 | 838.5 | 213.4 | 0.0 | 5.5 | — | — |
| Time, seconds (T) | — | 0.39 | 0.51 | 21.84 | 2.64 | 2.66 | 5.98 | 5.10 | 5.98 | 11.27 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 180 | 866 | 744 | 759 | 752 | 7098 | 803 | 7098 | 5955 | — | — |
| Irregular vertices (S) | — | 56 | 181 | 74 | 126 | 126 | 725 | 192 | 725 | 418 | — | — |
| … of which above the minimum | — | — | 172 | — | 62 | 62 | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 810 | — | 465 | 458 | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.42 | 14.10 | 10.16 | 9.99 | 10.11 | 9.10 | 10.26 | 10.66 | 10.58 | — | — |
| Mean angle deviation, degrees (F) | — | 24.14 | 14.78 | 12.69 | 13.62 | 13.11 | 9.97 | 13.02 | 9.97 | 9.51 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.70 | 45.64 | 33.98 | 39.94 | 36.87 | 29.98 | 40.40 | 29.98 | 28.14 | — | — |
| Mean aspect ratio (F) | — | 3.965 | 1.759 | 1.600 | 1.995 | 1.981 | 1.773 | 1.662 | 1.773 | 1.475 | — | — |
| Area variation (F) | — | 0.945 | 0.411 | 0.449 | 0.494 | 0.485 | 1.130 | 0.547 | 1.130 | 0.342 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 123.45 | 52.06 | 48.50 | 40.53 | 40.53 | 28.38 | 45.02 | 28.38 | 20.49 | — | — |
| Mean distance, ‰ (H) | — | 18.850 | 12.293 | 6.257 | 5.461 | 5.421 | 1.253 | 6.755 | 1.253 | 0.843 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 123.8 | 121.8 | 59.0 | 50.6 | 50.0 | 17.0 | 61.2 | 50.3 | 39.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 7.5 | — | — | 19.4 | 19.5 | 16.7 | — | — | 20.3 | — | — |
| Edges on spiralling loops, % (L) | — | 16.4 | — | — | 9.5 | 10.4 | 8.8 | — | — | 16.6 | — | — |
| Spiralling loops (L) | — | 14 | — | — | 40 | 52 | 55 | — | — | 105 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.1 | — | — | 71.1 | 70.1 | 74.4 | — | — | 63.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 11 | — | — | 35 | 35 | 59 | — | — | 137 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | 0.1 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 82.0 | 13.4 | 25.6 | 24.1 | 24.8 | 609.8 | 19.7 | 0.0 | 16.1 | — | — |
| Time, seconds (T) | — | 0.58 | 0.19 | 5.77 | 6.53 | 5.51 | 27.33 | 36.82 | 27.33 | 70.78 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 950 | 768 | 1066 | 923 | 1010 | 1010 | 915 | 915 | 915 | 900 | — | — |
| Irregular vertices (S) | 8 | 8 | 44 | 8 | 8 | 8 | 10 | 10 | 10 | 10 | — | — |
| … of which above the minimum | 0 | 0 | 36 | 0 | 0 | 0 | 2 | 2 | 2 | 2 | — | — |
| Base complex patches (C) | 937 | 768 | 1066 | 923 | 1010 | 1010 | 699 | 699 | 699 | 846 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.13 | 5.18 | 8.36 | 4.61 | 4.61 | 6.03 | 6.03 | 6.03 | 4.08 | — | — |
| Angle deviation, 95th percentile (F) | 12.31 | 13.45 | 17.01 | 22.46 | 11.13 | 11.13 | 14.57 | 14.57 | 14.57 | 10.24 | — | — |
| Mean aspect ratio (F) | 1.148 | 1.336 | 1.153 | 1.171 | 1.180 | 1.180 | 1.213 | 1.213 | 1.213 | 1.153 | — | — |
| Area variation (F) | 0.136 | 0.229 | 0.179 | 0.152 | 0.225 | 0.225 | 0.318 | 0.318 | 0.318 | 0.240 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 3.06 | 1.68 | 1.11 | 0.99 | 0.99 | 1.91 | 1.91 | 1.91 | 2.07 | — | — |
| Mean distance, ‰ (H) | 0.670 | 1.508 | 0.632 | 0.356 | 0.332 | 0.332 | 0.762 | 0.762 | 0.762 | 0.357 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.38 | 98.48 | 99.38 | 100.02 | 100.02 | 100.02 | 99.24 | 99.24 | 99.24 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 12.0 | — | — | 3.0 | — | — |
| Edges on spiralling loops, % (L) | 69.5 | 76.7 | — | — | 86.8 | 86.8 | 64.4 | — | — | 65.3 | — | — |
| Spiralling loops (L) | 4 | 4 | — | — | 5 | 5 | 5 | — | — | 7 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 25.1 | 23.3 | — | — | 13.2 | 13.2 | 23.6 | — | — | 31.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 126 | — | — | 88 | 88 | 110 | — | — | 135 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.0 | 23.2 | 6.6 | 7.7 | 1.0 | 1.0 | 8.5 | 8.5 | 0.0 | 1.6 | — | — |
| Time, seconds (T) | 0.45 | 0.18 | 0.19 | 1.75 | 1.12 | 1.18 | 4.77 | 3.75 | 4.77 | 1.95 | — | — |

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
| Area variation (F) | 0.180 | 0.121 | 0.237 | 0.294 | 0.350 | 0.351 | 0.280 | 0.280 | 0.280 | 0.356 | — | — |
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
| Time, seconds (T) | 0.33 | 0.44 | 0.27 | 1.02 | 0.61 | 0.88 | 4.41 | 2.61 | 4.41 | 0.93 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 1 | 9 | 0 | 5 | 2 | 1 | 0 | 0 | 0 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 4 | 0 | 5 | 3 | 2 | 0 | 1 | 0 | 0 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 0 | 13 | 0 | 1 | 1 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 0 | 0 | 9 | 0 | 2 |
| Mean aspect ratio (F) | 4 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 6 | 0 | 2 |
| Valid meshes (V) | 38/75 | 4/15 | 45/75 | 9/15 | 14/15 | 14/15 | 11/15 | 8/15 | 11/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 1 | 0 | 4 | 4 | 4 | 3 | 4 | 3 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 0 | 4 | 2 | 2 | 2 | 2 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 2 | 2 | 0 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 2 | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 1 | 0 | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 0 | 0 | 0 |
| Valid meshes (V) | 20/20 | 2/4 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
