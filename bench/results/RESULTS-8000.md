# Bench results

Requested budget: 8000 quads. Median over 5 seeds; AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7590 | 2795 | 8164 | 7124 | 8020 | 7934 | — | — |
| Irregular vertices (S) | 40 | 28 | 277 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 32 | 20 | 269 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 7412 | 1171 | 8164 | 5660 | 7590 | 2376 | — | — |
| Alignment, degrees (A) | 14.77 | 5.92 | 15.05 | 5.88 | 6.05 | 5.52 | — | — |
| Mean angle deviation, degrees (F) | 7.77 | 9.19 | 4.85 | 7.41 | 6.81 | 6.21 | — | — |
| Angle deviation, 95th percentile (F) | 19.95 | 34.45 | 15.77 | 19.73 | 18.56 | 16.44 | — | — |
| Mean aspect ratio (F) | 1.152 | 2.101 | 1.142 | 1.317 | 1.406 | 1.357 | — | — |
| Area variation (F) | 0.177 | 0.585 | 0.153 | 0.316 | 0.379 | 0.415 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.18 | 2.78 | 4.97 | 2.41 | 1.64 | 1.78 | — | — |
| Mean distance, ‰ (H) | 0.171 | 0.760 | 0.178 | 0.097 | 0.082 | 0.078 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.84 | 98.72 | 99.86 | 100.01 | 100.01 | 100.01 | — | — |
| Edges on closed loops, % (L) | 1.2 | 33.0 | — | — | 2.7 | 36.7 | — | — |
| Edges on spiralling loops, % (L) | 35.9 | 18.0 | — | — | 52.9 | 16.1 | — | — |
| Spiralling loops (L) | 12 | 4 | — | — | 12 | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 62.6 | 49.1 | — | — | 44.4 | 47.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 766 | 292 | — | — | 946 | 1624 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 5.1 | 65.1 | 2.1 | 10.9 | 0.2 | 0.8 | — | — |
| Time, seconds (T) | 4.32 | 2.57 | 1.85 | 10.94 | 7.07 | 7.86 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7342 | 5842 | 7792 | 7393 | 8037 | 8092 | — | — |
| Irregular vertices (S) | 10 | 34 | 162 | 114 | 116 | 132 | — | — |
| … of which above the minimum | 2 | 26 | 154 | 106 | 108 | 124 | — | — |
| Base complex patches (C) | 7342 | 5052 | 7792 | 7393 | 8037 | 8092 | — | — |
| Alignment, degrees (A) | 22.36 | 18.66 | 22.82 | 16.04 | 16.51 | 16.43 | — | — |
| Mean angle deviation, degrees (F) | 6.22 | 6.90 | 4.13 | 9.24 | 5.94 | 6.28 | — | — |
| Angle deviation, 95th percentile (F) | 18.32 | 17.38 | 12.43 | 22.12 | 14.46 | 15.57 | — | — |
| Mean aspect ratio (F) | 1.158 | 1.393 | 1.115 | 1.246 | 1.254 | 1.266 | — | — |
| Area variation (F) | 0.244 | 0.344 | 0.128 | 0.204 | 0.255 | 0.273 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.95 | 3.88 | 3.22 | 3.64 | 2.72 | 2.73 | — | — |
| Mean distance, ‰ (H) | 0.478 | 0.669 | 0.433 | 0.418 | 0.391 | 0.391 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.79 | 99.89 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 6.9 | — | — | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 70.9 | 30.4 | — | — | 1.6 | 1.4 | — | — |
| Spiralling loops (L) | 6 | 7 | — | — | 1 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 29.1 | 62.6 | — | — | 98.4 | 98.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1193 | 618 | — | — | 379 | 324 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 8.2 | 27.0 | 2.6 | 7.6 | 0.5 | 1.1 | — | — |
| Time, seconds (T) | 4.18 | 3.32 | 1.45 | 33.90 | 7.38 | 9.00 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8170 | 6819 | 8234 | 6922 | 7858 | 7842 | — | — |
| Irregular vertices (S) | 22 | 18 | 214 | 40 | 40 | 42 | — | — |
| … of which above the minimum | 14 | 10 | 206 | — | 32 | 34 | — | — |
| Base complex patches (C) | 8170 | 3683 | 7510 | 2490 | 2072 | 2114 | — | — |
| Alignment, degrees (A) | 13.70 | 9.77 | 11.81 | 6.40 | 6.44 | 5.59 | — | — |
| Mean angle deviation, degrees (F) | 6.02 | 3.78 | 3.91 | 8.75 | 7.01 | 5.79 | — | — |
| Angle deviation, 95th percentile (F) | 15.98 | 10.36 | 12.20 | 25.50 | 18.09 | 15.56 | — | — |
| Mean aspect ratio (F) | 1.154 | 1.965 | 1.116 | 1.352 | 1.359 | 1.320 | — | — |
| Area variation (F) | 0.157 | 0.565 | 0.127 | 0.334 | 0.388 | 0.494 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.19 | 1.09 | 1.70 | 2.35 | 1.20 | 0.89 | — | — |
| Mean distance, ‰ (H) | 0.100 | 0.161 | 0.106 | 0.099 | 0.087 | 0.084 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.74 | 99.81 | — | 100.01 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 23.0 | — | — | 39.5 | 37.3 | — | — |
| Edges on spiralling loops, % (L) | 44.8 | 43.6 | — | — | 14.2 | 11.9 | — | — |
| Spiralling loops (L) | 5 | 9 | — | — | 5 | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 55.2 | 33.4 | — | — | 46.4 | 50.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1471 | 594 | — | — | 306 | 571 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.1 | 14.8 | 2.9 | 13.5 | 1.8 | 2.0 | — | — |
| Time, seconds (T) | 5.64 | 3.36 | 1.55 | 24.32 | 15.17 | 14.12 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7932 | 5928 | 8050 | 7765 | 8124 | 8013 | — | — |
| Irregular vertices (S) | 72 | 58 | 250 | 40 | 48 | 58 | — | — |
| … of which above the minimum | 40 | — | 218 | 8 | 16 | 26 | — | — |
| Base complex patches (C) | 7283 | — | 7125 | 766 | 405 | 191 | — | — |
| Alignment, degrees (A) | 8.22 | 5.16 | 8.80 | 2.71 | 2.58 | 1.95 | — | — |
| Mean angle deviation, degrees (F) | 5.83 | 7.40 | 4.40 | 3.58 | 2.05 | 2.24 | — | — |
| Angle deviation, 95th percentile (F) | 16.15 | 22.71 | 14.23 | 10.77 | 6.73 | 6.75 | — | — |
| Mean aspect ratio (F) | 1.140 | 2.161 | 1.144 | 1.200 | 1.198 | 1.219 | — | — |
| Area variation (F) | 0.139 | 0.487 | 0.138 | 0.191 | 0.199 | 0.348 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.65 | 7.28 | 9.87 | 4.82 | 3.04 | 4.45 | — | — |
| Mean distance, ‰ (H) | 0.294 | 0.343 | 0.344 | 0.275 | 0.228 | 0.217 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.64 | 99.99 | 99.48 | 99.98 | 100.00 | 99.99 | — | — |
| Edges on closed loops, % (L) | 5.0 | 25.6 | — | — | 80.0 | 84.6 | — | — |
| Edges on spiralling loops, % (L) | 55.9 | 48.4 | — | — | 6.9 | 0.0 | — | — |
| Spiralling loops (L) | 33 | 29 | — | — | 2 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 40.8 | 26.0 | — | — | 13.1 | 15.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 332 | 108 | — | — | 70 | 110 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.9 | 25.9 | 0.6 | 2.9 | 1.6 | 0.2 | — | — |
| Time, seconds (T) | 5.53 | 3.23 | 0.57 | 25.95 | 5.26 | 8.63 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 2647 | 8098 | 7376 | 7884 | 7889 | — | — |
| Irregular vertices (S) | 55 | 84 | 346 | 67 | 59 | 74 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6194 | — | 7878 | 4267 | 3574 | 2878 | — | — |
| Alignment, degrees (A) | 14.27 | 12.11 | 16.64 | 10.01 | 10.26 | 9.88 | — | — |
| Mean angle deviation, degrees (F) | 6.09 | 9.37 | 5.17 | 7.60 | 4.83 | 5.59 | — | — |
| Angle deviation, 95th percentile (F) | 18.06 | 32.08 | 17.78 | 21.30 | 13.25 | 14.28 | — | — |
| Mean aspect ratio (F) | 1.208 | 1.892 | 1.169 | 1.223 | 1.187 | 1.224 | — | — |
| Area variation (F) | 0.208 | 0.650 | 0.164 | 0.218 | 0.234 | 0.364 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.87 | 31.86 | 5.94 | 4.86 | 5.95 | 3.71 | — | — |
| Mean distance, ‰ (H) | 0.267 | 1.112 | 0.286 | 0.272 | 0.225 | 0.187 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.4 | 9.5 | — | — | 17.3 | 25.6 | — | — |
| Edges on spiralling loops, % (L) | 11.3 | 20.3 | — | — | 12.6 | 13.2 | — | — |
| Spiralling loops (L) | 6 | 12 | — | — | 6 | 9 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 68.7 | 70.2 | — | — | 54.6 | 46.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 512 | 98 | — | — | 398 | 307 | — | — |
| Edges on loops from a border to a border, % (L) | 19.0 | 0.0 | — | — | 15.5 | 14.3 | — | — |
| Off budget, % (N) | 0.9 | 66.9 | 1.2 | 7.8 | 1.5 | 1.4 | — | — |
| Time, seconds (T) | 6.83 | 1.17 | 0.87 | 74.82 | 6.78 | 8.48 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7763 | 4622 | 7836 | 6967 | 8010 | 7979 | — | — |
| Irregular vertices (S) | 162 | 141 | 455 | 170 | 200 | 238 | — | — |
| … of which above the minimum | 154 | — | 429 | — | 192 | 230 | — | — |
| Base complex patches (C) | 7352 | — | 7656 | — | 5322 | 5407 | — | — |
| Alignment, degrees (A) | 17.78 | 12.94 | 17.72 | 11.64 | 11.53 | 10.67 | — | — |
| Mean angle deviation, degrees (F) | 11.59 | 7.43 | 6.18 | 9.04 | 6.88 | 6.72 | — | — |
| Angle deviation, 95th percentile (F) | 33.33 | 22.75 | 20.84 | 24.13 | 18.51 | 17.91 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.812 | 1.225 | 1.401 | 1.394 | 1.403 | — | — |
| Area variation (F) | 0.312 | 0.668 | 0.196 | 0.284 | 0.343 | 0.482 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 27.28 | 27.93 | 30.74 | 26.72 | 11.26 | 8.25 | — | — |
| Mean distance, ‰ (H) | 0.521 | 1.054 | 0.601 | 0.408 | 0.314 | 0.301 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.1 | 78.5 | 35.7 | 24.3 | 12.6 | 4.7 | — | — |
| Volume kept, % of the source (V) | 99.49 | 98.66 | 99.41 | 99.93 | 99.99 | 100.00 | — | — |
| Edges on closed loops, % (L) | 2.0 | 17.3 | — | — | 17.8 | 16.1 | — | — |
| Edges on spiralling loops, % (L) | 13.7 | 18.1 | — | — | 5.2 | 9.2 | — | — |
| Spiralling loops (L) | 23 | 23 | — | — | 14 | 15 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 84.3 | 64.7 | — | — | 77.0 | 74.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 484 | 175 | — | — | 275 | 261 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.0 | 42.2 | 2.1 | 12.9 | 0.1 | 0.3 | — | — |
| Time, seconds (T) | 4.83 | 3.89 | 2.51 | 34.81 | 16.62 | 21.11 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2520 | 7652 | 7214 | 8135 | 8083 | 10590 | 9426 |
| Irregular vertices (S) | — | 156 | 400 | 128 | 156 | 231 | 476 | 141 |
| … of which above the minimum | — | — | — | — | 148 | 223 | — | 133 |
| Base complex patches (C) | — | — | 5799 | — | 2961 | 2891 | — | 5382 |
| Alignment, degrees (A) | — | 10.84 | 11.76 | 7.87 | 8.30 | 8.02 | 9.88 | 8.45 |
| Mean angle deviation, degrees (F) | — | 8.90 | 6.09 | 5.84 | 3.90 | 4.37 | 9.24 | 6.33 |
| Angle deviation, 95th percentile (F) | — | 29.26 | 22.28 | 18.18 | 11.69 | 14.32 | 26.27 | 18.39 |
| Mean aspect ratio (F) | — | 1.933 | 1.235 | 1.342 | 1.329 | 1.320 | 1.616 | 1.333 |
| Area variation (F) | — | 0.738 | 0.213 | 0.305 | 0.340 | 0.492 | 1.417 | 0.294 |
| Hausdorff, ‰ of the diagonal (H) | — | 23.89 | 35.02 | 19.08 | 9.03 | 6.00 | 6.47 | 8.89 |
| Mean distance, ‰ (H) | — | 1.795 | 2.033 | 1.059 | 0.469 | 0.234 | 0.172 | 0.557 |
| Thin parts: mean distance, % of a quad edge (V) | — | 36.5 | 57.2 | 35.9 | 16.6 | 5.8 | 1.5 | 25.0 |
| Volume kept, % of the source (V) | — | — | — | 99.88 | 99.99 | 100.02 | 98.80 | 99.93 |
| Edges on closed loops, % (L) | — | — | 12.1 | 32.9 | 32.7 | 32.1 | 31.6 | 22.1 |
| Edges on spiralling loops, % (L) | — | — | 7.3 | 20.4 | 10.9 | 10.0 | 2.5 | 32.6 |
| Spiralling loops (L) | — | — | 37 | 44 | 38 | 10 | 3 | 44 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 79.3 | 46.6 | 56.4 | 57.9 | 65.8 | 45.2 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 180 | 312 | 366 | 345 | 274 | 467 |
| Edges on loops from a border to a border, % (L) | — | — | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.5 | 4.3 | 9.8 | 1.7 | 1.0 | 0.0 | 11.0 |
| Time, seconds (T) | — | 5.49 | 20.44 | 42.48 | 23.77 | 27.20 | — | 70.96 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2564 | 7648 | 7201 | 7870 | 7966 | 10590 | 10602 |
| Irregular vertices (S) | — | 150 | 414 | 140 | 170 | 259 | 476 | 280 |
| … of which above the minimum | — | — | 395 | — | 162 | 251 | — | 272 |
| Base complex patches (C) | — | — | 6412 | — | 2797 | 2794 | — | 3904 |
| Alignment, degrees (A) | — | 9.84 | 11.62 | 7.96 | 7.67 | 7.15 | 9.43 | 7.20 |
| Mean angle deviation, degrees (F) | — | 8.28 | 6.50 | 6.48 | 4.48 | 4.53 | 8.82 | 4.44 |
| Angle deviation, 95th percentile (F) | — | 26.20 | 22.21 | 21.70 | 14.30 | 14.40 | 25.47 | 14.29 |
| Mean aspect ratio (F) | — | 1.962 | 1.237 | 1.317 | 1.417 | 1.311 | 1.657 | 1.281 |
| Area variation (F) | — | 0.794 | 0.217 | 0.330 | 0.414 | 0.515 | 1.432 | 0.482 |
| Hausdorff, ‰ of the diagonal (H) | — | 26.27 | 32.73 | 14.77 | 8.59 | 6.18 | 4.44 | 4.92 |
| Mean distance, ‰ (H) | — | 2.147 | 2.076 | 1.073 | 0.611 | 0.282 | 0.156 | 0.181 |
| Thin parts: mean distance, % of a quad edge (V) | — | 50.9 | 73.5 | 41.9 | 22.6 | 7.9 | 1.4 | 6.1 |
| Volume kept, % of the source (V) | — | — | 98.91 | 99.90 | 99.97 | 100.01 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | — | 8.3 | 23.0 | 34.0 | 32.5 | 31.6 | 31.6 |
| Edges on spiralling loops, % (L) | — | — | 10.6 | 27.5 | 6.3 | 2.8 | 2.5 | 3.5 |
| Spiralling loops (L) | — | — | 49 | 46 | 33 | 14 | 3 | 8 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 80.4 | 49.5 | 59.7 | 64.7 | 65.8 | 64.9 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 166 | 402 | 375 | 331 | 274 | 325 |
| Edges on loops from a border to a border, % (L) | — | — | 0.3 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.0 | 4.4 | 10.0 | 1.6 | 0.4 | 0.0 | 0.1 |
| Time, seconds (T) | — | 6.63 | 20.66 | 36.31 | 24.11 | 26.02 | — | 33.55 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7874 | 2895 | 8098 | 7084 | 8050 | 8003 | — | — |
| Irregular vertices (S) | 76 | 74 | 286 | 80 | 78 | 78 | — | — |
| … of which above the minimum | 68 | — | 278 | 72 | 70 | 70 | — | — |
| Base complex patches (C) | 7772 | — | 7942 | 3692 | 2848 | 2609 | — | — |
| Alignment, degrees (A) | 16.85 | 9.88 | 16.86 | 8.49 | 8.29 | 7.68 | — | — |
| Mean angle deviation, degrees (F) | 6.23 | 7.56 | 4.47 | 9.07 | 7.25 | 6.37 | — | — |
| Angle deviation, 95th percentile (F) | 16.10 | 20.17 | 14.93 | 23.35 | 19.34 | 17.20 | — | — |
| Mean aspect ratio (F) | 1.177 | 1.481 | 1.136 | 1.348 | 1.398 | 1.373 | — | — |
| Area variation (F) | 0.178 | 0.712 | 0.144 | 0.296 | 0.356 | 0.473 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.30 | 6.25 | 7.10 | 2.61 | 3.51 | 2.31 | — | — |
| Mean distance, ‰ (H) | 0.161 | 0.635 | 0.172 | 0.093 | 0.085 | 0.081 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 98.89 | 99.78 | 100.01 | 100.00 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.1 | 16.4 | — | — | 35.6 | 37.0 | — | — |
| Edges on spiralling loops, % (L) | 19.0 | 13.5 | — | — | 11.3 | 2.9 | — | — |
| Spiralling loops (L) | 9 | 7 | — | — | 5 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.3 | 70.1 | — | — | 53.1 | 60.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 578 | 134 | — | — | 331 | 519 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.6 | 63.8 | 1.2 | 11.5 | 0.6 | 0.0 | — | — |
| Time, seconds (T) | 3.72 | 2.50 | 1.23 | 39.51 | 10.06 | 11.10 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8263 | 4012 | 8072 | 7052 | 8111 | 8034 | — | — |
| Irregular vertices (S) | 36 | 20 | 245 | 18 | 20 | 20 | — | — |
| … of which above the minimum | 36 | 20 | 245 | — | 20 | 20 | — | — |
| Base complex patches (C) | 8263 | 1318 | 8072 | 1541 | 1429 | 255 | — | — |
| Alignment, degrees (A) | 17.31 | 5.28 | 20.15 | 4.10 | 3.65 | 3.65 | — | — |
| Mean angle deviation, degrees (F) | 6.39 | 4.91 | 4.31 | 4.94 | 3.52 | 3.63 | — | — |
| Angle deviation, 95th percentile (F) | 17.71 | 13.45 | 14.25 | 16.54 | 10.71 | 11.45 | — | — |
| Mean aspect ratio (F) | 1.142 | 2.024 | 1.124 | 1.407 | 1.377 | 1.368 | — | — |
| Area variation (F) | 0.159 | 0.485 | 0.139 | 0.356 | 0.367 | 0.418 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.23 | 1.62 | 2.84 | 3.52 | 0.64 | 0.84 | — | — |
| Mean distance, ‰ (H) | 0.104 | 0.346 | 0.120 | 0.063 | 0.055 | 0.055 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.32 | 99.79 | — | 100.01 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 35.6 | — | — | 42.7 | 76.9 | — | — |
| Edges on spiralling loops, % (L) | 23.4 | 40.2 | — | — | 40.2 | 6.6 | — | — |
| Spiralling loops (L) | 7 | 10 | — | — | 4 | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 76.6 | 24.2 | — | — | 17.1 | 16.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 840 | 228 | — | — | 212 | 204 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 49.9 | 0.9 | 11.8 | 1.4 | 0.4 | — | — |
| Time, seconds (T) | 3.56 | 2.33 | 1.37 | 16.12 | 9.21 | 10.97 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7229 | 3438 | 8100 | 7057 | 8160 | 7884 | — | — |
| Irregular vertices (S) | 70 | 53 | 280 | 48 | 48 | 56 | — | — |
| … of which above the minimum | 62 | — | 272 | 40 | 40 | 48 | — | — |
| Base complex patches (C) | 6275 | — | 7910 | 2997 | 648 | 1974 | — | — |
| Alignment, degrees (A) | 15.23 | 7.01 | 14.61 | 7.16 | 6.91 | 6.77 | — | — |
| Mean angle deviation, degrees (F) | 10.30 | 6.30 | 4.50 | 8.89 | 6.00 | 5.87 | — | — |
| Angle deviation, 95th percentile (F) | 28.50 | 19.86 | 15.34 | 24.31 | 17.64 | 17.33 | — | — |
| Mean aspect ratio (F) | 1.267 | 2.320 | 1.139 | 1.332 | 1.322 | 1.337 | — | — |
| Area variation (F) | 0.371 | 0.598 | 0.144 | 0.270 | 0.308 | 0.388 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 21.90 | 8.37 | 5.99 | 4.33 | 2.24 | 6.48 | — | — |
| Mean distance, ‰ (H) | 0.213 | 0.399 | 0.132 | 0.085 | 0.067 | 0.071 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.18 | 99.71 | 99.99 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 6.4 | 28.0 | — | — | 68.7 | 43.6 | — | — |
| Edges on spiralling loops, % (L) | 20.2 | 18.6 | — | — | 0.0 | 22.9 | — | — |
| Spiralling loops (L) | 12 | 5 | — | — | 0 | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 73.2 | 53.3 | — | — | 31.3 | 33.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 660 | 163 | — | — | 294 | 309 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 9.6 | 57.0 | 1.2 | 11.8 | 2.0 | 1.5 | — | — |
| Time, seconds (T) | 3.58 | 3.90 | 1.16 | 19.91 | 9.22 | 9.18 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6973 | 4682 | 8116 | 7187 | 8061 | 8105 | — | — |
| Irregular vertices (S) | 136 | 135 | 370 | 81 | 92 | 100 | — | — |
| … of which above the minimum | 128 | — | 362 | — | 84 | 92 | — | — |
| Base complex patches (C) | 6841 | — | 7826 | 4517 | 4039 | 3841 | — | — |
| Alignment, degrees (A) | 15.84 | 11.22 | 14.90 | 9.35 | 9.27 | 8.25 | — | — |
| Mean angle deviation, degrees (F) | 11.44 | 6.80 | 5.04 | 8.83 | 6.54 | 5.89 | — | — |
| Angle deviation, 95th percentile (F) | 34.02 | 21.80 | 17.35 | 25.06 | 18.37 | 17.89 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.773 | 1.161 | 1.356 | 1.315 | 1.306 | — | — |
| Area variation (F) | 0.383 | 0.668 | 0.161 | 0.307 | 0.311 | 0.488 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.23 | 9.84 | 21.49 | 7.72 | 7.34 | 5.23 | — | — |
| Mean distance, ‰ (H) | 0.610 | 0.627 | 0.238 | 0.164 | 0.143 | 0.124 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 64.0 | 21.7 | 11.6 | 16.5 | 15.8 | 4.4 | — | — |
| Volume kept, % of the source (V) | 99.23 | 99.18 | 99.70 | — | 99.99 | 99.99 | — | — |
| Edges on closed loops, % (L) | 0.5 | 9.6 | — | — | 27.9 | 29.8 | — | — |
| Edges on spiralling loops, % (L) | 21.6 | 22.2 | — | — | 15.6 | 13.2 | — | — |
| Spiralling loops (L) | 22 | 18 | — | — | 15 | 17 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.2 | 68.1 | — | — | 56.5 | 57.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 500 | 161 | — | — | 567 | 342 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 12.8 | 41.5 | 1.5 | 10.2 | 0.8 | 1.3 | — | — |
| Time, seconds (T) | 3.13 | 4.60 | 1.42 | 34.04 | 12.89 | 15.22 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7781 | 2740 | 7933 | 7345 | 7970 | 8098 | — | — |
| Irregular vertices (S) | 112 | 109 | 490 | 173 | 150 | 295 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 4991 | — | 7797 | 5208 | 5451 | 7065 | — | — |
| Alignment, degrees (A) | 17.56 | 15.73 | 18.65 | 13.30 | 13.33 | 12.61 | — | — |
| Mean angle deviation, degrees (F) | 7.80 | 10.26 | 6.63 | 8.59 | 5.01 | 5.88 | — | — |
| Angle deviation, 95th percentile (F) | 22.58 | 29.98 | 23.27 | 23.17 | 14.14 | 16.46 | — | — |
| Mean aspect ratio (F) | 1.257 | 1.889 | 1.241 | 1.297 | 1.255 | 1.278 | — | — |
| Area variation (F) | 0.245 | 0.652 | 0.210 | 0.264 | 0.280 | 0.460 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 31.29 | 25.08 | 29.67 | 21.23 | 15.00 | 12.13 | — | — |
| Mean distance, ‰ (H) | 0.581 | 2.010 | 0.629 | 0.483 | 0.369 | 0.298 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.5 | 54.4 | 38.5 | 34.4 | 22.9 | 9.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 4.4 | 2.5 | — | — |
| Edges on spiralling loops, % (L) | 3.1 | — | — | — | 2.0 | 1.8 | — | — |
| Spiralling loops (L) | 5 | — | — | — | 3 | 7 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.6 | — | — | — | 81.0 | 91.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 404 | — | — | — | 231 | 204 | — | — |
| Edges on loops from a border to a border, % (L) | 19.8 | — | — | — | 12.6 | 4.1 | — | — |
| Off budget, % (N) | 2.7 | 65.8 | 0.8 | 8.2 | 0.4 | 1.2 | — | — |
| Time, seconds (T) | 6.02 | 2.44 | 0.92 | 36.84 | 8.54 | 12.38 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6758 | 3828 | 6932 | 6584 | 8226 | 8422 | — | — |
| Irregular vertices (S) | 917 | 556 | 1126 | 570 | 1222 | 1648 | — | — |
| … of which above the minimum | — | — | — | — | — | 1032 | — | — |
| Base complex patches (C) | 6524 | — | 6434 | — | — | 6194 | — | — |
| Alignment, degrees (A) | 18.21 | 18.28 | 18.49 | 14.12 | 13.00 | 12.54 | — | — |
| Mean angle deviation, degrees (F) | 14.81 | 17.84 | 12.06 | 10.22 | 12.22 | 13.06 | — | — |
| Angle deviation, 95th percentile (F) | 41.35 | 56.33 | 40.69 | 29.42 | 41.37 | 39.58 | — | — |
| Mean aspect ratio (F) | 1.515 | 3.415 | 1.595 | 1.412 | 2.504 | 28.278 | — | — |
| Area variation (F) | 0.407 | 0.904 | 0.359 | 0.322 | 0.693 | 1.438 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 74.30 | 58.87 | 72.22 | 73.55 | 16.49 | 14.63 | — | — |
| Mean distance, ‰ (H) | 1.489 | 4.497 | 1.750 | 1.267 | 0.666 | 0.879 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 44.1 | 83.6 | 47.5 | 37.1 | 13.2 | 12.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 97.41 | 96.02 | — | — |
| Edges on closed loops, % (L) | 0.4 | — | — | — | 13.9 | 13.4 | — | — |
| Edges on spiralling loops, % (L) | 17.0 | — | — | — | 18.0 | 16.5 | — | — |
| Spiralling loops (L) | 130 | — | — | — | 393 | 422 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.4 | — | — | — | 68.1 | 70.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | — | — | 106 | 64 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 15.5 | 52.1 | 13.4 | 17.7 | 2.8 | 5.3 | — | — |
| Time, seconds (T) | 3.62 | 1.88 | 1.79 | 135.60 | 7.87 | 22.31 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4954 | 7960 | 7468 | 8041 | 7922 | — | — |
| Irregular vertices (S) | — | 58 | 258 | 68 | 54 | 62 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 5201 | 1643 | 1606 | 1844 | — | — |
| Alignment, degrees (A) | — | 10.49 | 13.88 | 9.84 | 10.07 | 9.40 | — | — |
| Mean angle deviation, degrees (F) | — | 6.02 | 4.48 | 7.01 | 5.23 | 5.03 | — | — |
| Angle deviation, 95th percentile (F) | — | 18.67 | 14.92 | 19.83 | 13.90 | 14.50 | — | — |
| Mean aspect ratio (F) | — | 1.719 | 1.145 | 1.266 | 1.278 | 1.269 | — | — |
| Area variation (F) | — | 0.719 | 0.140 | 0.229 | 0.276 | 0.347 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 8.89 | 9.03 | 7.15 | 6.64 | 6.90 | — | — |
| Mean distance, ‰ (H) | — | 0.521 | 0.281 | 0.238 | 0.226 | 0.222 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 20.8 | — | — | 21.0 | 22.9 | — | — |
| Edges on spiralling loops, % (L) | — | 8.5 | — | — | 0.0 | 0.3 | — | — |
| Spiralling loops (L) | — | 5 | — | — | 0 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 53.1 | — | — | 47.4 | 50.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 154 | — | — | 237 | 255 | — | — |
| Edges on loops from a border to a border, % (L) | — | 17.6 | — | — | 31.5 | 26.7 | — | — |
| Off budget, % (N) | — | 38.1 | 0.5 | 6.7 | 0.5 | 1.0 | — | — |
| Time, seconds (T) | — | 3.19 | 2.16 | 23.83 | 15.44 | 9.25 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 4954 | 7732 | 7824 | 7904 | 8059 | — | — |
| Irregular vertices (S) | 38 | 94 | 337 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 30 | — | 329 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 1171 | — | 4678 | 128 | 66 | 66 | — | — |
| Alignment, degrees (A) | 13.31 | 14.04 | 13.52 | 11.84 | 12.20 | 10.82 | — | — |
| Mean angle deviation, degrees (F) | 2.56 | 13.77 | 4.52 | 1.66 | 1.01 | 0.98 | — | — |
| Angle deviation, 95th percentile (F) | 9.55 | 47.48 | 18.21 | 7.02 | 3.60 | 3.59 | — | — |
| Mean aspect ratio (F) | 1.074 | 3.591 | 1.178 | 1.151 | 1.082 | 1.106 | — | — |
| Area variation (F) | 0.087 | 0.912 | 0.166 | 0.105 | 0.112 | 0.297 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.14 | 11.34 | 5.05 | 0.32 | 0.33 | 0.26 | — | — |
| Mean distance, ‰ (H) | 0.034 | 0.506 | 0.023 | 0.021 | 0.022 | 0.024 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.06 | — | 100.06 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 59.2 | — | — | — | 90.6 | 90.8 | — | — |
| Edges on spiralling loops, % (L) | 22.9 | — | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 8 | — | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 17.7 | — | — | — | 9.4 | 9.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 160 | — | — | — | 93 | 82 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.3 | 38.1 | 3.4 | 2.2 | 1.2 | 0.7 | — | — |
| Time, seconds (T) | 8.90 | 3.06 | 2.22 | 10.17 | 7.53 | 8.19 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7920 | 2774 | 7802 | 8028 | 8066 | 8066 | — | — |
| Irregular vertices (S) | 8 | 10 | 18 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 10 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | — | 376 | 8028 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.46 | 2.60 | 0.96 | 0.68 | 0.11 | 0.28 | — | — |
| Mean angle deviation, degrees (F) | 0.36 | 5.97 | 0.91 | 0.95 | 0.01 | 0.21 | — | — |
| Angle deviation, 95th percentile (F) | 1.18 | 24.98 | 2.60 | 2.23 | 0.01 | 0.77 | — | — |
| Mean aspect ratio (F) | 1.020 | 2.866 | 1.030 | 1.032 | 1.018 | 1.074 | — | — |
| Area variation (F) | 0.023 | 0.740 | 0.045 | 0.044 | 0.013 | 0.092 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.85 | 4.81 | 0.00 | 5.40 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.007 | 0.165 | 0.000 | 0.046 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.99 | 100.00 | 99.99 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 97.2 | 27.9 | — | — | 97.3 | 97.3 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 63.5 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 6 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 2.8 | 8.5 | — | — | 2.7 | 2.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 37 | 85 | — | — | 37 | 37 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.0 | 65.3 | 2.5 | 0.4 | 0.8 | 0.8 | — | — |
| Time, seconds (T) | 4.14 | 1.30 | 0.78 | 65.82 | 5.56 | 6.46 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8016 | 3035 | 8000 | 8064 | 8064 | 8064 | — | — |
| Irregular vertices (S) | 13 | 20 | 24 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | — | 16 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 402 | — | 706 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 0.86 | 3.67 | 1.17 | 0.57 | 0.60 | 0.93 | — | — |
| Mean angle deviation, degrees (F) | 0.40 | 6.33 | 0.88 | 0.01 | 0.00 | 0.50 | — | — |
| Angle deviation, 95th percentile (F) | 1.40 | 24.33 | 2.62 | 0.03 | 0.01 | 1.47 | — | — |
| Mean aspect ratio (F) | 1.018 | 2.553 | 1.025 | 1.000 | 1.000 | 1.078 | — | — |
| Area variation (F) | 0.024 | 0.668 | 0.041 | 0.000 | 0.000 | 0.096 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.72 | 5.67 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.005 | 0.155 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.97 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 82.3 | 50.2 | — | — | 95.8 | 95.8 | — | — |
| Edges on spiralling loops, % (L) | 11.3 | 27.9 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 1 | 5 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 6.1 | 22.0 | — | — | 4.2 | 4.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 132 | 134 | — | — | 72 | 72 | — | — |
| Edges on loops from a border to a border, % (L) | 0.3 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.6 | 62.1 | 0.1 | 0.8 | 0.8 | 0.8 | — | — |
| Time, seconds (T) | 3.24 | 1.54 | 0.34 | 5.37 | 6.38 | 6.56 | — | — |

Failures: quadriflow: timed out after 1800 s.

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7916 | 2565 | 8024 | 7276 | 7946 | 7960 | — | — |
| Irregular vertices (S) | 8 | 10 | 165 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 157 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 2635 | — | 4940 | 1298 | 6 | 6 | — | — |
| Alignment, degrees (A) | 4.37 | 4.10 | 3.47 | 1.33 | 1.27 | 2.15 | — | — |
| Mean angle deviation, degrees (F) | 3.95 | 6.60 | 3.05 | 2.55 | 2.23 | 2.96 | — | — |
| Angle deviation, 95th percentile (F) | 16.68 | 20.66 | 11.70 | 13.62 | 8.48 | 10.79 | — | — |
| Mean aspect ratio (F) | 1.075 | 2.021 | 1.097 | 1.285 | 1.135 | 1.144 | — | — |
| Area variation (F) | 0.096 | 0.636 | 0.115 | 0.284 | 0.149 | 0.117 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.80 | 5.39 | 2.07 | 0.55 | 0.45 | 0.42 | — | — |
| Mean distance, ‰ (H) | 0.052 | 0.289 | 0.045 | 0.071 | 0.064 | 0.060 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.96 | 99.70 | 99.96 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 33.3 | 29.3 | — | — | 97.2 | 97.2 | — | — |
| Edges on spiralling loops, % (L) | 48.3 | 33.0 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 5 | 3 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 18.6 | 37.7 | — | — | 2.8 | 2.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 762 | 316 | — | — | 54 | 50 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.1 | 67.9 | 0.5 | 9.0 | 0.7 | 0.5 | — | — |
| Time, seconds (T) | 4.60 | 1.04 | 0.57 | 7.70 | 6.39 | 6.57 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 7212 | 7693 | 6950 | 7624 | 8945 | — | — |
| Irregular vertices (S) | — | 2158 | 977 | 270 | 255 | 1035 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4255 | — | — | — | — | — |
| Alignment, degrees (A) | — | 10.19 | 11.23 | 10.34 | 9.83 | 11.73 | — | — |
| Mean angle deviation, degrees (F) | — | 40.97 | 10.36 | 6.59 | 6.00 | 8.74 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.08 | 43.42 | 21.59 | 19.88 | 32.57 | — | — |
| Mean aspect ratio (F) | — | 9.234 | 1.775 | 1.305 | 1.272 | 1.717 | — | — |
| Area variation (F) | — | 2.203 | 0.455 | 0.267 | 0.286 | 0.466 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 37.63 | 24.82 | 35.65 | 38.87 | 38.87 | — | — |
| Mean distance, ‰ (H) | — | 2.866 | 1.379 | 2.882 | 2.387 | 2.177 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 28.8 | 18.1 | 40.3 | 34.2 | 32.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 23.0 | 20.9 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 30.7 | 20.5 | — | — |
| Spiralling loops (L) | — | — | — | — | 47 | 33 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 46.3 | 58.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 185 | 185 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 9.8 | 3.8 | 13.1 | 4.7 | 11.8 | — | — |
| Time, seconds (T) | — | 1.84 | 1.45 | 87.87 | 54.53 | 55.02 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4422 | 7700 | 6805 | 7946 | 8037 | — | — |
| Irregular vertices (S) | — | 172 | 442 | 163 | 198 | 340 | — | — |
| … of which above the minimum | — | — | 446 | — | 190 | 332 | — | — |
| Base complex patches (C) | — | — | 7440 | — | 4886 | 6369 | — | — |
| Alignment, degrees (A) | — | 13.99 | 17.01 | 12.30 | 12.01 | 10.63 | — | — |
| Mean angle deviation, degrees (F) | — | 7.16 | 6.55 | 8.73 | 6.42 | 6.44 | — | — |
| Angle deviation, 95th percentile (F) | — | 21.73 | 21.36 | 23.56 | 17.10 | 19.08 | — | — |
| Mean aspect ratio (F) | — | 1.812 | 1.226 | 1.417 | 1.413 | 1.367 | — | — |
| Area variation (F) | — | 0.657 | 0.198 | 0.304 | 0.339 | 0.941 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 22.79 | 30.78 | 25.14 | 11.75 | 7.74 | — | — |
| Mean distance, ‰ (H) | — | 1.109 | 0.706 | 0.441 | 0.313 | 0.336 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 44.3 | 48.7 | 16.3 | 9.8 | 1.6 | — | — |
| Volume kept, % of the source (V) | — | — | 99.39 | 99.93 | 100.00 | 100.02 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 20.1 | 10.5 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 7.0 | 14.1 | — | — |
| Spiralling loops (L) | — | — | — | — | 23 | 31 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 73.0 | 75.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 260 | 163 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 44.7 | 3.8 | 14.9 | 0.7 | 0.5 | — | — |
| Time, seconds (T) | — | 17.03 | 82.86 | 80.09 | 64.94 | 77.38 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 7305 | 2507 | — | — | 6622 | 6696 | — | — |
| Irregular vertices (S) | 642 | 428 | — | — | 403 | 418 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6985 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 13.37 | 16.69 | — | — | 13.23 | 13.30 | — | — |
| Mean angle deviation, degrees (F) | 9.18 | 21.18 | — | — | 8.22 | 8.48 | — | — |
| Angle deviation, 95th percentile (F) | 32.10 | 68.70 | — | — | 29.42 | 30.93 | — | — |
| Mean aspect ratio (F) | 1.331 | 2.614 | — | — | 1.339 | 1.362 | — | — |
| Area variation (F) | 0.268 | 0.825 | — | — | 0.280 | 0.299 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 50.24 | 90.14 | — | — | 53.47 | 53.47 | — | — |
| Mean distance, ‰ (H) | 1.990 | 7.482 | — | — | 2.483 | 2.465 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.3 | 69.7 | — | — | 33.0 | 33.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.3 | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 12.4 | — | — | — | — | — | — | — |
| Spiralling loops (L) | 125 | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.2 | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 151 | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | — | — | — | — |
| Off budget, % (N) | 8.7 | 68.7 | — | — | 17.2 | 16.3 | — | — |
| Time, seconds (T) | 7.40 | 4.79 | — | — | 76.69 | 103.29 | — | — |

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 7180 | 2664 | — | — | 5861 | — | — | — |
| Irregular vertices (S) | 624 | 292 | — | — | 261 | — | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6952 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 11.45 | 14.32 | — | — | 11.73 | — | — | — |
| Mean angle deviation, degrees (F) | 9.29 | 18.29 | — | — | 8.78 | — | — | — |
| Angle deviation, 95th percentile (F) | 29.68 | 60.76 | — | — | 29.04 | — | — | — |
| Mean aspect ratio (F) | 1.309 | 2.412 | — | — | 1.396 | — | — | — |
| Area variation (F) | 0.258 | 0.707 | — | — | 0.378 | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 34.62 | 60.39 | — | — | 53.73 | — | — | — |
| Mean distance, ‰ (H) | 1.468 | 4.925 | — | — | 2.316 | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.1 | 39.9 | — | — | 29.0 | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.3 | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 18.5 | — | — | — | — | — | — | — |
| Spiralling loops (L) | 50 | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.1 | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 117 | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | — | — | — | — |
| Off budget, % (N) | 10.2 | 66.7 | — | — | 26.7 | — | — | — |
| Time, seconds (T) | 7.17 | 4.13 | — | — | 98.95 | — | — | — |

Failures: openclare: timed out after 1800 s.

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 0/0 | 0/0 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6712 | 4327 | — | — | 7948 | 8161 | — | — |
| Irregular vertices (S) | 400 | 121 | — | — | 188 | 288 | — | — |
| … of which above the minimum | 341 | — | — | — | 124 | 224 | — | — |
| Base complex patches (C) | 6543 | — | — | — | 4808 | 6703 | — | — |
| Alignment, degrees (A) | 13.29 | 13.65 | — | — | 11.45 | 11.64 | — | — |
| Mean angle deviation, degrees (F) | 9.33 | 10.72 | — | — | 4.87 | 5.87 | — | — |
| Angle deviation, 95th percentile (F) | 30.16 | 31.96 | — | — | 15.21 | 17.77 | — | — |
| Mean aspect ratio (F) | 1.293 | 1.880 | — | — | 9.825 | 1.273 | — | — |
| Area variation (F) | 0.268 | 0.625 | — | — | 0.216 | 0.328 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.23 | 58.87 | — | — | 10.99 | 9.28 | — | — |
| Mean distance, ‰ (H) | 1.190 | 2.121 | — | — | 1.067 | 1.015 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.8 | 185.8 | — | — | 25.9 | 18.0 | — | — |
| Volume kept, % of the source (V) | 99.56 | 99.45 | — | — | 99.97 | 100.01 | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.8 | — | — | 22.1 | 9.6 | — | — |
| Edges on spiralling loops, % (L) | 8.9 | 24.3 | — | — | 2.3 | 7.8 | — | — |
| Spiralling loops (L) | 33 | 25 | — | — | 21 | 16 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.0 | 74.8 | — | — | 75.6 | 82.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 149 | 168 | — | — | 331 | 231 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 16.1 | 45.9 | — | — | 0.7 | 2.0 | — | — |
| Time, seconds (T) | 6.75 | 3.06 | — | — | 10.22 | 14.15 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 2364 | — | — | 253 | 646 | — | — |
| Irregular vertices (S) | — | 234 | — | — | 36 | 201 | — | — |
| … of which above the minimum | — | — | — | — | 12 | 177 | — | — |
| Base complex patches (C) | — | — | — | — | 223 | 646 | — | — |
| Alignment, degrees (A) | — | 13.30 | — | — | 15.31 | 20.13 | — | — |
| Mean angle deviation, degrees (F) | — | 14.00 | — | — | 24.10 | 19.91 | — | — |
| Angle deviation, 95th percentile (F) | — | 51.31 | — | — | 63.32 | 52.44 | — | — |
| Mean aspect ratio (F) | — | 2.278 | — | — | 8.351 | 2.425 | — | — |
| Area variation (F) | — | 0.698 | — | — | 1.175 | 1.185 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 93.86 | — | — | 528.59 | 529.46 | — | — |
| Mean distance, ‰ (H) | — | 6.515 | — | — | 242.244 | 220.054 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 80.5 | — | — | 1833.8 | 1827.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 1.71 | 1.75 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 5.9 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 27.1 | 5.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 8 | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 67.0 | 94.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 19 | 12 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 70.5 | — | — | 96.8 | 91.9 | — | — |
| Time, seconds (T) | — | 3.34 | — | — | 3.44 | 7.75 | — | — |

Failures: quadriflow: cancelled.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6963 | 2975 | 4807 | 6773 | 7903 | 8099 | — | — |
| Irregular vertices (S) | 188 | 108 | 273 | 124 | 130 | 231 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2067 | — | 3587 | 4296 | 5481 | 7242 | — | — |
| Alignment, degrees (A) | 21.04 | 20.95 | 21.43 | 17.68 | 17.74 | 17.48 | — | — |
| Mean angle deviation, degrees (F) | 5.79 | 7.30 | 7.45 | 10.25 | 6.61 | 6.75 | — | — |
| Angle deviation, 95th percentile (F) | 21.42 | 26.48 | 25.48 | 26.71 | 18.84 | 18.52 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.481 | 1.264 | 1.320 | 1.291 | 1.297 | — | — |
| Area variation (F) | 0.185 | 0.463 | 0.215 | 0.287 | 0.307 | 0.499 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 16.98 | 26.34 | 130.45 | 15.44 | 10.62 | 8.42 | — | — |
| Mean distance, ‰ (H) | 1.071 | 1.562 | 1.477 | 0.991 | 0.745 | 0.621 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 34.0 | 47.6 | 41.7 | 38.6 | 23.3 | 18.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.8 | 1.4 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.9 | 71.1 | — | — | 82.8 | 94.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 55 | — | — | 181 | 138 | — | — |
| Edges on loops from a border to a border, % (L) | 46.1 | 28.9 | — | — | 16.4 | 4.1 | — | — |
| Off budget, % (N) | 13.0 | 62.8 | 39.9 | 15.3 | 1.2 | 1.2 | — | — |
| Time, seconds (T) | 19.08 | 4.25 | 10.05 | 45.81 | 11.78 | 13.86 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2996 | 5345 | 6148 | 6577 | 7958 | — | — |
| Irregular vertices (S) | — | 419 | 420 | 166 | 538 | 579 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4008 | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.27 | 9.69 | 6.08 | 4.92 | 4.67 | — | — |
| Mean angle deviation, degrees (F) | — | 19.93 | 8.33 | 7.42 | 10.49 | 8.33 | — | — |
| Angle deviation, 95th percentile (F) | — | 77.95 | 32.25 | 24.85 | 63.48 | 51.69 | — | — |
| Mean aspect ratio (F) | — | 3.957 | 1.365 | 1.551 | 41.350 | 51.364 | — | — |
| Area variation (F) | — | 1.337 | 0.299 | 0.358 | 3.040 | 1.753 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.28 | 32.21 | 14.95 | 27.63 | 30.95 | — | — |
| Mean distance, ‰ (H) | — | 3.494 | 3.712 | 2.684 | 2.280 | 2.069 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.7 | 60.8 | 45.3 | 32.5 | 32.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 62.5 | 33.2 | 23.2 | 17.8 | 0.5 | — | — |
| Time, seconds (T) | — | 2.97 | 6.80 | 72.10 | 10.87 | 11.04 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2388 | 4146 | — | 12272 | 11937 | — | — |
| Irregular vertices (S) | — | 1056 | 1524 | — | 6104 | 6245 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 3685 | — | — | — | — | — |
| Alignment, degrees (A) | — | 15.08 | 17.92 | — | 11.62 | 12.16 | — | — |
| Mean angle deviation, degrees (F) | — | 28.51 | 20.55 | — | 17.26 | 17.78 | — | — |
| Angle deviation, 95th percentile (F) | — | 82.76 | 58.40 | — | 63.01 | 62.87 | — | — |
| Mean aspect ratio (F) | — | 7.658 | 3.745 | — | 5.468 | 3.730 | — | — |
| Area variation (F) | — | 1.726 | 0.689 | — | 2.323 | 2.344 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 6.92 | 6.21 | — | 3.78 | 3.95 | — | — |
| Mean distance, ‰ (H) | — | 1.765 | 1.017 | — | 0.312 | 0.320 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 94.0 | 58.2 | — | 15.5 | 15.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 17.1 | 15.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 25.7 | 25.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 3938 | 3895 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 56.6 | 59.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 36 | 40 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.6 | 0.4 | — | — |
| Off budget, % (N) | — | 70.2 | 48.2 | — | 53.4 | 49.2 | — | — |
| Time, seconds (T) | — | 1.13 | 4.87 | — | 14.75 | 25.69 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 10 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 6227 | 4548 | 6707 | 9161 | 9383 | — | — |
| Irregular vertices (S) | — | 2171 | 863 | 645 | 1856 | 1773 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 3548 | — | — | — | — | — |
| Alignment, degrees (A) | — | 10.29 | 8.12 | 5.58 | 5.63 | 4.85 | — | — |
| Mean angle deviation, degrees (F) | — | 29.31 | 14.41 | 9.45 | 11.02 | 10.05 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.69 | 54.18 | 32.95 | 53.16 | 55.59 | — | — |
| Mean aspect ratio (F) | — | 6.307 | 1.909 | 1.788 | 66.504 | 56.223 | — | — |
| Area variation (F) | — | 1.369 | 0.470 | 0.391 | 1.637 | 1.796 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 80.26 | 73.99 | 36.01 | 17.66 | 16.56 | — | — |
| Mean distance, ‰ (H) | — | 5.085 | 10.384 | 2.811 | 0.741 | 0.646 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 60.6 | 128.6 | 37.0 | 8.5 | 7.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 22.2 | 43.1 | 16.2 | 14.5 | 17.3 | — | — |
| Time, seconds (T) | — | 3.57 | 5.80 | 285.92 | 16.65 | 13.83 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4114 | 6839 | 7017 | 7720 | 8045 | — | — |
| Irregular vertices (S) | — | 403 | 532 | 170 | 235 | 245 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4440 | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.73 | 6.43 | 3.94 | 4.27 | 3.43 | — | — |
| Mean angle deviation, degrees (F) | — | 18.87 | 9.46 | 5.96 | 12.95 | 14.01 | — | — |
| Angle deviation, 95th percentile (F) | — | 74.17 | 38.06 | 23.18 | 72.63 | 72.50 | — | — |
| Mean aspect ratio (F) | — | 3.158 | 1.596 | 1.823 | 11.977 | 23.100 | — | — |
| Area variation (F) | — | 1.607 | 0.398 | 0.330 | 1.023 | 1.095 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 58.33 | 36.49 | 36.30 | 32.96 | 32.96 | — | — |
| Mean distance, ‰ (H) | — | 3.579 | 1.944 | 1.189 | 1.041 | 0.993 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 32.4 | 21.3 | 11.1 | 11.1 | 11.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 48.6 | 14.5 | 12.3 | 3.5 | 0.6 | — | — |
| Time, seconds (T) | — | 1.11 | 2.76 | 146.04 | 11.00 | 11.66 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2119 | 7656 | 6579 | 6750 | 6830 | — | — |
| Irregular vertices (S) | — | 195 | 835 | 380 | 422 | 462 | — | — |
| … of which above the minimum | — | — | 730 | — | — | — | — | — |
| Base complex patches (C) | — | — | 7378 | — | — | — | — | — |
| Alignment, degrees (A) | — | 13.19 | 15.59 | 10.07 | 10.44 | 10.78 | — | — |
| Mean angle deviation, degrees (F) | — | 10.86 | 8.99 | 9.19 | 9.88 | 10.09 | — | — |
| Angle deviation, 95th percentile (F) | — | 32.41 | 30.80 | 26.78 | 28.15 | 28.79 | — | — |
| Mean aspect ratio (F) | — | 1.955 | 1.370 | 1.416 | 1.446 | 1.456 | — | — |
| Area variation (F) | — | 0.849 | 0.275 | 0.321 | 0.317 | 0.338 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.12 | 31.17 | 13.40 | 12.80 | 12.80 | — | — |
| Mean distance, ‰ (H) | — | 3.139 | 1.125 | 0.922 | 0.718 | 0.698 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 103.8 | 74.2 | 59.6 | 37.6 | 35.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 12.3 | — | — | 13.4 | 13.2 | — | — |
| Edges on spiralling loops, % (L) | — | 3.9 | — | — | 22.8 | 22.7 | — | — |
| Spiralling loops (L) | — | 9 | — | — | 98 | 112 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 83.8 | — | — | 63.8 | 64.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 57 | — | — | 110 | 110 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 73.5 | 4.3 | 17.8 | 15.6 | 14.6 | — | — |
| Time, seconds (T) | — | 2.60 | 1.15 | 117.14 | 87.19 | 85.39 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7736 | 6520 | 8190 | 7568 | 8056 | 7991 | — | — |
| Irregular vertices (S) | 12 | 26 | 193 | 26 | 8 | 8 | — | — |
| … of which above the minimum | 4 | 18 | 185 | 18 | 0 | 0 | — | — |
| Base complex patches (C) | 7729 | 6252 | 8190 | 7568 | 6 | 7991 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.93 | 5.85 | 3.49 | 6.08 | 1.51 | 1.66 | — | — |
| Angle deviation, 95th percentile (F) | 12.52 | 14.46 | 11.93 | 13.87 | 3.79 | 3.98 | — | — |
| Mean aspect ratio (F) | 1.109 | 1.337 | 1.097 | 1.149 | 1.110 | 1.119 | — | — |
| Area variation (F) | 0.134 | 0.283 | 0.122 | 0.153 | 0.125 | 0.115 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.42 | 0.80 | 0.43 | 0.40 | 0.29 | 0.30 | — | — |
| Mean distance, ‰ (H) | 0.085 | 0.172 | 0.082 | 0.083 | 0.073 | 0.076 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.93 | 99.83 | 99.92 | 100.01 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 2.1 | — | — | 97.3 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 52.4 | 42.1 | — | — | 0.0 | 97.9 | — | — |
| Spiralling loops (L) | 5 | 10 | — | — | 0 | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 47.6 | 55.8 | — | — | 2.7 | 2.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1548 | 584 | — | — | 38 | 74 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 18.5 | 2.4 | 5.4 | 0.7 | 0.1 | — | — |
| Time, seconds (T) | 4.47 | 1.94 | 0.54 | 13.07 | 12.82 | 159.44 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 9147 | 7544 | 8038 | 7280 | 8024 | 8024 | — | — |
| Irregular vertices (S) | 22 | 0 | 262 | 0 | 0 | 0 | — | — |
| … of which above the minimum | 22 | 0 | 262 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 9070 | 1 | 7956 | 1 | 1 | 1 | — | — |
| Alignment, degrees (A) | 11.41 | 0.76 | 6.29 | 0.61 | 0.64 | 0.64 | — | — |
| Mean angle deviation, degrees (F) | 7.33 | 1.26 | 5.12 | 0.91 | 0.91 | 0.91 | — | — |
| Angle deviation, 95th percentile (F) | 19.57 | 2.30 | 15.85 | 1.45 | 1.41 | 1.41 | — | — |
| Mean aspect ratio (F) | 1.180 | 2.552 | 1.139 | 1.340 | 1.321 | 1.319 | — | — |
| Area variation (F) | 0.189 | 0.146 | 0.149 | 0.281 | 0.299 | 0.300 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.60 | 0.79 | 0.65 | 0.34 | 0.35 | 0.35 | — | — |
| Mean distance, ‰ (H) | 0.109 | 0.178 | 0.119 | 0.111 | 0.104 | 0.104 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.82 | 99.67 | 99.79 | 100.01 | 100.01 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 100.0 | — | — | 100.0 | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 38.2 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 8 | 0 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 61.8 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1065 | 0 | — | — | 0 | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 14.3 | 5.7 | 0.5 | 9.0 | 0.3 | 0.3 | — | — |
| Time, seconds (T) | 5.42 | 2.20 | 0.91 | 6.48 | 4.70 | 5.09 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 5 | 0 | 7 | 3 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 2 | 0 | 0 | 4 | 8 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 2 | 1 | 12 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 1 | 7 | 1 | 5 | 0 | 0 | 1 |
| Mean aspect ratio (F) | 1 | 0 | 13 | 1 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 33/75 | 4/15 | 50/75 | 8/15 | 14/15 | 15/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 0 | 0 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 1 | 4 | 4 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 2 | 1 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 3 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 0 |
| Valid meshes (V) | 14/20 | 0/4 | 19/20 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
