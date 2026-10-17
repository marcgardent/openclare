# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7590 | 2795 | 8164 | 7124 | 7843 | 8055 | 7911 | 7035 | 7911 | 7909 | — | — |
| Irregular vertices (S) | 40 | 28 | 277 | 24 | 24 | 24 | 24 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 32 | 20 | 269 | 16 | 16 | 16 | 16 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 7412 | 1171 | 8164 | 5660 | 3525 | 3380 | 4563 | 2919 | 4563 | 6335 | — | — |
| Alignment, degrees (A) | 14.77 | 5.92 | 15.05 | 5.88 | 6.00 | 5.44 | 4.08 | 4.45 | 4.08 | 5.56 | — | — |
| Mean angle deviation, degrees (F) | 7.77 | 9.19 | 4.85 | 7.41 | 6.95 | 6.04 | 5.52 | 5.55 | 5.52 | 6.21 | — | — |
| Angle deviation, 95th percentile (F) | 19.95 | 34.45 | 15.77 | 19.73 | 18.80 | 15.91 | 14.94 | 14.71 | 14.94 | 16.78 | — | — |
| Mean aspect ratio (F) | 1.152 | 2.101 | 1.142 | 1.317 | 1.435 | 1.354 | 1.414 | 1.323 | 1.414 | 1.361 | — | — |
| Area variation (F) | 0.177 | 0.585 | 0.153 | 0.316 | 0.396 | 0.405 | 0.395 | 0.331 | 0.395 | 0.389 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.18 | 2.78 | 4.97 | 2.41 | 1.73 | 1.78 | 3.00 | 3.66 | 3.00 | 1.46 | — | — |
| Mean distance, ‰ (H) | 0.171 | 0.760 | 0.178 | 0.097 | 0.084 | 0.078 | 0.185 | 0.216 | 0.185 | 0.079 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.84 | 98.72 | 99.86 | 100.01 | 100.01 | 100.01 | 99.77 | 99.81 | 99.77 | 100.01 | — | — |
| Edges on closed loops, % (L) | 1.2 | 33.0 | — | — | 27.9 | 28.3 | 21.2 | — | — | 10.0 | — | — |
| Edges on spiralling loops, % (L) | 35.9 | 18.0 | — | — | 21.6 | 34.8 | 46.2 | — | — | 56.8 | — | — |
| Spiralling loops (L) | 12 | 4 | — | — | 5 | 9 | 11 | — | — | 9 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 62.6 | 49.1 | — | — | 50.6 | 36.9 | 32.6 | — | — | 33.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 766 | 292 | — | — | 1318 | 1058 | 601 | — | — | 602 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 5.1 | 65.1 | 2.1 | 10.9 | 2.0 | 0.7 | 1.1 | 12.1 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 4.32 | 2.57 | 1.85 | 10.94 | 7.14 | 9.13 | 12.37 | 12.61 | 12.37 | 7.32 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7342 | 5842 | 7792 | 7393 | 7913 | 8009 | 7071 | 6767 | 7071 | 6961 | — | — |
| Irregular vertices (S) | 10 | 34 | 162 | 114 | 116 | 132 | 68 | 68 | 68 | 120 | — | — |
| … of which above the minimum | 2 | 26 | 154 | 106 | 108 | 124 | 60 | 60 | 60 | 112 | — | — |
| Base complex patches (C) | 7342 | 5052 | 7792 | 7393 | 7913 | 8009 | 7071 | 6767 | 7071 | 6961 | — | — |
| Alignment, degrees (A) | 22.36 | 18.66 | 22.82 | 16.04 | 16.62 | 16.39 | 17.28 | 17.32 | 16.72 | 16.18 | — | — |
| Mean angle deviation, degrees (F) | 6.22 | 6.90 | 4.13 | 9.24 | 6.00 | 5.81 | 9.80 | 9.89 | 9.80 | 6.32 | — | — |
| Angle deviation, 95th percentile (F) | 18.32 | 17.38 | 12.43 | 22.12 | 14.34 | 14.58 | 23.73 | 23.67 | 23.73 | 15.48 | — | — |
| Mean aspect ratio (F) | 1.158 | 1.393 | 1.115 | 1.246 | 1.220 | 1.231 | 1.350 | 1.340 | 1.350 | 1.233 | — | — |
| Area variation (F) | 0.244 | 0.344 | 0.128 | 0.204 | 0.254 | 0.271 | 0.287 | 0.281 | 0.287 | 0.272 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.95 | 3.88 | 3.22 | 3.64 | 3.00 | 3.06 | 4.18 | 4.36 | 4.18 | 3.15 | — | — |
| Mean distance, ‰ (H) | 0.478 | 0.669 | 0.433 | 0.418 | 0.395 | 0.392 | 0.515 | 0.530 | 0.515 | 0.428 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.79 | 99.89 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 6.9 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 70.9 | 30.4 | — | — | 2.1 | 1.1 | 8.2 | — | — | 2.0 | — | — |
| Spiralling loops (L) | 6 | 7 | — | — | 1 | 1 | 3 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 29.1 | 62.6 | — | — | 97.9 | 98.9 | 91.8 | — | — | 98.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1193 | 618 | — | — | 349 | 487 | 490 | — | — | 258 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 8.2 | 27.0 | 2.6 | 7.6 | 1.1 | 0.1 | 11.6 | 15.4 | 0.0 | 1.6 | — | — |
| Time, seconds (T) | 4.18 | 3.32 | 1.45 | 33.90 | 7.03 | 7.81 | 18.31 | 18.00 | 18.31 | 7.75 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8170 | 6819 | 8234 | 6922 | 7840 | 7870 | 7186 | 7207 | 7186 | 7298 | — | — |
| Irregular vertices (S) | 22 | 18 | 214 | 40 | 40 | 42 | 40 | 40 | 40 | 40 | — | — |
| … of which above the minimum | 14 | 10 | 206 | — | 32 | 34 | 32 | 32 | 32 | 32 | — | — |
| Base complex patches (C) | 8170 | 3683 | 7510 | 2490 | 2128 | 2036 | 3236 | 3467 | 3236 | 1930 | — | — |
| Alignment, degrees (A) | 13.70 | 9.77 | 11.81 | 6.40 | 6.39 | 5.64 | 6.64 | 6.66 | 6.64 | 5.69 | — | — |
| Mean angle deviation, degrees (F) | 6.02 | 3.78 | 3.91 | 8.75 | 6.83 | 5.81 | 6.13 | 6.16 | 6.13 | 5.73 | — | — |
| Angle deviation, 95th percentile (F) | 15.98 | 10.36 | 12.20 | 25.50 | 17.89 | 15.85 | 17.40 | 17.67 | 17.40 | 15.15 | — | — |
| Mean aspect ratio (F) | 1.154 | 1.965 | 1.116 | 1.352 | 1.360 | 1.324 | 1.277 | 1.278 | 1.277 | 1.320 | — | — |
| Area variation (F) | 0.157 | 0.565 | 0.127 | 0.334 | 0.384 | 0.493 | 0.277 | 0.275 | 0.277 | 0.498 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.19 | 1.09 | 1.70 | 2.35 | 1.20 | 0.88 | 1.24 | 1.34 | 1.24 | 0.97 | — | — |
| Mean distance, ‰ (H) | 0.100 | 0.161 | 0.106 | 0.099 | 0.087 | 0.085 | 0.144 | 0.145 | 0.144 | 0.089 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.74 | 99.81 | — | 100.01 | 100.01 | 99.72 | 99.71 | 99.72 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 23.0 | — | — | 39.1 | 38.3 | 27.5 | — | — | 39.0 | — | — |
| Edges on spiralling loops, % (L) | 44.8 | 43.6 | — | — | 14.8 | 11.8 | 30.8 | — | — | 13.6 | — | — |
| Spiralling loops (L) | 5 | 9 | — | — | 6 | 4 | 13 | — | — | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 55.2 | 33.4 | — | — | 46.1 | 49.9 | 41.7 | — | — | 47.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1471 | 594 | — | — | 369 | 524 | 510 | — | — | 580 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.1 | 14.8 | 2.9 | 13.5 | 2.0 | 1.6 | 10.2 | 9.9 | 0.0 | 1.6 | — | — |
| Time, seconds (T) | 5.64 | 3.36 | 1.55 | 24.32 | 15.53 | 12.41 | 12.49 | 8.52 | 12.49 | 12.50 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7932 | 5928 | 8050 | 7765 | 8154 | 8013 | 11009 | 7510 | 11009 | 10912 | — | — |
| Irregular vertices (S) | 72 | 58 | 250 | 40 | 48 | 58 | 62 | 64 | 62 | 64 | — | — |
| … of which above the minimum | 40 | — | 218 | 8 | 16 | 26 | 30 | 32 | 30 | 32 | — | — |
| Base complex patches (C) | 7283 | — | 7125 | 766 | 1086 | 191 | 3227 | 1954 | 3227 | 120 | — | — |
| Alignment, degrees (A) | 8.22 | 5.16 | 8.80 | 2.71 | 2.57 | 1.95 | 2.06 | 2.29 | 2.06 | 1.96 | — | — |
| Mean angle deviation, degrees (F) | 5.83 | 7.40 | 4.40 | 3.58 | 2.21 | 2.24 | 3.22 | 3.48 | 3.22 | 2.08 | — | — |
| Angle deviation, 95th percentile (F) | 16.15 | 22.71 | 14.23 | 10.77 | 6.97 | 6.75 | 9.04 | 10.22 | 9.04 | 6.34 | — | — |
| Mean aspect ratio (F) | 1.140 | 2.161 | 1.144 | 1.200 | 1.184 | 1.219 | 1.693 | 1.289 | 1.693 | 1.197 | — | — |
| Area variation (F) | 0.139 | 0.487 | 0.138 | 0.191 | 0.210 | 0.348 | 0.338 | 0.229 | 0.338 | 0.350 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.65 | 7.28 | 9.87 | 4.82 | 3.06 | 4.45 | 4.53 | 5.93 | 4.53 | 2.21 | — | — |
| Mean distance, ‰ (H) | 0.294 | 0.343 | 0.344 | 0.275 | 0.222 | 0.217 | 0.451 | 0.614 | 0.451 | 0.127 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.64 | 99.99 | 99.48 | 99.98 | 100.00 | 99.99 | 99.38 | 99.02 | 99.38 | 100.00 | — | — |
| Edges on closed loops, % (L) | 5.0 | 25.6 | — | — | 67.3 | 84.6 | 42.9 | — | — | 89.4 | — | — |
| Edges on spiralling loops, % (L) | 55.9 | 48.4 | — | — | 21.1 | 0.0 | 42.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 33 | 29 | — | — | 7 | 0 | 14 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 40.8 | 26.0 | — | — | 11.6 | 15.4 | 15.1 | — | — | 10.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 332 | 108 | — | — | 68 | 110 | 129 | — | — | 32 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 0.9 | 25.9 | 0.6 | 2.9 | 1.9 | 0.2 | 37.6 | 6.1 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | 5.53 | 3.23 | 0.57 | 25.95 | 6.63 | 6.64 | 5.76 | 4.92 | 5.76 | 8.72 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 2647 | 8098 | 7376 | 8062 | 8109 | 9246 | 7317 | 9246 | 9253 | — | — |
| Irregular vertices (S) | 55 | 84 | 346 | 67 | 63 | 84 | 78 | 72 | 78 | 86 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6194 | — | 7878 | 4267 | 2280 | 2802 | 4088 | 3340 | 4088 | 3325 | — | — |
| Alignment, degrees (A) | 14.27 | 12.11 | 16.64 | 10.01 | 9.94 | 9.60 | 8.58 | 9.86 | 8.58 | 9.25 | — | — |
| Mean angle deviation, degrees (F) | 6.09 | 9.37 | 5.17 | 7.60 | 5.27 | 5.87 | 7.04 | 6.91 | 7.04 | 5.77 | — | — |
| Angle deviation, 95th percentile (F) | 18.06 | 32.08 | 17.78 | 21.30 | 15.39 | 15.63 | 18.57 | 19.33 | 18.57 | 14.71 | — | — |
| Mean aspect ratio (F) | 1.208 | 1.892 | 1.169 | 1.223 | 1.212 | 1.236 | 1.470 | 1.246 | 1.470 | 1.228 | — | — |
| Area variation (F) | 0.208 | 0.650 | 0.164 | 0.218 | 0.267 | 0.357 | 0.366 | 0.248 | 0.366 | 0.381 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.87 | 31.86 | 5.94 | 4.86 | 4.44 | 3.15 | 3.57 | 4.99 | 3.57 | 2.35 | — | — |
| Mean distance, ‰ (H) | 0.267 | 1.112 | 0.286 | 0.272 | 0.217 | 0.175 | 0.322 | 0.422 | 0.322 | 0.156 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.4 | 9.5 | — | — | 25.4 | 27.0 | 20.0 | — | — | 26.0 | — | — |
| Edges on spiralling loops, % (L) | 11.3 | 20.3 | — | — | 3.8 | 6.7 | 5.2 | — | — | 9.2 | — | — |
| Spiralling loops (L) | 6 | 12 | — | — | 4 | 8 | 4 | — | — | 11 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 68.7 | 70.2 | — | — | 50.7 | 53.3 | 62.0 | — | — | 52.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 512 | 98 | — | — | 376 | 557 | 564 | — | — | 355 | — | — |
| Edges on loops from a border to a border, % (L) | 19.0 | 0.0 | — | — | 20.2 | 13.0 | 12.8 | — | — | 12.3 | — | — |
| Off budget, % (N) | 0.9 | 66.9 | 1.2 | 7.8 | 0.8 | 1.4 | 15.6 | 8.5 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | 6.83 | 1.17 | 0.87 | 74.82 | 4.99 | 7.07 | 7.68 | 4.94 | 7.68 | 8.43 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7763 | 4622 | 7836 | 6967 | 8076 | 8098 | 12717 | 6761 | 12717 | 12640 | — | — |
| Irregular vertices (S) | 162 | 141 | 455 | 170 | 200 | 238 | 338 | 322 | 338 | 274 | — | — |
| … of which above the minimum | 154 | — | 429 | — | 192 | 230 | 330 | — | 330 | 266 | — | — |
| Base complex patches (C) | 7352 | — | 7656 | — | 5108 | 5116 | 9903 | — | 9903 | 9934 | — | — |
| Alignment, degrees (A) | 17.78 | 12.94 | 17.72 | 11.64 | 11.45 | 10.63 | 8.50 | 9.79 | 8.88 | 10.36 | — | — |
| Mean angle deviation, degrees (F) | 11.59 | 7.43 | 6.18 | 9.04 | 6.66 | 6.76 | 7.95 | 8.39 | 7.95 | 6.05 | — | — |
| Angle deviation, 95th percentile (F) | 33.33 | 22.75 | 20.84 | 24.13 | 18.15 | 17.75 | 22.55 | 22.69 | 22.55 | 16.87 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.812 | 1.225 | 1.401 | 1.404 | 1.420 | 2.025 | 1.482 | 2.025 | 1.393 | — | — |
| Area variation (F) | 0.312 | 0.668 | 0.196 | 0.284 | 0.341 | 0.459 | 0.643 | 0.364 | 0.643 | 0.449 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 27.28 | 27.93 | 30.74 | 26.72 | 13.80 | 8.92 | 5.23 | 25.57 | 5.23 | 4.43 | — | — |
| Mean distance, ‰ (H) | 0.521 | 1.054 | 0.601 | 0.408 | 0.317 | 0.299 | 0.346 | 0.548 | 0.346 | 0.203 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.1 | 78.5 | 35.7 | 24.3 | 19.3 | 5.9 | 7.4 | 18.6 | 9.1 | 3.1 | — | — |
| Volume kept, % of the source (V) | 99.49 | 98.66 | 99.41 | 99.93 | 99.99 | 100.00 | 99.59 | 99.39 | 99.59 | 100.01 | — | — |
| Edges on closed loops, % (L) | 2.0 | 17.3 | — | — | 19.3 | 18.6 | 11.1 | — | — | 11.0 | — | — |
| Edges on spiralling loops, % (L) | 13.7 | 18.1 | — | — | 2.4 | 6.2 | 16.8 | — | — | 12.9 | — | — |
| Spiralling loops (L) | 23 | 23 | — | — | 12 | 13 | 29 | — | — | 22 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 84.3 | 64.7 | — | — | 78.3 | 75.2 | 72.2 | — | — | 76.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 484 | 175 | — | — | 325 | 361 | 333 | — | — | 263 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.0 | 42.2 | 2.1 | 12.9 | 0.9 | 1.2 | 59.0 | 15.5 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | 4.83 | 3.89 | 2.51 | 34.81 | 12.03 | 20.86 | 23.50 | 24.03 | 23.50 | 21.44 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2520 | 7652 | 7214 | 8135 | 8083 | 11084 | 7045 | 11084 | 10877 | 10590 | 9426 |
| Irregular vertices (S) | — | 156 | 400 | 128 | 156 | 231 | 268 | 201 | 268 | 247 | 476 | 141 |
| … of which above the minimum | — | — | — | — | 148 | 223 | 260 | — | 260 | 239 | — | 133 |
| Base complex patches (C) | — | — | 5799 | — | 2961 | 2891 | 4740 | — | 4740 | 3567 | — | 5382 |
| Alignment, degrees (A) | — | 10.84 | 11.76 | 7.87 | 8.30 | 8.02 | 7.12 | 7.16 | 7.58 | 8.10 | 9.88 | 8.45 |
| Mean angle deviation, degrees (F) | — | 8.90 | 6.09 | 5.84 | 3.90 | 4.37 | 6.54 | 6.43 | 6.54 | 4.46 | 9.24 | 6.33 |
| Angle deviation, 95th percentile (F) | — | 29.26 | 22.28 | 18.18 | 11.69 | 14.32 | 20.41 | 19.84 | 20.41 | 14.91 | 26.27 | 18.39 |
| Mean aspect ratio (F) | — | 1.933 | 1.235 | 1.342 | 1.329 | 1.320 | 1.668 | 1.355 | 1.668 | 1.316 | 1.616 | 1.333 |
| Area variation (F) | — | 0.738 | 0.213 | 0.305 | 0.340 | 0.492 | 0.538 | 0.311 | 0.538 | 0.470 | 1.417 | 0.294 |
| Hausdorff, ‰ of the diagonal (H) | — | 23.89 | 35.02 | 19.08 | 9.03 | 6.00 | 3.94 | 14.01 | 3.94 | 5.33 | 6.47 | 8.89 |
| Mean distance, ‰ (H) | — | 1.795 | 2.033 | 1.059 | 0.469 | 0.234 | 0.257 | 0.766 | 0.257 | 0.206 | 0.172 | 0.557 |
| Thin parts: mean distance, % of a quad edge (V) | — | 36.5 | 57.2 | 35.9 | 16.6 | 5.8 | 5.4 | 20.0 | 6.3 | 8.1 | 1.5 | 25.0 |
| Volume kept, % of the source (V) | — | — | — | 99.88 | 99.99 | 100.02 | 99.35 | 98.95 | 99.35 | 100.01 | 98.80 | 99.93 |
| Edges on closed loops, % (L) | — | — | 12.1 | 32.9 | 32.7 | 32.1 | 28.6 | 31.6 | 28.6 | 33.7 | 31.6 | 22.1 |
| Edges on spiralling loops, % (L) | — | — | 7.3 | 20.4 | 10.9 | 10.0 | 7.6 | 17.3 | 7.6 | 3.6 | 2.5 | 32.6 |
| Spiralling loops (L) | — | — | 37 | 44 | 38 | 10 | 21 | 55 | 21 | 19 | 3 | 44 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 79.3 | 46.6 | 56.4 | 57.9 | 63.7 | 51.1 | 63.7 | 62.7 | 65.8 | 45.2 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 180 | 312 | 366 | 345 | 492 | 252 | 492 | 545 | 274 | 467 |
| Edges on loops from a border to a border, % (L) | — | — | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.5 | 4.3 | 9.8 | 1.7 | 1.0 | 38.6 | 11.9 | 0.0 | 1.9 | 0.0 | 11.0 |
| Time, seconds (T) | — | 5.49 | 20.44 | 42.48 | 23.77 | 27.20 | 21.15 | 20.34 | 21.15 | 28.34 | — | 70.96 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2564 | 7648 | 7201 | 7870 | 7966 | 10897 | 7322 | 10897 | 10766 | 10590 | 10602 |
| Irregular vertices (S) | — | 150 | 414 | 140 | 170 | 259 | 280 | 233 | 280 | 288 | 476 | 280 |
| … of which above the minimum | — | — | 395 | — | 162 | 251 | 272 | — | 272 | 280 | — | 272 |
| Base complex patches (C) | — | — | 6412 | — | 2797 | 2794 | 4861 | — | 4861 | 4336 | — | 3904 |
| Alignment, degrees (A) | — | 9.84 | 11.62 | 7.96 | 7.67 | 7.15 | 6.16 | 6.66 | 6.55 | 7.15 | 9.43 | 7.20 |
| Mean angle deviation, degrees (F) | — | 8.28 | 6.50 | 6.48 | 4.48 | 4.53 | 6.14 | 6.42 | 6.14 | 4.38 | 8.82 | 4.44 |
| Angle deviation, 95th percentile (F) | — | 26.20 | 22.21 | 21.70 | 14.30 | 14.40 | 19.95 | 20.28 | 19.95 | 13.88 | 25.47 | 14.29 |
| Mean aspect ratio (F) | — | 1.962 | 1.237 | 1.317 | 1.417 | 1.311 | 1.689 | 1.358 | 1.689 | 1.291 | 1.657 | 1.281 |
| Area variation (F) | — | 0.794 | 0.217 | 0.330 | 0.414 | 0.515 | 0.535 | 0.327 | 0.535 | 0.460 | 1.432 | 0.482 |
| Hausdorff, ‰ of the diagonal (H) | — | 26.27 | 32.73 | 14.77 | 8.59 | 6.18 | 4.84 | 20.10 | 4.84 | 5.66 | 4.44 | 4.92 |
| Mean distance, ‰ (H) | — | 2.147 | 2.076 | 1.073 | 0.611 | 0.282 | 0.252 | 0.891 | 0.252 | 0.199 | 0.156 | 0.181 |
| Thin parts: mean distance, % of a quad edge (V) | — | 50.9 | 73.5 | 41.9 | 22.6 | 7.9 | 6.1 | 34.4 | 7.4 | 7.2 | 1.4 | 6.1 |
| Volume kept, % of the source (V) | — | — | 98.91 | 99.90 | 99.97 | 100.01 | 99.27 | 98.92 | 99.27 | 100.01 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | — | 8.3 | 23.0 | 34.0 | 32.5 | 27.7 | 25.6 | 27.7 | 29.9 | 31.6 | 31.6 |
| Edges on spiralling loops, % (L) | — | — | 10.6 | 27.5 | 6.3 | 2.8 | 3.2 | 17.2 | 3.2 | 4.2 | 2.5 | 3.5 |
| Spiralling loops (L) | — | — | 49 | 46 | 33 | 14 | 13 | 50 | 13 | 22 | 3 | 8 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 80.4 | 49.5 | 59.7 | 64.7 | 69.1 | 57.2 | 69.1 | 65.9 | 65.8 | 64.9 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 166 | 402 | 375 | 331 | 349 | 294 | 349 | 358 | 274 | 325 |
| Edges on loops from a border to a border, % (L) | — | — | 0.3 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.0 | 4.4 | 10.0 | 1.6 | 0.4 | 36.2 | 8.5 | 0.0 | 1.2 | 0.0 | 0.1 |
| Time, seconds (T) | — | 6.63 | 20.66 | 36.31 | 24.11 | 26.02 | 21.59 | 22.09 | 21.59 | 35.97 | — | 33.55 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7874 | 2895 | 8098 | 7084 | 7861 | 7870 | 7965 | 7274 | 7965 | 8022 | — | — |
| Irregular vertices (S) | 76 | 74 | 286 | 80 | 78 | 78 | 76 | 80 | 76 | 78 | — | — |
| … of which above the minimum | 68 | — | 278 | 72 | 70 | 70 | 68 | 72 | 68 | 70 | — | — |
| Base complex patches (C) | 7772 | — | 7942 | 3692 | 3215 | 3366 | 4605 | 3862 | 4605 | 3426 | — | — |
| Alignment, degrees (A) | 16.85 | 9.88 | 16.86 | 8.49 | 8.40 | 7.58 | 7.56 | 8.42 | 7.56 | 7.56 | — | — |
| Mean angle deviation, degrees (F) | 6.23 | 7.56 | 4.47 | 9.07 | 7.18 | 6.26 | 5.56 | 6.01 | 5.56 | 6.47 | — | — |
| Angle deviation, 95th percentile (F) | 16.10 | 20.17 | 14.93 | 23.35 | 19.73 | 16.85 | 14.90 | 15.81 | 14.90 | 17.24 | — | — |
| Mean aspect ratio (F) | 1.177 | 1.481 | 1.136 | 1.348 | 1.385 | 1.367 | 1.342 | 1.289 | 1.342 | 1.359 | — | — |
| Area variation (F) | 0.178 | 0.712 | 0.144 | 0.296 | 0.353 | 0.477 | 0.301 | 0.256 | 0.301 | 0.464 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.30 | 6.25 | 7.10 | 2.61 | 3.19 | 1.73 | 4.73 | 5.88 | 4.73 | 1.65 | — | — |
| Mean distance, ‰ (H) | 0.161 | 0.635 | 0.172 | 0.093 | 0.086 | 0.082 | 0.169 | 0.187 | 0.169 | 0.080 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 98.89 | 99.78 | 100.01 | 100.00 | 100.01 | 99.73 | 99.73 | 99.73 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.1 | 16.4 | — | — | 31.2 | 31.6 | 21.9 | — | — | 29.8 | — | — |
| Edges on spiralling loops, % (L) | 19.0 | 13.5 | — | — | 12.3 | 18.1 | 24.0 | — | — | 16.9 | — | — |
| Spiralling loops (L) | 9 | 7 | — | — | 6 | 9 | 10 | — | — | 9 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.3 | 70.1 | — | — | 56.4 | 50.3 | 54.1 | — | — | 53.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 578 | 134 | — | — | 504 | 268 | 349 | — | — | 617 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.6 | 63.8 | 1.2 | 11.5 | 1.7 | 1.6 | 0.4 | 9.1 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 3.72 | 2.50 | 1.23 | 39.51 | 10.24 | 11.31 | 17.71 | 17.39 | 17.71 | 12.38 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8263 | 4012 | 8072 | 7052 | 8111 | 7886 | 6896 | 6832 | 6896 | 6806 | — | — |
| Irregular vertices (S) | 36 | 20 | 245 | 18 | 20 | 20 | 34 | 30 | 34 | 20 | — | — |
| … of which above the minimum | 36 | 20 | 245 | — | 20 | 20 | 34 | 30 | 34 | 20 | — | — |
| Base complex patches (C) | 8263 | 1318 | 8072 | 1541 | 1429 | 230 | 4859 | 3789 | 4859 | 1782 | — | — |
| Alignment, degrees (A) | 17.31 | 5.28 | 20.15 | 4.10 | 3.65 | 3.66 | 5.48 | 5.82 | 5.46 | 3.70 | — | — |
| Mean angle deviation, degrees (F) | 6.39 | 4.91 | 4.31 | 4.94 | 3.52 | 3.61 | 6.43 | 6.86 | 6.43 | 3.54 | — | — |
| Angle deviation, 95th percentile (F) | 17.71 | 13.45 | 14.25 | 16.54 | 10.71 | 11.17 | 18.71 | 21.48 | 18.71 | 10.76 | — | — |
| Mean aspect ratio (F) | 1.142 | 2.024 | 1.124 | 1.407 | 1.377 | 1.366 | 1.407 | 1.407 | 1.407 | 1.357 | — | — |
| Area variation (F) | 0.159 | 0.485 | 0.139 | 0.356 | 0.367 | 0.400 | 0.369 | 0.373 | 0.369 | 0.419 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.23 | 1.62 | 2.84 | 3.52 | 0.64 | 0.52 | 1.50 | 1.30 | 1.50 | 0.70 | — | — |
| Mean distance, ‰ (H) | 0.104 | 0.346 | 0.120 | 0.063 | 0.055 | 0.055 | 0.150 | 0.153 | 0.150 | 0.063 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.32 | 99.79 | — | 100.01 | 100.01 | 99.70 | 99.70 | 99.70 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 35.6 | — | — | 42.7 | 77.5 | 14.8 | — | — | 47.6 | — | — |
| Edges on spiralling loops, % (L) | 23.4 | 40.2 | — | — | 40.2 | 6.8 | 40.8 | — | — | 39.9 | — | — |
| Spiralling loops (L) | 7 | 10 | — | — | 4 | 3 | 12 | — | — | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 76.6 | 24.2 | — | — | 17.1 | 15.8 | 44.4 | — | — | 12.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 840 | 228 | — | — | 212 | 194 | 848 | — | — | 164 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 49.9 | 0.9 | 11.8 | 1.4 | 1.4 | 13.8 | 14.6 | 0.0 | 1.3 | — | — |
| Time, seconds (T) | 3.56 | 2.33 | 1.37 | 16.12 | 11.11 | 7.57 | 20.38 | 16.98 | 20.38 | 7.57 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7229 | 3438 | 8100 | 7057 | 8001 | 8059 | 8513 | 6943 | 8513 | 8484 | — | — |
| Irregular vertices (S) | 70 | 53 | 280 | 48 | 48 | 56 | 51 | 48 | 51 | 56 | — | — |
| … of which above the minimum | 62 | — | 272 | 40 | 40 | 48 | 43 | 40 | 43 | 48 | — | — |
| Base complex patches (C) | 6275 | — | 7910 | 2997 | 2221 | 3105 | 6215 | 4909 | 6215 | 1706 | — | — |
| Alignment, degrees (A) | 15.23 | 7.01 | 14.61 | 7.16 | 6.88 | 6.61 | 6.32 | 7.75 | 6.41 | 6.24 | — | — |
| Mean angle deviation, degrees (F) | 10.30 | 6.30 | 4.50 | 8.89 | 5.96 | 5.97 | 6.44 | 7.53 | 6.44 | 5.86 | — | — |
| Angle deviation, 95th percentile (F) | 28.50 | 19.86 | 15.34 | 24.31 | 17.42 | 16.57 | 16.31 | 19.46 | 16.31 | 16.45 | — | — |
| Mean aspect ratio (F) | 1.267 | 2.320 | 1.139 | 1.332 | 1.326 | 1.337 | 1.595 | 1.351 | 1.595 | 1.346 | — | — |
| Area variation (F) | 0.371 | 0.598 | 0.144 | 0.270 | 0.304 | 0.397 | 0.365 | 0.270 | 0.365 | 0.403 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 21.90 | 8.37 | 5.99 | 4.33 | 3.86 | 2.09 | 4.29 | 8.04 | 4.29 | 2.22 | — | — |
| Mean distance, ‰ (H) | 0.213 | 0.399 | 0.132 | 0.085 | 0.073 | 0.058 | 0.128 | 0.176 | 0.128 | 0.055 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.18 | 99.71 | 99.99 | 100.00 | 100.01 | 99.74 | 99.61 | 99.74 | 100.00 | — | — |
| Edges on closed loops, % (L) | 6.4 | 28.0 | — | — | 49.2 | 37.4 | 13.5 | — | — | 53.9 | — | — |
| Edges on spiralling loops, % (L) | 20.2 | 18.6 | — | — | 17.5 | 18.9 | 45.9 | — | — | 6.8 | — | — |
| Spiralling loops (L) | 12 | 5 | — | — | 5 | 6 | 17 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 73.2 | 53.3 | — | — | 33.3 | 43.7 | 40.6 | — | — | 39.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 660 | 163 | — | — | 296 | 324 | 515 | — | — | 410 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 9.6 | 57.0 | 1.2 | 11.8 | 0.0 | 0.7 | 6.4 | 13.2 | 0.0 | 0.3 | — | — |
| Time, seconds (T) | 3.58 | 3.90 | 1.16 | 19.91 | 10.28 | 9.90 | 15.56 | 13.36 | 15.56 | 10.52 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6973 | 4682 | 8116 | 7187 | 7979 | 7914 | 9824 | 7070 | 9824 | 9840 | — | — |
| Irregular vertices (S) | 136 | 135 | 370 | 81 | 92 | 100 | 101 | 98 | 101 | 102 | — | — |
| … of which above the minimum | 128 | — | 362 | — | 84 | 92 | — | 90 | — | 94 | — | — |
| Base complex patches (C) | 6841 | — | 7826 | 4517 | 3719 | 4572 | — | 4284 | — | 4418 | — | — |
| Alignment, degrees (A) | 15.84 | 11.22 | 14.90 | 9.35 | 9.29 | 8.40 | 7.39 | 8.84 | 7.49 | 8.16 | — | — |
| Mean angle deviation, degrees (F) | 11.44 | 6.80 | 5.04 | 8.83 | 6.62 | 6.04 | 6.40 | 6.59 | 6.40 | 6.05 | — | — |
| Angle deviation, 95th percentile (F) | 34.02 | 21.80 | 17.35 | 25.06 | 18.35 | 17.71 | 18.75 | 19.80 | 18.75 | 18.03 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.773 | 1.161 | 1.356 | 1.320 | 1.331 | 1.546 | 1.329 | 1.546 | 1.326 | — | — |
| Area variation (F) | 0.383 | 0.668 | 0.161 | 0.307 | 0.312 | 0.507 | 0.462 | 0.260 | 0.462 | 0.488 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.23 | 9.84 | 21.49 | 7.72 | 7.34 | 6.40 | 9.11 | 9.71 | 9.11 | 7.16 | — | — |
| Mean distance, ‰ (H) | 0.610 | 0.627 | 0.238 | 0.164 | 0.147 | 0.123 | 0.222 | 0.301 | 0.222 | 0.099 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 64.0 | 21.7 | 11.6 | 16.5 | 15.9 | 4.5 | 3.7 | 15.8 | 4.9 | 3.2 | — | — |
| Volume kept, % of the source (V) | 99.23 | 99.18 | 99.70 | — | 99.99 | 99.99 | 99.74 | 99.55 | 99.74 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.5 | 9.6 | — | — | 31.1 | 23.5 | 24.7 | — | — | 32.2 | — | — |
| Edges on spiralling loops, % (L) | 21.6 | 22.2 | — | — | 15.2 | 13.5 | 15.3 | — | — | 7.3 | — | — |
| Spiralling loops (L) | 22 | 18 | — | — | 10 | 23 | 21 | — | — | 19 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.2 | 68.1 | — | — | 53.7 | 63.0 | 60.0 | — | — | 60.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 500 | 161 | — | — | 420 | 303 | 456 | — | — | 637 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 12.8 | 41.5 | 1.5 | 10.2 | 0.3 | 1.1 | 22.8 | 11.6 | 0.0 | 0.2 | — | — |
| Time, seconds (T) | 3.13 | 4.60 | 1.42 | 34.04 | 10.71 | 12.75 | 16.84 | 16.13 | 16.84 | 17.12 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/5 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7781 | 2740 | 7933 | 7345 | 7974 | 7845 | 13714 | 6947 | 13714 | 13858 | — | — |
| Irregular vertices (S) | 112 | 109 | 490 | 173 | 168 | 287 | 244 | 247 | 244 | 369 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 4991 | — | 7797 | 5208 | 5368 | 6801 | 11709 | — | 11709 | 11598 | — | — |
| Alignment, degrees (A) | 17.56 | 15.73 | 18.65 | 13.30 | 12.93 | 12.37 | 10.64 | 12.69 | 10.91 | 11.57 | — | — |
| Mean angle deviation, degrees (F) | 7.80 | 10.26 | 6.63 | 8.59 | 5.56 | 5.92 | 8.15 | 8.38 | 8.15 | 5.84 | — | — |
| Angle deviation, 95th percentile (F) | 22.58 | 29.98 | 23.27 | 23.17 | 15.31 | 17.26 | 22.38 | 23.18 | 22.38 | 16.36 | — | — |
| Mean aspect ratio (F) | 1.257 | 1.889 | 1.241 | 1.297 | 1.261 | 1.292 | 1.879 | 1.380 | 1.879 | 1.279 | — | — |
| Area variation (F) | 0.245 | 0.652 | 0.210 | 0.264 | 0.309 | 0.505 | 0.651 | 0.279 | 0.651 | 0.473 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 31.29 | 25.08 | 29.67 | 21.23 | 14.22 | 11.23 | 4.22 | 10.31 | 4.22 | 9.84 | — | — |
| Mean distance, ‰ (H) | 0.581 | 2.010 | 0.629 | 0.483 | 0.349 | 0.284 | 0.310 | 0.598 | 0.310 | 0.166 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.5 | 54.4 | 38.5 | 34.4 | 20.9 | 10.4 | 3.9 | 14.2 | 5.1 | 10.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 4.8 | 2.2 | 3.3 | — | — | 2.7 | — | — |
| Edges on spiralling loops, % (L) | 3.1 | — | — | — | 2.8 | 3.3 | 9.1 | — | — | 2.5 | — | — |
| Spiralling loops (L) | 5 | — | — | — | 5 | 7 | 17 | — | — | 9 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.6 | — | — | — | 80.6 | 89.9 | 83.4 | — | — | 88.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 404 | — | — | — | 219 | 183 | 467 | — | — | 249 | — | — |
| Edges on loops from a border to a border, % (L) | 19.8 | — | — | — | 11.8 | 4.5 | 4.2 | — | — | 6.0 | — | — |
| Off budget, % (N) | 2.7 | 65.8 | 0.8 | 8.2 | 0.3 | 1.9 | 71.4 | 13.2 | 0.0 | 1.1 | — | — |
| Time, seconds (T) | 6.02 | 2.44 | 0.92 | 36.84 | 6.90 | 13.12 | 9.49 | 8.46 | 9.49 | 20.43 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 6758 | 3828 | 6932 | 6584 | 8259 | 8441 | 53874 | 6896 | 53874 | 54079 | — | — |
| Irregular vertices (S) | 917 | 556 | 1126 | 570 | 1222 | 1642 | 1899 | 1164 | 1899 | 2534 | — | — |
| … of which above the minimum | — | — | — | — | — | 1026 | — | — | — | 1918 | — | — |
| Base complex patches (C) | 6524 | — | 6434 | — | — | 6323 | — | — | — | 36232 | — | — |
| Alignment, degrees (A) | 18.21 | 18.28 | 18.49 | 14.12 | 12.72 | 12.59 | 9.05 | 12.40 | 11.32 | 9.35 | — | — |
| Mean angle deviation, degrees (F) | 14.81 | 17.84 | 12.06 | 10.22 | 12.20 | 13.54 | 8.61 | 11.61 | 8.61 | 6.50 | — | — |
| Angle deviation, 95th percentile (F) | 41.35 | 56.33 | 40.69 | 29.42 | 43.62 | 41.97 | 24.64 | 31.66 | 24.64 | 19.60 | — | — |
| Mean aspect ratio (F) | 1.515 | 3.415 | 1.595 | 1.412 | 2.386 | 2.373 | 1.839 | 1.518 | 1.839 | 1.439 | — | — |
| Area variation (F) | 0.407 | 0.904 | 0.359 | 0.322 | 0.623 | 1.438 | 1.220 | 0.433 | 1.220 | 0.487 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 74.30 | 58.87 | 72.22 | 73.55 | 16.54 | 14.23 | 73.68 | 75.13 | 73.68 | 7.75 | — | — |
| Mean distance, ‰ (H) | 1.489 | 4.497 | 1.750 | 1.267 | 0.671 | 0.853 | 0.689 | 1.593 | 0.689 | 0.113 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 44.1 | 83.6 | 47.5 | 37.1 | 13.0 | 11.7 | 22.2 | 42.9 | 159.7 | 10.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 97.42 | 96.05 | — | — | — | 99.77 | — | — |
| Edges on closed loops, % (L) | 0.4 | — | — | — | 14.6 | 12.8 | — | — | — | 17.0 | — | — |
| Edges on spiralling loops, % (L) | 17.0 | — | — | — | 19.7 | 19.2 | — | — | — | 10.6 | — | — |
| Spiralling loops (L) | 130 | — | — | — | 413 | 431 | — | — | — | 294 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.4 | — | — | — | 65.7 | 68.1 | — | — | — | 72.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | — | — | 106 | 45 | — | — | — | 407 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | 15.5 | 52.1 | 13.4 | 17.7 | 3.2 | 5.5 | 573.4 | 13.8 | 0.0 | 0.4 | — | — |
| Time, seconds (T) | 3.62 | 1.88 | 1.79 | 135.60 | 6.91 | 13.67 | 9.67 | 7.85 | 9.67 | 47.55 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4954 | 7960 | 7468 | 8034 | 8076 | 8805 | 7054 | 8805 | 8688 | — | — |
| Irregular vertices (S) | — | 58 | 258 | 68 | 70 | 81 | 82 | 79 | 82 | 89 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 5201 | 1643 | 1891 | 2396 | 2942 | — | 2942 | 2386 | — | — |
| Alignment, degrees (A) | — | 10.49 | 13.88 | 9.84 | 9.80 | 8.90 | 8.33 | 9.16 | 8.67 | 9.27 | — | — |
| Mean angle deviation, degrees (F) | — | 6.02 | 4.48 | 7.01 | 4.68 | 4.62 | 6.17 | 6.74 | 6.17 | 4.50 | — | — |
| Angle deviation, 95th percentile (F) | — | 18.67 | 14.92 | 19.83 | 12.44 | 12.26 | 17.33 | 19.99 | 17.33 | 12.80 | — | — |
| Mean aspect ratio (F) | — | 1.719 | 1.145 | 1.266 | 1.249 | 1.271 | 1.480 | 1.386 | 1.480 | 1.259 | — | — |
| Area variation (F) | — | 0.719 | 0.140 | 0.229 | 0.280 | 0.325 | 0.354 | 0.285 | 0.354 | 0.370 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 8.89 | 9.03 | 7.15 | 6.49 | 6.74 | 6.69 | 6.45 | 6.69 | 7.27 | — | — |
| Mean distance, ‰ (H) | — | 0.521 | 0.281 | 0.238 | 0.218 | 0.215 | 0.264 | 0.325 | 0.264 | 0.204 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 20.8 | — | — | 22.0 | 22.6 | 19.3 | — | — | 24.1 | — | — |
| Edges on spiralling loops, % (L) | — | 8.5 | — | — | 0.4 | 3.4 | 7.6 | — | — | 2.7 | — | — |
| Spiralling loops (L) | — | 5 | — | — | 1 | 5 | 8 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 53.1 | — | — | 50.1 | 50.9 | 51.4 | — | — | 49.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 154 | — | — | 230 | 224 | 183 | — | — | 200 | — | — |
| Edges on loops from a border to a border, % (L) | — | 17.6 | — | — | 27.5 | 23.2 | 21.7 | — | — | 23.7 | — | — |
| Off budget, % (N) | — | 38.1 | 0.5 | 6.7 | 0.4 | 0.9 | 10.1 | 11.8 | 0.0 | 1.3 | — | — |
| Time, seconds (T) | — | 3.19 | 2.16 | 23.83 | 9.00 | 7.96 | 15.51 | 11.76 | 15.51 | 8.84 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 4954 | 7732 | 7824 | 8092 | 8098 | 7353 | 7353 | 7353 | 7279 | — | — |
| Irregular vertices (S) | 38 | 94 | 337 | 24 | 24 | 24 | 24 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 30 | — | 329 | 16 | 16 | 16 | 16 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 1171 | — | 4678 | 128 | 78 | 92 | 730 | 730 | 730 | 92 | — | — |
| Alignment, degrees (A) | 13.31 | 14.04 | 13.52 | 11.84 | 11.97 | 10.32 | 10.87 | 10.87 | 10.87 | 10.44 | — | — |
| Mean angle deviation, degrees (F) | 2.56 | 13.77 | 4.52 | 1.66 | 1.18 | 1.00 | 3.63 | 3.63 | 3.63 | 1.07 | — | — |
| Angle deviation, 95th percentile (F) | 9.55 | 47.48 | 18.21 | 7.02 | 4.31 | 3.80 | 14.81 | 14.81 | 14.81 | 4.15 | — | — |
| Mean aspect ratio (F) | 1.074 | 3.591 | 1.178 | 1.151 | 1.108 | 1.129 | 1.301 | 1.301 | 1.301 | 1.115 | — | — |
| Area variation (F) | 0.087 | 0.912 | 0.166 | 0.105 | 0.114 | 0.328 | 0.244 | 0.244 | 0.244 | 0.339 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.14 | 11.34 | 5.05 | 0.32 | 0.32 | 0.25 | 0.68 | 0.68 | 0.68 | 0.27 | — | — |
| Mean distance, ‰ (H) | 0.034 | 0.506 | 0.023 | 0.021 | 0.021 | 0.024 | 0.040 | 0.040 | 0.040 | 0.026 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.06 | — | 100.06 | 100.00 | 100.00 | 100.00 | 100.06 | 100.06 | 100.06 | 100.00 | — | — |
| Edges on closed loops, % (L) | 59.2 | — | — | — | 90.1 | 89.3 | 67.0 | — | — | 88.7 | — | — |
| Edges on spiralling loops, % (L) | 22.9 | — | — | — | 0.0 | 0.0 | 18.3 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 8 | — | — | — | 0 | 0 | 6 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 17.7 | — | — | — | 9.9 | 10.7 | 14.7 | — | — | 11.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 160 | — | — | — | 93 | 82 | 130 | — | — | 77 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 0.3 | 38.1 | 3.4 | 2.2 | 1.1 | 1.2 | 8.1 | 8.1 | 0.0 | 1.0 | — | — |
| Time, seconds (T) | 8.90 | 3.06 | 2.22 | 10.17 | 7.44 | 6.17 | 2.65 | 2.72 | 2.65 | 7.56 | — | — |

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
| Angle deviation, 95th percentile (F) | 1.18 | 24.98 | 2.60 | 2.23 | 0.01 | 0.74 | 0.00 | 0.00 | 0.00 | 0.76 | — | — |
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
| Time, seconds (T) | 4.14 | 1.30 | 0.78 | 65.82 | 3.79 | 5.32 | 3.50 | 2.90 | 3.50 | 6.09 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8016 | 3035 | 8000 | 8064 | 8064 | 8064 | 8064 | 8064 | 8064 | 8064 | — | — |
| Irregular vertices (S) | 13 | 20 | 24 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | — | 16 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 402 | — | 706 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 0.86 | 3.67 | 1.17 | 0.57 | 0.60 | 0.92 | 0.54 | 0.54 | 0.54 | 0.92 | — | — |
| Mean angle deviation, degrees (F) | 0.40 | 6.33 | 0.88 | 0.01 | 0.00 | 0.49 | 0.00 | 0.00 | 0.00 | 0.49 | — | — |
| Angle deviation, 95th percentile (F) | 1.40 | 24.33 | 2.62 | 0.03 | 0.01 | 1.48 | 0.00 | 0.00 | 0.00 | 1.49 | — | — |
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
| Time, seconds (T) | 3.24 | 1.54 | 0.34 | 5.37 | 5.50 | 5.23 | 3.10 | 3.18 | 3.10 | 5.97 | — | — |

Failures: quadriflow: timed out after 1800 s.

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7916 | 2565 | 8024 | 7276 | 7980 | 7850 | 6922 | 6922 | 6922 | 6836 | — | — |
| Irregular vertices (S) | 8 | 10 | 165 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 157 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 2635 | — | 4940 | 1298 | 12 | 6 | 850 | 850 | 850 | 14 | — | — |
| Alignment, degrees (A) | 4.37 | 4.10 | 3.47 | 1.33 | 1.53 | 1.52 | 1.73 | 1.73 | 1.73 | 1.30 | — | — |
| Mean angle deviation, degrees (F) | 3.95 | 6.60 | 3.05 | 2.55 | 2.70 | 2.79 | 4.41 | 4.41 | 4.41 | 2.26 | — | — |
| Angle deviation, 95th percentile (F) | 16.68 | 20.66 | 11.70 | 13.62 | 10.15 | 10.22 | 18.99 | 18.99 | 18.99 | 9.46 | — | — |
| Mean aspect ratio (F) | 1.075 | 2.021 | 1.097 | 1.285 | 1.293 | 1.410 | 1.349 | 1.349 | 1.349 | 1.390 | — | — |
| Area variation (F) | 0.096 | 0.636 | 0.115 | 0.284 | 0.298 | 0.208 | 0.322 | 0.322 | 0.322 | 0.187 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.80 | 5.39 | 2.07 | 0.55 | 0.54 | 0.55 | 0.73 | 0.73 | 0.73 | 0.60 | — | — |
| Mean distance, ‰ (H) | 0.052 | 0.289 | 0.045 | 0.071 | 0.064 | 0.074 | 0.147 | 0.147 | 0.147 | 0.078 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.96 | 99.70 | 99.96 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 | — | — |
| Edges on closed loops, % (L) | 33.3 | 29.3 | — | — | 95.6 | 97.0 | 57.3 | — | — | 94.4 | — | — |
| Edges on spiralling loops, % (L) | 48.3 | 33.0 | — | — | 0.0 | 0.0 | 34.4 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 5 | 3 | — | — | 0 | 0 | 4 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 18.6 | 37.7 | — | — | 4.4 | 3.0 | 8.3 | — | — | 5.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 762 | 316 | — | — | 155 | 66 | 205 | — | — | 145 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.1 | 67.9 | 0.5 | 9.0 | 0.2 | 1.9 | 13.5 | 13.5 | 0.0 | 1.2 | — | — |
| Time, seconds (T) | 4.60 | 1.04 | 0.57 | 7.70 | 7.28 | 6.12 | 4.42 | 4.47 | 4.42 | 5.25 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 7212 | 7693 | 6950 | 7534 | 8451 | 19704 | 14699 | 19704 | 19212 | — | — |
| Irregular vertices (S) | — | 2158 | 977 | 270 | 271 | 872 | 695 | 691 | 695 | 761 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4255 | — | — | — | — | — | — | 12909 | — | — |
| Alignment, degrees (A) | — | 10.19 | 11.23 | 10.34 | 10.04 | 11.25 | 7.51 | 7.00 | 8.22 | 9.59 | — | — |
| Mean angle deviation, degrees (F) | — | 40.97 | 10.36 | 6.59 | 6.49 | 8.98 | 8.05 | 8.32 | 8.05 | 5.72 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.08 | 43.42 | 21.59 | 20.93 | 32.59 | 34.33 | 41.53 | 34.33 | 19.51 | — | — |
| Mean aspect ratio (F) | — | 9.234 | 1.775 | 1.305 | 1.282 | 1.697 | 2.096 | 2.216 | 2.096 | 1.332 | — | — |
| Area variation (F) | — | 2.203 | 0.455 | 0.267 | 0.290 | 0.431 | 1.067 | 0.959 | 1.067 | 0.290 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 37.63 | 24.82 | 35.65 | 39.30 | 39.30 | 14.05 | 15.28 | 14.05 | 14.81 | — | — |
| Mean distance, ‰ (H) | — | 2.866 | 1.379 | 2.882 | 2.405 | 2.255 | 0.241 | 0.383 | 0.241 | 0.828 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 28.8 | 18.1 | 40.3 | 34.4 | 32.8 | 1.8 | 2.8 | 2.5 | 22.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 9.0 | 8.9 | 6.4 | — | — | 15.2 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 39.2 | 29.5 | 1.5 | — | — | 31.1 | — | — |
| Spiralling loops (L) | — | — | — | — | 48 | 37 | 12 | — | — | 50 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 51.5 | 61.3 | 27.6 | — | — | 49.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 221 | 221 | 132 | — | — | 447 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.4 | 0.3 | 64.6 | — | — | 4.4 | — | — |
| Off budget, % (N) | — | 9.8 | 3.8 | 13.1 | 5.8 | 5.6 | 146.3 | 83.7 | 0.0 | 2.5 | — | — |
| Time, seconds (T) | — | 1.84 | 1.45 | 87.87 | 49.62 | 42.96 | 5.17 | 5.88 | 5.17 | 104.39 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4422 | 7700 | 6805 | 8064 | 8007 | 13801 | 6640 | 13801 | 13526 | — | — |
| Irregular vertices (S) | — | 172 | 442 | 163 | 198 | 354 | 368 | 344 | 368 | 430 | — | — |
| … of which above the minimum | — | — | 446 | — | 190 | 346 | 360 | — | 360 | 422 | — | — |
| Base complex patches (C) | — | — | 7440 | — | 5106 | 5585 | 10719 | — | 10719 | 10002 | — | — |
| Alignment, degrees (A) | — | 13.99 | 17.01 | 12.30 | 12.01 | 10.07 | 8.76 | 10.53 | 9.13 | 9.79 | — | — |
| Mean angle deviation, degrees (F) | — | 7.16 | 6.55 | 8.73 | 6.25 | 6.57 | 7.98 | 8.73 | 7.98 | 6.58 | — | — |
| Angle deviation, 95th percentile (F) | — | 21.73 | 21.36 | 23.56 | 16.56 | 19.92 | 22.44 | 23.83 | 22.44 | 18.57 | — | — |
| Mean aspect ratio (F) | — | 1.812 | 1.226 | 1.417 | 1.404 | 1.394 | 2.045 | 1.480 | 2.045 | 1.378 | — | — |
| Area variation (F) | — | 0.657 | 0.198 | 0.304 | 0.349 | 0.967 | 0.686 | 0.354 | 0.686 | 1.009 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 22.79 | 30.78 | 25.14 | 9.19 | 7.58 | 7.42 | 30.01 | 7.42 | 6.23 | — | — |
| Mean distance, ‰ (H) | — | 1.109 | 0.706 | 0.441 | 0.319 | 0.343 | 0.321 | 0.572 | 0.321 | 0.227 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 44.3 | 48.7 | 16.3 | 14.0 | 1.4 | 3.6 | 15.6 | 5.9 | 1.5 | — | — |
| Volume kept, % of the source (V) | — | — | 99.39 | 99.93 | 99.99 | 100.01 | 99.56 | 99.30 | 99.56 | 100.01 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 18.6 | 15.5 | 11.3 | — | — | 12.9 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 8.0 | 7.1 | 12.1 | — | — | 9.4 | — | — |
| Spiralling loops (L) | — | — | — | — | 27 | 17 | 29 | — | — | 33 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 73.4 | 77.4 | 76.7 | — | — | 77.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 286 | 164 | 313 | — | — | 243 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 44.7 | 3.8 | 14.9 | 0.8 | 0.1 | 72.5 | 17.0 | 0.0 | 2.0 | — | — |
| Time, seconds (T) | — | 17.03 | 82.86 | 80.09 | 58.18 | 69.44 | 44.60 | 45.61 | 44.60 | 82.75 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 7305 | 2507 | — | — | 6612 | 6616 | 42790 | — | — | — | — | — |
| Irregular vertices (S) | 642 | 428 | — | — | 403 | 396 | 1504 | — | — | — | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6985 | — | — | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 13.37 | 16.69 | — | — | 13.26 | 13.29 | 10.22 | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 9.18 | 21.18 | — | — | 8.23 | 8.34 | 7.41 | — | — | — | — | — |
| Angle deviation, 95th percentile (F) | 32.10 | 68.70 | — | — | 29.42 | 30.27 | 22.87 | — | — | — | — | — |
| Mean aspect ratio (F) | 1.331 | 2.614 | — | — | 1.342 | 1.350 | 1.878 | — | — | — | — | — |
| Area variation (F) | 0.268 | 0.825 | — | — | 0.280 | 0.281 | 0.822 | — | — | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 50.24 | 90.14 | — | — | 53.47 | 53.47 | 30.04 | — | — | — | — | — |
| Mean distance, ‰ (H) | 1.990 | 7.482 | — | — | 2.492 | 2.494 | 0.641 | — | — | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.3 | 69.7 | — | — | 33.2 | 33.3 | 13.0 | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.3 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 12.4 | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | 125 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.2 | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 151 | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 8.7 | 68.7 | — | — | 17.3 | 17.3 | 434.9 | — | — | — | — | — |
| Time, seconds (T) | 7.40 | 4.79 | — | — | 81.11 | 76.94 | 261.56 | — | — | — | — | — |

Failures: openclare-matched: timed out after 1800 s.

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 | 0/1 | 0/0 | 0/0 |
| Quads made | 7180 | 2664 | — | — | 5861 | — | 35806 | — | — | 32315 | — | — |
| Irregular vertices (S) | 624 | 292 | — | — | 261 | — | 1135 | — | — | 496 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6952 | — | — | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 11.45 | 14.32 | — | — | 11.73 | — | 10.17 | — | — | 11.11 | — | — |
| Mean angle deviation, degrees (F) | 9.29 | 18.29 | — | — | 8.78 | — | 7.80 | — | — | 5.43 | — | — |
| Angle deviation, 95th percentile (F) | 29.68 | 60.76 | — | — | 29.04 | — | 23.07 | — | — | 17.97 | — | — |
| Mean aspect ratio (F) | 1.309 | 2.412 | — | — | 1.396 | — | 2.088 | — | — | 1.212 | — | — |
| Area variation (F) | 0.258 | 0.707 | — | — | 0.378 | — | 0.757 | — | — | 0.207 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 34.62 | 60.39 | — | — | 53.73 | — | 15.92 | — | — | 20.89 | — | — |
| Mean distance, ‰ (H) | 1.468 | 4.925 | — | — | 2.316 | — | 0.833 | — | — | 0.567 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.1 | 39.9 | — | — | 29.0 | — | 11.4 | — | — | 28.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | 98.95 | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.3 | — | — | — | — | — | 2.1 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 18.5 | — | — | — | — | — | 15.7 | — | — | — | — | — |
| Spiralling loops (L) | 50 | — | — | — | — | — | 91 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.1 | — | — | — | — | — | 82.3 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 117 | — | — | — | — | — | 229 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | — | — | 0.0 | — | — | — | — | — |
| Off budget, % (N) | 10.2 | 66.7 | — | — | 26.7 | — | 347.6 | — | — | 9.7 | — | — |
| Time, seconds (T) | 7.17 | 4.13 | — | — | 95.41 | — | 321.94 | — | — | 185.78 | — | — |

Failures: openclare: timed out after 1800 s.

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 0/0 | 0/0 | 1/1 | 1/1 | 0/1 | 0/0 | 0/0 | 1/1 | 0/0 | 0/0 |
| Quads made | 6712 | 4327 | — | — | 8059 | 8078 | 33780 | — | — | 34204 | — | — |
| Irregular vertices (S) | 400 | 121 | — | — | 188 | 280 | 963 | — | — | 424 | — | — |
| … of which above the minimum | 341 | — | — | — | 124 | 216 | 899 | — | — | 360 | — | — |
| Base complex patches (C) | 6543 | — | — | — | 5296 | 6848 | 31433 | — | — | 27430 | — | — |
| Alignment, degrees (A) | 13.29 | 13.65 | — | — | 11.39 | 11.40 | 11.22 | — | — | 11.72 | — | — |
| Mean angle deviation, degrees (F) | 9.33 | 10.72 | — | — | 4.58 | 5.36 | 7.11 | — | — | 4.56 | — | — |
| Angle deviation, 95th percentile (F) | 30.16 | 31.96 | — | — | 14.94 | 16.87 | 20.64 | — | — | 13.46 | — | — |
| Mean aspect ratio (F) | 1.293 | 1.880 | — | — | 1.197 | 1.361 | 1.963 | — | — | 1.197 | — | — |
| Area variation (F) | 0.268 | 0.625 | — | — | 0.214 | 0.357 | 0.555 | — | — | 0.276 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.23 | 58.87 | — | — | 10.55 | 15.79 | 10.81 | — | — | 6.64 | — | — |
| Mean distance, ‰ (H) | 1.190 | 2.121 | — | — | 1.062 | 1.029 | 0.585 | — | — | 0.353 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.8 | 185.8 | — | — | 22.0 | 23.6 | 12.3 | — | — | 9.8 | — | — |
| Volume kept, % of the source (V) | 99.56 | 99.45 | — | — | 100.00 | 100.02 | 99.37 | — | — | 100.01 | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.8 | — | — | 18.0 | 8.1 | 3.7 | — | — | 11.1 | — | — |
| Edges on spiralling loops, % (L) | 8.9 | 24.3 | — | — | 11.2 | 8.9 | 6.8 | — | — | 11.8 | — | — |
| Spiralling loops (L) | 33 | 25 | — | — | 28 | 12 | 40 | — | — | 22 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.0 | 74.8 | — | — | 70.8 | 83.0 | 89.5 | — | — | 77.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 149 | 168 | — | — | 229 | 202 | 340 | — | — | 610 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 16.1 | 45.9 | — | — | 0.7 | 1.0 | 322.2 | — | — | 1.3 | — | — |
| Time, seconds (T) | 6.75 | 3.06 | — | — | 8.70 | 11.97 | 133.40 | — | — | 23.55 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 1/1 | 1/1 | 0/1 | 0/0 | 0/0 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 2364 | — | — | 260 | 414 | 36766 | — | — | 1197 | — | — |
| Irregular vertices (S) | — | 234 | — | — | 36 | 110 | 1245 | — | — | 216 | — | — |
| … of which above the minimum | — | — | — | — | 12 | 86 | — | — | — | 192 | — | — |
| Base complex patches (C) | — | — | — | — | 236 | 414 | — | — | — | 1041 | — | — |
| Alignment, degrees (A) | — | 13.30 | — | — | 14.84 | 19.72 | 9.54 | — | — | 13.66 | — | — |
| Mean angle deviation, degrees (F) | — | 14.00 | — | — | 25.06 | 23.08 | 7.28 | — | — | 12.88 | — | — |
| Angle deviation, 95th percentile (F) | — | 51.31 | — | — | 64.75 | 61.91 | 22.98 | — | — | 39.67 | — | — |
| Mean aspect ratio (F) | — | 2.278 | — | — | 7.197 | 3.065 | 1.983 | — | — | 2.146 | — | — |
| Area variation (F) | — | 0.698 | — | — | 1.088 | 1.056 | 0.768 | — | — | 0.882 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 93.86 | — | — | 531.87 | 529.17 | 11.92 | — | — | 528.33 | — | — |
| Mean distance, ‰ (H) | — | 6.515 | — | — | 243.919 | 233.047 | 0.466 | — | — | 195.112 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 80.5 | — | — | 1848.7 | 1832.3 | 9.0 | — | — | 3762.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 1.62 | 1.52 | — | — | — | 1.89 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 4.6 | 0.0 | — | — | — | 7.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 26.9 | 9.1 | — | — | — | 9.6 | — | — |
| Spiralling loops (L) | — | — | — | — | 6 | 7 | — | — | — | 8 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 68.5 | 90.9 | — | — | — | 83.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 22 | 20 | — | — | — | 33 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 70.5 | — | — | 96.8 | 94.8 | 359.6 | — | — | 96.7 | — | — |
| Time, seconds (T) | — | 3.34 | — | — | 2.13 | 3.92 | 181.10 | — | — | 6.58 | — | — |

Failures: quadriflow: cancelled.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6963 | 2975 | 4807 | 6773 | 7903 | 8099 | 11819 | 7183 | 11819 | 11815 | — | — |
| Irregular vertices (S) | 188 | 108 | 273 | 124 | 130 | 231 | 296 | 253 | 296 | 317 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2067 | — | 3587 | 4296 | 5481 | 7242 | 8228 | 4351 | 8228 | 10028 | — | — |
| Alignment, degrees (A) | 21.04 | 20.95 | 21.43 | 17.68 | 17.74 | 17.48 | 18.10 | 18.39 | 18.33 | 17.42 | — | — |
| Mean angle deviation, degrees (F) | 5.79 | 7.30 | 7.45 | 10.25 | 6.61 | 6.75 | 8.69 | 7.98 | 8.69 | 6.37 | — | — |
| Angle deviation, 95th percentile (F) | 21.42 | 26.48 | 25.48 | 26.71 | 18.84 | 18.52 | 25.15 | 22.11 | 25.15 | 17.19 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.481 | 1.264 | 1.320 | 1.291 | 1.297 | 1.668 | 1.342 | 1.668 | 1.255 | — | — |
| Area variation (F) | 0.185 | 0.463 | 0.215 | 0.287 | 0.307 | 0.499 | 0.518 | 0.237 | 0.518 | 0.502 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 16.98 | 26.34 | 130.45 | 15.44 | 10.62 | 8.42 | 10.70 | 16.84 | 10.70 | 8.56 | — | — |
| Mean distance, ‰ (H) | 1.071 | 1.562 | 1.477 | 0.991 | 0.745 | 0.621 | 0.719 | 1.071 | 0.719 | 0.504 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 34.0 | 47.6 | 41.7 | 38.6 | 23.3 | 18.1 | 20.8 | 34.8 | 27.4 | 18.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.8 | 1.4 | 0.0 | — | — | 2.4 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.3 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 0 | 1 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.9 | 71.1 | — | — | 82.8 | 94.5 | 82.6 | — | — | 92.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 55 | — | — | 181 | 138 | 166 | — | — | 197 | — | — |
| Edges on loops from a border to a border, % (L) | 46.1 | 28.9 | — | — | 16.4 | 4.1 | 17.1 | — | — | 5.6 | — | — |
| Off budget, % (N) | 13.0 | 62.8 | 39.9 | 15.3 | 1.2 | 1.2 | 47.7 | 10.2 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 19.08 | 4.25 | 10.05 | 45.81 | 11.78 | 13.86 | 24.96 | 24.23 | 24.96 | 19.52 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2996 | 5345 | 6148 | 6577 | 7958 | 55181 | 8791 | 55181 | 53768 | — | — |
| Irregular vertices (S) | — | 419 | 420 | 166 | 538 | 579 | 3155 | 1146 | 3155 | 1179 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4008 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.27 | 9.69 | 6.08 | 4.92 | 4.67 | 11.56 | 8.99 | 12.11 | 5.07 | — | — |
| Mean angle deviation, degrees (F) | — | 19.93 | 8.33 | 7.42 | 10.49 | 8.33 | 11.13 | 8.88 | 11.13 | 5.78 | — | — |
| Angle deviation, 95th percentile (F) | — | 77.95 | 32.25 | 24.85 | 63.48 | 51.69 | 36.88 | 32.39 | 36.88 | 20.97 | — | — |
| Mean aspect ratio (F) | — | 3.957 | 1.365 | 1.551 | 41.350 | 51.364 | 1.978 | 1.569 | 1.978 | 5.443 | — | — |
| Area variation (F) | — | 1.337 | 0.299 | 0.358 | 3.040 | 1.753 | 1.998 | 0.626 | 1.998 | 0.887 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.28 | 32.21 | 14.95 | 27.63 | 30.95 | 13.01 | 18.76 | 13.01 | 13.50 | — | — |
| Mean distance, ‰ (H) | — | 3.494 | 3.712 | 2.684 | 2.280 | 2.069 | 0.784 | 1.567 | 0.784 | 0.604 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.7 | 60.8 | 45.3 | 32.5 | 32.3 | 17.5 | 26.0 | 49.4 | 41.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | 22.7 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | 21.0 | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | 404 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | 38.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | 673 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | 18.0 | — | — |
| Off budget, % (N) | — | 62.5 | 33.2 | 23.2 | 17.8 | 0.5 | 589.8 | 9.9 | 0.0 | 2.6 | — | — |
| Time, seconds (T) | — | 2.97 | 6.80 | 72.10 | 10.87 | 11.04 | 13.92 | 11.71 | 13.92 | 92.93 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2388 | 4146 | — | 12272 | 11937 | 246063 | 14915 | 246063 | 231766 | — | — |
| Irregular vertices (S) | — | 1056 | 1524 | — | 6104 | 6245 | 10419 | 5525 | 10419 | 12168 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 3685 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 15.08 | 17.92 | — | 11.62 | 12.16 | 10.30 | 13.42 | 11.42 | 8.38 | — | — |
| Mean angle deviation, degrees (F) | — | 28.51 | 20.55 | — | 17.26 | 17.78 | 8.58 | 15.13 | 8.58 | 4.94 | — | — |
| Angle deviation, 95th percentile (F) | — | 82.76 | 58.40 | — | 63.01 | 62.87 | 25.88 | 47.93 | 25.88 | 18.62 | — | — |
| Mean aspect ratio (F) | — | 7.658 | 3.745 | — | 5.468 | 3.730 | 1.549 | 1.767 | 1.549 | 1.336 | — | — |
| Area variation (F) | — | 1.726 | 0.689 | — | 2.323 | 2.344 | 1.812 | 1.232 | 1.812 | 0.540 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 6.92 | 6.21 | — | 3.78 | 3.95 | 2.08 | 2.57 | 2.08 | 3.02 | — | — |
| Mean distance, ‰ (H) | — | 1.765 | 1.017 | — | 0.312 | 0.320 | 0.091 | 0.324 | 0.091 | 0.033 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 94.0 | 58.2 | — | 15.5 | 15.5 | 10.1 | 19.7 | 52.2 | 18.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 17.1 | 15.1 | — | — | — | 37.6 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 25.7 | 25.2 | — | — | — | 12.0 | — | — |
| Spiralling loops (L) | — | — | — | — | 3938 | 3895 | — | — | — | 1226 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 56.6 | 59.2 | — | — | — | 44.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 36 | 40 | — | — | — | 351 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.6 | 0.4 | — | — | — | 6.5 | — | — |
| Off budget, % (N) | — | 70.2 | 48.2 | — | 53.4 | 49.2 | 2975.8 | 86.4 | 0.0 | 5.8 | — | — |
| Time, seconds (T) | — | 1.13 | 4.87 | — | 14.75 | 25.69 | 13.22 | 13.40 | 13.22 | 126.83 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 10 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 6227 | 4548 | 6707 | 9161 | 9383 | 46194 | 10673 | 46194 | 43255 | — | — |
| Irregular vertices (S) | — | 2171 | 863 | 645 | 1856 | 1773 | 2662 | 1428 | 2662 | 1960 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 3548 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 10.29 | 8.12 | 5.58 | 5.63 | 4.85 | 4.69 | 5.48 | 4.75 | 3.89 | — | — |
| Mean angle deviation, degrees (F) | — | 29.31 | 14.41 | 9.45 | 11.02 | 10.05 | 9.22 | 10.50 | 9.22 | 5.98 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.69 | 54.18 | 32.95 | 53.16 | 55.59 | 33.32 | 37.63 | 33.32 | 22.31 | — | — |
| Mean aspect ratio (F) | — | 6.307 | 1.909 | 1.788 | 66.504 | 56.223 | 2.322 | 2.777 | 2.322 | 14.182 | — | — |
| Area variation (F) | — | 1.369 | 0.470 | 0.391 | 1.637 | 1.796 | 1.514 | 0.867 | 1.514 | 0.759 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 80.26 | 73.99 | 36.01 | 17.66 | 16.56 | 6.30 | 42.51 | 6.30 | 12.17 | — | — |
| Mean distance, ‰ (H) | — | 5.085 | 10.384 | 2.811 | 0.741 | 0.646 | 0.257 | 1.405 | 0.257 | 0.210 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 60.6 | 128.6 | 37.0 | 8.5 | 7.7 | 4.1 | 18.5 | 9.1 | 7.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 22.2 | 43.1 | 16.2 | 14.5 | 17.3 | 477.4 | 33.4 | 0.0 | 6.4 | — | — |
| Time, seconds (T) | — | 3.57 | 5.80 | 285.92 | 16.65 | 13.83 | 10.81 | 9.70 | 10.81 | 86.49 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4114 | 6839 | 7017 | 7720 | 8045 | 20326 | 10881 | 20326 | 19726 | — | — |
| Irregular vertices (S) | — | 403 | 532 | 170 | 235 | 245 | 458 | 360 | 458 | 289 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 4440 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.73 | 6.43 | 3.94 | 4.27 | 3.43 | 3.48 | 3.17 | 3.48 | 3.31 | — | — |
| Mean angle deviation, degrees (F) | — | 18.87 | 9.46 | 5.96 | 12.95 | 14.01 | 5.75 | 5.64 | 5.75 | 8.03 | — | — |
| Angle deviation, 95th percentile (F) | — | 74.17 | 38.06 | 23.18 | 72.63 | 72.50 | 21.00 | 22.13 | 21.00 | 43.37 | — | — |
| Mean aspect ratio (F) | — | 3.158 | 1.596 | 1.823 | 11.977 | 23.100 | 6.194 | 9.720 | 6.194 | 5.461 | — | — |
| Area variation (F) | — | 1.607 | 0.398 | 0.330 | 1.023 | 1.095 | 1.143 | 0.765 | 1.143 | 0.748 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 58.33 | 36.49 | 36.30 | 32.96 | 32.96 | 7.91 | 17.13 | 7.91 | 20.93 | — | — |
| Mean distance, ‰ (H) | — | 3.579 | 1.944 | 1.189 | 1.041 | 0.993 | 0.334 | 0.577 | 0.334 | 0.338 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 32.4 | 21.3 | 11.1 | 11.1 | 11.1 | 2.8 | 4.7 | 3.9 | 9.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | 31.7 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | 8.2 | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | 17 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | 33.6 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | 664 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | 26.5 | — | — | — | — | — |
| Off budget, % (N) | — | 48.6 | 14.5 | 12.3 | 3.5 | 0.6 | 154.1 | 36.0 | 0.0 | 3.0 | — | — |
| Time, seconds (T) | — | 1.11 | 2.76 | 146.04 | 11.00 | 11.66 | 7.25 | 6.14 | 7.25 | 22.41 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2119 | 7656 | 6579 | 6750 | 6830 | 28909 | 6170 | 28909 | 25282 | — | — |
| Irregular vertices (S) | — | 195 | 835 | 380 | 422 | 462 | 961 | 603 | 961 | 719 | — | — |
| … of which above the minimum | — | — | 730 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 7378 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 13.19 | 15.59 | 10.07 | 10.44 | 10.78 | 9.87 | 10.45 | 11.04 | 11.42 | — | — |
| Mean angle deviation, degrees (F) | — | 10.86 | 8.99 | 9.19 | 9.88 | 10.09 | 9.44 | 10.16 | 9.44 | 8.41 | — | — |
| Angle deviation, 95th percentile (F) | — | 32.41 | 30.80 | 26.78 | 28.15 | 28.79 | 27.87 | 30.40 | 27.87 | 25.60 | — | — |
| Mean aspect ratio (F) | — | 1.955 | 1.370 | 1.416 | 1.446 | 1.456 | 1.826 | 1.547 | 1.826 | 1.343 | — | — |
| Area variation (F) | — | 0.849 | 0.275 | 0.321 | 0.317 | 0.338 | 0.850 | 0.373 | 0.850 | 0.302 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.12 | 31.17 | 13.40 | 12.80 | 12.80 | 9.82 | 39.39 | 9.82 | 7.26 | — | — |
| Mean distance, ‰ (H) | — | 3.139 | 1.125 | 0.922 | 0.718 | 0.698 | 0.330 | 1.288 | 0.330 | 0.203 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 103.8 | 74.2 | 59.6 | 37.6 | 35.5 | 20.6 | 67.3 | 46.3 | 37.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 12.3 | — | — | 13.4 | 13.2 | — | — | — | 13.8 | — | — |
| Edges on spiralling loops, % (L) | — | 3.9 | — | — | 22.8 | 22.7 | — | — | — | 19.7 | — | — |
| Spiralling loops (L) | — | 9 | — | — | 98 | 112 | — | — | — | 90 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 83.8 | — | — | 63.8 | 64.1 | — | — | — | 66.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 57 | — | — | 110 | 110 | — | — | — | 395 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 73.5 | 4.3 | 17.8 | 15.6 | 14.6 | 261.4 | 22.9 | 0.0 | 12.5 | — | — |
| Time, seconds (T) | — | 2.60 | 1.15 | 117.14 | 87.19 | 85.39 | 41.01 | 41.67 | 41.01 | 508.66 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7736 | 6520 | 8190 | 7568 | 8056 | 7991 | 7088 | 7088 | 7088 | 7072 | — | — |
| Irregular vertices (S) | 12 | 26 | 193 | 26 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 4 | 18 | 185 | 18 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 7729 | 6252 | 8190 | 7568 | 6 | 7991 | 4992 | 4992 | 4992 | 6 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.93 | 5.85 | 3.49 | 6.08 | 1.51 | 1.66 | 7.65 | 7.65 | 7.65 | 1.51 | — | — |
| Angle deviation, 95th percentile (F) | 12.52 | 14.46 | 11.93 | 13.87 | 3.79 | 3.98 | 18.00 | 18.00 | 18.00 | 3.71 | — | — |
| Mean aspect ratio (F) | 1.109 | 1.337 | 1.097 | 1.149 | 1.110 | 1.119 | 1.187 | 1.187 | 1.187 | 1.123 | — | — |
| Area variation (F) | 0.134 | 0.283 | 0.122 | 0.153 | 0.125 | 0.115 | 0.391 | 0.391 | 0.391 | 0.113 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.42 | 0.80 | 0.43 | 0.40 | 0.29 | 0.30 | 0.55 | 0.55 | 0.55 | 0.31 | — | — |
| Mean distance, ‰ (H) | 0.085 | 0.172 | 0.082 | 0.083 | 0.073 | 0.076 | 0.159 | 0.159 | 0.159 | 0.086 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.93 | 99.83 | 99.92 | 100.01 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 2.1 | — | — | 97.3 | 0.0 | 15.2 | — | — | 97.1 | — | — |
| Edges on spiralling loops, % (L) | 52.4 | 42.1 | — | — | 0.0 | 97.9 | 60.2 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 5 | 10 | — | — | 0 | 4 | 4 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 47.6 | 55.8 | — | — | 2.7 | 2.1 | 24.6 | — | — | 2.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1548 | 584 | — | — | 38 | 74 | 1726 | — | — | 35 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 18.5 | 2.4 | 5.4 | 0.7 | 0.1 | 11.4 | 11.4 | 0.0 | 0.2 | — | — |
| Time, seconds (T) | 4.47 | 1.94 | 0.54 | 13.07 | 11.26 | 163.57 | 4.18 | 4.20 | 4.18 | 152.47 | — | — |

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
| Angle deviation, 95th percentile (F) | 19.57 | 2.30 | 15.85 | 1.45 | 1.41 | 1.41 | 10.81 | 10.81 | 10.81 | 1.47 | — | — |
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
| Time, seconds (T) | 5.42 | 2.20 | 0.91 | 6.48 | 4.67 | 5.36 | 6.20 | 5.73 | 6.20 | 5.40 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 6 | 0 | 7 | 2 | 1 | 1 | 2 | 1 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 2 | 0 | 1 | 4 | 4 | 0 | 0 | 0 | 3 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 1 | 2 | 9 | 0 | 2 | 2 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 1 | 7 | 0 | 3 | 0 | 0 | 0 | 0 | 4 | 0 | 0 |
| Mean aspect ratio (F) | 1 | 0 | 13 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 33/75 | 4/15 | 50/75 | 8/15 | 14/15 | 14/15 | 12/15 | 9/15 | 12/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 0 | 0 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 1 | 3 | 3 | 2 | 2 | 2 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 2 | 2 | 1 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 0 | 1 | 2 | 2 | 2 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 0 | 0 | 0 |
| Valid meshes (V) | 14/20 | 0/4 | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
