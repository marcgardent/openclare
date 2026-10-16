# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7590 | 2795 | 8164 | 7124 | 7931 | 8017 | 7911 | 7035 | 7911 | 7838 | — | — |
| Irregular vertices (S) | 40 | 28 | 277 | 24 | 24 | 24 | 24 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 32 | 20 | 269 | 16 | 16 | 16 | 16 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 7412 | 1171 | 8164 | 5660 | 3561 | 2629 | 4563 | 2919 | 4563 | 2498 | — | — |
| Alignment, degrees (A) | 14.77 | 5.92 | 15.05 | 5.88 | 5.70 | 5.22 | 4.08 | 4.45 | 4.08 | 5.40 | — | — |
| Mean angle deviation, degrees (F) | 7.77 | 9.19 | 4.85 | 7.41 | 6.41 | 5.95 | 5.52 | 5.55 | 5.52 | 6.28 | — | — |
| Angle deviation, 95th percentile (F) | 19.95 | 34.45 | 15.77 | 19.73 | 17.16 | 16.59 | 14.94 | 14.71 | 14.94 | 16.77 | — | — |
| Mean aspect ratio (F) | 1.152 | 2.101 | 1.142 | 1.317 | 1.313 | 1.281 | 1.414 | 1.323 | 1.414 | 1.295 | — | — |
| Area variation (F) | 0.177 | 0.585 | 0.153 | 0.316 | 0.349 | 0.383 | 0.395 | 0.331 | 0.395 | 0.380 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.18 | 2.78 | 4.97 | 2.41 | 1.88 | 1.70 | 3.00 | 3.66 | 3.00 | 1.50 | — | — |
| Mean distance, ‰ (H) | 0.171 | 0.760 | 0.178 | 0.097 | 0.084 | 0.077 | 0.185 | 0.216 | 0.185 | 0.079 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.84 | 98.72 | 99.86 | 100.01 | 100.01 | 100.01 | 99.77 | 99.81 | 99.77 | 100.01 | — | — |
| Edges on closed loops, % (L) | 1.2 | 33.0 | — | — | 28.6 | 34.8 | 21.2 | — | — | 34.7 | — | — |
| Edges on spiralling loops, % (L) | 35.9 | 18.0 | — | — | 28.9 | 15.5 | 46.2 | — | — | 20.9 | — | — |
| Spiralling loops (L) | 12 | 4 | — | — | 7 | 5 | 11 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 62.6 | 49.1 | — | — | 42.5 | 49.7 | 32.6 | — | — | 44.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 766 | 292 | — | — | 778 | 719 | 601 | — | — | 952 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.1 | 65.1 | 2.1 | 10.9 | 0.9 | 0.2 | 1.1 | 12.1 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | 4.32 | 2.57 | 1.85 | 10.94 | 6.39 | 7.77 | 12.37 | 12.61 | 12.37 | 7.19 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7342 | 5842 | 7792 | 7393 | 7986 | 8046 | 7071 | 6767 | 7071 | 7159 | — | — |
| Irregular vertices (S) | 10 | 34 | 162 | 114 | 116 | 134 | 68 | 68 | 68 | 116 | — | — |
| … of which above the minimum | 2 | 26 | 154 | 106 | 108 | 126 | 60 | 60 | 60 | 108 | — | — |
| Base complex patches (C) | 7342 | 5052 | 7792 | 7393 | 7986 | 8046 | 7071 | 6767 | 7071 | 7159 | — | — |
| Alignment, degrees (A) | 22.36 | 18.66 | 22.82 | 16.04 | 16.60 | 16.31 | 17.28 | 17.32 | 16.72 | 16.04 | — | — |
| Mean angle deviation, degrees (F) | 6.22 | 6.90 | 4.13 | 9.24 | 6.10 | 5.45 | 9.80 | 9.89 | 9.80 | 5.84 | — | — |
| Angle deviation, 95th percentile (F) | 18.32 | 17.38 | 12.43 | 22.12 | 14.54 | 13.33 | 23.73 | 23.67 | 23.73 | 14.39 | — | — |
| Mean aspect ratio (F) | 1.158 | 1.393 | 1.115 | 1.246 | 1.227 | 1.210 | 1.350 | 1.340 | 1.350 | 1.238 | — | — |
| Area variation (F) | 0.244 | 0.344 | 0.128 | 0.204 | 0.254 | 0.267 | 0.287 | 0.281 | 0.287 | 0.272 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.95 | 3.88 | 3.22 | 3.64 | 3.10 | 2.82 | 4.18 | 4.36 | 4.18 | 3.09 | — | — |
| Mean distance, ‰ (H) | 0.478 | 0.669 | 0.433 | 0.418 | 0.394 | 0.385 | 0.515 | 0.530 | 0.515 | 0.416 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.79 | 99.89 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 6.9 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 70.9 | 30.4 | — | — | 2.7 | 2.0 | 8.2 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 6 | 7 | — | — | 2 | 1 | 3 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 29.1 | 62.6 | — | — | 97.3 | 98.0 | 91.8 | — | — | 100.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1193 | 618 | — | — | 301 | 316 | 490 | — | — | 416 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 8.2 | 27.0 | 2.6 | 7.6 | 0.2 | 0.6 | 11.6 | 15.4 | 0.0 | 1.2 | — | — |
| Time, seconds (T) | 4.18 | 3.32 | 1.45 | 33.90 | 6.57 | 6.20 | 18.31 | 18.00 | 18.31 | 5.37 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8170 | 6819 | 8234 | 6922 | 7884 | 8163 | 7186 | 7207 | 7186 | 7139 | — | — |
| Irregular vertices (S) | 22 | 18 | 214 | 40 | 40 | 42 | 40 | 40 | 40 | 40 | — | — |
| … of which above the minimum | 14 | 10 | 206 | — | 32 | 34 | 32 | 32 | 32 | 32 | — | — |
| Base complex patches (C) | 8170 | 3683 | 7510 | 2490 | 2864 | 1853 | 3236 | 3467 | 3236 | 3545 | — | — |
| Alignment, degrees (A) | 13.70 | 9.77 | 11.81 | 6.40 | 6.02 | 5.76 | 6.64 | 6.66 | 6.64 | 5.69 | — | — |
| Mean angle deviation, degrees (F) | 6.02 | 3.78 | 3.91 | 8.75 | 6.71 | 5.91 | 6.13 | 6.16 | 6.13 | 5.58 | — | — |
| Angle deviation, 95th percentile (F) | 15.98 | 10.36 | 12.20 | 25.50 | 18.89 | 16.43 | 17.40 | 17.67 | 17.40 | 14.91 | — | — |
| Mean aspect ratio (F) | 1.154 | 1.965 | 1.116 | 1.352 | 1.345 | 1.333 | 1.277 | 1.278 | 1.277 | 1.336 | — | — |
| Area variation (F) | 0.157 | 0.565 | 0.127 | 0.334 | 0.364 | 0.472 | 0.277 | 0.275 | 0.277 | 0.489 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.19 | 1.09 | 1.70 | 2.35 | 0.96 | 1.01 | 1.24 | 1.34 | 1.24 | 1.00 | — | — |
| Mean distance, ‰ (H) | 0.100 | 0.161 | 0.106 | 0.099 | 0.085 | 0.079 | 0.144 | 0.145 | 0.144 | 0.091 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.74 | 99.81 | — | 100.01 | 100.01 | 99.72 | 99.71 | 99.72 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 23.0 | — | — | 32.4 | 41.2 | 27.5 | — | — | 25.2 | — | — |
| Edges on spiralling loops, % (L) | 44.8 | 43.6 | — | — | 23.7 | 21.0 | 30.8 | — | — | 31.7 | — | — |
| Spiralling loops (L) | 5 | 9 | — | — | 11 | 7 | 13 | — | — | 8 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 55.2 | 33.4 | — | — | 43.9 | 37.8 | 41.7 | — | — | 43.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1471 | 594 | — | — | 519 | 413 | 510 | — | — | 643 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.1 | 14.8 | 2.9 | 13.5 | 1.5 | 2.0 | 10.2 | 9.9 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 5.64 | 3.36 | 1.55 | 24.32 | 11.75 | 15.20 | 12.49 | 8.52 | 12.49 | 10.85 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7932 | 5928 | 8050 | 7765 | 8004 | 7897 | 11009 | 7510 | 11009 | 11166 | — | — |
| Irregular vertices (S) | 72 | 58 | 250 | 40 | 48 | 58 | 62 | 64 | 62 | 64 | — | — |
| … of which above the minimum | 40 | — | 218 | 8 | 16 | 26 | 30 | 32 | 30 | 32 | — | — |
| Base complex patches (C) | 7283 | — | 7125 | 766 | 812 | 153 | 3227 | 1954 | 3227 | 120 | — | — |
| Alignment, degrees (A) | 8.22 | 5.16 | 8.80 | 2.71 | 2.46 | 1.92 | 2.06 | 2.29 | 2.06 | 1.91 | — | — |
| Mean angle deviation, degrees (F) | 5.83 | 7.40 | 4.40 | 3.58 | 2.09 | 2.22 | 3.22 | 3.48 | 3.22 | 2.04 | — | — |
| Angle deviation, 95th percentile (F) | 16.15 | 22.71 | 14.23 | 10.77 | 6.65 | 6.70 | 9.04 | 10.22 | 9.04 | 6.24 | — | — |
| Mean aspect ratio (F) | 1.140 | 2.161 | 1.144 | 1.200 | 1.187 | 1.241 | 1.693 | 1.289 | 1.693 | 1.194 | — | — |
| Area variation (F) | 0.139 | 0.487 | 0.138 | 0.191 | 0.221 | 0.319 | 0.338 | 0.229 | 0.338 | 0.344 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.65 | 7.28 | 9.87 | 4.82 | 2.67 | 2.82 | 4.53 | 5.93 | 4.53 | 1.87 | — | — |
| Mean distance, ‰ (H) | 0.294 | 0.343 | 0.344 | 0.275 | 0.238 | 0.213 | 0.451 | 0.614 | 0.451 | 0.120 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.64 | 99.99 | 99.48 | 99.98 | 99.99 | 100.00 | 99.38 | 99.02 | 99.38 | 100.00 | — | — |
| Edges on closed loops, % (L) | 5.0 | 25.6 | — | — | 69.9 | 86.2 | 42.9 | — | — | 89.5 | — | — |
| Edges on spiralling loops, % (L) | 55.9 | 48.4 | — | — | 18.4 | 0.0 | 42.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 33 | 29 | — | — | 5 | 0 | 14 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 40.8 | 26.0 | — | — | 11.7 | 13.8 | 15.1 | — | — | 10.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 332 | 108 | — | — | 74 | 38 | 129 | — | — | 33 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 0.9 | 25.9 | 0.6 | 2.9 | 0.1 | 1.3 | 37.6 | 6.1 | 0.0 | 1.4 | — | — |
| Time, seconds (T) | 5.53 | 3.23 | 0.57 | 25.95 | 5.47 | 6.08 | 5.76 | 4.92 | 5.76 | 8.55 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 2647 | 8098 | 7376 | 8061 | 7925 | 9246 | 7317 | 9246 | 9149 | — | — |
| Irregular vertices (S) | 55 | 84 | 346 | 67 | 64 | 82 | 78 | 72 | 78 | 86 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6194 | — | 7878 | 4267 | 3811 | 3330 | 4088 | 3340 | 4088 | 3557 | — | — |
| Alignment, degrees (A) | 14.27 | 12.11 | 16.64 | 10.01 | 10.12 | 9.77 | 8.58 | 9.86 | 8.58 | 9.42 | — | — |
| Mean angle deviation, degrees (F) | 6.09 | 9.37 | 5.17 | 7.60 | 5.75 | 5.84 | 7.04 | 6.91 | 7.04 | 6.07 | — | — |
| Angle deviation, 95th percentile (F) | 18.06 | 32.08 | 17.78 | 21.30 | 16.02 | 15.26 | 18.57 | 19.33 | 18.57 | 16.04 | — | — |
| Mean aspect ratio (F) | 1.208 | 1.892 | 1.169 | 1.223 | 1.214 | 1.236 | 1.470 | 1.246 | 1.470 | 1.256 | — | — |
| Area variation (F) | 0.208 | 0.650 | 0.164 | 0.218 | 0.255 | 0.377 | 0.366 | 0.248 | 0.366 | 0.378 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.87 | 31.86 | 5.94 | 4.86 | 4.32 | 3.19 | 3.57 | 4.99 | 3.57 | 2.62 | — | — |
| Mean distance, ‰ (H) | 0.267 | 1.112 | 0.286 | 0.272 | 0.223 | 0.181 | 0.322 | 0.422 | 0.322 | 0.156 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.4 | 9.5 | — | — | 17.0 | 21.9 | 20.0 | — | — | 22.9 | — | — |
| Edges on spiralling loops, % (L) | 11.3 | 20.3 | — | — | 14.5 | 13.1 | 5.2 | — | — | 7.8 | — | — |
| Spiralling loops (L) | 6 | 12 | — | — | 13 | 13 | 4 | — | — | 8 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 68.7 | 70.2 | — | — | 54.2 | 52.0 | 62.0 | — | — | 55.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 512 | 98 | — | — | 352 | 309 | 564 | — | — | 602 | — | — |
| Edges on loops from a border to a border, % (L) | 19.0 | 0.0 | — | — | 14.3 | 13.0 | 12.8 | — | — | 13.8 | — | — |
| Off budget, % (N) | 0.9 | 66.9 | 1.2 | 7.8 | 0.8 | 0.9 | 15.6 | 8.5 | 0.0 | 1.0 | — | — |
| Time, seconds (T) | 6.83 | 1.17 | 0.87 | 74.82 | 5.58 | 7.73 | 7.68 | 4.94 | 7.68 | 7.99 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7763 | 4622 | 7836 | 6967 | 8085 | 8038 | 12717 | 6761 | 12717 | 12825 | — | — |
| Irregular vertices (S) | 162 | 141 | 455 | 170 | 202 | 240 | 338 | 322 | 338 | 278 | — | — |
| … of which above the minimum | 154 | — | 429 | — | 194 | 232 | 330 | — | 330 | 270 | — | — |
| Base complex patches (C) | 7352 | — | 7656 | — | 5155 | 5300 | 9903 | — | 9903 | 8657 | — | — |
| Alignment, degrees (A) | 17.78 | 12.94 | 17.72 | 11.64 | 11.35 | 10.78 | 8.50 | 9.79 | 8.88 | 10.22 | — | — |
| Mean angle deviation, degrees (F) | 11.59 | 7.43 | 6.18 | 9.04 | 6.48 | 6.53 | 7.95 | 8.39 | 7.95 | 5.90 | — | — |
| Angle deviation, 95th percentile (F) | 33.33 | 22.75 | 20.84 | 24.13 | 17.93 | 18.11 | 22.55 | 22.69 | 22.55 | 16.36 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.812 | 1.225 | 1.401 | 1.405 | 1.387 | 2.025 | 1.482 | 2.025 | 1.386 | — | — |
| Area variation (F) | 0.312 | 0.668 | 0.196 | 0.284 | 0.353 | 0.454 | 0.643 | 0.364 | 0.643 | 0.437 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 27.28 | 27.93 | 30.74 | 26.72 | 11.07 | 8.62 | 5.23 | 25.57 | 5.23 | 5.13 | — | — |
| Mean distance, ‰ (H) | 0.521 | 1.054 | 0.601 | 0.408 | 0.319 | 0.301 | 0.346 | 0.548 | 0.346 | 0.205 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.1 | 78.5 | 35.7 | 24.3 | 13.2 | 3.4 | 7.4 | 18.6 | 9.1 | 2.5 | — | — |
| Volume kept, % of the source (V) | 99.49 | 98.66 | 99.41 | 99.93 | 100.00 | 100.01 | 99.59 | 99.39 | 99.59 | 100.01 | — | — |
| Edges on closed loops, % (L) | 2.0 | 17.3 | — | — | 19.0 | 17.0 | 11.1 | — | — | 16.2 | — | — |
| Edges on spiralling loops, % (L) | 13.7 | 18.1 | — | — | 7.3 | 8.1 | 16.8 | — | — | 9.9 | — | — |
| Spiralling loops (L) | 23 | 23 | — | — | 18 | 17 | 29 | — | — | 22 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 84.3 | 64.7 | — | — | 73.6 | 74.9 | 72.2 | — | — | 73.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 484 | 175 | — | — | 325 | 207 | 333 | — | — | 321 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.0 | 42.2 | 2.1 | 12.9 | 1.1 | 0.5 | 59.0 | 15.5 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 4.83 | 3.89 | 2.51 | 34.81 | 12.73 | 13.78 | 23.50 | 24.03 | 23.50 | 20.92 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2520 | 7652 | 7214 | 7974 | 8115 | 11084 | 7045 | 11084 | 11211 | 10590 | 10709 |
| Irregular vertices (S) | — | 156 | 400 | 128 | 156 | 231 | 268 | 201 | 268 | 249 | 476 | 257 |
| … of which above the minimum | — | — | — | — | 148 | 223 | 260 | — | 260 | 241 | — | 249 |
| Base complex patches (C) | — | — | 5799 | — | 5026 | 3081 | 4740 | — | 4740 | 5723 | — | 6523 |
| Alignment, degrees (A) | — | 10.84 | 11.76 | 7.87 | 8.00 | 8.01 | 7.12 | 7.16 | 7.58 | 8.25 | 9.88 | 8.32 |
| Mean angle deviation, degrees (F) | — | 8.90 | 6.09 | 5.84 | 4.27 | 4.22 | 6.54 | 6.43 | 6.54 | 5.03 | 9.24 | 4.33 |
| Angle deviation, 95th percentile (F) | — | 29.26 | 22.28 | 18.18 | 13.11 | 14.05 | 20.41 | 19.84 | 20.41 | 16.18 | 26.27 | 13.76 |
| Mean aspect ratio (F) | — | 1.933 | 1.235 | 1.342 | 1.346 | 1.376 | 1.668 | 1.355 | 1.668 | 1.312 | 1.616 | 1.312 |
| Area variation (F) | — | 0.738 | 0.213 | 0.305 | 0.342 | 0.473 | 0.538 | 0.311 | 0.538 | 0.472 | 1.417 | 0.461 |
| Hausdorff, ‰ of the diagonal (H) | — | 23.89 | 35.02 | 19.08 | 8.30 | 6.04 | 3.94 | 14.01 | 3.94 | 5.11 | 6.47 | 4.93 |
| Mean distance, ‰ (H) | — | 1.795 | 2.033 | 1.059 | 0.462 | 0.241 | 0.257 | 0.766 | 0.257 | 0.195 | 0.172 | 0.193 |
| Thin parts: mean distance, % of a quad edge (V) | — | 36.5 | 57.2 | 35.9 | 16.0 | 6.2 | 5.4 | 20.0 | 6.3 | 8.3 | 1.5 | 6.9 |
| Volume kept, % of the source (V) | — | — | — | 99.88 | 99.99 | 100.02 | 99.35 | 98.95 | 99.35 | 100.01 | 98.80 | 100.01 |
| Edges on closed loops, % (L) | — | — | 12.1 | 32.9 | 18.6 | 31.0 | 28.6 | 31.6 | 28.6 | 24.5 | 31.6 | 19.5 |
| Edges on spiralling loops, % (L) | — | — | 7.3 | 20.4 | 27.9 | 11.6 | 7.6 | 17.3 | 7.6 | 15.3 | 2.5 | 18.1 |
| Spiralling loops (L) | — | — | 37 | 44 | 38 | 15 | 21 | 55 | 21 | 17 | 3 | 18 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 79.3 | 46.6 | 53.5 | 57.4 | 63.7 | 51.1 | 63.7 | 60.3 | 65.8 | 62.4 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 180 | 312 | 305 | 368 | 492 | 252 | 492 | 462 | 274 | 521 |
| Edges on loops from a border to a border, % (L) | — | — | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.5 | 4.3 | 9.8 | 0.3 | 1.4 | 38.6 | 11.9 | 0.0 | 1.1 | 0.0 | 1.1 |
| Time, seconds (T) | — | 5.49 | 20.44 | 42.48 | 21.26 | 26.11 | 21.15 | 20.34 | 21.15 | 30.36 | — | 28.91 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2564 | 7648 | 7201 | 7977 | 7890 | 10897 | 7322 | 10897 | 10896 | 10590 | 10656 |
| Irregular vertices (S) | — | 150 | 414 | 140 | 170 | 257 | 280 | 233 | 280 | 278 | 476 | 278 |
| … of which above the minimum | — | — | 395 | — | 162 | 249 | 272 | — | 272 | 270 | — | 270 |
| Base complex patches (C) | — | — | 6412 | — | 3093 | 3036 | 4861 | — | 4861 | 4842 | — | 4118 |
| Alignment, degrees (A) | — | 9.84 | 11.62 | 7.96 | 7.67 | 7.06 | 6.16 | 6.66 | 6.55 | 7.32 | 9.43 | 7.47 |
| Mean angle deviation, degrees (F) | — | 8.28 | 6.50 | 6.48 | 4.74 | 4.17 | 6.14 | 6.42 | 6.14 | 4.64 | 8.82 | 4.18 |
| Angle deviation, 95th percentile (F) | — | 26.20 | 22.21 | 21.70 | 15.92 | 14.39 | 19.95 | 20.28 | 19.95 | 14.96 | 25.47 | 14.56 |
| Mean aspect ratio (F) | — | 1.962 | 1.237 | 1.317 | 1.404 | 1.286 | 1.689 | 1.358 | 1.689 | 1.288 | 1.657 | 1.277 |
| Area variation (F) | — | 0.794 | 0.217 | 0.330 | 0.424 | 0.499 | 0.535 | 0.327 | 0.535 | 0.467 | 1.432 | 0.488 |
| Hausdorff, ‰ of the diagonal (H) | — | 26.27 | 32.73 | 14.77 | 9.04 | 5.88 | 4.84 | 20.10 | 4.84 | 4.93 | 4.44 | 4.97 |
| Mean distance, ‰ (H) | — | 2.147 | 2.076 | 1.073 | 0.566 | 0.280 | 0.252 | 0.891 | 0.252 | 0.183 | 0.156 | 0.189 |
| Thin parts: mean distance, % of a quad edge (V) | — | 50.9 | 73.5 | 41.9 | 21.5 | 7.9 | 6.1 | 34.4 | 7.4 | 6.7 | 1.4 | 6.8 |
| Volume kept, % of the source (V) | — | — | 98.91 | 99.90 | 99.98 | 100.01 | 99.27 | 98.92 | 99.27 | 100.02 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | — | 8.3 | 23.0 | 30.6 | 30.8 | 27.7 | 25.6 | 27.7 | 27.8 | 31.6 | 30.7 |
| Edges on spiralling loops, % (L) | — | — | 10.6 | 27.5 | 7.1 | 6.6 | 3.2 | 17.2 | 3.2 | 6.4 | 2.5 | 4.0 |
| Spiralling loops (L) | — | — | 49 | 46 | 32 | 21 | 13 | 50 | 13 | 15 | 3 | 14 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 80.4 | 49.5 | 62.3 | 62.7 | 69.1 | 57.2 | 69.1 | 65.8 | 65.8 | 65.3 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 166 | 402 | 370 | 366 | 349 | 294 | 349 | 275 | 274 | 389 |
| Edges on loops from a border to a border, % (L) | — | — | 0.3 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.0 | 4.4 | 10.0 | 0.3 | 1.4 | 36.2 | 8.5 | 0.0 | 0.0 | 0.0 | 0.6 |
| Time, seconds (T) | — | 6.63 | 20.66 | 36.31 | 24.87 | 26.63 | 21.59 | 22.09 | 21.59 | 29.05 | — | 29.61 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7874 | 2895 | 8098 | 7084 | 7972 | 8068 | 7965 | 7274 | 7965 | 7982 | — | — |
| Irregular vertices (S) | 76 | 74 | 286 | 80 | 80 | 78 | 76 | 80 | 76 | 78 | — | — |
| … of which above the minimum | 68 | — | 278 | 72 | 72 | 70 | 68 | 72 | 68 | 70 | — | — |
| Base complex patches (C) | 7772 | — | 7942 | 3692 | 3860 | 4022 | 4605 | 3862 | 4605 | 3516 | — | — |
| Alignment, degrees (A) | 16.85 | 9.88 | 16.86 | 8.49 | 7.91 | 7.38 | 7.56 | 8.42 | 7.56 | 7.42 | — | — |
| Mean angle deviation, degrees (F) | 6.23 | 7.56 | 4.47 | 9.07 | 6.64 | 6.25 | 5.56 | 6.01 | 5.56 | 6.00 | — | — |
| Angle deviation, 95th percentile (F) | 16.10 | 20.17 | 14.93 | 23.35 | 17.05 | 16.70 | 14.90 | 15.81 | 14.90 | 15.20 | — | — |
| Mean aspect ratio (F) | 1.177 | 1.481 | 1.136 | 1.348 | 1.348 | 1.340 | 1.342 | 1.289 | 1.342 | 1.337 | — | — |
| Area variation (F) | 0.178 | 0.712 | 0.144 | 0.296 | 0.329 | 0.434 | 0.301 | 0.256 | 0.301 | 0.439 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.30 | 6.25 | 7.10 | 2.61 | 3.43 | 2.26 | 4.73 | 5.88 | 4.73 | 2.11 | — | — |
| Mean distance, ‰ (H) | 0.161 | 0.635 | 0.172 | 0.093 | 0.083 | 0.079 | 0.169 | 0.187 | 0.169 | 0.079 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 98.89 | 99.78 | 100.01 | 100.01 | 100.01 | 99.73 | 99.73 | 99.73 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.1 | 16.4 | — | — | 26.8 | 26.6 | 21.9 | — | — | 29.4 | — | — |
| Edges on spiralling loops, % (L) | 19.0 | 13.5 | — | — | 17.2 | 19.8 | 24.0 | — | — | 18.6 | — | — |
| Spiralling loops (L) | 9 | 7 | — | — | 9 | 9 | 10 | — | — | 8 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.3 | 70.1 | — | — | 56.0 | 53.6 | 54.1 | — | — | 52.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 578 | 134 | — | — | 464 | 426 | 349 | — | — | 338 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.6 | 63.8 | 1.2 | 11.5 | 0.4 | 0.9 | 0.4 | 9.1 | 0.0 | 0.2 | — | — |
| Time, seconds (T) | 3.72 | 2.50 | 1.23 | 39.51 | 10.31 | 13.45 | 17.71 | 17.39 | 17.71 | 12.58 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8263 | 4012 | 8072 | 7052 | 7952 | 7914 | 6896 | 6832 | 6896 | 6785 | — | — |
| Irregular vertices (S) | 36 | 20 | 245 | 18 | 20 | 20 | 34 | 30 | 34 | 20 | — | — |
| … of which above the minimum | 36 | 20 | 245 | — | 20 | 20 | 34 | 30 | 34 | 20 | — | — |
| Base complex patches (C) | 8263 | 1318 | 8072 | 1541 | 1711 | 1192 | 4859 | 3789 | 4859 | 1988 | — | — |
| Alignment, degrees (A) | 17.31 | 5.28 | 20.15 | 4.10 | 3.72 | 3.69 | 5.48 | 5.82 | 5.46 | 3.63 | — | — |
| Mean angle deviation, degrees (F) | 6.39 | 4.91 | 4.31 | 4.94 | 3.75 | 3.69 | 6.43 | 6.86 | 6.43 | 3.56 | — | — |
| Angle deviation, 95th percentile (F) | 17.71 | 13.45 | 14.25 | 16.54 | 11.94 | 11.44 | 18.71 | 21.48 | 18.71 | 10.73 | — | — |
| Mean aspect ratio (F) | 1.142 | 2.024 | 1.124 | 1.407 | 1.403 | 1.381 | 1.407 | 1.407 | 1.407 | 1.385 | — | — |
| Area variation (F) | 0.159 | 0.485 | 0.139 | 0.356 | 0.375 | 0.423 | 0.369 | 0.373 | 0.369 | 0.418 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.23 | 1.62 | 2.84 | 3.52 | 0.71 | 0.98 | 1.50 | 1.30 | 1.50 | 0.99 | — | — |
| Mean distance, ‰ (H) | 0.104 | 0.346 | 0.120 | 0.063 | 0.057 | 0.057 | 0.150 | 0.153 | 0.150 | 0.063 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.32 | 99.79 | — | 100.01 | 100.01 | 99.70 | 99.70 | 99.70 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 35.6 | — | — | 52.1 | 41.3 | 14.8 | — | — | 38.1 | — | — |
| Edges on spiralling loops, % (L) | 23.4 | 40.2 | — | — | 34.2 | 43.1 | 40.8 | — | — | 46.0 | — | — |
| Spiralling loops (L) | 7 | 10 | — | — | 5 | 5 | 12 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 76.6 | 24.2 | — | — | 13.7 | 15.6 | 44.4 | — | — | 16.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 840 | 228 | — | — | 199 | 320 | 848 | — | — | 298 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 49.9 | 0.9 | 11.8 | 0.6 | 1.1 | 13.8 | 14.6 | 0.0 | 1.6 | — | — |
| Time, seconds (T) | 3.56 | 2.33 | 1.37 | 16.12 | 9.13 | 8.10 | 20.38 | 16.98 | 20.38 | 6.98 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7229 | 3438 | 8100 | 7057 | 7961 | 7851 | 8513 | 6943 | 8513 | 8570 | — | — |
| Irregular vertices (S) | 70 | 53 | 280 | 48 | 48 | 56 | 51 | 48 | 51 | 56 | — | — |
| … of which above the minimum | 62 | — | 272 | 40 | 40 | 48 | 43 | 40 | 43 | 48 | — | — |
| Base complex patches (C) | 6275 | — | 7910 | 2997 | 1763 | 2265 | 6215 | 4909 | 6215 | 2012 | — | — |
| Alignment, degrees (A) | 15.23 | 7.01 | 14.61 | 7.16 | 6.88 | 6.38 | 6.32 | 7.75 | 6.41 | 6.52 | — | — |
| Mean angle deviation, degrees (F) | 10.30 | 6.30 | 4.50 | 8.89 | 6.48 | 5.90 | 6.44 | 7.53 | 6.44 | 5.78 | — | — |
| Angle deviation, 95th percentile (F) | 28.50 | 19.86 | 15.34 | 24.31 | 18.25 | 17.10 | 16.31 | 19.46 | 16.31 | 16.22 | — | — |
| Mean aspect ratio (F) | 1.267 | 2.320 | 1.139 | 1.332 | 1.338 | 1.360 | 1.595 | 1.351 | 1.595 | 1.358 | — | — |
| Area variation (F) | 0.371 | 0.598 | 0.144 | 0.270 | 0.305 | 0.416 | 0.365 | 0.270 | 0.365 | 0.406 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 21.90 | 8.37 | 5.99 | 4.33 | 2.80 | 2.43 | 4.29 | 8.04 | 4.29 | 3.14 | — | — |
| Mean distance, ‰ (H) | 0.213 | 0.399 | 0.132 | 0.085 | 0.070 | 0.060 | 0.128 | 0.176 | 0.128 | 0.058 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.18 | 99.71 | 99.99 | 100.00 | 100.00 | 99.74 | 99.61 | 99.74 | 100.00 | — | — |
| Edges on closed loops, % (L) | 6.4 | 28.0 | — | — | 51.5 | 42.0 | 13.5 | — | — | 45.1 | — | — |
| Edges on spiralling loops, % (L) | 20.2 | 18.6 | — | — | 23.6 | 27.0 | 45.9 | — | — | 14.7 | — | — |
| Spiralling loops (L) | 12 | 5 | — | — | 6 | 10 | 17 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 73.2 | 53.3 | — | — | 24.9 | 30.9 | 40.6 | — | — | 40.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 660 | 163 | — | — | 280 | 310 | 515 | — | — | 437 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 9.6 | 57.0 | 1.2 | 11.8 | 0.5 | 1.9 | 6.4 | 13.2 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 3.58 | 3.90 | 1.16 | 19.91 | 7.21 | 8.68 | 15.56 | 13.36 | 15.56 | 10.52 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6973 | 4682 | 8116 | 7187 | 7920 | 8148 | 9824 | 7070 | 9824 | 9969 | — | — |
| Irregular vertices (S) | 136 | 135 | 370 | 81 | 88 | 98 | 101 | 98 | 101 | 98 | — | — |
| … of which above the minimum | 128 | — | 362 | — | 80 | 90 | — | 90 | — | 90 | — | — |
| Base complex patches (C) | 6841 | — | 7826 | 4517 | 3452 | 4358 | — | 4284 | — | 5465 | — | — |
| Alignment, degrees (A) | 15.84 | 11.22 | 14.90 | 9.35 | 9.45 | 8.19 | 7.39 | 8.84 | 7.49 | 8.44 | — | — |
| Mean angle deviation, degrees (F) | 11.44 | 6.80 | 5.04 | 8.83 | 7.00 | 5.81 | 6.40 | 6.59 | 6.40 | 6.11 | — | — |
| Angle deviation, 95th percentile (F) | 34.02 | 21.80 | 17.35 | 25.06 | 20.00 | 17.30 | 18.75 | 19.80 | 18.75 | 18.40 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.773 | 1.161 | 1.356 | 1.363 | 1.321 | 1.546 | 1.329 | 1.546 | 1.339 | — | — |
| Area variation (F) | 0.383 | 0.668 | 0.161 | 0.307 | 0.334 | 0.497 | 0.462 | 0.260 | 0.462 | 0.493 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.23 | 9.84 | 21.49 | 7.72 | 7.37 | 6.61 | 9.11 | 9.71 | 9.11 | 6.82 | — | — |
| Mean distance, ‰ (H) | 0.610 | 0.627 | 0.238 | 0.164 | 0.146 | 0.120 | 0.222 | 0.301 | 0.222 | 0.099 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 64.0 | 21.7 | 11.6 | 16.5 | 16.0 | 4.3 | 3.7 | 15.8 | 4.9 | 4.4 | — | — |
| Volume kept, % of the source (V) | 99.23 | 99.18 | 99.70 | — | 99.99 | 100.00 | 99.74 | 99.55 | 99.74 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.5 | 9.6 | — | — | 31.6 | 25.3 | 24.7 | — | — | 23.4 | — | — |
| Edges on spiralling loops, % (L) | 21.6 | 22.2 | — | — | 8.6 | 17.3 | 15.3 | — | — | 21.8 | — | — |
| Spiralling loops (L) | 22 | 18 | — | — | 16 | 28 | 21 | — | — | 27 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.2 | 68.1 | — | — | 59.9 | 57.4 | 60.0 | — | — | 54.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 500 | 161 | — | — | 541 | 423 | 456 | — | — | 782 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 12.8 | 41.5 | 1.5 | 10.2 | 1.0 | 1.8 | 22.8 | 11.6 | 0.0 | 1.5 | — | — |
| Time, seconds (T) | 3.13 | 4.60 | 1.42 | 34.04 | 9.12 | 11.94 | 16.84 | 16.13 | 16.84 | 13.96 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7781 | 2740 | 7933 | 7345 | 7846 | 7951 | 13714 | 6947 | 13714 | 13817 | — | — |
| Irregular vertices (S) | 112 | 109 | 490 | 173 | 168 | 279 | 244 | 247 | 244 | 355 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 4991 | — | 7797 | 5208 | 5418 | 6820 | 11709 | — | 11709 | 11799 | — | — |
| Alignment, degrees (A) | 17.56 | 15.73 | 18.65 | 13.30 | 12.80 | 12.37 | 10.64 | 12.69 | 10.91 | 11.71 | — | — |
| Mean angle deviation, degrees (F) | 7.80 | 10.26 | 6.63 | 8.59 | 5.29 | 5.71 | 8.15 | 8.38 | 8.15 | 5.90 | — | — |
| Angle deviation, 95th percentile (F) | 22.58 | 29.98 | 23.27 | 23.17 | 14.86 | 16.65 | 22.38 | 23.18 | 22.38 | 15.73 | — | — |
| Mean aspect ratio (F) | 1.257 | 1.889 | 1.241 | 1.297 | 1.254 | 1.266 | 1.879 | 1.380 | 1.879 | 1.284 | — | — |
| Area variation (F) | 0.245 | 0.652 | 0.210 | 0.264 | 0.314 | 0.526 | 0.651 | 0.279 | 0.651 | 0.462 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 31.29 | 25.08 | 29.67 | 21.23 | 15.24 | 10.48 | 4.22 | 10.31 | 4.22 | 10.37 | — | — |
| Mean distance, ‰ (H) | 0.581 | 2.010 | 0.629 | 0.483 | 0.352 | 0.279 | 0.310 | 0.598 | 0.310 | 0.170 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.5 | 54.4 | 38.5 | 34.4 | 22.8 | 9.6 | 3.9 | 14.2 | 5.1 | 11.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 5.3 | 1.1 | 3.3 | — | — | 0.7 | — | — |
| Edges on spiralling loops, % (L) | 3.1 | — | — | — | 3.9 | 3.5 | 9.1 | — | — | 5.5 | — | — |
| Spiralling loops (L) | 5 | — | — | — | 6 | 8 | 17 | — | — | 15 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.6 | — | — | — | 79.3 | 89.2 | 83.4 | — | — | 86.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 404 | — | — | — | 197 | 276 | 467 | — | — | 365 | — | — |
| Edges on loops from a border to a border, % (L) | 19.8 | — | — | — | 11.4 | 6.2 | 4.2 | — | — | 7.0 | — | — |
| Off budget, % (N) | 2.7 | 65.8 | 0.8 | 8.2 | 1.9 | 0.6 | 71.4 | 13.2 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 6.02 | 2.44 | 0.92 | 36.84 | 7.27 | 10.46 | 9.49 | 8.46 | 9.49 | 19.81 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 6758 | 3828 | 6932 | 6584 | 8329 | 8368 | 53874 | 6896 | 53874 | 53782 | — | — |
| Irregular vertices (S) | 917 | 556 | 1126 | 570 | 1222 | 1643 | 1899 | 1164 | 1899 | 2544 | — | — |
| … of which above the minimum | — | — | — | — | 606 | 1027 | — | — | — | 1928 | — | — |
| Base complex patches (C) | 6524 | — | 6434 | — | 6335 | 6352 | — | — | — | 38476 | — | — |
| Alignment, degrees (A) | 18.21 | 18.28 | 18.49 | 14.12 | 12.62 | 12.51 | 9.05 | 12.40 | 11.32 | 9.30 | — | — |
| Mean angle deviation, degrees (F) | 14.81 | 17.84 | 12.06 | 10.22 | 11.83 | 13.45 | 8.61 | 11.61 | 8.61 | 6.37 | — | — |
| Angle deviation, 95th percentile (F) | 41.35 | 56.33 | 40.69 | 29.42 | 41.53 | 41.11 | 24.64 | 31.66 | 24.64 | 19.45 | — | — |
| Mean aspect ratio (F) | 1.515 | 3.415 | 1.595 | 1.412 | 2.367 | 2.136 | 1.839 | 1.518 | 1.839 | 1.370 | — | — |
| Area variation (F) | 0.407 | 0.904 | 0.359 | 0.322 | 0.624 | 1.422 | 1.220 | 0.433 | 1.220 | 0.482 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 74.30 | 58.87 | 72.22 | 73.55 | 16.29 | 15.48 | 73.68 | 75.13 | 73.68 | 7.95 | — | — |
| Mean distance, ‰ (H) | 1.489 | 4.497 | 1.750 | 1.267 | 0.674 | 0.899 | 0.689 | 1.593 | 0.689 | 0.113 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 44.1 | 83.6 | 47.5 | 37.1 | 13.1 | 11.7 | 22.2 | 42.9 | 159.7 | 10.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 97.37 | 95.67 | — | — | — | 99.78 | — | — |
| Edges on closed loops, % (L) | 0.4 | — | — | — | 12.5 | 12.4 | — | — | — | 14.6 | — | — |
| Edges on spiralling loops, % (L) | 17.0 | — | — | — | 25.8 | 20.2 | — | — | — | 15.7 | — | — |
| Spiralling loops (L) | 130 | — | — | — | 499 | 424 | — | — | — | 367 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.4 | — | — | — | 61.8 | 67.5 | — | — | — | 69.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | — | — | 125 | 48 | — | — | — | 405 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | 15.5 | 52.1 | 13.4 | 17.7 | 4.1 | 4.6 | 573.4 | 13.8 | 0.0 | 0.2 | — | — |
| Time, seconds (T) | 3.62 | 1.88 | 1.79 | 135.60 | 5.25 | 13.26 | 9.67 | 7.85 | 9.67 | 26.51 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4954 | 7960 | 7468 | 8032 | 7992 | 8805 | 7054 | 8805 | 8799 | — | — |
| Irregular vertices (S) | — | 58 | 258 | 68 | 70 | 81 | 82 | 79 | 82 | 89 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 5201 | 1643 | 2232 | 2367 | 2942 | — | 2942 | 3637 | — | — |
| Alignment, degrees (A) | — | 10.49 | 13.88 | 9.84 | 9.76 | 8.94 | 8.33 | 9.16 | 8.67 | 9.22 | — | — |
| Mean angle deviation, degrees (F) | — | 6.02 | 4.48 | 7.01 | 4.65 | 4.47 | 6.17 | 6.74 | 6.17 | 4.39 | — | — |
| Angle deviation, 95th percentile (F) | — | 18.67 | 14.92 | 19.83 | 12.32 | 11.93 | 17.33 | 19.99 | 17.33 | 12.50 | — | — |
| Mean aspect ratio (F) | — | 1.719 | 1.145 | 1.266 | 1.289 | 1.265 | 1.480 | 1.386 | 1.480 | 1.274 | — | — |
| Area variation (F) | — | 0.719 | 0.140 | 0.229 | 0.259 | 0.327 | 0.354 | 0.285 | 0.354 | 0.363 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 8.89 | 9.03 | 7.15 | 6.82 | 6.74 | 6.69 | 6.45 | 6.69 | 7.19 | — | — |
| Mean distance, ‰ (H) | — | 0.521 | 0.281 | 0.238 | 0.215 | 0.216 | 0.264 | 0.325 | 0.264 | 0.202 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 20.8 | — | — | 20.6 | 22.3 | 19.3 | — | — | 16.9 | — | — |
| Edges on spiralling loops, % (L) | — | 8.5 | — | — | 4.7 | 3.9 | 7.6 | — | — | 11.1 | — | — |
| Spiralling loops (L) | — | 5 | — | — | 5 | 5 | 8 | — | — | 7 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 53.1 | — | — | 50.0 | 50.9 | 51.4 | — | — | 53.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 154 | — | — | 185 | 224 | 183 | — | — | 201 | — | — |
| Edges on loops from a border to a border, % (L) | — | 17.6 | — | — | 24.6 | 22.9 | 21.7 | — | — | 18.9 | — | — |
| Off budget, % (N) | — | 38.1 | 0.5 | 6.7 | 0.4 | 0.1 | 10.1 | 11.8 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | — | 3.19 | 2.16 | 23.83 | 7.53 | 7.56 | 15.51 | 11.76 | 15.51 | 8.53 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 4954 | 7732 | 7824 | 8028 | 8098 | 7353 | 7353 | 7353 | 7279 | — | — |
| Irregular vertices (S) | 38 | 94 | 337 | 24 | 24 | 24 | 24 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 30 | — | 329 | 16 | 16 | 16 | 16 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 1171 | — | 4678 | 128 | 120 | 92 | 730 | 730 | 730 | 92 | — | — |
| Alignment, degrees (A) | 13.31 | 14.04 | 13.52 | 11.84 | 11.96 | 10.32 | 10.87 | 10.87 | 10.87 | 10.44 | — | — |
| Mean angle deviation, degrees (F) | 2.56 | 13.77 | 4.52 | 1.66 | 1.26 | 1.00 | 3.63 | 3.63 | 3.63 | 1.07 | — | — |
| Angle deviation, 95th percentile (F) | 9.55 | 47.48 | 18.21 | 7.02 | 3.86 | 3.80 | 14.81 | 14.81 | 14.81 | 4.16 | — | — |
| Mean aspect ratio (F) | 1.074 | 3.591 | 1.178 | 1.151 | 1.094 | 1.129 | 1.301 | 1.301 | 1.301 | 1.115 | — | — |
| Area variation (F) | 0.087 | 0.912 | 0.166 | 0.105 | 0.129 | 0.328 | 0.244 | 0.244 | 0.244 | 0.339 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.14 | 11.34 | 5.05 | 0.32 | 0.31 | 0.25 | 0.68 | 0.68 | 0.68 | 0.27 | — | — |
| Mean distance, ‰ (H) | 0.034 | 0.506 | 0.023 | 0.021 | 0.022 | 0.024 | 0.040 | 0.040 | 0.040 | 0.026 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.06 | — | 100.06 | 100.00 | 100.00 | 100.00 | 100.06 | 100.06 | 100.06 | 100.00 | — | — |
| Edges on closed loops, % (L) | 59.2 | — | — | — | 87.5 | 89.3 | 67.0 | — | — | 88.7 | — | — |
| Edges on spiralling loops, % (L) | 22.9 | — | — | — | 0.0 | 0.0 | 18.3 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 8 | — | — | — | 0 | 0 | 6 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 17.7 | — | — | — | 12.5 | 10.7 | 14.7 | — | — | 11.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 160 | — | — | — | 148 | 82 | 130 | — | — | 77 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 0.3 | 38.1 | 3.4 | 2.2 | 0.4 | 1.2 | 8.1 | 8.1 | 0.0 | 1.0 | — | — |
| Time, seconds (T) | 8.90 | 3.06 | 2.22 | 10.17 | 5.90 | 5.91 | 2.65 | 2.72 | 2.65 | 7.13 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7920 | 2774 | 7802 | 8028 | 8066 | 8066 | 7920 | 7920 | 7920 | 7776 | — | — |
| Irregular vertices (S) | 8 | 10 | 18 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | — | 376 | 8028 | 6 | 6 | 6 | 6 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.46 | 2.60 | 0.96 | 0.68 | 0.11 | 0.27 | 0.11 | 0.11 | 0.11 | 0.28 | — | — |
| Mean angle deviation, degrees (F) | 0.36 | 5.97 | 0.91 | 0.95 | 0.01 | 0.20 | 0.00 | 0.00 | 0.00 | 0.21 | — | — |
| Angle deviation, 95th percentile (F) | 1.18 | 24.98 | 2.60 | 2.23 | 0.02 | 0.74 | 0.00 | 0.00 | 0.00 | 0.76 | — | — |
| Mean aspect ratio (F) | 1.020 | 2.866 | 1.030 | 1.032 | 1.018 | 1.074 | 1.019 | 1.019 | 1.019 | 1.066 | — | — |
| Area variation (F) | 0.023 | 0.740 | 0.045 | 0.044 | 0.013 | 0.093 | 0.013 | 0.013 | 0.013 | 0.091 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.85 | 4.81 | 0.00 | 5.40 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.007 | 0.165 | 0.000 | 0.046 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.99 | 100.00 | 99.99 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 97.2 | 27.9 | — | — | 97.3 | 97.3 | 97.2 | — | — | 97.2 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 63.5 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 0 | 6 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 2.8 | 8.5 | — | — | 2.7 | 2.7 | 2.8 | — | — | 2.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 37 | 85 | — | — | 37 | 37 | 37 | — | — | 36 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.0 | 65.3 | 2.5 | 0.4 | 0.8 | 0.8 | 1.0 | 1.0 | 0.0 | 1.8 | — | — |
| Time, seconds (T) | 4.14 | 1.30 | 0.78 | 65.82 | 4.06 | 4.40 | 3.50 | 2.90 | 3.50 | 4.35 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8016 | 3035 | 8000 | 8064 | 8064 | 8064 | 8064 | 8064 | 8064 | 8064 | — | — |
| Irregular vertices (S) | 13 | 20 | 24 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | — | 16 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 402 | — | 706 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 0.86 | 3.67 | 1.17 | 0.57 | 0.61 | 0.92 | 0.54 | 0.54 | 0.54 | 0.92 | — | — |
| Mean angle deviation, degrees (F) | 0.40 | 6.33 | 0.88 | 0.01 | 0.00 | 0.49 | 0.00 | 0.00 | 0.00 | 0.49 | — | — |
| Angle deviation, 95th percentile (F) | 1.40 | 24.33 | 2.62 | 0.03 | 0.01 | 1.49 | 0.00 | 0.00 | 0.00 | 1.49 | — | — |
| Mean aspect ratio (F) | 1.018 | 2.553 | 1.025 | 1.000 | 1.000 | 1.078 | 1.000 | 1.000 | 1.000 | 1.078 | — | — |
| Area variation (F) | 0.024 | 0.668 | 0.041 | 0.000 | 0.000 | 0.096 | 0.000 | 0.000 | 0.000 | 0.096 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.72 | 5.67 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.005 | 0.155 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.97 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 82.3 | 50.2 | — | — | 95.8 | 95.8 | 95.8 | — | — | 95.8 | — | — |
| Edges on spiralling loops, % (L) | 11.3 | 27.9 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 1 | 5 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 6.1 | 22.0 | — | — | 4.2 | 4.2 | 4.2 | — | — | 4.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 132 | 134 | — | — | 72 | 72 | 72 | — | — | 72 | — | — |
| Edges on loops from a border to a border, % (L) | 0.3 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 0.6 | 62.1 | 0.1 | 0.8 | 0.8 | 0.8 | 0.8 | 0.8 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 3.24 | 1.54 | 0.34 | 5.37 | 4.57 | 5.08 | 3.10 | 3.18 | 3.10 | 4.19 | — | — |

Failures: quadriflow: timed out after 1800 s.

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7916 | 2565 | 8024 | 7276 | 7980 | 7899 | 6922 | 6922 | 6922 | 7007 | — | — |
| Irregular vertices (S) | 8 | 10 | 165 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 157 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 2635 | — | 4940 | 1298 | 1452 | 1399 | 850 | 850 | 850 | 1247 | — | — |
| Alignment, degrees (A) | 4.37 | 4.10 | 3.47 | 1.33 | 1.32 | 1.28 | 1.73 | 1.73 | 1.73 | 1.25 | — | — |
| Mean angle deviation, degrees (F) | 3.95 | 6.60 | 3.05 | 2.55 | 2.31 | 2.33 | 4.41 | 4.41 | 4.41 | 2.23 | — | — |
| Angle deviation, 95th percentile (F) | 16.68 | 20.66 | 11.70 | 13.62 | 10.22 | 10.46 | 18.99 | 18.99 | 18.99 | 9.61 | — | — |
| Mean aspect ratio (F) | 1.075 | 2.021 | 1.097 | 1.285 | 1.288 | 1.398 | 1.349 | 1.349 | 1.349 | 1.369 | — | — |
| Area variation (F) | 0.096 | 0.636 | 0.115 | 0.284 | 0.296 | 0.213 | 0.322 | 0.322 | 0.322 | 0.191 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.80 | 5.39 | 2.07 | 0.55 | 0.54 | 0.55 | 0.73 | 0.73 | 0.73 | 0.58 | — | — |
| Mean distance, ‰ (H) | 0.052 | 0.289 | 0.045 | 0.071 | 0.066 | 0.072 | 0.147 | 0.147 | 0.147 | 0.075 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.96 | 99.70 | 99.96 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 | — | — |
| Edges on closed loops, % (L) | 33.3 | 29.3 | — | — | 40.9 | 41.1 | 57.3 | — | — | 41.1 | — | — |
| Edges on spiralling loops, % (L) | 48.3 | 33.0 | — | — | 56.3 | 56.0 | 34.4 | — | — | 56.7 | — | — |
| Spiralling loops (L) | 5 | 3 | — | — | 2 | 2 | 4 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 18.6 | 37.7 | — | — | 2.8 | 2.8 | 8.3 | — | — | 2.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 762 | 316 | — | — | 91 | 91 | 205 | — | — | 61 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.1 | 67.9 | 0.5 | 9.0 | 0.2 | 1.3 | 13.5 | 13.5 | 0.0 | 1.2 | — | — |
| Time, seconds (T) | 4.60 | 1.04 | 0.57 | 7.70 | 4.67 | 5.43 | 4.42 | 4.47 | 4.42 | 4.50 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 7212 | 7693 | 6950 | 7585 | 8376 | 19704 | 14699 | 19704 | 19042 | — | — |
| Irregular vertices (S) | — | 2158 | 977 | 270 | 283 | 884 | 695 | 691 | 695 | 736 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4255 | — | 5209 | 6123 | — | — | — | 14283 | — | — |
| Alignment, degrees (A) | — | 10.19 | 11.23 | 10.34 | 10.31 | 11.72 | 7.51 | 7.00 | 8.22 | 9.61 | — | — |
| Mean angle deviation, degrees (F) | — | 40.97 | 10.36 | 6.59 | 6.39 | 8.83 | 8.05 | 8.32 | 8.05 | 5.77 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.08 | 43.42 | 21.59 | 21.48 | 32.44 | 34.33 | 41.53 | 34.33 | 19.45 | — | — |
| Mean aspect ratio (F) | — | 9.234 | 1.775 | 1.305 | 1.297 | 1.864 | 2.096 | 2.216 | 2.096 | 1.338 | — | — |
| Area variation (F) | — | 2.203 | 0.455 | 0.267 | 0.300 | 0.436 | 1.067 | 0.959 | 1.067 | 0.297 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 37.63 | 24.82 | 35.65 | 36.17 | 36.17 | 14.05 | 15.28 | 14.05 | 15.92 | — | — |
| Mean distance, ‰ (H) | — | 2.866 | 1.379 | 2.882 | 2.144 | 2.025 | 0.241 | 0.383 | 0.241 | 0.872 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 28.8 | 18.1 | 40.3 | 28.3 | 26.0 | 1.8 | 2.8 | 2.5 | 25.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 16.4 | 14.6 | 6.4 | — | — | 12.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 28.4 | 22.6 | 1.5 | — | — | 38.1 | — | — |
| Spiralling loops (L) | — | — | — | — | 40 | 31 | 12 | — | — | 54 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 55.1 | 62.9 | 27.6 | — | — | 48.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 163 | 163 | 132 | — | — | 492 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 64.6 | — | — | 1.6 | — | — |
| Off budget, % (N) | — | 9.8 | 3.8 | 13.1 | 5.2 | 4.7 | 146.3 | 83.7 | 0.0 | 3.4 | — | — |
| Time, seconds (T) | — | 1.84 | 1.45 | 87.87 | 51.45 | 36.35 | 5.17 | 5.88 | 5.17 | 100.08 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4422 | 7700 | 6805 | 7960 | 7952 | 13801 | 6640 | 13801 | 13746 | — | — |
| Irregular vertices (S) | — | 172 | 442 | 163 | 198 | 354 | 368 | 344 | 368 | 432 | — | — |
| … of which above the minimum | — | — | 446 | — | 190 | 346 | 360 | — | 360 | 424 | — | — |
| Base complex patches (C) | — | — | 7440 | — | 5886 | 6122 | 10719 | — | 10719 | 11058 | — | — |
| Alignment, degrees (A) | — | 13.99 | 17.01 | 12.30 | 11.97 | 10.05 | 8.76 | 10.53 | 9.13 | 9.73 | — | — |
| Mean angle deviation, degrees (F) | — | 7.16 | 6.55 | 8.73 | 6.31 | 6.61 | 7.98 | 8.73 | 7.98 | 6.45 | — | — |
| Angle deviation, 95th percentile (F) | — | 21.73 | 21.36 | 23.56 | 16.96 | 19.28 | 22.44 | 23.83 | 22.44 | 18.68 | — | — |
| Mean aspect ratio (F) | — | 1.812 | 1.226 | 1.417 | 1.397 | 1.381 | 2.045 | 1.480 | 2.045 | 1.373 | — | — |
| Area variation (F) | — | 0.657 | 0.198 | 0.304 | 0.349 | 0.968 | 0.686 | 0.354 | 0.686 | 0.989 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 22.79 | 30.78 | 25.14 | 10.19 | 7.19 | 7.42 | 30.01 | 7.42 | 5.56 | — | — |
| Mean distance, ‰ (H) | — | 1.109 | 0.706 | 0.441 | 0.324 | 0.336 | 0.321 | 0.572 | 0.321 | 0.216 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 44.3 | 48.7 | 16.3 | 11.0 | 1.4 | 3.6 | 15.6 | 5.9 | 1.5 | — | — |
| Volume kept, % of the source (V) | — | — | 99.39 | 99.93 | 100.00 | 100.01 | 99.56 | 99.30 | 99.56 | 100.01 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 13.1 | 11.6 | 11.3 | — | — | 9.9 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 11.6 | 10.9 | 12.1 | — | — | 14.1 | — | — |
| Spiralling loops (L) | — | — | — | — | 23 | 26 | 29 | — | — | 43 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 75.3 | 77.5 | 76.7 | — | — | 76.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 347 | 173 | 313 | — | — | 188 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 44.7 | 3.8 | 14.9 | 0.5 | 0.6 | 72.5 | 17.0 | 0.0 | 0.4 | — | — |
| Time, seconds (T) | — | 17.03 | 82.86 | 80.09 | 59.80 | 62.48 | 44.60 | 45.61 | 44.60 | 75.16 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 7305 | 2507 | — | — | 6743 | 6739 | 42790 | — | — | — | — | — |
| Irregular vertices (S) | 642 | 428 | — | — | 423 | 423 | 1504 | — | — | — | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6985 | — | — | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 13.37 | 16.69 | — | — | 12.95 | 12.99 | 10.22 | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 9.18 | 21.18 | — | — | 8.09 | 8.20 | 7.41 | — | — | — | — | — |
| Angle deviation, 95th percentile (F) | 32.10 | 68.70 | — | — | 28.49 | 29.38 | 22.87 | — | — | — | — | — |
| Mean aspect ratio (F) | 1.331 | 2.614 | — | — | 1.332 | 1.340 | 1.878 | — | — | — | — | — |
| Area variation (F) | 0.268 | 0.825 | — | — | 0.277 | 0.277 | 0.822 | — | — | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 50.24 | 90.14 | — | — | 53.45 | 53.45 | 30.04 | — | — | — | — | — |
| Mean distance, ‰ (H) | 1.990 | 7.482 | — | — | 2.383 | 2.383 | 0.641 | — | — | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.3 | 69.7 | — | — | 32.9 | 32.9 | 13.0 | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.3 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 12.4 | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | 125 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.2 | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 151 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 8.7 | 68.7 | — | — | 15.7 | 15.8 | 434.9 | — | — | — | — | — |
| Time, seconds (T) | 7.40 | 4.79 | — | — | 90.94 | 89.80 | 261.56 | — | — | — | — | — |

Failures: openclare-matched: timed out after 1800 s.

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 7180 | 2664 | — | — | 6213 | — | 35806 | — | — | 32245 | — | — |
| Irregular vertices (S) | 624 | 292 | — | — | 295 | — | 1135 | — | — | 530 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6952 | — | — | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 11.45 | 14.32 | — | — | 11.00 | — | 10.17 | — | — | 11.41 | — | — |
| Mean angle deviation, degrees (F) | 9.29 | 18.29 | — | — | 8.57 | — | 7.80 | — | — | 5.55 | — | — |
| Angle deviation, 95th percentile (F) | 29.68 | 60.76 | — | — | 28.53 | — | 23.07 | — | — | 18.29 | — | — |
| Mean aspect ratio (F) | 1.309 | 2.412 | — | — | 1.381 | — | 2.088 | — | — | 1.216 | — | — |
| Area variation (F) | 0.258 | 0.707 | — | — | 0.314 | — | 0.757 | — | — | 0.210 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 34.62 | 60.39 | — | — | 38.98 | — | 15.92 | — | — | 21.21 | — | — |
| Mean distance, ‰ (H) | 1.468 | 4.925 | — | — | 2.188 | — | 0.833 | — | — | 0.572 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.1 | 39.9 | — | — | 27.5 | — | 11.4 | — | — | 28.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | 98.95 | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.3 | — | — | — | — | — | 2.1 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 18.5 | — | — | — | — | — | 15.7 | — | — | — | — | — |
| Spiralling loops (L) | 50 | — | — | — | — | — | 91 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.1 | — | — | — | — | — | 82.3 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 117 | — | — | — | — | — | 229 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | — | — | 0.0 | — | — | — | — | — |
| Off budget, % (N) | 10.2 | 66.7 | — | — | 22.3 | — | 347.6 | — | — | 9.9 | — | — |
| Time, seconds (T) | 7.17 | 4.13 | — | — | 92.10 | — | 321.94 | — | — | 189.47 | — | — |

Failures: openclare: timed out after 1800 s.

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 0/0 | 0/0 | 1/1 | 1/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 6712 | 4327 | — | — | 7867 | 7998 | 33780 | — | — | 34237 | — | — |
| Irregular vertices (S) | 400 | 121 | — | — | 190 | 280 | 963 | — | — | 420 | — | — |
| … of which above the minimum | 341 | — | — | — | 126 | 216 | 899 | — | — | 356 | — | — |
| Base complex patches (C) | 6543 | — | — | — | 6331 | 6653 | 31433 | — | — | 28345 | — | — |
| Alignment, degrees (A) | 13.29 | 13.65 | — | — | 11.27 | 11.45 | 11.22 | — | — | 11.84 | — | — |
| Mean angle deviation, degrees (F) | 9.33 | 10.72 | — | — | 4.40 | 5.58 | 7.11 | — | — | 4.71 | — | — |
| Angle deviation, 95th percentile (F) | 30.16 | 31.96 | — | — | 14.37 | 17.24 | 20.64 | — | — | 13.80 | — | — |
| Mean aspect ratio (F) | 1.293 | 1.880 | — | — | 1.196 | 1.272 | 1.963 | — | — | 1.191 | — | — |
| Area variation (F) | 0.268 | 0.625 | — | — | 0.215 | 0.355 | 0.555 | — | — | 0.267 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.23 | 58.87 | — | — | 11.27 | 14.15 | 10.81 | — | — | 7.85 | — | — |
| Mean distance, ‰ (H) | 1.190 | 2.121 | — | — | 1.074 | 1.028 | 0.585 | — | — | 0.354 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.8 | 185.8 | — | — | 22.9 | 23.0 | 12.3 | — | — | 10.9 | — | — |
| Volume kept, % of the source (V) | 99.56 | 99.45 | — | — | 100.01 | 100.03 | 99.37 | — | — | 100.01 | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.8 | — | — | 10.2 | 9.1 | 3.7 | — | — | 9.5 | — | — |
| Edges on spiralling loops, % (L) | 8.9 | 24.3 | — | — | 19.2 | 7.8 | 6.8 | — | — | 9.6 | — | — |
| Spiralling loops (L) | 33 | 25 | — | — | 27 | 15 | 40 | — | — | 28 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.0 | 74.8 | — | — | 70.6 | 83.2 | 89.5 | — | — | 80.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 149 | 168 | — | — | 192 | 295 | 340 | — | — | 794 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 16.1 | 45.9 | — | — | 1.7 | 0.0 | 322.2 | — | — | 1.4 | — | — |
| Time, seconds (T) | 6.75 | 3.06 | — | — | 7.76 | 10.35 | 133.40 | — | — | 19.89 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 1/1 | 0/1 | 0/0 | 0/0 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 2364 | — | — | 249 | 439 | 36766 | — | — | 1196 | — | — |
| Irregular vertices (S) | — | 234 | — | — | 36 | 116 | 1245 | — | — | 220 | — | — |
| … of which above the minimum | — | — | — | — | 12 | 92 | — | — | — | 196 | — | — |
| Base complex patches (C) | — | — | — | — | 225 | 439 | — | — | — | 1104 | — | — |
| Alignment, degrees (A) | — | 13.30 | — | — | 16.30 | 18.87 | 9.54 | — | — | 13.20 | — | — |
| Mean angle deviation, degrees (F) | — | 14.00 | — | — | 29.46 | 21.80 | 7.28 | — | — | 12.04 | — | — |
| Angle deviation, 95th percentile (F) | — | 51.31 | — | — | 66.48 | 56.93 | 22.98 | — | — | 37.95 | — | — |
| Mean aspect ratio (F) | — | 2.278 | — | — | 6.553 | 2.886 | 1.983 | — | — | 7.813 | — | — |
| Area variation (F) | — | 0.698 | — | — | 1.174 | 0.901 | 0.768 | — | — | 0.827 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 93.86 | — | — | 531.90 | 529.94 | 11.92 | — | — | 529.52 | — | — |
| Mean distance, ‰ (H) | — | 6.515 | — | — | 248.343 | 232.732 | 0.466 | — | — | 195.204 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 80.5 | — | — | 1874.7 | 1842.0 | 9.0 | — | — | 3762.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 0.84 | 1.58 | — | — | — | 1.88 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 4.8 | 0.0 | — | — | — | 3.8 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 0.0 | 8.4 | — | — | — | 18.1 | — | — |
| Spiralling loops (L) | — | — | — | — | 0 | 9 | — | — | — | 11 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 95.2 | 91.6 | — | — | — | 78.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 25 | 16 | — | — | — | 28 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 70.5 | — | — | 96.9 | 94.5 | 359.6 | — | — | 96.7 | — | — |
| Time, seconds (T) | — | 3.34 | — | — | 2.69 | 5.66 | 181.10 | — | — | 6.74 | — | — |

Failures: quadriflow: cancelled.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6963 | 2975 | 4807 | 6773 | 8016 | 7918 | 11819 | 7183 | 11819 | 11855 | — | — |
| Irregular vertices (S) | 188 | 108 | 273 | 124 | 124 | 229 | 296 | 253 | 296 | 315 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2067 | — | 3587 | 4296 | 5005 | 6841 | 8228 | 4351 | 8228 | 10547 | — | — |
| Alignment, degrees (A) | 21.04 | 20.95 | 21.43 | 17.68 | 17.68 | 17.57 | 18.10 | 18.39 | 18.33 | 17.46 | — | — |
| Mean angle deviation, degrees (F) | 5.79 | 7.30 | 7.45 | 10.25 | 6.72 | 7.14 | 8.69 | 7.98 | 8.69 | 6.37 | — | — |
| Angle deviation, 95th percentile (F) | 21.42 | 26.48 | 25.48 | 26.71 | 18.40 | 19.54 | 25.15 | 22.11 | 25.15 | 16.76 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.481 | 1.264 | 1.320 | 1.299 | 1.309 | 1.668 | 1.342 | 1.668 | 1.259 | — | — |
| Area variation (F) | 0.185 | 0.463 | 0.215 | 0.287 | 0.326 | 0.495 | 0.518 | 0.237 | 0.518 | 0.499 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 16.98 | 26.34 | 130.45 | 15.44 | 11.35 | 8.78 | 10.70 | 16.84 | 10.70 | 8.30 | — | — |
| Mean distance, ‰ (H) | 1.071 | 1.562 | 1.477 | 0.991 | 0.744 | 0.626 | 0.719 | 1.071 | 0.719 | 0.503 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 34.0 | 47.6 | 41.7 | 38.6 | 23.4 | 17.7 | 20.8 | 34.8 | 27.4 | 19.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 1.6 | 2.1 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.3 | — | — | 3.2 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 0 | 1 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.9 | 71.1 | — | — | 78.5 | 92.9 | 82.6 | — | — | 91.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 55 | — | — | 264 | 138 | 166 | — | — | 237 | — | — |
| Edges on loops from a border to a border, % (L) | 46.1 | 28.9 | — | — | 19.9 | 5.0 | 17.1 | — | — | 5.8 | — | — |
| Off budget, % (N) | 13.0 | 62.8 | 39.9 | 15.3 | 0.2 | 1.0 | 47.7 | 10.2 | 0.0 | 0.3 | — | — |
| Time, seconds (T) | 19.08 | 4.25 | 10.05 | 45.81 | 11.62 | 14.86 | 24.96 | 24.23 | 24.96 | 18.70 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2996 | 5345 | 6148 | 6472 | 7885 | 55181 | 8791 | 55181 | 53358 | — | — |
| Irregular vertices (S) | — | 419 | 420 | 166 | 533 | 567 | 3155 | 1146 | 3155 | 1198 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4008 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.27 | 9.69 | 6.08 | 4.91 | 4.87 | 11.56 | 8.99 | 12.11 | 5.05 | — | — |
| Mean angle deviation, degrees (F) | — | 19.93 | 8.33 | 7.42 | 9.89 | 8.28 | 11.13 | 8.88 | 11.13 | 5.71 | — | — |
| Angle deviation, 95th percentile (F) | — | 77.95 | 32.25 | 24.85 | 64.66 | 51.97 | 36.88 | 32.39 | 36.88 | 20.97 | — | — |
| Mean aspect ratio (F) | — | 3.957 | 1.365 | 1.551 | 54.906 | 64.875 | 1.978 | 1.569 | 1.978 | 7.264 | — | — |
| Area variation (F) | — | 1.337 | 0.299 | 0.358 | 3.121 | 1.751 | 1.998 | 0.626 | 1.998 | 0.917 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.28 | 32.21 | 14.95 | 32.27 | 27.00 | 13.01 | 18.76 | 13.01 | 13.39 | — | — |
| Mean distance, ‰ (H) | — | 3.494 | 3.712 | 2.684 | 2.389 | 2.011 | 0.784 | 1.567 | 0.784 | 0.609 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.7 | 60.8 | 45.3 | 34.4 | 31.1 | 17.5 | 26.0 | 49.4 | 41.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 30.8 | 45.3 | — | — | — | 22.6 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 45.3 | 29.2 | — | — | — | 22.3 | — | — |
| Spiralling loops (L) | — | — | — | — | 530 | 560 | — | — | — | 652 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 14.8 | 20.7 | — | — | — | 38.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 29 | 136 | — | — | — | 830 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 9.1 | 4.8 | — | — | — | 16.8 | — | — |
| Off budget, % (N) | — | 62.5 | 33.2 | 23.2 | 19.1 | 1.4 | 589.8 | 9.9 | 0.0 | 3.3 | — | — |
| Time, seconds (T) | — | 2.97 | 6.80 | 72.10 | 8.37 | 8.18 | 13.92 | 11.71 | 13.92 | 51.06 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2388 | 4146 | — | 12358 | 11983 | 246063 | 14915 | 246063 | 231974 | — | — |
| Irregular vertices (S) | — | 1056 | 1524 | — | 6142 | 6226 | 10419 | 5525 | 10419 | 11799 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 3685 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 15.08 | 17.92 | — | 11.75 | 11.89 | 10.30 | 13.42 | 11.42 | 8.36 | — | — |
| Mean angle deviation, degrees (F) | — | 28.51 | 20.55 | — | 17.39 | 17.89 | 8.58 | 15.13 | 8.58 | 4.89 | — | — |
| Angle deviation, 95th percentile (F) | — | 82.76 | 58.40 | — | 62.96 | 63.64 | 25.88 | 47.93 | 25.88 | 18.22 | — | — |
| Mean aspect ratio (F) | — | 7.658 | 3.745 | — | 6.374 | 3.791 | 1.549 | 1.767 | 1.549 | 1.329 | — | — |
| Area variation (F) | — | 1.726 | 0.689 | — | 2.337 | 2.425 | 1.812 | 1.232 | 1.812 | 0.533 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 6.92 | 6.21 | — | 3.85 | 3.85 | 2.08 | 2.57 | 2.08 | 3.02 | — | — |
| Mean distance, ‰ (H) | — | 1.765 | 1.017 | — | 0.309 | 0.330 | 0.091 | 0.324 | 0.091 | 0.033 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 94.0 | 58.2 | — | 15.2 | 16.1 | 10.1 | 19.7 | 52.2 | 19.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 16.0 | 15.7 | — | — | — | 35.6 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 27.4 | 24.9 | — | — | — | 14.7 | — | — |
| Spiralling loops (L) | — | — | — | — | 4008 | 3980 | — | — | — | 1379 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 55.5 | 59.0 | — | — | — | 43.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 36 | 36 | — | — | — | 554 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 1.1 | 0.4 | — | — | — | 6.6 | — | — |
| Off budget, % (N) | — | 70.2 | 48.2 | — | 54.5 | 49.8 | 2975.8 | 86.4 | 0.0 | 5.7 | — | — |
| Time, seconds (T) | — | 1.13 | 4.87 | — | 12.31 | 18.84 | 13.22 | 13.40 | 13.22 | 54.37 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 10 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 6227 | 4548 | 6707 | 9460 | 9509 | 46194 | 10673 | 46194 | 43856 | — | — |
| Irregular vertices (S) | — | 2171 | 863 | 645 | 1840 | 1786 | 2662 | 1428 | 2662 | 2032 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 3548 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 10.29 | 8.12 | 5.58 | 5.51 | 4.87 | 4.69 | 5.48 | 4.75 | 3.96 | — | — |
| Mean angle deviation, degrees (F) | — | 29.31 | 14.41 | 9.45 | 11.57 | 10.65 | 9.22 | 10.50 | 9.22 | 6.20 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.69 | 54.18 | 32.95 | 59.20 | 59.46 | 33.32 | 37.63 | 33.32 | 23.11 | — | — |
| Mean aspect ratio (F) | — | 6.307 | 1.909 | 1.788 | 56.955 | 54.850 | 2.322 | 2.777 | 2.322 | 16.472 | — | — |
| Area variation (F) | — | 1.369 | 0.470 | 0.391 | 1.680 | 1.827 | 1.514 | 0.867 | 1.514 | 0.766 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 80.26 | 73.99 | 36.01 | 18.18 | 23.76 | 6.30 | 42.51 | 6.30 | 12.17 | — | — |
| Mean distance, ‰ (H) | — | 5.085 | 10.384 | 2.811 | 0.637 | 0.641 | 0.257 | 1.405 | 0.257 | 0.214 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 60.6 | 128.6 | 37.0 | 7.4 | 7.8 | 4.1 | 18.5 | 9.1 | 8.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 22.2 | 43.1 | 16.2 | 18.2 | 18.9 | 477.4 | 33.4 | 0.0 | 5.1 | — | — |
| Time, seconds (T) | — | 3.57 | 5.80 | 285.92 | 10.58 | 11.30 | 10.81 | 9.70 | 10.81 | 85.70 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4114 | 6839 | 7017 | 7801 | 8051 | 20326 | 10881 | 20326 | 19734 | — | — |
| Irregular vertices (S) | — | 403 | 532 | 170 | 223 | 257 | 458 | 360 | 458 | 295 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4440 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.73 | 6.43 | 3.94 | 3.30 | 3.42 | 3.48 | 3.17 | 3.48 | 3.31 | — | — |
| Mean angle deviation, degrees (F) | — | 18.87 | 9.46 | 5.96 | 13.20 | 13.97 | 5.75 | 5.64 | 5.75 | 8.39 | — | — |
| Angle deviation, 95th percentile (F) | — | 74.17 | 38.06 | 23.18 | 71.97 | 72.26 | 21.00 | 22.13 | 21.00 | 47.61 | — | — |
| Mean aspect ratio (F) | — | 3.158 | 1.596 | 1.823 | 22.891 | 16.237 | 6.194 | 9.720 | 6.194 | 5.441 | — | — |
| Area variation (F) | — | 1.607 | 0.398 | 0.330 | 1.027 | 1.096 | 1.143 | 0.765 | 1.143 | 0.761 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 58.33 | 36.49 | 36.30 | 32.96 | 32.96 | 7.91 | 17.13 | 7.91 | 20.02 | — | — |
| Mean distance, ‰ (H) | — | 3.579 | 1.944 | 1.189 | 1.026 | 0.990 | 0.334 | 0.577 | 0.334 | 0.342 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 32.4 | 21.3 | 11.1 | 11.2 | 11.0 | 2.8 | 4.7 | 3.9 | 9.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | 31.7 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | 8.2 | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | 17 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | 33.6 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | 664 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | 26.5 | — | — | — | — | — |
| Off budget, % (N) | — | 48.6 | 14.5 | 12.3 | 2.5 | 0.6 | 154.1 | 36.0 | 0.0 | 2.9 | — | — |
| Time, seconds (T) | — | 1.11 | 2.76 | 146.04 | 10.18 | 11.06 | 7.25 | 6.14 | 7.25 | 22.27 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2119 | 7656 | 6579 | 6760 | 6901 | 28909 | 6170 | 28909 | 25235 | — | — |
| Irregular vertices (S) | — | 195 | 835 | 380 | 417 | 457 | 961 | 603 | 961 | 727 | — | — |
| … of which above the minimum | — | — | 730 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 7378 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 13.19 | 15.59 | 10.07 | 10.31 | 11.06 | 9.87 | 10.45 | 11.04 | 11.35 | — | — |
| Mean angle deviation, degrees (F) | — | 10.86 | 8.99 | 9.19 | 9.34 | 9.86 | 9.44 | 10.16 | 9.44 | 8.62 | — | — |
| Angle deviation, 95th percentile (F) | — | 32.41 | 30.80 | 26.78 | 26.93 | 28.71 | 27.87 | 30.40 | 27.87 | 25.33 | — | — |
| Mean aspect ratio (F) | — | 1.955 | 1.370 | 1.416 | 1.458 | 1.466 | 1.826 | 1.547 | 1.826 | 1.343 | — | — |
| Area variation (F) | — | 0.849 | 0.275 | 0.321 | 0.311 | 0.344 | 0.850 | 0.373 | 0.850 | 0.305 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.12 | 31.17 | 13.40 | 12.56 | 12.56 | 9.82 | 39.39 | 9.82 | 8.60 | — | — |
| Mean distance, ‰ (H) | — | 3.139 | 1.125 | 0.922 | 0.688 | 0.677 | 0.330 | 1.288 | 0.330 | 0.206 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 103.8 | 74.2 | 59.6 | 34.0 | 33.4 | 20.6 | 67.3 | 46.3 | 39.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 12.3 | — | — | — | — | — | — | — | 16.0 | — | — |
| Edges on spiralling loops, % (L) | — | 3.9 | — | — | — | — | — | — | — | 20.5 | — | — |
| Spiralling loops (L) | — | 9 | — | — | — | — | — | — | — | 79 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 83.8 | — | — | — | — | — | — | — | 63.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 57 | — | — | — | — | — | — | — | 616 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | — | — | — | — | — | 0.1 | — | — |
| Off budget, % (N) | — | 73.5 | 4.3 | 17.8 | 15.5 | 13.7 | 261.4 | 22.9 | 0.0 | 12.7 | — | — |
| Time, seconds (T) | — | 2.60 | 1.15 | 117.14 | 131.90 | 131.07 | 41.01 | 41.67 | 41.01 | 312.59 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7736 | 6520 | 8190 | 7568 | 7984 | 8074 | 7088 | 7088 | 7088 | 6992 | — | — |
| Irregular vertices (S) | 12 | 26 | 193 | 26 | 26 | 26 | 8 | 8 | 8 | 18 | — | — |
| … of which above the minimum | 4 | 18 | 185 | 18 | 18 | 18 | 0 | 0 | 0 | 10 | — | — |
| Base complex patches (C) | 7729 | 6252 | 8190 | 7568 | 7984 | 8074 | 4992 | 4992 | 4992 | 5824 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.93 | 5.85 | 3.49 | 6.08 | 4.34 | 4.52 | 7.65 | 7.65 | 7.65 | 5.53 | — | — |
| Angle deviation, 95th percentile (F) | 12.52 | 14.46 | 11.93 | 13.87 | 9.72 | 9.91 | 18.00 | 18.00 | 18.00 | 12.58 | — | — |
| Mean aspect ratio (F) | 1.109 | 1.337 | 1.097 | 1.149 | 1.130 | 1.134 | 1.187 | 1.187 | 1.187 | 1.229 | — | — |
| Area variation (F) | 0.134 | 0.283 | 0.122 | 0.153 | 0.174 | 0.168 | 0.391 | 0.391 | 0.391 | 0.205 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.42 | 0.80 | 0.43 | 0.40 | 0.35 | 0.36 | 0.55 | 0.55 | 0.55 | 0.33 | — | — |
| Mean distance, ‰ (H) | 0.085 | 0.172 | 0.082 | 0.083 | 0.078 | 0.078 | 0.159 | 0.159 | 0.159 | 0.089 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.93 | 99.83 | 99.92 | 100.01 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 2.1 | — | — | 0.0 | 0.0 | 15.2 | — | — | 8.5 | — | — |
| Edges on spiralling loops, % (L) | 52.4 | 42.1 | — | — | 16.4 | 35.3 | 60.2 | — | — | 29.6 | — | — |
| Spiralling loops (L) | 5 | 10 | — | — | 3 | 6 | 4 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 47.6 | 55.8 | — | — | 83.6 | 64.7 | 24.6 | — | — | 61.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1548 | 584 | — | — | 1426 | 1130 | 1726 | — | — | 1152 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 18.5 | 2.4 | 5.4 | 0.2 | 0.9 | 11.4 | 11.4 | 0.0 | 1.4 | — | — |
| Time, seconds (T) | 4.47 | 1.94 | 0.54 | 13.07 | 5.63 | 5.52 | 4.18 | 4.20 | 4.18 | 6.33 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 9147 | 7544 | 8038 | 7280 | 8024 | 8024 | 7401 | 7401 | 7401 | 7467 | — | — |
| Irregular vertices (S) | 22 | 0 | 262 | 0 | 0 | 0 | 20 | 20 | 20 | 0 | — | — |
| … of which above the minimum | 22 | 0 | 262 | 0 | 0 | 0 | 20 | 20 | 20 | 0 | — | — |
| Base complex patches (C) | 9070 | 1 | 7956 | 1 | 1 | 1 | 5983 | 5983 | 5983 | 1 | — | — |
| Alignment, degrees (A) | 11.41 | 0.76 | 6.29 | 0.61 | 0.64 | 0.64 | 4.25 | 4.25 | 4.25 | 0.62 | — | — |
| Mean angle deviation, degrees (F) | 7.33 | 1.26 | 5.12 | 0.91 | 0.91 | 0.91 | 3.68 | 3.68 | 3.68 | 0.91 | — | — |
| Angle deviation, 95th percentile (F) | 19.57 | 2.30 | 15.85 | 1.45 | 1.42 | 1.42 | 10.81 | 10.81 | 10.81 | 1.47 | — | — |
| Mean aspect ratio (F) | 1.180 | 2.552 | 1.139 | 1.340 | 1.321 | 1.319 | 1.233 | 1.233 | 1.233 | 1.321 | — | — |
| Area variation (F) | 0.189 | 0.146 | 0.149 | 0.281 | 0.299 | 0.300 | 0.225 | 0.225 | 0.225 | 0.299 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.60 | 0.79 | 0.65 | 0.34 | 0.35 | 0.35 | 0.69 | 0.69 | 0.69 | 0.34 | — | — |
| Mean distance, ‰ (H) | 0.109 | 0.178 | 0.119 | 0.111 | 0.104 | 0.104 | 0.222 | 0.222 | 0.222 | 0.108 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.82 | 99.67 | 99.79 | 100.01 | 100.01 | 100.01 | 99.56 | 99.56 | 99.56 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 100.0 | — | — | 100.0 | 100.0 | 9.9 | — | — | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 38.2 | 0.0 | — | — | 0.0 | 0.0 | 69.8 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 8 | 0 | — | — | 0 | 0 | 14 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 61.8 | 0.0 | — | — | 0.0 | 0.0 | 20.3 | — | — | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1065 | 0 | — | — | 0 | 0 | 450 | — | — | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 14.3 | 5.7 | 0.5 | 9.0 | 0.3 | 0.3 | 7.5 | 7.5 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | 5.42 | 2.20 | 0.91 | 6.48 | 4.28 | 5.38 | 6.20 | 5.73 | 6.20 | 5.54 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 6 | 0 | 7 | 2 | 1 | 1 | 2 | 1 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 2 | 0 | 1 | 4 | 5 | 0 | 0 | 0 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 1 | 10 | 0 | 2 | 4 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 1 | 6 | 0 | 1 | 2 | 0 | 0 | 0 | 5 | 0 | 0 |
| Mean aspect ratio (F) | 1 | 0 | 13 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
| Valid meshes (V) | 33/75 | 4/15 | 50/75 | 8/15 | 14/15 | 14/15 | 12/15 | 9/15 | 12/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 0 | 0 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 1 | 2 | 3 | 3 | 3 | 3 | 3 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 2 | 2 | 1 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 0 | 1 | 2 | 2 | 2 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 0 | 0 | 0 |
| Valid meshes (V) | 14/20 | 0/4 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
