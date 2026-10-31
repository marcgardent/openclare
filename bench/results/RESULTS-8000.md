# Bench results

Requested budget: 8000 quads. Median over 5 seeds; AutoRemesher, integer-grid, instant-meshes-jakob and quadwild-bimdf have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7590 | 2795 | 8152 | 7983 | 8164 | 7529 | 8126 | 8093 | — | — |
| Irregular vertices (S) | 40 | 28 | 245 | 24 | 277 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 32 | 20 | 237 | 16 | 269 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 7412 | 1171 | 8152 | 5169 | 8164 | 7529 | 4332 | 4049 | — | — |
| Alignment, degrees (A) | 14.77 | 5.92 | 12.36 | 8.10 | 15.05 | 5.65 | 5.92 | 5.40 | — | — |
| Mean angle deviation, degrees (F) | 7.77 | 9.19 | 4.25 | 7.37 | 4.85 | 5.74 | 4.88 | 4.62 | — | — |
| Angle deviation, 95th percentile (F) | 19.95 | 34.45 | 13.32 | 19.56 | 15.77 | 14.65 | 13.84 | 12.22 | — | — |
| Mean aspect ratio (F) | 1.152 | 2.101 | 1.123 | 1.226 | 1.142 | 1.223 | 1.233 | 1.212 | — | — |
| Area variation (F) | 0.177 | 0.585 | 0.148 | 0.357 | 0.153 | 0.227 | 0.251 | 0.287 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.18 | 2.78 | 4.52 | 3.98 | 4.97 | 2.45 | 1.73 | 1.82 | — | — |
| Mean distance, ‰ (H) | 0.171 | 0.760 | 0.173 | 0.586 | 0.178 | 0.093 | 0.083 | 0.076 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.84 | 98.72 | 99.85 | 99.31 | 99.86 | 100.01 | 100.00 | 100.01 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.2 | 33.0 | 0.0 | 17.9 | 0.0 | 0.0 | 24.4 | 28.7 | — | — |
| Edges on spiralling loops, % (L) | 35.9 | 18.0 | 3.6 | 65.8 | 2.4 | 68.8 | 32.8 | 25.9 | — | — |
| Spiralling loops (L) | 12 | 4 | 4 | 12 | 4 | 16 | 5 | 5 | — | — |
| Edges on helices of any pitch, % (L) | 40.1 | 7.6 | 0.0 | 60.2 | 0.0 | 71.8 | 46.3 | 30.4 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 1 | 0 | 0 | 1 | 0 | 2 | 6 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 62.6 | 49.1 | 96.4 | 16.3 | 97.6 | 31.2 | 42.8 | 45.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 766 | 292 | 162 | 487 | 196 | 976 | 1032 | 755 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 5.1 | 65.1 | 1.9 | 0.2 | 2.1 | 5.9 | 1.6 | 1.2 | — | — |
| Time, seconds (T) | 3.65 | 2.13 | 1.10 | 16.47 | 1.46 | 16.82 | 13.08 | 11.94 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7342 | 5842 | 8006 | 8115 | 7792 | 7291 | 8037 | 8081 | — | — |
| Irregular vertices (S) | 10 | 34 | 156 | 52 | 162 | 114 | 116 | 132 | — | — |
| … of which above the minimum | 2 | 26 | 148 | 44 | 154 | 106 | 108 | 124 | — | — |
| Base complex patches (C) | 7342 | 5052 | 8006 | 8115 | 7792 | 7291 | 8037 | 8081 | — | — |
| Alignment, degrees (A) | 22.36 | 18.66 | 22.71 | 20.16 | 22.82 | 15.98 | 16.51 | 16.43 | — | — |
| Mean angle deviation, degrees (F) | 6.22 | 6.90 | 3.52 | 9.40 | 4.13 | 8.78 | 5.94 | 6.25 | — | — |
| Angle deviation, 95th percentile (F) | 18.32 | 17.38 | 10.68 | 26.29 | 12.43 | 21.05 | 14.46 | 15.62 | — | — |
| Mean aspect ratio (F) | 1.158 | 1.393 | 1.094 | 1.250 | 1.115 | 1.250 | 1.254 | 1.268 | — | — |
| Area variation (F) | 0.244 | 0.344 | 0.121 | 0.345 | 0.128 | 0.210 | 0.255 | 0.274 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.95 | 3.88 | 2.88 | 3.75 | 3.22 | 2.99 | 2.72 | 2.61 | — | — |
| Mean distance, ‰ (H) | 0.478 | 0.669 | 0.412 | 0.630 | 0.433 | 0.417 | 0.391 | 0.391 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.79 | 99.91 | 99.79 | 99.89 | 100.00 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 6.9 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 70.9 | 30.4 | 0.0 | 53.2 | 2.8 | 5.8 | 1.6 | 0.0 | — | — |
| Spiralling loops (L) | 6 | 7 | 0 | 20 | 2 | 3 | 1 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 89.2 | 23.5 | 0.0 | 18.5 | 0.0 | 2.2 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 29.1 | 62.6 | 100.0 | 46.8 | 97.2 | 94.2 | 98.4 | 100.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1193 | 618 | 242 | 563 | 266 | 610 | 379 | 298 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 8.2 | 27.0 | 0.1 | 1.4 | 2.6 | 8.9 | 0.5 | 1.0 | — | — |
| Time, seconds (T) | 3.12 | 2.10 | 2.70 | 48.18 | 1.00 | 35.03 | 17.56 | 18.59 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8170 | 6819 | 8100 | 7982 | 8234 | 6919 | 8044 | 7939 | — | — |
| Irregular vertices (S) | 22 | 18 | 183 | 28 | 214 | 38 | 40 | 42 | — | — |
| … of which above the minimum | 14 | 10 | 175 | 20 | 206 | — | 32 | 34 | — | — |
| Base complex patches (C) | 8170 | 3683 | 6854 | 5112 | 7510 | 5302 | 1898 | 1815 | — | — |
| Alignment, degrees (A) | 13.70 | 9.77 | 11.47 | 11.25 | 11.81 | 6.84 | 6.38 | 5.70 | — | — |
| Mean angle deviation, degrees (F) | 6.02 | 3.78 | 3.38 | 8.54 | 3.91 | 9.16 | 6.97 | 6.02 | — | — |
| Angle deviation, 95th percentile (F) | 15.98 | 10.36 | 10.60 | 21.86 | 12.20 | 24.39 | 18.18 | 15.83 | — | — |
| Mean aspect ratio (F) | 1.154 | 1.965 | 1.097 | 1.290 | 1.116 | 1.349 | 1.352 | 1.314 | — | — |
| Area variation (F) | 0.157 | 0.565 | 0.124 | 0.334 | 0.127 | 0.330 | 0.390 | 0.498 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.19 | 1.09 | 1.34 | 2.83 | 1.70 | 2.49 | 1.00 | 0.87 | — | — |
| Mean distance, ‰ (H) | 0.100 | 0.161 | 0.106 | 0.344 | 0.106 | 0.098 | 0.083 | 0.083 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.74 | 99.81 | 99.30 | 99.81 | — | 100.01 | 100.01 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 23.0 | 7.7 | 18.9 | 4.4 | 10.7 | 42.7 | 42.4 | — | — |
| Edges on spiralling loops, % (L) | 44.8 | 43.6 | 4.7 | 62.5 | 9.8 | 49.2 | 9.7 | 5.9 | — | — |
| Spiralling loops (L) | 5 | 9 | 6 | 12 | 11 | 17 | 3 | 2 | — | — |
| Edges on helices of any pitch, % (L) | 70.7 | 36.9 | 1.3 | 50.2 | 4.8 | 39.2 | 6.4 | 3.2 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 8 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 55.2 | 33.4 | 87.6 | 18.6 | 87.6 | 38.6 | 47.6 | 51.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1471 | 594 | 210 | 435 | 230 | 303 | 625 | 682 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 1.5 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.1 | 14.8 | 1.2 | 0.2 | 2.9 | 13.5 | 0.5 | 0.8 | — | — |
| Time, seconds (T) | 4.99 | 2.51 | 1.41 | 11.38 | 1.05 | 33.97 | 17.95 | 12.87 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7932 | 5928 | 8024 | 7841 | 8050 | 7793 | 7963 | 7980 | — | — |
| Irregular vertices (S) | 72 | 58 | 218 | 50 | 250 | 46 | 48 | 58 | — | — |
| … of which above the minimum | 40 | — | 186 | 18 | 218 | 14 | 16 | 26 | — | — |
| Base complex patches (C) | 7283 | — | 6544 | 1041 | 7125 | 1139 | 161 | 235 | — | — |
| Alignment, degrees (A) | 8.22 | 5.16 | 7.52 | 6.27 | 8.80 | 3.88 | 2.62 | 2.26 | — | — |
| Mean angle deviation, degrees (F) | 5.83 | 7.40 | 4.06 | 7.55 | 4.40 | 3.63 | 1.70 | 2.21 | — | — |
| Angle deviation, 95th percentile (F) | 16.15 | 22.71 | 12.13 | 19.81 | 14.23 | 13.49 | 5.82 | 6.67 | — | — |
| Mean aspect ratio (F) | 1.140 | 2.161 | 1.121 | 1.413 | 1.144 | 1.173 | 1.185 | 1.187 | — | — |
| Area variation (F) | 0.139 | 0.487 | 0.130 | 0.336 | 0.138 | 0.171 | 0.194 | 0.335 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.65 | 7.28 | 8.22 | 12.55 | 9.87 | 3.88 | 3.36 | 2.70 | — | — |
| Mean distance, ‰ (H) | 0.294 | 0.343 | 0.308 | 0.629 | 0.344 | 0.246 | 0.237 | 0.206 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.64 | 99.99 | 99.56 | 99.13 | 99.48 | 99.92 | 99.92 | 99.93 | — | — |
| Sharp edges kept, % of their length (E) | 66.3 | 56.6 | 63.0 | 37.7 | 65.6 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 5.0 | 25.6 | 9.5 | 63.1 | 6.4 | 62.7 | 85.2 | 82.4 | — | — |
| Edges on spiralling loops, % (L) | 55.9 | 48.4 | 31.3 | 17.1 | 33.4 | 18.6 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 33 | 29 | 40 | 7 | 43 | 8 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 47.3 | 44.5 | 10.9 | 16.1 | 21.4 | 18.6 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 40.8 | 26.0 | 59.2 | 19.8 | 59.9 | 18.7 | 14.8 | 17.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 332 | 108 | 134 | 479 | 138 | 128 | 125 | 103 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.9 | 25.9 | 0.3 | 2.0 | 0.6 | 2.6 | 0.5 | 0.2 | — | — |
| Time, seconds (T) | 3.55 | 2.42 | 0.31 | 12.26 | 0.72 | 23.88 | 6.49 | 7.15 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 2647 | 8156 | 8101 | 8098 | 7521 | 8082 | 8030 | — | — |
| Irregular vertices (S) | 55 | 84 | 343 | 56 | 346 | 60 | 59 | 76 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6194 | — | 7722 | 3188 | 7878 | 4210 | 2738 | 3384 | — | — |
| Alignment, degrees (A) | 14.27 | 12.11 | 15.57 | 14.07 | 16.64 | 10.27 | 10.62 | 9.86 | — | — |
| Mean angle deviation, degrees (F) | 6.09 | 9.37 | 4.88 | 8.04 | 5.17 | 7.22 | 6.24 | 5.46 | — | — |
| Angle deviation, 95th percentile (F) | 18.06 | 32.08 | 16.94 | 22.39 | 17.78 | 20.03 | 16.44 | 14.37 | — | — |
| Mean aspect ratio (F) | 1.208 | 1.892 | 1.144 | 1.212 | 1.169 | 1.223 | 1.269 | 1.221 | — | — |
| Area variation (F) | 0.208 | 0.650 | 0.159 | 0.284 | 0.164 | 0.213 | 0.307 | 0.348 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.87 | 31.86 | 4.86 | 7.08 | 5.94 | 4.48 | 3.88 | 3.46 | — | — |
| Mean distance, ‰ (H) | 0.267 | 1.112 | 0.268 | 0.530 | 0.286 | 0.256 | 0.203 | 0.178 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 47.8 | 42.9 | 51.6 | 52.0 | 52.2 | 74.1 | 98.9 | 99.3 | — | — |
| Edges on closed loops, % (L) | 1.4 | 9.5 | 2.0 | 22.5 | 0.2 | 9.9 | 23.3 | 20.7 | — | — |
| Edges on spiralling loops, % (L) | 11.3 | 20.3 | 2.0 | 13.0 | 4.1 | 25.0 | 12.7 | 4.8 | — | — |
| Spiralling loops (L) | 6 | 12 | 4 | 4 | 6 | 12 | 7 | 5 | — | — |
| Edges on helices of any pitch, % (L) | 17.7 | 10.0 | 1.0 | 8.3 | 1.7 | 23.1 | 8.7 | 6.5 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 5 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 68.7 | 70.2 | 95.2 | 50.6 | 94.3 | 49.8 | 46.9 | 59.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 512 | 98 | 220 | 420 | 168 | 591 | 314 | 364 | — | — |
| Edges on loops from a border to a border, % (L) | 19.0 | 0.0 | 0.8 | 13.9 | 1.3 | 15.3 | 17.2 | 14.7 | — | — |
| Off budget, % (N) | 0.9 | 66.9 | 1.9 | 1.3 | 1.2 | 6.0 | 1.0 | 0.4 | — | — |
| Time, seconds (T) | 5.01 | 1.98 | 0.32 | 16.89 | 0.66 | 22.29 | 13.14 | 26.36 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 1/1 | 0/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7763 | 4622 | 8042 | 8168 | 7836 | 6846 | 8034 | 8086 | — | — |
| Irregular vertices (S) | 162 | 141 | 379 | 192 | 467 | 160 | 186 | 226 | — | — |
| … of which above the minimum | 154 | — | — | 184 | 432 | 152 | 178 | 218 | — | — |
| Base complex patches (C) | 7352 | — | 7858 | 6315 | 7682 | 5228 | 4846 | 5188 | — | — |
| Alignment, degrees (A) | 17.78 | 12.94 | 17.06 | 15.71 | 17.72 | 12.35 | 12.29 | 11.82 | — | — |
| Mean angle deviation, degrees (F) | 11.59 | 7.43 | 5.31 | 12.81 | 6.20 | 9.05 | 7.45 | 8.26 | — | — |
| Angle deviation, 95th percentile (F) | 33.33 | 22.75 | 18.30 | 34.75 | 20.95 | 25.06 | 21.40 | 22.14 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.812 | 1.205 | 1.590 | 1.225 | 1.409 | 1.514 | 1.513 | — | — |
| Area variation (F) | 0.312 | 0.668 | 0.194 | 0.518 | 0.196 | 0.282 | 0.332 | 0.493 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 27.28 | 27.93 | 20.78 | 8.17 | 30.25 | 27.49 | 10.88 | 8.55 | — | — |
| Mean distance, ‰ (H) | 0.521 | 1.054 | 0.510 | 0.792 | 0.618 | 0.406 | 0.305 | 0.298 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.1 | 78.5 | 36.0 | 17.1 | 37.9 | 22.6 | 10.4 | 3.3 | — | — |
| Volume kept, % of the source (V) | 99.49 | 98.66 | — | 98.99 | 99.40 | 99.91 | 99.99 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 40.5 | 26.6 | 30.1 | 12.2 | 26.3 | 66.4 | 76.7 | 74.0 | — | — |
| Edges on closed loops, % (L) | 2.0 | 17.3 | 1.1 | 11.7 | 1.5 | 11.9 | 20.0 | 18.0 | — | — |
| Edges on spiralling loops, % (L) | 13.7 | 18.1 | 8.7 | 24.9 | 1.7 | 29.6 | 4.6 | 5.7 | — | — |
| Spiralling loops (L) | 23 | 23 | 36 | 20 | 17 | 45 | 10 | 13 | — | — |
| Edges on helices of any pitch, % (L) | 25.4 | 5.3 | 3.2 | 16.9 | 0.0 | 12.4 | 0.2 | 3.8 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 84.3 | 64.7 | 90.1 | 63.4 | 95.8 | 58.5 | 75.4 | 76.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 484 | 175 | 172 | 253 | 136 | 306 | 308 | 209 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.0 | 42.2 | 0.5 | 2.1 | 2.1 | 14.4 | 0.4 | 1.1 | — | — |
| Time, seconds (T) | 4.24 | 3.10 | 2.03 | 58.23 | 2.09 | 64.92 | 163.11 | 167.28 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2520 | 7877 | 7971 | 7652 | 7430 | 8109 | 8095 | 10590 | 10484 |
| Irregular vertices (S) | — | 156 | 345 | 62 | 400 | 111 | 148 | 226 | 476 | 254 |
| … of which above the minimum | — | — | — | 54 | — | — | 140 | 218 | — | 246 |
| Base complex patches (C) | — | — | 5733 | 3114 | 5799 | — | 2439 | 3023 | — | 3162 |
| Alignment, degrees (A) | — | 10.84 | 12.09 | 10.43 | 11.76 | 7.97 | 7.98 | 8.09 | 9.88 | 8.34 |
| Mean angle deviation, degrees (F) | — | 8.90 | 5.75 | 8.93 | 6.09 | 5.72 | 4.07 | 4.48 | 9.24 | 4.29 |
| Angle deviation, 95th percentile (F) | — | 29.26 | 21.35 | 25.68 | 22.28 | 18.23 | 12.85 | 14.36 | 26.27 | 13.79 |
| Mean aspect ratio (F) | — | 1.933 | 1.222 | 1.492 | 1.235 | 1.336 | 1.361 | 1.336 | 1.616 | 1.345 |
| Area variation (F) | — | 0.738 | 0.206 | 0.445 | 0.213 | 0.301 | 0.361 | 0.503 | 1.417 | 0.461 |
| Hausdorff, ‰ of the diagonal (H) | — | 23.89 | 31.23 | 56.73 | 35.02 | 11.15 | 9.24 | 4.94 | 6.47 | 4.74 |
| Mean distance, ‰ (H) | — | 1.795 | 1.354 | 5.028 | 2.033 | 0.962 | 0.524 | 0.236 | 0.172 | 0.190 |
| Thin parts: mean distance, % of a quad edge (V) | — | 36.5 | 40.4 | 138.1 | 57.2 | 29.9 | 18.8 | 5.3 | 1.5 | 5.8 |
| Volume kept, % of the source (V) | — | — | — | 96.41 | — | 99.92 | 99.98 | 100.02 | 98.80 | 100.02 |
| Sharp edges kept, % of their length (E) | — | 2.8 | 16.7 | 0.0 | 22.2 | 76.1 | 87.1 | 93.5 | 71.3 | 94.3 |
| Edges on closed loops, % (L) | — | — | 13.3 | 33.3 | 12.1 | 39.4 | 35.1 | 32.7 | 31.6 | 36.5 |
| Edges on spiralling loops, % (L) | — | — | 9.7 | 24.1 | 7.3 | 21.2 | 6.9 | 3.9 | 2.5 | 1.5 |
| Spiralling loops (L) | — | — | 51 | 6 | 37 | 45 | 31 | 11 | 3 | 11 |
| Edges on helices of any pitch, % (L) | — | — | 4.2 | 20.2 | 5.0 | 9.5 | 1.8 | 3.2 | 0.0 | 0.0 |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 76.6 | 42.6 | 79.3 | 39.4 | 58.0 | 63.3 | 65.8 | 62.1 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 178 | 670 | 180 | 320 | 539 | 401 | 274 | 401 |
| Edges on loops from a border to a border, % (L) | — | — | 0.4 | 0.0 | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.5 | 1.5 | 0.4 | 4.3 | 7.1 | 1.4 | 1.2 | 0.0 | 1.0 |
| Time, seconds (T) | — | 5.54 | 9.43 | 22.95 | 19.64 | 51.03 | 78.94 | 160.95 | — | 165.69 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 2564 | 7907 | 7945 | 7648 | 7230 | 7881 | 8045 | 10590 | 10551 |
| Irregular vertices (S) | — | 150 | 358 | 68 | 414 | 137 | 160 | 234 | 476 | 274 |
| … of which above the minimum | — | — | — | 60 | 395 | — | 152 | 226 | — | 266 |
| Base complex patches (C) | — | — | 5719 | 6217 | 6412 | — | 2639 | 2805 | — | 3845 |
| Alignment, degrees (A) | — | 9.84 | 11.15 | 10.90 | 11.62 | 7.78 | 7.71 | 7.08 | 9.43 | 7.33 |
| Mean angle deviation, degrees (F) | — | 8.28 | 5.91 | 9.45 | 6.50 | 6.25 | 4.54 | 4.22 | 8.82 | 4.37 |
| Angle deviation, 95th percentile (F) | — | 26.20 | 20.66 | 25.95 | 22.21 | 20.78 | 16.18 | 13.88 | 25.47 | 13.74 |
| Mean aspect ratio (F) | — | 1.962 | 1.195 | 1.414 | 1.237 | 1.307 | 1.411 | 1.272 | 1.657 | 1.277 |
| Area variation (F) | — | 0.794 | 0.206 | 0.419 | 0.217 | 0.309 | 0.401 | 0.472 | 1.432 | 0.484 |
| Hausdorff, ‰ of the diagonal (H) | — | 26.27 | 25.67 | 52.73 | 32.73 | 11.94 | 9.76 | 5.57 | 4.44 | 4.09 |
| Mean distance, ‰ (H) | — | 2.147 | 1.607 | 4.729 | 2.076 | 0.993 | 0.612 | 0.289 | 0.156 | 0.182 |
| Thin parts: mean distance, % of a quad edge (V) | — | 50.9 | 61.7 | 169.3 | 73.5 | 40.0 | 24.2 | 7.1 | 1.4 | 5.6 |
| Volume kept, % of the source (V) | — | — | — | 96.14 | 98.91 | — | 99.95 | 100.03 | 98.78 | 100.02 |
| Sharp edges kept, % of their length (E) | — | 10.7 | 48.2 | 0.0 | 26.2 | 94.1 | 86.4 | 95.8 | 67.0 | 88.9 |
| Edges on closed loops, % (L) | — | — | 13.7 | 11.0 | 8.3 | 18.2 | 33.3 | 33.8 | 31.6 | 31.8 |
| Edges on spiralling loops, % (L) | — | — | 9.3 | 53.3 | 10.6 | 30.7 | 9.1 | 3.4 | 2.5 | 1.4 |
| Spiralling loops (L) | — | — | 48 | 21 | 49 | 45 | 40 | 10 | 3 | 6 |
| Edges on helices of any pitch, % (L) | — | — | 3.3 | 43.6 | 7.3 | 17.9 | 2.3 | 0.3 | 0.0 | 0.6 |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 76.6 | 35.8 | 80.4 | 51.0 | 57.7 | 62.7 | 65.8 | 66.9 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 182 | 343 | 166 | 445 | 340 | 269 | 274 | 485 |
| Edges on loops from a border to a border, % (L) | — | — | 0.4 | 0.0 | 0.3 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 68.0 | 1.2 | 0.7 | 4.4 | 9.6 | 1.5 | 0.6 | 0.0 | 0.4 |
| Time, seconds (T) | — | 6.08 | 8.71 | 26.37 | 22.42 | 51.35 | 88.59 | 143.55 | — | 197.27 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7874 | 2895 | 8076 | 8156 | 8160 | 7188 | 8011 | 8054 | — | — |
| Irregular vertices (S) | 76 | 74 | 206 | 68 | 281 | 72 | 70 | 70 | — | — |
| … of which above the minimum | 68 | — | 198 | 60 | 273 | 64 | 62 | 62 | — | — |
| Base complex patches (C) | 7772 | — | 8034 | 3980 | 8022 | 3822 | 2571 | 2500 | — | — |
| Alignment, degrees (A) | 16.85 | 9.88 | 16.03 | 11.62 | 17.02 | 8.93 | 8.64 | 8.01 | — | — |
| Mean angle deviation, degrees (F) | 6.23 | 7.56 | 3.80 | 7.91 | 4.53 | 8.77 | 6.35 | 5.97 | — | — |
| Angle deviation, 95th percentile (F) | 16.10 | 20.17 | 12.33 | 21.47 | 15.39 | 22.64 | 16.68 | 15.19 | — | — |
| Mean aspect ratio (F) | 1.177 | 1.481 | 1.102 | 1.378 | 1.142 | 1.329 | 1.353 | 1.358 | — | — |
| Area variation (F) | 0.178 | 0.712 | 0.134 | 0.386 | 0.144 | 0.278 | 0.319 | 0.419 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.30 | 6.25 | 5.25 | 4.44 | 10.14 | 3.03 | 2.54 | 3.47 | — | — |
| Mean distance, ‰ (H) | 0.161 | 0.635 | 0.166 | 0.500 | 0.172 | 0.090 | 0.082 | 0.080 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 98.89 | 99.78 | 99.20 | 99.77 | 100.00 | 100.01 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 46.9 | 43.1 | 52.1 | 23.7 | 14.7 | 96.0 | 83.9 | 89.3 | — | — |
| Edges on closed loops, % (L) | 0.1 | 16.4 | 0.3 | 26.8 | 1.0 | 24.5 | 38.6 | 39.2 | — | — |
| Edges on spiralling loops, % (L) | 19.0 | 13.5 | 9.6 | 28.5 | 7.3 | 30.4 | 12.7 | 5.8 | — | — |
| Spiralling loops (L) | 9 | 7 | 15 | 13 | 11 | 15 | 7 | 3 | — | — |
| Edges on helices of any pitch, % (L) | 28.7 | 9.1 | 3.6 | 18.3 | 1.8 | 20.0 | 4.7 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.3 | 70.1 | 90.1 | 44.8 | 91.4 | 45.1 | 48.7 | 55.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 578 | 134 | 324 | 442 | 234 | 410 | 363 | 449 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.6 | 63.8 | 0.9 | 1.9 | 2.0 | 10.2 | 0.1 | 0.7 | — | — |
| Time, seconds (T) | 3.14 | 1.70 | 0.73 | 16.22 | 1.48 | 54.88 | 25.37 | 22.34 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 1/1 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8263 | 4012 | 7884 | 8101 | 8068 | 6943 | 8111 | 8034 | — | — |
| Irregular vertices (S) | 36 | 20 | 204 | 18 | 249 | 17 | 20 | 20 | — | — |
| … of which above the minimum | 36 | 20 | 204 | 18 | 249 | — | 20 | 20 | — | — |
| Base complex patches (C) | 8263 | 1318 | 7884 | 656 | 8068 | 2022 | 1429 | 255 | — | — |
| Alignment, degrees (A) | 17.31 | 5.28 | 19.79 | 6.58 | 19.67 | 4.00 | 3.65 | 3.65 | — | — |
| Mean angle deviation, degrees (F) | 6.39 | 4.91 | 4.09 | 7.80 | 4.56 | 4.85 | 3.52 | 3.63 | — | — |
| Angle deviation, 95th percentile (F) | 17.71 | 13.45 | 12.26 | 19.72 | 14.77 | 15.41 | 10.71 | 11.45 | — | — |
| Mean aspect ratio (F) | 1.142 | 2.024 | 1.104 | 1.391 | 1.129 | 1.386 | 1.377 | 1.368 | — | — |
| Area variation (F) | 0.159 | 0.485 | 0.128 | 0.505 | 0.141 | 0.342 | 0.367 | 0.418 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.23 | 1.62 | 2.92 | 4.64 | 3.43 | 5.92 | 0.64 | 0.84 | — | — |
| Mean distance, ‰ (H) | 0.104 | 0.346 | 0.121 | 0.420 | 0.118 | 0.063 | 0.055 | 0.055 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.32 | 99.79 | 99.14 | 99.79 | — | 100.01 | 100.01 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 35.6 | 0.0 | 77.7 | 0.0 | 44.5 | 42.7 | 76.9 | — | — |
| Edges on spiralling loops, % (L) | 23.4 | 40.2 | 0.0 | 14.4 | 1.6 | 34.5 | 40.2 | 6.6 | — | — |
| Spiralling loops (L) | 7 | 10 | 0 | 5 | 3 | 8 | 4 | 3 | — | — |
| Edges on helices of any pitch, % (L) | 59.2 | 34.8 | 0.0 | 14.4 | 0.0 | 28.7 | 35.7 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 76.6 | 24.2 | 100.0 | 7.9 | 98.4 | 11.6 | 17.1 | 16.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 840 | 228 | 220 | 184 | 232 | 224 | 212 | 204 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 9.4 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 49.9 | 1.5 | 1.3 | 0.9 | 13.2 | 1.4 | 0.4 | — | — |
| Time, seconds (T) | 3.34 | 1.99 | 1.12 | 15.10 | 1.16 | 25.10 | 11.41 | 9.96 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7229 | 3438 | 7946 | 7910 | 8116 | 7212 | 8003 | 7918 | — | — |
| Irregular vertices (S) | 70 | 53 | 235 | 36 | 267 | 43 | 44 | 54 | — | — |
| … of which above the minimum | 62 | — | 227 | 28 | 259 | 35 | 36 | 46 | — | — |
| Base complex patches (C) | 6275 | — | 7750 | 4515 | 7954 | 3852 | 1529 | 1790 | — | — |
| Alignment, degrees (A) | 15.23 | 7.01 | 13.82 | 11.11 | 14.79 | 7.90 | 7.58 | 7.28 | — | — |
| Mean angle deviation, degrees (F) | 10.30 | 6.30 | 4.39 | 9.37 | 4.31 | 8.37 | 6.18 | 6.05 | — | — |
| Angle deviation, 95th percentile (F) | 28.50 | 19.86 | 13.46 | 25.22 | 15.08 | 23.50 | 18.87 | 17.47 | — | — |
| Mean aspect ratio (F) | 1.267 | 2.320 | 1.119 | 1.419 | 1.139 | 1.315 | 1.320 | 1.364 | — | — |
| Area variation (F) | 0.371 | 0.598 | 0.144 | 0.394 | 0.143 | 0.256 | 0.295 | 0.373 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 21.90 | 8.37 | 6.73 | 7.38 | 5.90 | 5.98 | 2.73 | 2.02 | — | — |
| Mean distance, ‰ (H) | 0.213 | 0.399 | 0.128 | 0.382 | 0.131 | 0.084 | 0.074 | 0.063 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.18 | 99.72 | 99.13 | 99.71 | 99.98 | 99.99 | 99.99 | — | — |
| Sharp edges kept, % of their length (E) | 41.6 | 42.3 | 59.3 | 20.7 | 57.6 | 72.2 | 86.9 | 86.1 | — | — |
| Edges on closed loops, % (L) | 6.4 | 28.0 | 1.2 | 24.4 | 1.3 | 27.3 | 51.2 | 45.6 | — | — |
| Edges on spiralling loops, % (L) | 20.2 | 18.6 | 6.5 | 36.3 | 5.4 | 43.1 | 19.4 | 3.9 | — | — |
| Spiralling loops (L) | 12 | 5 | 21 | 10 | 12 | 16 | 4 | 3 | — | — |
| Edges on helices of any pitch, % (L) | 24.3 | 15.1 | 1.6 | 34.5 | 1.4 | 32.6 | 17.2 | 0.3 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 73.2 | 53.3 | 92.3 | 39.3 | 92.7 | 29.6 | 29.4 | 50.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 660 | 163 | 232 | 563 | 216 | 322 | 302 | 434 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 9.6 | 57.0 | 0.7 | 1.1 | 1.5 | 9.8 | 0.0 | 1.0 | — | — |
| Time, seconds (T) | 2.72 | 2.29 | 1.36 | 13.47 | 1.28 | 47.02 | 21.22 | 27.63 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6973 | 4682 | 8094 | 8056 | 8092 | 7441 | 7886 | 8001 | — | — |
| Irregular vertices (S) | 136 | 135 | 339 | 92 | 354 | 82 | 88 | 102 | — | — |
| … of which above the minimum | 128 | — | 331 | 84 | 355 | — | 80 | 94 | — | — |
| Base complex patches (C) | 6841 | — | 7482 | 6354 | 7780 | — | 3186 | 3509 | — | — |
| Alignment, degrees (A) | 15.84 | 11.22 | 14.13 | 14.32 | 14.87 | 9.90 | 9.77 | 8.79 | — | — |
| Mean angle deviation, degrees (F) | 11.44 | 6.80 | 4.59 | 12.38 | 4.98 | 8.16 | 6.68 | 5.55 | — | — |
| Angle deviation, 95th percentile (F) | 34.02 | 21.80 | 16.81 | 33.96 | 17.33 | 23.10 | 18.77 | 17.84 | — | — |
| Mean aspect ratio (F) | 1.339 | 1.773 | 1.147 | 1.488 | 1.161 | 1.243 | 1.339 | 1.302 | — | — |
| Area variation (F) | 0.383 | 0.668 | 0.155 | 0.453 | 0.159 | 0.242 | 0.344 | 0.429 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.23 | 9.84 | 17.19 | 11.25 | 16.69 | 7.27 | 10.91 | 5.08 | — | — |
| Mean distance, ‰ (H) | 0.610 | 0.627 | 0.222 | 0.562 | 0.235 | 0.166 | 0.159 | 0.116 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 64.0 | 21.7 | 21.0 | 27.6 | 14.5 | 15.5 | 7.9 | 5.0 | — | — |
| Volume kept, % of the source (V) | 99.23 | 99.18 | 99.71 | 99.27 | 99.70 | 99.96 | 99.95 | 99.98 | — | — |
| Sharp edges kept, % of their length (E) | 30.7 | 39.9 | 47.7 | 23.5 | 39.4 | 47.3 | 81.5 | 85.4 | — | — |
| Edges on closed loops, % (L) | 0.5 | 9.6 | 3.8 | 11.0 | 2.1 | 19.1 | 36.4 | 33.7 | — | — |
| Edges on spiralling loops, % (L) | 21.6 | 22.2 | 6.5 | 41.7 | 6.3 | 34.9 | 7.1 | 5.1 | — | — |
| Spiralling loops (L) | 22 | 18 | 18 | 18 | 11 | 19 | 13 | 15 | — | — |
| Edges on helices of any pitch, % (L) | 24.3 | 8.8 | 0.8 | 29.7 | 1.2 | 20.3 | 4.6 | 2.3 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.2 | 68.1 | 89.7 | 47.4 | 91.6 | 46.0 | 56.5 | 61.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 500 | 161 | 294 | 559 | 166 | 513 | 415 | 390 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 12.8 | 41.5 | 1.2 | 0.7 | 1.1 | 7.0 | 1.4 | 0.0 | — | — |
| Time, seconds (T) | 2.60 | 5.66 | 0.68 | 19.57 | 1.17 | 74.64 | 45.32 | 55.49 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 0/1 | 1/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7781 | 2740 | 8112 | 8047 | 7962 | 7348 | 7992 | 8040 | — | — |
| Irregular vertices (S) | 112 | 109 | 431 | 145 | 491 | 168 | 160 | 264 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 4991 | — | 7867 | 5271 | 7847 | — | 5085 | 6626 | — | — |
| Alignment, degrees (A) | 17.56 | 15.73 | 17.66 | 16.93 | 18.05 | 13.55 | 12.94 | 12.49 | — | — |
| Mean angle deviation, degrees (F) | 7.80 | 10.26 | 5.97 | 10.91 | 6.61 | 8.17 | 5.84 | 6.20 | — | — |
| Angle deviation, 95th percentile (F) | 22.58 | 29.98 | 21.38 | 32.52 | 22.97 | 22.66 | 15.81 | 18.41 | — | — |
| Mean aspect ratio (F) | 1.257 | 1.889 | 1.207 | 1.390 | 1.250 | 1.283 | 1.339 | 1.332 | — | — |
| Area variation (F) | 0.245 | 0.652 | 0.204 | 0.467 | 0.209 | 0.240 | 0.306 | 0.483 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 31.29 | 25.08 | 17.84 | 15.43 | 29.33 | 15.78 | 7.21 | 10.04 | — | — |
| Mean distance, ‰ (H) | 0.581 | 2.010 | 0.562 | 0.906 | 0.654 | 0.445 | 0.296 | 0.255 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.5 | 54.4 | 42.3 | 33.4 | 45.4 | 21.6 | 6.9 | 2.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 48.6 | 15.4 | 27.5 | 21.2 | 27.0 | 45.9 | 76.1 | 90.1 | — | — |
| Edges on closed loops, % (L) | 0.0 | — | 0.0 | 3.0 | 0.1 | 0.6 | 6.0 | 3.3 | — | — |
| Edges on spiralling loops, % (L) | 3.1 | — | 0.0 | 10.5 | 0.0 | 11.4 | 0.9 | 1.2 | — | — |
| Spiralling loops (L) | 5 | — | 0 | 8 | 3 | 12 | 1 | 5 | — | — |
| Edges on helices of any pitch, % (L) | 2.6 | — | 0.0 | 4.1 | 0.0 | 2.4 | 0.9 | 0.1 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.6 | — | 96.3 | 75.0 | 98.6 | 76.5 | 78.9 | 89.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 404 | — | 102 | 316 | 157 | 214 | 417 | 164 | — | — |
| Edges on loops from a border to a border, % (L) | 19.8 | — | 3.7 | 11.5 | 1.3 | 11.4 | 14.2 | 6.1 | — | — |
| Off budget, % (N) | 2.7 | 65.8 | 1.4 | 0.6 | 0.5 | 8.2 | 0.1 | 0.5 | — | — |
| Time, seconds (T) | 4.43 | 1.67 | 1.27 | 30.39 | 0.70 | 73.09 | 62.55 | 119.66 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 6758 | 3828 | 8157 | 27486 | 6927 | 6775 | 8157 | 8256 | — | — |
| Irregular vertices (S) | 917 | 556 | 1081 | 2539 | 1149 | 624 | 1321 | 2113 | — | — |
| … of which above the minimum | — | — | — | — | — | — | 705 | 1497 | — | — |
| Base complex patches (C) | 6524 | — | 7484 | 24172 | 6449 | — | 6264 | 6751 | — | — |
| Alignment, degrees (A) | 18.20 | 18.28 | 18.94 | 17.08 | 18.64 | 16.04 | 14.33 | 13.58 | — | — |
| Mean angle deviation, degrees (F) | 14.81 | 17.84 | 12.42 | 20.60 | 12.49 | 11.62 | 13.83 | 17.24 | — | — |
| Angle deviation, 95th percentile (F) | 41.39 | 56.33 | 43.12 | 52.58 | 41.64 | 33.46 | 47.59 | 51.73 | — | — |
| Mean aspect ratio (F) | 1.514 | 3.415 | 1.523 | 2.149 | 1.607 | 1.462 | 4.223 | 3.134 | — | — |
| Area variation (F) | 0.407 | 0.904 | 0.355 | 1.162 | 0.360 | 0.326 | 0.630 | 1.561 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 74.30 | 58.87 | 74.87 | 40.11 | 73.00 | 73.14 | 15.04 | 15.37 | — | — |
| Mean distance, ‰ (H) | 1.492 | 4.497 | 1.370 | 0.668 | 1.734 | 1.197 | 0.620 | 0.869 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 44.1 | 83.6 | 44.1 | 15.4 | 47.9 | 35.9 | 10.6 | 9.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | 97.47 | 95.94 | — | — |
| Sharp edges kept, % of their length (E) | 17.6 | 3.7 | 13.5 | 35.4 | 9.5 | 18.0 | 40.0 | 38.1 | — | — |
| Edges on closed loops, % (L) | 0.4 | — | 4.8 | 5.7 | 4.8 | — | 11.7 | 9.0 | — | — |
| Edges on spiralling loops, % (L) | 17.0 | — | 11.0 | 17.3 | 8.2 | — | 18.8 | 15.2 | — | — |
| Spiralling loops (L) | 130 | — | 124 | 265 | 101 | — | 554 | 571 | — | — |
| Edges on helices of any pitch, % (L) | 9.4 | — | 2.6 | 3.5 | 2.1 | — | 5.9 | 6.6 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 5 | — | 0 | 2 | 0 | — | 2 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.4 | — | 83.0 | 76.7 | 86.3 | — | 69.5 | 75.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | 72 | 111 | 44 | — | 99 | 60 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 1.2 | 0.3 | 0.7 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 15.5 | 52.1 | 2.0 | 243.6 | 13.4 | 15.3 | 2.0 | 3.2 | — | — |
| Time, seconds (T) | 2.44 | 2.36 | 2.19 | 252.94 | 1.82 | 221.87 | 241.82 | 548.14 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4954 | 7991 | 8226 | 7874 | 7220 | 7892 | 8092 | — | — |
| Irregular vertices (S) | — | 58 | 239 | 174 | 249 | 51 | 57 | 69 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 5473 | 5744 | 4813 | 1319 | 1810 | 2112 | — | — |
| Alignment, degrees (A) | — | 10.49 | 13.96 | 11.94 | 13.75 | 10.18 | 10.21 | 9.28 | — | — |
| Mean angle deviation, degrees (F) | — | 6.02 | 4.37 | 11.96 | 4.36 | 6.90 | 4.63 | 4.87 | — | — |
| Angle deviation, 95th percentile (F) | — | 18.67 | 14.06 | 29.91 | 14.77 | 19.25 | 13.21 | 13.80 | — | — |
| Mean aspect ratio (F) | — | 1.719 | 1.131 | 1.539 | 1.140 | 1.276 | 1.267 | 1.260 | — | — |
| Area variation (F) | — | 0.719 | 0.147 | 0.394 | 0.140 | 0.231 | 0.273 | 0.332 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 8.89 | 6.41 | 6.97 | 11.88 | 7.05 | 6.40 | 6.34 | — | — |
| Mean distance, ‰ (H) | — | 0.521 | 0.273 | 0.295 | 0.284 | 0.246 | 0.225 | 0.216 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 14.3 | 12.7 | 56.0 | 17.5 | 25.6 | 47.7 | 66.4 | — | — |
| Edges on closed loops, % (L) | — | 20.8 | 3.4 | 9.4 | 4.6 | 13.6 | 19.3 | 21.3 | — | — |
| Edges on spiralling loops, % (L) | — | 8.5 | 4.3 | 21.4 | 3.3 | 17.7 | 0.4 | 1.4 | — | — |
| Spiralling loops (L) | — | 5 | 10 | 24 | 12 | 8 | 1 | 2 | — | — |
| Edges on helices of any pitch, % (L) | — | 6.8 | 2.5 | 13.1 | 0.4 | 7.7 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 53.1 | 75.7 | 61.6 | 71.1 | 31.8 | 49.9 | 52.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 154 | 210 | 258 | 188 | 158 | 222 | 270 | — | — |
| Edges on loops from a border to a border, % (L) | — | 17.6 | 16.5 | 7.6 | 20.3 | 36.9 | 30.4 | 25.2 | — | — |
| Off budget, % (N) | — | 38.1 | 0.1 | 2.8 | 1.6 | 9.8 | 1.4 | 1.1 | — | — |
| Time, seconds (T) | — | 2.74 | 0.92 | 307.81 | 1.93 | 141.10 | 16.74 | 21.93 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7983 | 4954 | 8116 | 7987 | 7732 | 7902 | 7904 | 7877 | — | — |
| Irregular vertices (S) | 38 | 94 | 122 | 24 | 337 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 30 | — | 114 | 16 | 329 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 1171 | — | 3687 | 287 | 4678 | 76 | 66 | 66 | — | — |
| Alignment, degrees (A) | 13.31 | 14.04 | 12.82 | 11.05 | 13.52 | 12.11 | 12.20 | 11.15 | — | — |
| Mean angle deviation, degrees (F) | 2.56 | 13.77 | 2.42 | 4.20 | 4.52 | 1.99 | 1.01 | 0.66 | — | — |
| Angle deviation, 95th percentile (F) | 9.55 | 47.48 | 9.15 | 15.76 | 18.21 | 6.53 | 3.60 | 2.75 | — | — |
| Mean aspect ratio (F) | 1.074 | 3.591 | 1.093 | 1.310 | 1.178 | 1.105 | 1.082 | 1.057 | — | — |
| Area variation (F) | 0.087 | 0.912 | 0.107 | 0.245 | 0.166 | 0.099 | 0.112 | 0.271 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.14 | 11.34 | 6.86 | 0.98 | 5.05 | 5.51 | 0.33 | 0.27 | — | — |
| Mean distance, ‰ (H) | 0.034 | 0.506 | 0.066 | 0.074 | 0.023 | 0.157 | 0.022 | 0.026 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.06 | — | 100.03 | 100.13 | 100.06 | 99.96 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 99.5 | 8.8 | 97.5 | 100.0 | 99.7 | 62.6 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 59.2 | — | 31.2 | 73.5 | 21.1 | 89.9 | 90.6 | 90.7 | — | — |
| Edges on spiralling loops, % (L) | 22.9 | — | 24.1 | 18.7 | 7.6 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 8 | — | 22 | 4 | 12 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 21.5 | — | 7.9 | 18.7 | 1.4 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 17.7 | — | 44.7 | 7.8 | 72.4 | 10.1 | 9.4 | 9.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 160 | — | 188 | 104 | 184 | 92 | 93 | 86 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.3 | 38.1 | 1.5 | 0.2 | 3.4 | 1.2 | 1.2 | 1.5 | — | — |
| Time, seconds (T) | 6.18 | 2.94 | 0.54 | 7.60 | 2.05 | 26.93 | 8.30 | 7.48 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7920 | 2774 | 8110 | 8214 | 7802 | 8028 | 8066 | 8066 | — | — |
| Irregular vertices (S) | 8 | 10 | 12 | 8 | 18 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 4 | 0 | 10 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | — | 110 | 6 | 376 | 8028 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.46 | 2.60 | 0.36 | 0.11 | 0.96 | 0.68 | 0.11 | 0.11 | — | — |
| Mean angle deviation, degrees (F) | 0.36 | 5.97 | 0.37 | 0.00 | 0.91 | 0.95 | 0.01 | 0.01 | — | — |
| Angle deviation, 95th percentile (F) | 1.18 | 24.98 | 1.09 | 0.00 | 2.60 | 2.23 | 0.01 | 0.01 | — | — |
| Mean aspect ratio (F) | 1.020 | 2.866 | 1.047 | 1.000 | 1.030 | 1.032 | 1.018 | 1.018 | — | — |
| Area variation (F) | 0.023 | 0.740 | 0.035 | 0.000 | 0.045 | 0.044 | 0.013 | 0.013 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.85 | 4.81 | 0.81 | 0.00 | 0.00 | 5.40 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.007 | 0.165 | 0.011 | 0.000 | 0.000 | 0.046 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.99 | 99.99 | 100.00 | 100.00 | 99.99 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 35.0 | 100.0 | 100.0 | 100.0 | 83.8 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 97.2 | 27.9 | 86.9 | 97.3 | 75.7 | 0.0 | 97.3 | 97.3 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 63.5 | 4.8 | 0.0 | 12.0 | 97.5 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 6 | 2 | 0 | 4 | 3 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 63.5 | 4.8 | 0.0 | 12.0 | 97.5 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 2.8 | 8.5 | 8.2 | 2.7 | 13.1 | 2.5 | 2.7 | 2.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 37 | 85 | 150 | 37 | 146 | 73 | 37 | 37 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.0 | 65.3 | 1.4 | 2.7 | 2.5 | 0.4 | 0.8 | 0.8 | — | — |
| Time, seconds (T) | 2.98 | 2.51 | 0.93 | 8.70 | 0.49 | 69.41 | 5.44 | 5.73 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 8016 | 3035 | 8064 | 8016 | 7958 | 8064 | 8064 | 8064 | — | — |
| Irregular vertices (S) | 13 | 20 | 12 | 12 | 28 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | — | 4 | 4 | 20 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 402 | — | 14 | 14 | 1644 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 0.86 | 3.67 | 0.70 | 0.94 | 1.44 | 0.57 | 0.60 | 0.60 | — | — |
| Mean angle deviation, degrees (F) | 0.40 | 6.33 | 0.17 | 1.16 | 1.09 | 0.01 | 0.00 | 0.00 | — | — |
| Angle deviation, 95th percentile (F) | 1.40 | 24.33 | 0.44 | 6.33 | 3.80 | 0.03 | 0.01 | 0.01 | — | — |
| Mean aspect ratio (F) | 1.018 | 2.553 | 1.008 | 1.033 | 1.035 | 1.000 | 1.000 | 1.000 | — | — |
| Area variation (F) | 0.024 | 0.668 | 0.010 | 0.038 | 0.048 | 0.000 | 0.000 | 0.000 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.72 | 5.67 | 0.36 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.005 | 0.155 | 0.004 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.97 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 61.9 | 100.0 | 100.0 | 100.0 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 82.3 | 50.2 | 95.8 | 95.8 | 50.4 | 95.8 | 95.8 | 95.8 | — | — |
| Edges on spiralling loops, % (L) | 11.3 | 27.9 | 0.0 | 0.0 | 31.3 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 1 | 5 | 0 | 0 | 10 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 11.3 | 25.3 | 0.0 | 0.0 | 26.5 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 6.1 | 22.0 | 4.2 | 4.2 | 18.7 | 4.2 | 4.2 | 4.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 132 | 134 | 72 | 72 | 192 | 72 | 72 | 72 | — | — |
| Edges on loops from a border to a border, % (L) | 0.3 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.6 | 62.1 | 0.8 | 0.2 | 0.7 | 0.8 | 0.8 | 0.8 | — | — |
| Time, seconds (T) | 2.69 | 1.49 | 0.66 | 10.15 | 0.36 | 7.59 | 5.74 | 5.56 | — | — |

Failures: quadriflow: timed out after 1800 s.

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 3/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7916 | 2565 | 7960 | 7874 | 7936 | 7768 | 7946 | 8062 | — | — |
| Irregular vertices (S) | 8 | 10 | 106 | 8 | 150 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 98 | 0 | 142 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 2635 | — | 3308 | 6 | 5000 | 76 | 6 | 6 | — | — |
| Alignment, degrees (A) | 4.37 | 4.10 | 4.26 | 3.84 | 4.10 | 1.24 | 1.27 | 1.40 | — | — |
| Mean angle deviation, degrees (F) | 3.95 | 6.60 | 2.14 | 5.96 | 3.00 | 2.35 | 2.23 | 2.67 | — | — |
| Angle deviation, 95th percentile (F) | 16.68 | 20.66 | 8.46 | 14.76 | 11.13 | 11.10 | 8.48 | 9.57 | — | — |
| Mean aspect ratio (F) | 1.075 | 2.021 | 1.066 | 1.059 | 1.092 | 1.137 | 1.135 | 1.198 | — | — |
| Area variation (F) | 0.096 | 0.636 | 0.099 | 0.140 | 0.114 | 0.167 | 0.149 | 0.113 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.80 | 5.39 | 4.39 | 0.76 | 6.27 | 0.45 | 0.45 | 0.44 | — | — |
| Mean distance, ‰ (H) | 0.052 | 0.289 | 0.055 | 0.118 | 0.046 | 0.054 | 0.064 | 0.067 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.96 | 99.70 | 99.95 | 99.88 | 99.96 | 100.00 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 74.2 | 99.2 | 100.0 | 99.1 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 33.3 | 29.3 | 29.2 | 97.2 | 18.6 | 66.6 | 97.2 | 97.2 | — | — |
| Edges on spiralling loops, % (L) | 48.3 | 33.0 | 4.2 | 0.0 | 9.1 | 30.4 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 5 | 3 | 3 | 0 | 2 | 2 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 64.3 | 60.6 | 0.0 | 0.0 | 9.1 | 30.4 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 4 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 18.6 | 37.7 | 66.6 | 2.8 | 73.7 | 3.0 | 2.8 | 2.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 762 | 316 | 282 | 48 | 244 | 80 | 54 | 55 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.1 | 67.9 | 0.5 | 1.6 | 0.8 | 2.9 | 0.7 | 0.8 | — | — |
| Time, seconds (T) | 3.39 | 1.07 | 0.54 | 5.75 | 0.79 | 9.27 | 6.45 | 6.70 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 7212 | 7990 | 10870 | 7693 | 7234 | 7942 | 8236 | — | — |
| Irregular vertices (S) | — | 2158 | 486 | 1133 | 977 | 311 | 312 | 546 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | 4255 | — | — | — | — | — |
| Alignment, degrees (A) | — | 10.19 | 9.89 | 9.52 | 11.23 | 10.81 | 9.67 | 10.60 | — | — |
| Mean angle deviation, degrees (F) | — | 40.97 | 7.66 | 23.43 | 10.36 | 7.70 | 7.34 | 9.92 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.08 | 37.49 | 67.98 | 43.42 | 26.95 | 25.13 | 39.48 | — | — |
| Mean aspect ratio (F) | — | 9.234 | 1.619 | 7.569 | 1.775 | 1.520 | 1.479 | 2.186 | — | — |
| Area variation (F) | — | 2.203 | 0.462 | 1.250 | 0.455 | 0.273 | 0.310 | 0.375 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 37.63 | 36.86 | 12.57 | 24.82 | 21.11 | 35.40 | 35.40 | — | — |
| Mean distance, ‰ (H) | — | 2.866 | 2.739 | 0.589 | 1.379 | 1.597 | 1.376 | 1.307 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 28.8 | 30.9 | 6.7 | 18.1 | 18.3 | 17.7 | 17.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 19.8 | 8.2 | 4.7 | 10.0 | 40.3 | 40.0 | 39.7 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 2.3 | 20.4 | 24.3 | 22.4 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 2.7 | 20.2 | 21.4 | 20.7 | — | — |
| Spiralling loops (L) | — | — | — | — | 189 | 28 | 43 | 45 | — | — |
| Edges on helices of any pitch, % (L) | — | — | — | — | 0.0 | 7.3 | 8.0 | 7.7 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | — | — | 0 | 0 | 0 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 60.1 | 56.8 | 53.5 | 56.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 51 | 236 | 217 | 217 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 35.4 | 2.6 | 0.8 | 0.7 | — | — |
| Off budget, % (N) | — | 9.8 | 0.1 | 35.9 | 3.8 | 9.6 | 0.7 | 2.9 | — | — |
| Time, seconds (T) | — | 1.54 | 1.46 | 258.03 | 1.45 | 106.28 | 76.18 | 116.21 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 1/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4422 | 8063 | 7746 | 7700 | 6468 | 8087 | 8086 | — | — |
| Irregular vertices (S) | — | 172 | 385 | 56 | 442 | 152 | 198 | 338 | — | — |
| … of which above the minimum | — | — | — | 48 | 446 | — | 190 | 330 | — | — |
| Base complex patches (C) | — | — | 7971 | 3300 | 7440 | — | 4543 | 5364 | — | — |
| Alignment, degrees (A) | — | 13.99 | 17.05 | 16.70 | 17.01 | 13.00 | 11.91 | 9.78 | — | — |
| Mean angle deviation, degrees (F) | — | 7.16 | 5.71 | 15.55 | 6.55 | 9.19 | 6.09 | 6.42 | — | — |
| Angle deviation, 95th percentile (F) | — | 21.73 | 20.00 | 44.30 | 21.36 | 24.96 | 16.41 | 18.98 | — | — |
| Mean aspect ratio (F) | — | 1.812 | 1.186 | 1.673 | 1.226 | 1.400 | 1.403 | 1.412 | — | — |
| Area variation (F) | — | 0.657 | 0.194 | 0.448 | 0.198 | 0.293 | 0.336 | 0.929 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 22.79 | 24.62 | 53.57 | 30.78 | 29.87 | 11.46 | 6.71 | — | — |
| Mean distance, ‰ (H) | — | 1.109 | 0.543 | 2.107 | 0.706 | 0.471 | 0.309 | 0.331 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 44.3 | 25.0 | 114.5 | 48.7 | 20.6 | 13.7 | 3.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | 97.95 | 99.39 | 99.98 | 99.99 | 100.01 | — | — |
| Sharp edges kept, % of their length (E) | — | 10.6 | 28.7 | 2.0 | 14.9 | 13.1 | 20.5 | 12.7 | — | — |
| Edges on closed loops, % (L) | — | — | 0.5 | 29.9 | 1.1 | 9.9 | 23.2 | 17.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | 9.6 | 23.9 | 3.1 | 32.2 | 6.8 | 4.5 | — | — |
| Spiralling loops (L) | — | — | 30 | 8 | 15 | 51 | 21 | 11 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 7.1 | 16.8 | 1.2 | 9.9 | 1.1 | 3.9 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 1 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 89.8 | 46.3 | 95.6 | 58.0 | 70.1 | 78.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 188 | 847 | 142 | 244 | 234 | 202 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 44.7 | 0.8 | 3.2 | 3.8 | 19.1 | 1.1 | 1.1 | — | — |
| Time, seconds (T) | — | 15.77 | 49.65 | 54.64 | 80.63 | 111.98 | 212.25 | 385.00 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 7305 | 2507 | 8583 | 27216 | 5504 | 7458 | 7363 | 7363 | — | — |
| Irregular vertices (S) | 642 | 428 | 749 | 2257 | 755 | 471 | 483 | 477 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6985 | — | — | 25688 | 4890 | — | — | — | — | — |
| Alignment, degrees (A) | 13.37 | 16.69 | 14.47 | 16.20 | 14.71 | 13.80 | 13.93 | 13.93 | — | — |
| Mean angle deviation, degrees (F) | 9.18 | 21.18 | 12.53 | 22.29 | 13.98 | 9.31 | 10.08 | 10.11 | — | — |
| Angle deviation, 95th percentile (F) | 32.10 | 68.70 | 51.29 | 54.68 | 52.48 | 32.67 | 35.90 | 36.05 | — | — |
| Mean aspect ratio (F) | 1.331 | 2.614 | 1.638 | 2.322 | 1.697 | 1.427 | 1.554 | 1.557 | — | — |
| Area variation (F) | 0.268 | 0.825 | 0.432 | 1.485 | 0.404 | 0.303 | 0.321 | 0.322 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 50.24 | 90.14 | 80.85 | 22.41 | 46.42 | 49.33 | 48.62 | 48.62 | — | — |
| Mean distance, ‰ (H) | 1.990 | 7.482 | 2.119 | 0.910 | 2.827 | 1.843 | 1.914 | 1.915 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.3 | 69.7 | 49.2 | 16.3 | 37.1 | 23.2 | 23.8 | 23.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 34.8 | 3.7 | 24.4 | 39.9 | 12.0 | 30.6 | 29.6 | 29.3 | — | — |
| Edges on closed loops, % (L) | 2.3 | — | — | 2.9 | 4.3 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 12.4 | — | — | 6.8 | 3.8 | — | — | — | — | — |
| Spiralling loops (L) | 125 | — | — | 101 | 133 | — | — | — | — | — |
| Edges on helices of any pitch, % (L) | 2.6 | — | — | 0.6 | 0.2 | — | — | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | — | 0 | 0 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.2 | — | — | 89.9 | 81.4 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 151 | — | — | 148 | 74 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | 0.4 | 10.5 | — | — | — | — | — |
| Off budget, % (N) | 8.7 | 68.7 | 7.3 | 240.2 | 31.2 | 6.8 | 8.0 | 8.0 | — | — |
| Time, seconds (T) | 6.11 | 6.65 | 1.29 | 285.86 | 1.35 | 225.08 | 383.53 | 398.94 | — | — |

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 7180 | 2664 | 9083 | 22444 | 5696 | 7106 | 7106 | 7103 | — | — |
| Irregular vertices (S) | 624 | 292 | 788 | 1647 | 896 | 314 | 314 | 307 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 6952 | — | — | 20927 | 5391 | — | — | — | — | — |
| Alignment, degrees (A) | 11.45 | 14.32 | 13.72 | 15.82 | 14.00 | 12.13 | 12.13 | 12.14 | — | — |
| Mean angle deviation, degrees (F) | 9.29 | 18.29 | 13.47 | 18.98 | 15.31 | 9.68 | 9.68 | 9.68 | — | — |
| Angle deviation, 95th percentile (F) | 29.68 | 60.76 | 50.61 | 48.33 | 53.81 | 31.59 | 31.59 | 31.59 | — | — |
| Mean aspect ratio (F) | 1.309 | 2.412 | 1.752 | 1.981 | 1.755 | 3.419 | 3.419 | 3.419 | — | — |
| Area variation (F) | 0.258 | 0.707 | 0.429 | 0.925 | 0.427 | 0.341 | 0.341 | 0.341 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 34.62 | 60.39 | 65.69 | 17.90 | 41.57 | 29.48 | 29.48 | 29.48 | — | — |
| Mean distance, ‰ (H) | 1.468 | 4.925 | 2.010 | 0.714 | 2.452 | 1.683 | 1.683 | 1.682 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.1 | 39.9 | 44.6 | 12.2 | 32.2 | 20.0 | 20.0 | 20.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 43.5 | 3.2 | 25.6 | 46.5 | 13.9 | 26.8 | 26.8 | 26.8 | — | — |
| Edges on closed loops, % (L) | 1.3 | — | — | 3.4 | 2.8 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 18.5 | — | — | 7.9 | 2.3 | — | — | — | — | — |
| Spiralling loops (L) | 50 | — | — | 62 | 80 | — | — | — | — | — |
| Edges on helices of any pitch, % (L) | 8.4 | — | — | 0.6 | 0.0 | — | — | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | — | 0 | 0 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.1 | — | — | 88.6 | 85.1 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 117 | — | — | 181 | 60 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | 0.1 | 9.0 | — | — | — | — | — |
| Off budget, % (N) | 10.2 | 66.7 | 13.5 | 180.6 | 28.8 | 11.2 | 11.2 | 11.2 | — | — |
| Time, seconds (T) | 6.62 | 4.13 | 1.28 | 456.68 | 0.97 | 207.74 | 267.91 | 269.20 | — | — |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6712 | 4327 | 8126 | 18981 | 7233 | 5539 | 8025 | 7924 | — | — |
| Irregular vertices (S) | 400 | 121 | 430 | 1520 | 531 | 213 | 192 | 273 | — | — |
| … of which above the minimum | 341 | — | — | — | — | — | 128 | 209 | — | — |
| Base complex patches (C) | 6543 | — | — | 18822 | 7016 | — | 4655 | 5767 | — | — |
| Alignment, degrees (A) | 13.29 | 13.65 | 12.61 | 17.80 | 13.49 | 15.02 | 11.52 | 11.24 | — | — |
| Mean angle deviation, degrees (F) | 9.33 | 10.72 | 6.10 | 20.69 | 7.74 | 12.62 | 4.93 | 5.48 | — | — |
| Angle deviation, 95th percentile (F) | 30.16 | 31.96 | 22.20 | 52.19 | 26.60 | 39.03 | 15.46 | 16.93 | — | — |
| Mean aspect ratio (F) | 1.293 | 1.880 | 1.234 | 1.894 | 1.331 | 1.896 | 1.221 | 1.279 | — | — |
| Area variation (F) | 0.268 | 0.625 | 0.209 | 1.018 | 0.246 | 0.441 | 0.223 | 0.328 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.23 | 58.87 | 32.87 | 23.25 | 29.56 | 30.62 | 11.59 | 11.19 | — | — |
| Mean distance, ‰ (H) | 1.190 | 2.121 | 0.996 | 1.215 | 1.250 | 1.336 | 1.056 | 1.031 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.8 | 185.8 | 83.9 | 48.9 | 58.2 | 27.9 | 21.6 | 17.1 | — | — |
| Volume kept, % of the source (V) | 99.56 | 99.45 | — | — | — | — | 99.99 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | 28.7 | 4.9 | 22.8 | 13.5 | 13.0 | 24.4 | 24.1 | 27.2 | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.8 | — | 0.4 | 1.3 | — | 23.9 | 15.3 | — | — |
| Edges on spiralling loops, % (L) | 8.9 | 24.3 | — | 2.0 | 1.5 | — | 1.6 | 4.5 | — | — |
| Spiralling loops (L) | 33 | 25 | — | 20 | 10 | — | 15 | 5 | — | — |
| Edges on helices of any pitch, % (L) | 2.0 | 10.2 | — | 0.0 | 0.6 | — | 0.7 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | — | 0 | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.0 | 74.8 | — | 97.1 | 97.1 | — | 74.6 | 80.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 149 | 168 | — | 116 | 148 | — | 429 | 215 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.4 | 0.2 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 16.1 | 45.9 | 1.6 | 137.3 | 9.6 | 30.8 | 0.3 | 0.9 | — | — |
| Time, seconds (T) | 5.67 | 3.04 | 1.09 | 94.87 | 1.02 | 308.88 | 72.67 | 95.68 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 2364 | 9421 | 25801 | 5699 | — | 262 | 254 | — | — |
| Irregular vertices (S) | — | 234 | 852 | 1837 | 704 | — | 38 | 48 | — | — |
| … of which above the minimum | — | — | — | — | — | — | 14 | 24 | — | — |
| Base complex patches (C) | — | — | — | 23848 | 5305 | — | 162 | 244 | — | — |
| Alignment, degrees (A) | — | 13.30 | 14.19 | 15.63 | 14.32 | — | 14.65 | 18.20 | — | — |
| Mean angle deviation, degrees (F) | — | 14.00 | 11.93 | 20.29 | 12.59 | — | 22.47 | 23.05 | — | — |
| Angle deviation, 95th percentile (F) | — | 51.31 | 47.84 | 50.57 | 47.68 | — | 62.14 | 63.56 | — | — |
| Mean aspect ratio (F) | — | 2.278 | 1.603 | 2.164 | 1.597 | — | 7.450 | 3.330 | — | — |
| Area variation (F) | — | 0.698 | 0.412 | 1.129 | 0.362 | — | 1.017 | 1.195 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 93.86 | 56.87 | 13.38 | 36.36 | — | 529.09 | 528.69 | — | — |
| Mean distance, ‰ (H) | — | 6.515 | 1.673 | 0.509 | 1.933 | — | 241.873 | 242.249 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 80.5 | 50.4 | 11.9 | 30.1 | — | 1833.4 | 1835.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | 1.82 | 1.86 | — | — |
| Sharp edges kept, % of their length (E) | — | 4.8 | 23.2 | 45.7 | 10.2 | — | 2.2 | 2.0 | — | — |
| Edges on closed loops, % (L) | — | — | — | 3.9 | 2.7 | — | 20.6 | 2.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 7.7 | 2.1 | — | 6.9 | 16.7 | — | — |
| Spiralling loops (L) | — | — | — | 67 | 69 | — | 3 | 8 | — | — |
| Edges on helices of any pitch, % (L) | — | — | — | 1.8 | 0.0 | — | 3.6 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | — | 1 | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 88.3 | 87.0 | — | 72.5 | 81.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 188 | 64 | — | 26 | 22 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.2 | 7.9 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 70.5 | 17.8 | 222.5 | 28.8 | — | 96.7 | 96.8 | — | — |
| Time, seconds (T) | — | 2.97 | 1.78 | 501.22 | 1.14 | — | 6.56 | 143.33 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 2 non-manifold or inconsistently oriented edges.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 0/0 | 0/0 | 2/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 6963 | 2975 | — | — | 4698 | 6553 | 7859 | 8029 | — | — |
| Irregular vertices (S) | 188 | 108 | — | — | 278 | 266 | 94 | 196 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2067 | — | — | — | 3517 | 4818 | 5251 | 6398 | — | — |
| Alignment, degrees (A) | 21.04 | 20.95 | — | — | 21.57 | 17.87 | 18.05 | 18.26 | — | — |
| Mean angle deviation, degrees (F) | 5.79 | 7.30 | — | — | 7.48 | 10.73 | 6.52 | 7.01 | — | — |
| Angle deviation, 95th percentile (F) | 21.42 | 26.48 | — | — | 25.61 | 27.71 | 16.75 | 18.91 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.481 | — | — | 1.265 | 1.357 | 1.289 | 1.300 | — | — |
| Area variation (F) | 0.185 | 0.463 | — | — | 0.222 | 0.306 | 0.289 | 0.493 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 16.98 | 26.34 | — | — | 136.11 | 14.39 | 12.38 | 8.74 | — | — |
| Mean distance, ‰ (H) | 1.071 | 1.562 | — | — | 1.596 | 0.873 | 0.782 | 0.622 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 34.0 | 47.6 | — | — | 46.2 | 28.4 | 25.6 | 16.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 12.2 | 4.0 | — | — | 5.7 | 12.3 | 15.2 | 20.1 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | 1.5 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.1 | 2.4 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 3 | 2 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 0.0 | — | — | 0.0 | 1.6 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | — | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.9 | 71.1 | — | — | 79.3 | 83.7 | 81.2 | 89.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 55 | — | — | 66 | 151 | 285 | 262 | — | — |
| Edges on loops from a border to a border, % (L) | 46.1 | 28.9 | — | — | 20.7 | 13.9 | 18.8 | 9.4 | — | — |
| Off budget, % (N) | 13.0 | 62.8 | — | — | 41.3 | 18.1 | 1.8 | 0.4 | — | — |
| Time, seconds (T) | 18.30 | 4.29 | — | — | 7.96 | 343.10 | 45.66 | 71.21 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2996 | — | — | 5308 | 5713 | 7925 | 7765 | — | — |
| Irregular vertices (S) | — | 419 | — | — | 414 | 172 | 522 | 550 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | 3826 | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.27 | — | — | 9.41 | 6.87 | 5.83 | 10.13 | — | — |
| Mean angle deviation, degrees (F) | — | 19.93 | — | — | 8.33 | 8.54 | 9.49 | 13.70 | — | — |
| Angle deviation, 95th percentile (F) | — | 77.95 | — | — | 31.65 | 26.90 | 52.60 | 56.00 | — | — |
| Mean aspect ratio (F) | — | 3.957 | — | — | 1.363 | 1.526 | 20.029 | 55.238 | — | — |
| Area variation (F) | — | 1.337 | — | — | 0.300 | 0.441 | 1.575 | 1.825 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.28 | — | — | 34.84 | 22.52 | 32.87 | 31.53 | — | — |
| Mean distance, ‰ (H) | — | 3.494 | — | — | 3.739 | 2.615 | 1.722 | 1.932 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.7 | — | — | 60.4 | 43.0 | 27.0 | 29.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 10.4 | — | — | 3.5 | 19.8 | 46.9 | 38.1 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 5.8 | 26.7 | 41.4 | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 2.9 | 15.1 | 32.3 | — | — | — |
| Spiralling loops (L) | — | — | — | — | 52 | 31 | 711 | — | — | — |
| Edges on helices of any pitch, % (L) | — | — | — | — | 0.6 | 0.9 | 12.7 | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | — | — | 0 | 0 | 0 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 75.4 | 35.5 | 17.1 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 72 | 142 | 114 | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 15.8 | 22.7 | 9.1 | — | — | — |
| Off budget, % (N) | — | 62.5 | — | — | 33.7 | 28.6 | 0.9 | 2.9 | — | — |
| Time, seconds (T) | — | 2.29 | — | — | 5.83 | 92.29 | 165.29 | 232.95 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2388 | — | — | 4129 | — | 11893 | 11773 | — | — |
| Irregular vertices (S) | — | 1056 | — | — | 1552 | — | 6177 | 6289 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | 3784 | — | — | 7545 | — | — |
| Alignment, degrees (A) | — | 15.08 | — | — | 17.85 | — | 11.58 | 11.61 | — | — |
| Mean angle deviation, degrees (F) | — | 28.51 | — | — | 20.41 | — | 16.87 | 18.14 | — | — |
| Angle deviation, 95th percentile (F) | — | 82.76 | — | — | 58.14 | — | 67.20 | 65.35 | — | — |
| Mean aspect ratio (F) | — | 7.658 | — | — | 2.427 | — | 6.162 | 47.946 | — | — |
| Area variation (F) | — | 1.726 | — | — | 0.685 | — | 2.389 | 2.618 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 6.92 | — | — | 6.24 | — | 4.21 | 4.22 | — | — |
| Mean distance, ‰ (H) | — | 1.765 | — | — | 1.031 | — | 0.282 | 0.286 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 94.0 | — | — | 58.9 | — | 13.3 | 13.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 3.1 | — | — | 10.3 | — | 55.7 | 55.1 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 4.4 | — | — | 22.8 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 12.9 | — | — | 33.6 | — | — |
| Spiralling loops (L) | — | — | — | — | 480 | — | — | 5993 | — | — |
| Edges on helices of any pitch, % (L) | — | — | — | — | 0.0 | — | — | 4.6 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | — | — | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 76.6 | — | — | 42.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 23 | — | — | 31 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 6.1 | — | — | 1.0 | — | — |
| Off budget, % (N) | — | 70.2 | — | — | 48.4 | — | 48.7 | 47.2 | — | — |
| Time, seconds (T) | — | 1.20 | — | — | 3.80 | — | 410.32 | 480.38 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 10 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 6227 | — | — | 4548 | 6493 | 9359 | 9326 | — | — |
| Irregular vertices (S) | — | 2171 | — | — | 863 | 453 | 2021 | 2003 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | 3548 | — | 3038 | 3090 | — | — |
| Alignment, degrees (A) | — | 10.29 | — | — | 8.12 | 5.80 | 4.76 | 4.85 | — | — |
| Mean angle deviation, degrees (F) | — | 29.31 | — | — | 14.41 | 9.15 | 9.48 | 9.46 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.69 | — | — | 54.18 | 31.65 | 53.90 | 54.21 | — | — |
| Mean aspect ratio (F) | — | 6.307 | — | — | 1.909 | 2.197 | 32.394 | 33.420 | — | — |
| Area variation (F) | — | 1.369 | — | — | 0.470 | 0.368 | 1.579 | 1.745 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 80.26 | — | — | 73.99 | 47.84 | 16.87 | 14.07 | — | — |
| Mean distance, ‰ (H) | — | 5.085 | — | — | 10.384 | 5.907 | 0.631 | 0.625 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 60.6 | — | — | 128.6 | 78.4 | 7.4 | 7.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 9.0 | — | — | 33.5 | 34.9 | 74.0 | 73.1 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 6.9 | — | 34.6 | 33.4 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 9.3 | — | 21.2 | 24.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 330 | — | 1893 | 1868 | — | — |
| Edges on helices of any pitch, % (L) | — | — | — | — | 0.6 | — | 1.6 | 2.9 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | — | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 72.6 | — | 42.2 | 40.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 62 | — | 75 | 86 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 11.6 | — | 1.9 | 2.2 | — | — |
| Off budget, % (N) | — | 22.2 | — | — | 43.1 | 18.8 | 17.0 | 16.6 | — | — |
| Time, seconds (T) | — | 2.93 | — | — | 5.30 | 262.98 | 83.49 | 112.55 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 4114 | — | — | 6839 | 7147 | 8129 | 8142 | — | — |
| Irregular vertices (S) | — | 403 | — | — | 532 | 152 | 184 | 196 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | 4440 | — | 620 | 976 | — | — |
| Alignment, degrees (A) | — | 8.73 | — | — | 6.43 | 3.74 | 3.12 | 3.23 | — | — |
| Mean angle deviation, degrees (F) | — | 18.87 | — | — | 9.46 | 5.57 | 11.90 | 12.89 | — | — |
| Angle deviation, 95th percentile (F) | — | 74.17 | — | — | 38.06 | 22.74 | 68.10 | 69.38 | — | — |
| Mean aspect ratio (F) | — | 3.158 | — | — | 1.596 | 1.633 | 19.180 | 15.968 | — | — |
| Area variation (F) | — | 1.607 | — | — | 0.398 | 0.313 | 0.723 | 0.756 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 58.33 | — | — | 36.49 | 36.17 | 20.32 | 20.40 | — | — |
| Mean distance, ‰ (H) | — | 3.579 | — | — | 1.944 | 1.288 | 0.739 | 0.723 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 32.4 | — | — | 21.3 | 12.4 | 7.6 | 7.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 8.6 | — | — | 26.9 | 43.6 | 85.2 | 85.0 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 15.0 | — | 56.4 | 48.8 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 8.1 | — | 2.8 | 4.9 | — | — |
| Spiralling loops (L) | — | — | — | — | 52 | — | 25 | 34 | — | — |
| Edges on helices of any pitch, % (L) | — | — | — | — | 2.0 | — | 2.0 | 2.1 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | — | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 72.8 | — | 19.3 | 26.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 106 | — | 136 | 240 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 8.5 | — | 21.5 | 19.8 | — | — |
| Off budget, % (N) | — | 48.6 | — | — | 14.5 | 10.7 | 1.6 | 1.8 | — | — |
| Time, seconds (T) | — | 1.15 | — | — | 3.07 | 155.45 | 11.98 | 35.17 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/0 | 0/0 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2119 | — | — | 7656 | 6608 | 6930 | 6925 | — | — |
| Irregular vertices (S) | — | 195 | — | — | 835 | 439 | 457 | 489 | — | — |
| … of which above the minimum | — | — | — | — | 730 | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | 7378 | — | — | — | — | — |
| Alignment, degrees (A) | — | 13.19 | — | — | 15.59 | 13.29 | 13.23 | 13.30 | — | — |
| Mean angle deviation, degrees (F) | — | 10.86 | — | — | 8.99 | 13.30 | 13.10 | 13.08 | — | — |
| Angle deviation, 95th percentile (F) | — | 32.41 | — | — | 30.80 | 34.84 | 34.94 | 34.81 | — | — |
| Mean aspect ratio (F) | — | 1.955 | — | — | 1.370 | 1.560 | 1.943 | 1.554 | — | — |
| Area variation (F) | — | 0.849 | — | — | 0.275 | 0.360 | 0.354 | 0.358 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 36.12 | — | — | 31.17 | 13.29 | 13.28 | 13.28 | — | — |
| Mean distance, ‰ (H) | — | 3.139 | — | — | 1.125 | 0.782 | 0.628 | 0.614 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 103.8 | — | — | 74.2 | 47.5 | 32.6 | 30.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 6.3 | — | — | 21.4 | 45.2 | 43.8 | 43.9 | — | — |
| Edges on closed loops, % (L) | — | 12.3 | — | — | 2.5 | 8.6 | 8.1 | 8.2 | — | — |
| Edges on spiralling loops, % (L) | — | 3.9 | — | — | 8.1 | 29.5 | 31.0 | 31.1 | — | — |
| Spiralling loops (L) | — | 9 | — | — | 74 | 106 | 120 | 133 | — | — |
| Edges on helices of any pitch, % (L) | — | 0.0 | — | — | 2.9 | 14.2 | 16.3 | 15.8 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | — | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 83.8 | — | — | 89.9 | 61.8 | 60.8 | 60.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 57 | — | — | 108 | 121 | 102 | 102 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 73.5 | — | — | 4.3 | 17.4 | 13.4 | 13.4 | — | — |
| Time, seconds (T) | — | 2.90 | — | — | 0.59 | 117.05 | 171.50 | 172.88 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 7736 | 6520 | 8140 | 8118 | 8186 | 7873 | 8056 | 7991 | — | — |
| Irregular vertices (S) | 12 | 26 | 156 | 8 | 190 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 4 | 18 | 148 | 0 | 182 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 7729 | 6252 | 8140 | 100 | 8186 | 265 | 6 | 7991 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.93 | 5.85 | 2.99 | 6.97 | 3.51 | 2.32 | 1.51 | 1.66 | — | — |
| Angle deviation, 95th percentile (F) | 12.52 | 14.46 | 10.16 | 17.20 | 11.50 | 6.23 | 3.79 | 3.98 | — | — |
| Mean aspect ratio (F) | 1.109 | 1.337 | 1.082 | 1.192 | 1.098 | 1.119 | 1.110 | 1.119 | — | — |
| Area variation (F) | 0.134 | 0.283 | 0.120 | 0.241 | 0.128 | 0.113 | 0.125 | 0.115 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.42 | 0.80 | 0.41 | 0.94 | 0.42 | 0.31 | 0.29 | 0.30 | — | — |
| Mean distance, ‰ (H) | 0.085 | 0.172 | 0.082 | 0.289 | 0.081 | 0.077 | 0.073 | 0.076 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.93 | 99.83 | 99.92 | 99.71 | 99.92 | 100.01 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 2.1 | 0.0 | 88.6 | 0.0 | 62.9 | 97.3 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 52.4 | 42.1 | 16.2 | 0.0 | 1.8 | 32.7 | 0.0 | 97.9 | — | — |
| Spiralling loops (L) | 5 | 10 | 9 | 0 | 1 | 1 | 0 | 4 | — | — |
| Edges on helices of any pitch, % (L) | 90.9 | 40.1 | 3.9 | 0.0 | 0.0 | 32.7 | 0.0 | 97.9 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 6 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 47.6 | 55.8 | 83.8 | 11.4 | 98.2 | 4.4 | 2.7 | 2.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1548 | 584 | 202 | 396 | 260 | 142 | 38 | 74 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 18.5 | 1.8 | 1.5 | 2.3 | 1.6 | 0.7 | 0.1 | — | — |
| Time, seconds (T) | 3.83 | 1.97 | 0.41 | 7.69 | 0.62 | 17.12 | 11.49 | 160.05 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 1/1 | 1/1 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 9147 | 7544 | 8038 | 8001 | 7964 | 7280 | 8024 | 8024 | — | — |
| Irregular vertices (S) | 22 | 0 | 228 | 0 | 260 | 0 | 0 | 0 | — | — |
| … of which above the minimum | 22 | 0 | 228 | 0 | 260 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 9070 | 1 | 7700 | 1 | 7928 | 1 | 1 | 1 | — | — |
| Alignment, degrees (A) | 11.41 | 0.76 | 3.58 | 2.23 | 6.52 | 0.61 | 0.64 | 0.64 | — | — |
| Mean angle deviation, degrees (F) | 7.33 | 1.26 | 3.88 | 3.23 | 4.83 | 0.91 | 0.91 | 0.91 | — | — |
| Angle deviation, 95th percentile (F) | 19.57 | 2.30 | 11.86 | 10.42 | 15.28 | 1.45 | 1.41 | 1.41 | — | — |
| Mean aspect ratio (F) | 1.180 | 2.552 | 1.117 | 1.231 | 1.138 | 1.340 | 1.321 | 1.319 | — | — |
| Area variation (F) | 0.189 | 0.146 | 0.138 | 0.457 | 0.147 | 0.281 | 0.299 | 0.300 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.60 | 0.79 | 0.56 | 0.95 | 0.67 | 0.34 | 0.35 | 0.35 | — | — |
| Mean distance, ‰ (H) | 0.109 | 0.178 | 0.117 | 0.277 | 0.119 | 0.111 | 0.104 | 0.104 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.82 | 99.67 | 99.79 | 99.37 | 99.79 | 100.01 | 100.01 | 100.01 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 100.0 | 2.1 | 100.0 | 0.0 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 38.2 | 0.0 | 32.1 | 0.0 | 13.8 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 8 | 0 | 42 | 0 | 24 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 84.0 | 0.0 | 3.7 | 0.0 | 1.6 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 61.8 | 0.0 | 65.8 | 0.0 | 86.2 | 0.0 | 0.0 | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1065 | 0 | 164 | 0 | 214 | 0 | 0 | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 14.3 | 5.7 | 0.5 | 0.0 | 0.9 | 9.0 | 0.3 | 0.3 | — | — |
| Time, seconds (T) | 4.68 | 2.35 | 0.39 | 6.42 | 0.87 | 5.02 | 3.75 | 5.93 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 4 | 0 | 5 | 0 | 5 | 1 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 2 | 0 | 0 | 0 | 1 | 8 | 3 | 0 | 0 |
| Alignment, degrees (A) | 0 | 1 | 0 | 0 | 0 | 2 | 1 | 11 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 7 | 0 | 2 | 1 | 4 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 0 | 0 | 14 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 33/75 | 4/15 | 9/15 | 10/15 | 49/75 | 8/15 | 14/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes-jakob | quadwild-bimdf | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 0 | 1 | 4 | 0 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 1 | 3 | 0 | 1 | 4 | 4 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 0 | 0 | 0 | 2 | 0 | 0 | 1 | 1 | 0 | 0 |
| Valid meshes (V) | 14/20 | 0/4 | 4/4 | 4/4 | 18/20 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
