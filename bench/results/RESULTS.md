# Bench results

Requested budget: 3000 quads. Median over 5 seeds; AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2781 | 1165 | 3090 | 2429 | 2976 | 3047 | — | — |
| Irregular vertices (S) | 38 | 26 | 143 | 27 | 24 | 24 | — | — |
| … of which above the minimum | 30 | — | 135 | — | 16 | 16 | — | — |
| Base complex patches (C) | 2697 | — | 3048 | 1424 | 1084 | 849 | — | — |
| Alignment, degrees (A) | 17.48 | 6.27 | 15.74 | 5.85 | 5.37 | 4.86 | — | — |
| Mean angle deviation, degrees (F) | 8.71 | 9.45 | 5.87 | 7.84 | 6.68 | 6.15 | — | — |
| Angle deviation, 95th percentile (F) | 27.30 | 30.58 | 18.26 | 21.71 | 20.94 | 16.71 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.934 | 1.174 | 1.477 | 1.448 | 1.421 | — | — |
| Area variation (F) | 0.205 | 0.575 | 0.175 | 0.361 | 0.433 | 0.460 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 6.35 | 10.14 | 10.24 | 3.26 | 2.61 | — | — |
| Mean distance, ‰ (H) | 0.486 | 1.885 | 0.459 | 0.277 | 0.198 | 0.188 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.56 | 96.90 | 99.63 | — | 100.02 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 17.2 | 0.8 | 15.6 | 31.8 | 40.3 | — | — |
| Edges on spiralling loops, % (L) | 14.7 | 31.9 | 2.6 | 26.9 | 19.6 | 11.6 | — | — |
| Spiralling loops (L) | 3 | 7 | 2 | 5 | 4 | 2 | — | — |
| Edges on helices of any pitch, % (L) | 15.4 | 9.9 | 0.0 | 23.6 | 2.7 | 9.8 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 2 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.3 | 50.9 | 96.5 | 40.1 | 48.6 | 48.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 321 | 120 | 122 | 170 | 382 | 324 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 17.4 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 7.3 | 61.2 | 3.0 | 19.0 | 0.8 | 1.6 | — | — |
| Time, seconds (T) | 2.78 | 1.23 | 1.63 | 7.75 | 3.27 | 3.50 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3036 | 2412 | 2898 | 2793 | 3039 | 3010 | — | — |
| Irregular vertices (S) | 8 | 20 | 91 | 55 | 54 | 64 | — | — |
| … of which above the minimum | 0 | 12 | 83 | 47 | 46 | 56 | — | — |
| Base complex patches (C) | 2855 | 2412 | 2896 | 2793 | 3039 | 3010 | — | — |
| Alignment, degrees (A) | 22.96 | 18.78 | 23.21 | 15.67 | 16.23 | 15.75 | — | — |
| Mean angle deviation, degrees (F) | 5.84 | 7.48 | 5.18 | 9.09 | 5.35 | 5.10 | — | — |
| Angle deviation, 95th percentile (F) | 16.05 | 17.91 | 14.78 | 22.55 | 13.54 | 13.37 | — | — |
| Mean aspect ratio (F) | 1.164 | 1.409 | 1.146 | 1.242 | 1.211 | 1.203 | — | — |
| Area variation (F) | 0.202 | 0.331 | 0.151 | 0.230 | 0.270 | 0.282 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 7.64 | 5.28 | 4.51 | 4.50 | 4.53 | — | — |
| Mean distance, ‰ (H) | 0.788 | 1.157 | 0.820 | 0.737 | 0.694 | 0.696 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 99.49 | 99.70 | 100.00 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 56.9 | 27.1 | 4.1 | 28.7 | 11.0 | 9.3 | — | — |
| Spiralling loops (L) | 5 | 4 | 2 | 4 | 3 | 3 | — | — |
| Edges on helices of any pitch, % (L) | 69.3 | 26.4 | 0.0 | 18.5 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 3 | 2 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 43.1 | 72.9 | 95.9 | 71.3 | 89.0 | 90.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 528 | 317 | 142 | 262 | 238 | 207 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.2 | 19.6 | 3.4 | 6.9 | 1.3 | 0.3 | — | — |
| Time, seconds (T) | 1.48 | 0.95 | 0.78 | 9.31 | 3.31 | 2.90 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2667 | 1991 | 3070 | 2748 | 2946 | 2953 | — | — |
| Irregular vertices (S) | 24 | 20 | 111 | 34 | 36 | 40 | — | — |
| … of which above the minimum | 16 | 12 | 103 | — | 28 | 32 | — | — |
| Base complex patches (C) | 2667 | 747 | 2856 | 1022 | 1124 | 861 | — | — |
| Alignment, degrees (A) | 12.89 | 9.58 | 12.09 | 7.38 | 7.41 | 5.59 | — | — |
| Mean angle deviation, degrees (F) | 6.85 | 4.75 | 4.65 | 7.61 | 5.63 | 5.00 | — | — |
| Angle deviation, 95th percentile (F) | 18.98 | 13.44 | 14.73 | 23.65 | 16.63 | 13.79 | — | — |
| Mean aspect ratio (F) | 1.189 | 1.839 | 1.144 | 1.257 | 1.272 | 1.315 | — | — |
| Area variation (F) | 0.237 | 0.549 | 0.146 | 0.281 | 0.378 | 0.474 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 2.39 | 3.47 | 9.76 | 1.88 | 1.99 | — | — |
| Mean distance, ‰ (H) | 0.320 | 0.537 | 0.279 | 0.212 | 0.193 | 0.205 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.41 | 98.97 | 99.47 | — | 100.02 | 100.03 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 31.2 | 3.3 | 33.0 | 31.7 | 38.1 | — | — |
| Edges on spiralling loops, % (L) | 30.9 | 33.6 | 9.6 | 12.4 | 4.8 | 5.9 | — | — |
| Spiralling loops (L) | 4 | 6 | 7 | 4 | 2 | 3 | — | — |
| Edges on helices of any pitch, % (L) | 52.6 | 25.1 | 4.4 | 5.9 | 0.0 | 3.3 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 2 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 69.1 | 35.2 | 83.9 | 54.6 | 63.5 | 56.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 377 | 166 | 164 | 172 | 318 | 321 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 11.1 | 33.6 | 2.3 | 8.4 | 1.8 | 1.6 | — | — |
| Time, seconds (T) | 1.09 | 1.16 | 0.74 | 6.21 | 4.35 | 3.02 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2891 | 4039 | 2944 | 2994 | 3051 | 2999 | — | — |
| Irregular vertices (S) | 66 | 97 | 200 | 46 | 48 | 48 | — | — |
| … of which above the minimum | 34 | — | 168 | 14 | 16 | 16 | — | — |
| Base complex patches (C) | 1943 | — | 2319 | 584 | 184 | 162 | — | — |
| Alignment, degrees (A) | 8.10 | 8.92 | 7.97 | 4.33 | 3.62 | 2.33 | — | — |
| Mean angle deviation, degrees (F) | 5.86 | 9.02 | 6.39 | 4.19 | 2.67 | 2.94 | — | — |
| Angle deviation, 95th percentile (F) | 17.11 | 28.25 | 23.33 | 13.69 | 9.23 | 9.76 | — | — |
| Mean aspect ratio (F) | 1.183 | 2.030 | 1.252 | 1.206 | 1.189 | 1.201 | — | — |
| Area variation (F) | 0.147 | 0.496 | 0.203 | 0.178 | 0.186 | 0.386 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 13.32 | 14.46 | 9.61 | 8.90 | 5.66 | — | — |
| Mean distance, ‰ (H) | 0.734 | 0.800 | 0.860 | 0.704 | 0.636 | 0.516 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.99 | 99.63 | 98.70 | 99.78 | 99.71 | 99.71 | — | — |
| Sharp edges kept, % of their length (E) | 79.6 | 40.6 | 82.6 | 81.0 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 18.2 | 4.2 | 11.3 | 52.4 | 75.6 | 76.9 | — | — |
| Edges on spiralling loops, % (L) | 38.5 | 29.9 | 9.6 | 21.2 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 20 | 24 | 11 | 9 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 34.5 | 5.5 | 1.5 | 15.2 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 41.8 | 65.8 | 79.5 | 26.4 | 24.4 | 23.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 86 | 110 | 82 | 136 | 42 | 44 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 34.6 | 1.9 | 0.2 | 1.7 | 0.0 | — | — |
| Time, seconds (T) | 1.23 | 3.29 | 0.38 | 9.81 | 3.56 | 4.51 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3030 | 1255 | 3070 | 2879 | 3018 | 3003 | — | — |
| Irregular vertices (S) | 78 | 69 | 212 | 52 | 54 | 56 | — | — |
| … of which above the minimum | 54 | — | — | — | — | — | — | — |
| Base complex patches (C) | 2884 | — | 3002 | 1808 | 1360 | 1594 | — | — |
| Alignment, degrees (A) | 15.33 | 13.19 | 17.02 | 10.98 | 10.78 | 10.00 | — | — |
| Mean angle deviation, degrees (F) | 8.61 | 11.13 | 7.08 | 6.78 | 5.66 | 5.32 | — | — |
| Angle deviation, 95th percentile (F) | 26.08 | 41.63 | 25.30 | 17.31 | 15.34 | 14.44 | — | — |
| Mean aspect ratio (F) | 1.260 | 1.871 | 1.263 | 1.215 | 1.299 | 1.302 | — | — |
| Area variation (F) | 0.837 | 0.611 | 0.221 | 0.206 | 0.283 | 0.390 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 33.04 | 20.15 | 7.80 | 7.81 | 7.72 | — | — |
| Mean distance, ‰ (H) | 0.834 | 2.167 | 0.906 | 0.652 | 0.515 | 0.456 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.4 | 28.9 | 21.4 | 7.2 | 3.8 | 4.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 42.5 | 3.2 | 19.5 | 91.3 | 73.9 | 93.1 | — | — |
| Edges on closed loops, % (L) | 1.9 | 8.6 | 0.8 | 10.1 | 20.4 | 16.4 | — | — |
| Edges on spiralling loops, % (L) | 15.3 | 14.6 | 1.4 | 16.6 | 9.9 | 13.2 | — | — |
| Spiralling loops (L) | 12 | 8 | 3 | 7 | 2 | 6 | — | — |
| Edges on helices of any pitch, % (L) | 7.2 | 1.2 | 0.0 | 11.9 | 5.9 | 10.4 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.2 | 76.8 | 96.1 | 58.2 | 53.2 | 60.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 233 | 82 | 94 | 221 | 107 | 171 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.6 | 15.1 | 16.6 | 10.1 | — | — |
| Off budget, % (N) | 1.0 | 58.2 | 2.3 | 4.0 | 0.6 | 0.1 | — | — |
| Time, seconds (T) | 3.35 | 1.04 | 0.39 | 5.35 | 3.45 | 4.48 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2935 | 1081 | 2876 | 2661 | 2976 | 2972 | — | — |
| Irregular vertices (S) | 106 | 84 | 213 | 73 | 76 | 154 | — | — |
| … of which above the minimum | 98 | — | 205 | 65 | 68 | 146 | — | — |
| Base complex patches (C) | 2857 | — | 2804 | 1816 | 2266 | 1891 | — | — |
| Alignment, degrees (A) | 19.31 | 16.24 | 18.66 | 14.16 | 13.77 | 12.22 | — | — |
| Mean angle deviation, degrees (F) | 12.01 | 10.37 | 7.50 | 8.51 | 6.63 | 7.22 | — | — |
| Angle deviation, 95th percentile (F) | 32.96 | 32.56 | 23.93 | 23.99 | 19.59 | 19.96 | — | — |
| Mean aspect ratio (F) | 1.403 | 1.826 | 1.291 | 1.373 | 1.410 | 1.437 | — | — |
| Area variation (F) | 0.324 | 0.590 | 0.223 | 0.255 | 0.319 | 0.513 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 31.05 | 46.10 | 39.09 | 27.65 | 27.71 | — | — |
| Mean distance, ‰ (H) | 1.268 | 3.042 | 1.498 | 1.106 | 0.765 | 0.738 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 54.0 | 66.6 | 101.9 | 77.6 | 29.8 | 4.9 | — | — |
| Volume kept, % of the source (V) | 98.56 | — | 98.56 | 99.66 | 99.84 | 99.90 | — | — |
| Sharp edges kept, % of their length (E) | 38.9 | 8.2 | 13.6 | 61.0 | 59.3 | 54.5 | — | — |
| Edges on closed loops, % (L) | 1.7 | — | 1.8 | 15.2 | 11.9 | 17.5 | — | — |
| Edges on spiralling loops, % (L) | 9.6 | — | 5.4 | 11.9 | 18.2 | 9.3 | — | — |
| Spiralling loops (L) | 14 | — | 8 | 10 | 10 | 21 | — | — |
| Edges on helices of any pitch, % (L) | 20.7 | — | 1.4 | 2.7 | 8.1 | 4.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 3 | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.6 | — | 93.5 | 72.9 | 69.9 | 73.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 250 | — | 84 | 149 | 151 | 122 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 4.3 | 64.0 | 4.1 | 11.3 | 0.8 | 0.9 | — | — |
| Time, seconds (T) | 3.00 | 1.12 | 1.87 | 17.37 | 14.37 | 21.96 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 854 | 2814 | 2570 | 2955 | 3032 | 10590 | 10520 |
| Irregular vertices (S) | — | 88 | 194 | 73 | 102 | 158 | 476 | 254 |
| … of which above the minimum | — | — | 178 | — | 94 | 150 | — | 246 |
| Base complex patches (C) | — | — | 2014 | — | 1023 | 1048 | — | 3148 |
| Alignment, degrees (A) | — | 9.53 | 10.54 | 7.44 | 8.21 | 6.46 | 9.88 | 8.31 |
| Mean angle deviation, degrees (F) | — | 10.46 | 7.19 | 7.06 | 5.58 | 5.63 | 9.24 | 4.27 |
| Angle deviation, 95th percentile (F) | — | 37.89 | 25.75 | 21.86 | 19.84 | 19.27 | 26.27 | 13.84 |
| Mean aspect ratio (F) | — | 2.541 | 1.301 | 1.354 | 1.556 | 1.339 | 1.616 | 1.341 |
| Area variation (F) | — | 0.757 | 0.248 | 0.315 | 0.407 | 0.670 | 1.417 | 0.459 |
| Hausdorff, ‰ of the diagonal (H) | — | 31.67 | 47.23 | 38.24 | 14.96 | 9.46 | 6.47 | 4.76 |
| Mean distance, ‰ (H) | — | 5.764 | 4.380 | 2.908 | 1.265 | 0.784 | 0.172 | 0.202 |
| Thin parts: mean distance, % of a quad edge (V) | — | 79.0 | 84.2 | 59.6 | 18.2 | 8.8 | 1.5 | 6.2 |
| Volume kept, % of the source (V) | — | — | 97.16 | 99.62 | 99.91 | 100.02 | 98.80 | 100.01 |
| Sharp edges kept, % of their length (E) | — | 0.0 | 0.0 | 78.8 | 93.2 | 87.7 | 71.3 | 94.4 |
| Edges on closed loops, % (L) | — | — | 14.8 | 31.4 | 32.7 | 32.7 | 31.6 | 36.6 |
| Edges on spiralling loops, % (L) | — | — | 7.8 | 16.8 | 2.8 | 1.1 | 2.5 | 1.7 |
| Spiralling loops (L) | — | — | 8 | 11 | 8 | 4 | 3 | 16 |
| Edges on helices of any pitch, % (L) | — | — | 3.7 | 8.0 | 1.1 | 0.6 | 0.0 | 0.3 |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | 0 | 0 | 0 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 76.8 | 51.7 | 64.5 | 66.1 | 65.8 | 61.7 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 86 | 180 | 288 | 155 | 274 | 404 |
| Edges on loops from a border to a border, % (L) | — | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 71.5 | 6.2 | 14.3 | 1.5 | 1.1 | 0.0 | 0.7 |
| Time, seconds (T) | — | 3.95 | 18.69 | 27.64 | 21.71 | 36.75 | — | 58.32 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 689 | 2837 | 2712 | 2969 | 2995 | 10590 | 10642 |
| Irregular vertices (S) | — | 77 | 204 | 73 | 102 | 176 | 476 | 274 |
| … of which above the minimum | — | — | 196 | 65 | 94 | 168 | — | 266 |
| Base complex patches (C) | — | — | 2366 | 1414 | 1073 | 1257 | — | 3850 |
| Alignment, degrees (A) | — | 11.80 | 10.59 | 7.34 | 8.08 | 5.82 | 9.43 | 7.35 |
| Mean angle deviation, degrees (F) | — | 14.87 | 7.68 | 7.06 | 5.08 | 5.48 | 8.82 | 4.39 |
| Angle deviation, 95th percentile (F) | — | 40.05 | 25.19 | 22.61 | 18.20 | 20.19 | 25.47 | 13.83 |
| Mean aspect ratio (F) | — | 2.191 | 1.291 | 1.347 | 1.510 | 1.371 | 1.657 | 1.274 |
| Area variation (F) | — | 0.762 | 0.239 | 0.325 | 0.443 | 0.738 | 1.432 | 0.494 |
| Hausdorff, ‰ of the diagonal (H) | — | 40.87 | 56.57 | 40.75 | 20.28 | 11.34 | 4.44 | 4.28 |
| Mean distance, ‰ (H) | — | 6.121 | 4.562 | 2.923 | 1.543 | 0.845 | 0.156 | 0.186 |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.6 | 97.9 | 64.7 | 30.7 | 10.2 | 1.4 | 5.5 |
| Volume kept, % of the source (V) | — | 85.52 | 96.97 | 99.64 | 99.94 | 100.11 | 98.78 | 100.02 |
| Sharp edges kept, % of their length (E) | — | 0.0 | 26.1 | 75.0 | 61.2 | 61.4 | 67.0 | 85.7 |
| Edges on closed loops, % (L) | — | 11.9 | 8.2 | 23.9 | 33.4 | 29.0 | 31.6 | 31.9 |
| Edges on spiralling loops, % (L) | — | 20.2 | 9.7 | 25.6 | 4.3 | 4.8 | 2.5 | 1.9 |
| Spiralling loops (L) | — | 18 | 18 | 13 | 23 | 13 | 3 | 13 |
| Edges on helices of any pitch, % (L) | — | 13.7 | 5.5 | 20.2 | 1.7 | 0.9 | 0.0 | 1.5 |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 67.9 | 80.5 | 50.5 | 62.3 | 66.2 | 65.8 | 66.2 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 46 | 108 | 311 | 226 | 157 | 274 | 385 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 77.0 | 5.4 | 9.6 | 1.0 | 0.2 | 0.0 | 0.5 |
| Time, seconds (T) | — | 4.80 | 23.90 | 21.60 | 19.94 | 41.69 | — | 64.05 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2964 | 1284 | 3022 | 2733 | 2980 | 2952 | — | — |
| Irregular vertices (S) | 68 | 68 | 160 | 69 | 70 | 70 | — | — |
| … of which above the minimum | 60 | 60 | 152 | 61 | 62 | 62 | — | — |
| Base complex patches (C) | 2902 | 1022 | 2986 | 2369 | 1484 | 1372 | — | — |
| Alignment, degrees (A) | 18.12 | 12.08 | 17.18 | 10.09 | 9.58 | 8.43 | — | — |
| Mean angle deviation, degrees (F) | 7.25 | 8.87 | 5.91 | 8.36 | 5.45 | 5.61 | — | — |
| Angle deviation, 95th percentile (F) | 20.42 | 24.77 | 19.88 | 22.07 | 15.14 | 14.63 | — | — |
| Mean aspect ratio (F) | 1.194 | 1.451 | 1.179 | 1.291 | 1.297 | 1.303 | — | — |
| Area variation (F) | 0.177 | 0.666 | 0.172 | 0.237 | 0.293 | 0.426 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 17.26 | 15.66 | 8.76 | 5.49 | 3.39 | — | — |
| Mean distance, ‰ (H) | 0.448 | 1.588 | 0.481 | 0.243 | 0.207 | 0.196 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 56.3 | 54.6 | 46.1 | 9.5 | 5.5 | 2.9 | — | — |
| Volume kept, % of the source (V) | 99.37 | 97.44 | 99.38 | 99.99 | 100.02 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 10.2 | 1.0 | 6.7 | 26.6 | 27.4 | — | — |
| Edges on spiralling loops, % (L) | 22.5 | 5.6 | 12.8 | 32.0 | 2.2 | 0.0 | — | — |
| Spiralling loops (L) | 10 | 3 | 10 | 14 | 1 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 17.3 | 4.1 | 0.0 | 19.3 | 2.2 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 2 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.5 | 84.2 | 86.1 | 61.4 | 71.2 | 72.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 299 | 90 | 112 | 150 | 177 | 219 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.2 | 57.2 | 0.7 | 8.9 | 0.7 | 1.6 | — | — |
| Time, seconds (T) | 1.43 | 1.63 | 0.95 | 8.88 | 3.48 | 4.66 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2928 | 1583 | 3028 | 2757 | 2967 | 2968 | — | — |
| Irregular vertices (S) | 32 | 18 | 136 | 20 | 20 | 20 | — | — |
| … of which above the minimum | 32 | 18 | 136 | 20 | 20 | 20 | — | — |
| Base complex patches (C) | 2903 | 533 | 3018 | 328 | 219 | 231 | — | — |
| Alignment, degrees (A) | 19.37 | 6.64 | 19.60 | 3.97 | 3.97 | 3.88 | — | — |
| Mean angle deviation, degrees (F) | 6.41 | 7.07 | 5.40 | 4.45 | 3.39 | 3.48 | — | — |
| Angle deviation, 95th percentile (F) | 18.61 | 21.04 | 18.46 | 13.16 | 9.41 | 10.08 | — | — |
| Mean aspect ratio (F) | 1.175 | 2.090 | 1.161 | 1.389 | 1.362 | 1.345 | — | — |
| Area variation (F) | 0.159 | 0.474 | 0.164 | 0.357 | 0.401 | 0.438 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 4.03 | 6.38 | 2.47 | 2.24 | 1.62 | — | — |
| Mean distance, ‰ (H) | 0.309 | 0.960 | 0.321 | 0.145 | 0.136 | 0.137 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 98.10 | 99.45 | 100.01 | 100.02 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.6 | 45.2 | 0.0 | 70.7 | 75.3 | 74.2 | — | — |
| Edges on spiralling loops, % (L) | 35.4 | 11.1 | 2.9 | 11.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 9 | 3 | 2 | 4 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 20.6 | 5.4 | 0.0 | 1.3 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 2 | 1 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.6 | 43.7 | 97.1 | 18.3 | 24.7 | 25.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 309 | 172 | 126 | 100 | 112 | 122 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.4 | 47.2 | 0.9 | 8.1 | 1.1 | 1.1 | — | — |
| Time, seconds (T) | 1.65 | 1.12 | 1.38 | 5.73 | 2.32 | 3.53 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2698 | 1343 | 3044 | 2883 | 2977 | 3021 | — | — |
| Irregular vertices (S) | 52 | 47 | 136 | 47 | 48 | 56 | — | — |
| … of which above the minimum | 44 | — | 128 | 39 | 40 | 48 | — | — |
| Base complex patches (C) | 2584 | — | 2850 | 1711 | 909 | 737 | — | — |
| Alignment, degrees (A) | 15.95 | 10.62 | 15.43 | 8.63 | 8.44 | 8.14 | — | — |
| Mean angle deviation, degrees (F) | 10.72 | 9.14 | 5.36 | 6.51 | 4.37 | 4.63 | — | — |
| Angle deviation, 95th percentile (F) | 28.70 | 28.42 | 18.47 | 17.81 | 12.89 | 14.76 | — | — |
| Mean aspect ratio (F) | 1.271 | 2.046 | 1.179 | 1.237 | 1.282 | 1.285 | — | — |
| Area variation (F) | 0.261 | 0.522 | 0.171 | 0.240 | 0.302 | 0.392 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 17.71 | 11.98 | 9.27 | 7.89 | 7.32 | — | — |
| Mean distance, ‰ (H) | 0.588 | 1.082 | 0.380 | 0.230 | 0.198 | 0.189 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 46.5 | 17.7 | 22.0 | 15.4 | 7.9 | 7.0 | — | — |
| Volume kept, % of the source (V) | 98.93 | — | 99.19 | 99.89 | 99.95 | 99.94 | — | — |
| Sharp edges kept, % of their length (E) | 39.8 | 43.9 | 30.4 | 36.8 | 67.1 | 59.1 | — | — |
| Edges on closed loops, % (L) | 0.0 | — | 2.4 | 21.0 | 39.6 | 44.8 | — | — |
| Edges on spiralling loops, % (L) | 32.5 | — | 6.7 | 19.5 | 4.2 | 6.5 | — | — |
| Spiralling loops (L) | 13 | — | 8 | 6 | 2 | 2 | — | — |
| Edges on helices of any pitch, % (L) | 26.3 | — | 1.9 | 14.6 | 0.9 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 1 | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.2 | — | 89.4 | 59.6 | 56.2 | 48.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 225 | — | 158 | 227 | 200 | 227 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 10.1 | 55.2 | 1.5 | 3.9 | 0.8 | 0.7 | — | — |
| Time, seconds (T) | 1.51 | 1.25 | 1.00 | 7.74 | 4.21 | 7.57 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2675 | 1808 | 3048 | 2746 | 3040 | 3012 | — | — |
| Irregular vertices (S) | 122 | 99 | 214 | 70 | 74 | 94 | — | — |
| … of which above the minimum | 114 | — | 195 | — | 66 | 86 | — | — |
| Base complex patches (C) | 2612 | — | 2902 | — | 1348 | 1766 | — | — |
| Alignment, degrees (A) | 17.41 | 11.38 | 16.06 | 11.65 | 10.91 | 9.47 | — | — |
| Mean angle deviation, degrees (F) | 14.32 | 8.90 | 6.55 | 8.16 | 6.11 | 5.88 | — | — |
| Angle deviation, 95th percentile (F) | 41.52 | 29.13 | 23.95 | 22.22 | 17.77 | 17.12 | — | — |
| Mean aspect ratio (F) | 1.383 | 1.804 | 1.233 | 1.235 | 1.234 | 1.266 | — | — |
| Area variation (F) | 0.398 | 0.713 | 0.208 | 0.236 | 0.295 | 0.408 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 52.50 | 37.15 | 21.87 | 9.45 | 7.59 | — | — |
| Mean distance, ‰ (H) | 1.056 | 2.185 | 0.910 | 0.519 | 0.380 | 0.294 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 67.8 | 29.1 | 18.6 | 21.0 | 8.7 | 3.4 | — | — |
| Volume kept, % of the source (V) | 98.78 | 97.54 | 98.95 | 99.69 | 99.83 | 99.99 | — | — |
| Sharp edges kept, % of their length (E) | 21.2 | 17.5 | 15.3 | 31.0 | 56.6 | 64.3 | — | — |
| Edges on closed loops, % (L) | 0.4 | 10.6 | 2.6 | 16.0 | 31.4 | 22.8 | — | — |
| Edges on spiralling loops, % (L) | 9.5 | 5.7 | 5.2 | 31.5 | 15.6 | 9.5 | — | — |
| Spiralling loops (L) | 12 | 5 | 9 | 14 | 5 | 10 | — | — |
| Edges on helices of any pitch, % (L) | 23.1 | 2.3 | 1.5 | 24.5 | 1.2 | 4.3 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 5 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 90.5 | 83.7 | 92.3 | 52.5 | 52.9 | 67.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 218 | 113 | 114 | 156 | 194 | 175 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 10.8 | 39.7 | 1.6 | 8.5 | 1.3 | 0.4 | — | — |
| Time, seconds (T) | 1.97 | 1.56 | 0.96 | 7.73 | 5.77 | 8.49 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3122 | 1102 | 2926 | 2698 | 2986 | 3000 | — | — |
| Irregular vertices (S) | 104 | 84 | 252 | 99 | 102 | 173 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | — | 2894 | — | 2337 | 2589 | — | — |
| Alignment, degrees (A) | 17.93 | 17.98 | 18.61 | 15.23 | 14.56 | 14.26 | — | — |
| Mean angle deviation, degrees (F) | 11.25 | 15.26 | 8.38 | 9.33 | 6.34 | 8.56 | — | — |
| Angle deviation, 95th percentile (F) | 29.73 | 53.91 | 28.25 | 25.31 | 19.36 | 26.00 | — | — |
| Mean aspect ratio (F) | 1.301 | 2.330 | 1.338 | 1.335 | 1.454 | 1.503 | — | — |
| Area variation (F) | 0.278 | 2.551 | 0.249 | 0.271 | 0.320 | 0.546 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 200.69 | 39.39 | 32.82 | 14.86 | 12.62 | — | — |
| Mean distance, ‰ (H) | 1.467 | 5.546 | 2.191 | 1.441 | 0.870 | 0.808 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.4 | 76.1 | 52.6 | 29.2 | 11.8 | 5.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 47.5 | 6.5 | 13.7 | 57.3 | 82.5 | 82.9 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | 0.3 | 0.8 | 5.7 | 1.0 | — | — |
| Edges on spiralling loops, % (L) | 0.7 | 13.4 | 0.2 | 14.9 | 0.0 | 0.3 | — | — |
| Spiralling loops (L) | 1 | 6 | 6 | 8 | 0 | 1 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 3.6 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 82.0 | 86.6 | 98.4 | 72.2 | 88.9 | 92.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 155 | 64 | 76 | 161 | 148 | 128 | — | — |
| Edges on loops from a border to a border, % (L) | 17.0 | 0.0 | 0.8 | 12.2 | 5.5 | 6.6 | — | — |
| Off budget, % (N) | 4.1 | 63.3 | 2.5 | 10.1 | 0.5 | 0.0 | — | — |
| Time, seconds (T) | 1.46 | 1.44 | 0.47 | 11.26 | 3.25 | 7.97 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2085 | 636 | 2219 | 2204 | 3273 | 3521 | — | — |
| Irregular vertices (S) | 539 | 243 | 518 | 331 | 866 | 1196 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 1872 | — | 1980 | — | — | — | — | — |
| Alignment, degrees (A) | 18.29 | 19.39 | 18.93 | 17.44 | 15.40 | 15.69 | — | — |
| Mean angle deviation, degrees (F) | 16.52 | 31.06 | 15.46 | 13.95 | 21.62 | 29.30 | — | — |
| Angle deviation, 95th percentile (F) | 48.25 | 82.38 | 48.89 | 41.14 | 65.34 | 69.54 | — | — |
| Mean aspect ratio (F) | 1.647 | 5.541 | 1.811 | 1.601 | 5.679 | 6.603 | — | — |
| Area variation (F) | 0.588 | 1.193 | 0.467 | 0.392 | 1.282 | 2.063 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 74.84 | 108.28 | 78.60 | 72.30 | 24.37 | 34.60 | — | — |
| Mean distance, ‰ (H) | 3.755 | 21.969 | 4.274 | 3.223 | 2.295 | 3.379 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 39.4 | 157.4 | 46.5 | 36.3 | 21.1 | 28.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 84.44 | — | — | — |
| Sharp edges kept, % of their length (E) | 11.5 | 1.0 | 7.8 | 10.2 | 35.3 | 34.8 | — | — |
| Edges on closed loops, % (L) | 3.2 | — | 6.3 | — | 7.6 | 2.9 | — | — |
| Edges on spiralling loops, % (L) | 17.6 | — | 3.0 | — | 28.9 | 28.3 | — | — |
| Spiralling loops (L) | 82 | — | 31 | — | 518 | 589 | — | — |
| Edges on helices of any pitch, % (L) | 8.0 | — | 0.0 | — | 8.2 | 11.8 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.7 | — | 88.7 | — | 63.5 | 68.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 38 | — | 26 | — | 66 | 39 | — | — |
| Edges on loops from a border to a border, % (L) | 0.5 | — | 2.0 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 30.5 | 78.8 | 26.0 | 26.5 | 9.1 | 17.4 | — | — |
| Time, seconds (T) | 1.99 | 1.29 | 1.45 | 38.23 | 18.40 | 48.28 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1893 | 2903 | 2864 | 2997 | 2971 | — | — |
| Irregular vertices (S) | — | 63 | 162 | 36 | 38 | 40 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1937 | 508 | 572 | 500 | — | — |
| Alignment, degrees (A) | — | 9.17 | 12.40 | 8.62 | 8.30 | 7.54 | — | — |
| Mean angle deviation, degrees (F) | — | 7.60 | 6.06 | 5.75 | 3.52 | 4.31 | — | — |
| Angle deviation, 95th percentile (F) | — | 22.56 | 21.08 | 16.21 | 11.86 | 12.84 | — | — |
| Mean aspect ratio (F) | — | 1.778 | 1.215 | 1.268 | 1.271 | 1.268 | — | — |
| Area variation (F) | — | 0.765 | 0.177 | 0.237 | 0.279 | 0.418 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 14.76 | 26.62 | 6.82 | 6.90 | 9.33 | — | — |
| Mean distance, ‰ (H) | — | 1.042 | 0.626 | 0.495 | 0.466 | 0.459 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.4 | 4.4 | 19.7 | 21.4 | 26.1 | — | — |
| Edges on spiralling loops, % (L) | — | 8.2 | 5.5 | 10.0 | 0.0 | 0.4 | — | — |
| Spiralling loops (L) | — | 4 | 8 | 4 | 0 | 1 | — | — |
| Edges on helices of any pitch, % (L) | — | 5.8 | 2.0 | 4.5 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 62.1 | 74.6 | 39.0 | 44.2 | 39.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 70 | 58 | 125 | 94 | 94 | — | — |
| Edges on loops from a border to a border, % (L) | — | 10.3 | 18.4 | 31.3 | 34.5 | 33.7 | — | — |
| Off budget, % (N) | — | 36.9 | 3.2 | 4.5 | 0.1 | 1.0 | — | — |
| Time, seconds (T) | — | 1.95 | 2.84 | 8.30 | 4.00 | 4.78 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2890 | 1855 | 2938 | 3026 | 2945 | 3036 | — | — |
| Irregular vertices (S) | 28 | 117 | 204 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 20 | — | 196 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 619 | — | 1782 | 576 | 78 | 66 | — | — |
| Alignment, degrees (A) | 12.97 | 15.75 | 13.96 | 12.23 | 12.29 | 11.03 | — | — |
| Mean angle deviation, degrees (F) | 2.66 | 12.33 | 6.10 | 2.88 | 1.38 | 1.17 | — | — |
| Angle deviation, 95th percentile (F) | 10.11 | 37.42 | 25.03 | 9.03 | 5.07 | 4.36 | — | — |
| Mean aspect ratio (F) | 1.116 | 2.476 | 1.233 | 1.146 | 1.133 | 1.102 | — | — |
| Area variation (F) | 0.100 | 0.672 | 0.199 | 0.110 | 0.126 | 0.309 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 17.76 | 9.41 | 6.26 | 11.57 | 0.56 | — | — |
| Mean distance, ‰ (H) | 0.114 | 1.229 | 0.067 | 0.159 | 0.068 | 0.065 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.15 | — | 100.14 | 99.98 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 99.9 | 7.8 | 99.4 | 88.7 | 99.5 | 100.0 | — | — |
| Edges on closed loops, % (L) | 51.3 | — | 22.2 | 50.1 | 83.5 | 85.0 | — | — |
| Edges on spiralling loops, % (L) | 26.0 | — | 3.8 | 36.4 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 7 | — | 4 | 6 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 26.0 | — | 0.0 | 35.3 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 21.7 | — | 73.5 | 13.5 | 16.5 | 15.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | 112 | 80 | 56 | 50 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.7 | 38.2 | 2.1 | 0.9 | 1.8 | 1.2 | — | — |
| Time, seconds (T) | 2.11 | 1.24 | 0.55 | 9.28 | 2.45 | 2.98 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2904 | 1260 | 2974 | 2992 | 2992 | 2992 | — | — |
| Irregular vertices (S) | 8 | 8 | 24 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 16 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | 36 | 274 | 1102 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.29 | 2.29 | 1.97 | 0.75 | 0.14 | 0.14 | — | — |
| Mean angle deviation, degrees (F) | 0.18 | 3.61 | 1.65 | 0.75 | 0.01 | 0.01 | — | — |
| Angle deviation, 95th percentile (F) | 0.43 | 11.71 | 4.89 | 2.01 | 0.01 | 0.01 | — | — |
| Mean aspect ratio (F) | 1.005 | 2.174 | 1.062 | 1.039 | 1.031 | 1.031 | — | — |
| Area variation (F) | 0.004 | 0.662 | 0.090 | 0.045 | 0.021 | 0.021 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 10.77 | 0.00 | 8.83 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.017 | 0.439 | 0.000 | 0.067 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.89 | 100.00 | 99.99 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 45.3 | 100.0 | 91.2 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 95.5 | 82.9 | 68.8 | 31.6 | 95.5 | 95.5 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | 6.8 | 64.7 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | 2 | 2 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 0.0 | 6.8 | 64.7 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 4.5 | 17.1 | 29.0 | 3.7 | 4.5 | 4.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 22 | 58 | 90 | 23 | 23 | 23 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.2 | 58.0 | 1.0 | 0.3 | 0.3 | 0.3 | — | — |
| Time, seconds (T) | 0.66 | 1.25 | 0.16 | 4.91 | 2.32 | 3.53 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2928 | 1328 | 2806 | 2800 | 3060 | 3060 | — | — |
| Irregular vertices (S) | 12 | 12 | 20 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | 4 | 12 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 86 | 628 | 724 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 1.09 | 3.78 | 1.79 | 0.59 | 0.52 | 0.52 | — | — |
| Mean angle deviation, degrees (F) | 0.80 | 5.83 | 1.99 | 0.23 | 0.01 | 0.01 | — | — |
| Angle deviation, 95th percentile (F) | 2.44 | 18.79 | 5.65 | 0.98 | 0.05 | 0.05 | — | — |
| Mean aspect ratio (F) | 1.047 | 1.994 | 1.052 | 1.022 | 1.030 | 1.030 | — | — |
| Area variation (F) | 0.039 | 0.572 | 0.062 | 0.023 | 0.033 | 0.033 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 10.33 | 11.65 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.034 | 0.454 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.83 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 99.4 | 58.4 | 100.0 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 78.3 | 26.4 | 47.0 | 92.9 | 93.2 | 93.2 | — | — |
| Edges on spiralling loops, % (L) | 13.8 | 62.2 | 26.2 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 1 | 5 | 5 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 13.8 | 62.2 | 26.2 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 7.9 | 11.4 | 22.0 | 7.1 | 6.8 | 6.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 86 | 58 | 114 | 42 | 45 | 45 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.4 | 55.7 | 6.5 | 6.7 | 2.0 | 2.0 | — | — |
| Time, seconds (T) | 1.08 | 0.71 | 0.14 | 16.83 | 1.96 | 3.08 | — | — |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2900 | 1306 | 2972 | 2871 | 2992 | 2958 | — | — |
| Irregular vertices (S) | 8 | 8 | 88 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 80 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 1003 | 68 | 2964 | 125 | 6 | 6 | — | — |
| Alignment, degrees (A) | 4.18 | 4.72 | 5.27 | 1.23 | 1.52 | 1.73 | — | — |
| Mean angle deviation, degrees (F) | 4.32 | 8.13 | 4.25 | 2.31 | 2.45 | 3.12 | — | — |
| Angle deviation, 95th percentile (F) | 17.78 | 26.13 | 15.62 | 10.43 | 6.67 | 7.30 | — | — |
| Mean aspect ratio (F) | 1.087 | 2.019 | 1.145 | 1.157 | 1.166 | 1.299 | — | — |
| Area variation (F) | 0.103 | 0.640 | 0.145 | 0.182 | 0.171 | 0.137 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 9.51 | 0.76 | 0.79 | 0.78 | 0.75 | — | — |
| Mean distance, ‰ (H) | 0.144 | 0.790 | 0.120 | 0.115 | 0.132 | 0.126 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.21 | 99.90 | 100.00 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 32.7 | 100.0 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 33.0 | 74.5 | 0.0 | 64.5 | 95.4 | 95.3 | — | — |
| Edges on spiralling loops, % (L) | 55.4 | 0.0 | 26.5 | 29.9 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 6 | 0 | 2 | 3 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 47.3 | 0.0 | 22.1 | 29.9 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 1 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 11.6 | 25.5 | 73.5 | 5.6 | 4.6 | 4.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 207 | 178 | 130 | 51 | 34 | 35 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 56.5 | 0.9 | 4.3 | 0.3 | 1.4 | — | — |
| Time, seconds (T) | 0.97 | 1.06 | 0.21 | 2.17 | 1.69 | 3.25 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4046 | 2489 | 2729 | 2852 | 2905 | — | — |
| Irregular vertices (S) | — | 1527 | 432 | 192 | 206 | 274 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1464 | — | — | — | — | — |
| Alignment, degrees (A) | — | 11.84 | 11.84 | 11.90 | 10.19 | 11.93 | — | — |
| Mean angle deviation, degrees (F) | — | 48.87 | 13.84 | 9.52 | 9.40 | 11.43 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.70 | 52.08 | 33.20 | 31.35 | 40.64 | — | — |
| Mean aspect ratio (F) | — | 12.801 | 1.928 | 1.620 | 1.564 | 2.270 | — | — |
| Area variation (F) | — | 2.373 | 0.475 | 0.326 | 0.316 | 0.472 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 71.10 | 40.69 | 39.53 | 38.29 | 38.29 | — | — |
| Mean distance, ‰ (H) | — | 5.233 | 4.298 | 4.151 | 2.859 | 3.071 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 24.5 | 28.2 | 26.8 | 16.7 | 17.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 3.3 | 7.3 | 9.6 | 45.8 | 43.8 | — | — |
| Edges on closed loops, % (L) | — | — | 2.6 | 19.0 | 20.6 | 16.2 | — | — |
| Edges on spiralling loops, % (L) | — | — | 3.7 | 12.2 | 22.0 | 18.5 | — | — |
| Spiralling loops (L) | — | — | 84 | 17 | 57 | 53 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | 5.5 | 10.0 | 10.2 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 63.4 | 66.4 | 57.1 | 65.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 29 | 113 | 94 | 94 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 30.0 | 2.5 | 0.3 | 0.3 | — | — |
| Off budget, % (N) | — | 34.9 | 17.0 | 9.0 | 4.9 | 3.2 | — | — |
| Time, seconds (T) | — | 1.22 | 0.73 | 19.15 | 19.80 | 29.64 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1283 | 2836 | 2412 | 2952 | 3039 | — | — |
| Irregular vertices (S) | — | 87 | 223 | 74 | 90 | 192 | — | — |
| … of which above the minimum | — | — | 219 | 66 | 82 | 184 | — | — |
| Base complex patches (C) | — | — | 2718 | 1940 | 1872 | 1931 | — | — |
| Alignment, degrees (A) | — | 16.86 | 18.44 | 14.64 | 13.60 | 11.67 | — | — |
| Mean angle deviation, degrees (F) | — | 10.34 | 7.84 | 9.48 | 6.76 | 6.82 | — | — |
| Angle deviation, 95th percentile (F) | — | 29.60 | 25.71 | 27.03 | 18.65 | 19.89 | — | — |
| Mean aspect ratio (F) | — | 1.786 | 1.295 | 1.526 | 1.560 | 1.411 | — | — |
| Area variation (F) | — | 0.654 | 0.236 | 0.307 | 0.379 | 0.820 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 30.78 | 61.14 | 33.66 | 27.56 | 28.77 | — | — |
| Mean distance, ‰ (H) | — | 2.902 | 1.839 | 1.158 | 0.782 | 0.770 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 57.6 | 145.0 | 59.9 | 24.0 | 2.5 | — | — |
| Volume kept, % of the source (V) | — | 95.87 | 98.43 | 99.82 | 99.93 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | — | 2.2 | 6.6 | 48.1 | 42.8 | 27.2 | — | — |
| Edges on closed loops, % (L) | — | 10.2 | 2.2 | 9.8 | 18.3 | 19.2 | — | — |
| Edges on spiralling loops, % (L) | — | 16.6 | 2.4 | 32.5 | 4.9 | 5.2 | — | — |
| Spiralling loops (L) | — | 12 | 4 | 13 | 4 | 8 | — | — |
| Edges on helices of any pitch, % (L) | — | 5.3 | 0.0 | 14.6 | 0.0 | 3.3 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 73.3 | 96.0 | 57.7 | 76.8 | 75.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 51 | 108 | 124 | 170 | 122 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 57.2 | 5.5 | 19.6 | 1.6 | 1.3 | — | — |
| Time, seconds (T) | — | 13.33 | 81.08 | 68.76 | 106.34 | 229.20 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2675 | 709 | 1784 | 2484 | 2499 | 2496 | — | — |
| Irregular vertices (S) | 302 | 129 | 307 | 241 | 254 | 256 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2588 | — | 1676 | — | — | — | — | — |
| Alignment, degrees (A) | 14.64 | 19.69 | 16.44 | 15.61 | 15.75 | 15.78 | — | — |
| Mean angle deviation, degrees (F) | 11.57 | 21.19 | 17.11 | 11.19 | 12.64 | 12.65 | — | — |
| Angle deviation, 95th percentile (F) | 39.87 | 71.32 | 57.84 | 38.89 | 46.37 | 47.18 | — | — |
| Mean aspect ratio (F) | 1.472 | 2.842 | 1.945 | 1.563 | 1.721 | 1.725 | — | — |
| Area variation (F) | 0.335 | 0.833 | 0.490 | 0.405 | 0.614 | 0.618 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 86.07 | 187.44 | 74.17 | 58.97 | 59.13 | 59.13 | — | — |
| Mean distance, ‰ (H) | 5.115 | 23.533 | 7.799 | 4.757 | 4.449 | 4.549 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 31.4 | 104.7 | 43.1 | 27.0 | 24.2 | 25.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 30.5 | 2.6 | 11.7 | 20.7 | 21.3 | 20.6 | — | — |
| Edges on closed loops, % (L) | 2.2 | — | 3.8 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.9 | — | 4.2 | — | — | — | — | — |
| Spiralling loops (L) | 27 | — | 56 | — | — | — | — | — |
| Edges on helices of any pitch, % (L) | 2.6 | — | 0.0 | — | — | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.4 | — | 80.0 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | — | 41 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 2.6 | — | 14.4 | — | — | — | — | — |
| Off budget, % (N) | 10.8 | 76.4 | 40.5 | 17.2 | 16.7 | 16.8 | — | — |
| Time, seconds (T) | 2.11 | 2.12 | 0.61 | 40.03 | 127.27 | 57.56 | — | — |

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2724 | 1088 | 1904 | 2360 | 3015 | 2360 | — | — |
| Irregular vertices (S) | 378 | 194 | 370 | 157 | 327 | 157 | — | — |
| … of which above the minimum | — | — | — | — | 23 | — | — | — |
| Base complex patches (C) | 2661 | — | 1899 | — | 2665 | — | — | — |
| Alignment, degrees (A) | 13.28 | 16.22 | 16.31 | 13.33 | 14.25 | 13.33 | — | — |
| Mean angle deviation, degrees (F) | 11.93 | 19.35 | 19.44 | 10.97 | 13.56 | 10.97 | — | — |
| Angle deviation, 95th percentile (F) | 38.83 | 64.57 | 61.40 | 36.94 | 45.03 | 36.94 | — | — |
| Mean aspect ratio (F) | 1.501 | 2.846 | 2.098 | 1.674 | 3.822 | 1.674 | — | — |
| Area variation (F) | 0.329 | 0.711 | 0.515 | 0.352 | 0.674 | 0.352 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.00 | 98.83 | 77.09 | 42.76 | 53.34 | 42.76 | — | — |
| Mean distance, ‰ (H) | 3.285 | 12.802 | 6.169 | 4.001 | 3.406 | 4.001 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.0 | 56.9 | 34.1 | 21.1 | 20.6 | 21.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 94.49 | — | — | — |
| Sharp edges kept, % of their length (E) | 33.2 | 4.6 | 11.5 | 19.1 | 21.7 | 19.1 | — | — |
| Edges on closed loops, % (L) | 0.6 | — | 1.7 | — | 6.2 | — | — | — |
| Edges on spiralling loops, % (L) | 10.8 | — | 4.3 | — | 6.1 | — | — | — |
| Spiralling loops (L) | 20 | — | 47 | — | 42 | — | — | — |
| Edges on helices of any pitch, % (L) | 1.7 | — | 0.0 | — | 1.9 | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | 0 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.2 | — | 84.9 | — | 87.7 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 61 | — | 34 | — | 92 | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.4 | — | 9.7 | — | 0.0 | — | — | — |
| Off budget, % (N) | 9.2 | 63.7 | 36.5 | 21.3 | 0.5 | 21.3 | — | — |
| Time, seconds (T) | 1.87 | 1.75 | 0.60 | 27.15 | 44.01 | 53.01 | — | — |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2635 | 1809 | 2664 | 2607 | 3004 | 3005 | — | — |
| Irregular vertices (S) | 242 | 99 | 287 | 118 | 128 | 212 | — | — |
| … of which above the minimum | 206 | — | 247 | — | 64 | 148 | — | — |
| Base complex patches (C) | 2605 | — | 2586 | — | 2543 | 2501 | — | — |
| Alignment, degrees (A) | 13.78 | 14.83 | 14.29 | 14.10 | 13.67 | 13.39 | — | — |
| Mean angle deviation, degrees (F) | 9.37 | 11.26 | 9.56 | 9.83 | 6.26 | 9.18 | — | — |
| Angle deviation, 95th percentile (F) | 28.53 | 33.83 | 32.90 | 29.23 | 19.74 | 28.94 | — | — |
| Mean aspect ratio (F) | 1.324 | 2.004 | 1.440 | 1.374 | 1.289 | 1.651 | — | — |
| Area variation (F) | 0.239 | 0.710 | 0.288 | 0.283 | 0.271 | 0.563 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.66 | 106.61 | 66.01 | 30.17 | 20.39 | 28.07 | — | — |
| Mean distance, ‰ (H) | 2.229 | 4.106 | 2.742 | 2.328 | 1.927 | 1.737 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 40.9 | 155.1 | 65.7 | 37.0 | 19.2 | 19.8 | — | — |
| Volume kept, % of the source (V) | 98.96 | — | 98.49 | — | 99.87 | 99.83 | — | — |
| Sharp edges kept, % of their length (E) | 34.3 | 6.7 | 13.0 | 23.3 | 24.5 | 39.4 | — | — |
| Edges on closed loops, % (L) | 2.5 | — | 1.3 | — | 8.1 | 8.7 | — | — |
| Edges on spiralling loops, % (L) | 5.2 | — | 2.3 | — | 9.8 | 2.3 | — | — |
| Spiralling loops (L) | 12 | — | 7 | — | 20 | 6 | — | — |
| Edges on helices of any pitch, % (L) | 0.5 | — | 0.0 | — | 0.4 | 1.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 92.4 | — | 95.9 | — | 82.1 | 89.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 92 | — | 81 | — | 183 | 131 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 0.1 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 12.2 | 39.7 | 11.2 | 13.1 | 0.1 | 0.2 | — | — |
| Time, seconds (T) | 1.89 | 0.84 | 0.86 | 24.99 | 16.76 | 25.55 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1003 | 1886 | — | 140 | 187 | — | — |
| Irregular vertices (S) | — | 145 | 312 | — | 33 | 40 | — | — |
| … of which above the minimum | — | — | — | — | 9 | 16 | — | — |
| Base complex patches (C) | — | — | 1778 | — | 132 | 187 | — | — |
| Alignment, degrees (A) | — | 14.39 | 15.52 | — | 16.61 | 17.22 | — | — |
| Mean angle deviation, degrees (F) | — | 20.17 | 15.16 | — | 31.07 | 30.40 | — | — |
| Angle deviation, 95th percentile (F) | — | 69.91 | 53.30 | — | 71.99 | 72.47 | — | — |
| Mean aspect ratio (F) | — | 2.697 | 1.760 | — | 3.905 | 8.490 | — | — |
| Area variation (F) | — | 0.807 | 0.437 | — | 1.460 | 1.399 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 193.85 | 69.08 | — | 529.03 | 531.21 | — | — |
| Mean distance, ‰ (H) | — | 18.937 | 5.814 | — | 254.555 | 251.519 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 122.8 | 42.9 | — | 1186.2 | 1185.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 0.76 | 0.75 | — | — |
| Sharp edges kept, % of their length (E) | — | 2.0 | 10.0 | — | 1.7 | 2.0 | — | — |
| Edges on closed loops, % (L) | — | — | 2.2 | — | 2.9 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | 3.3 | — | 19.6 | 0.0 | — | — |
| Spiralling loops (L) | — | — | 49 | — | 8 | 0 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | — | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 85.4 | — | 77.5 | 100.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 36 | — | 20 | 20 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 9.5 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 66.6 | 37.1 | — | 95.3 | 93.8 | — | — |
| Time, seconds (T) | — | 1.58 | 0.61 | — | 2.55 | 11.47 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 2 non-manifold or inconsistently oriented edges.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2717 | 919 | 1785 | 2417 | 2978 | 3015 | — | — |
| Irregular vertices (S) | 112 | 45 | 115 | 144 | 40 | 90 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 826 | — | 1283 | — | 953 | 1742 | — | — |
| Alignment, degrees (A) | 20.16 | 20.27 | 21.08 | 17.62 | 19.23 | 18.51 | — | — |
| Mean angle deviation, degrees (F) | 6.49 | 6.70 | 8.35 | 11.44 | 5.75 | 6.20 | — | — |
| Angle deviation, 95th percentile (F) | 23.46 | 24.56 | 27.43 | 30.12 | 16.30 | 18.85 | — | — |
| Mean aspect ratio (F) | 1.209 | 1.423 | 1.306 | 1.464 | 1.239 | 1.297 | — | — |
| Area variation (F) | 0.187 | 0.413 | 0.220 | 0.361 | 0.247 | 0.440 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 23.91 | 34.95 | 146.92 | 20.80 | 22.09 | 17.87 | — | — |
| Mean distance, ‰ (H) | 1.851 | 3.051 | 2.346 | 1.548 | 1.307 | 1.116 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 56.8 | 38.2 | 28.1 | 23.0 | 18.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 11.4 | 2.1 | 4.6 | 10.8 | 13.4 | 18.9 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | 0.0 | — | 0.0 | 3.1 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | 0.0 | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | 0 | — | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 0.0 | 0.0 | — | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.0 | 67.6 | 78.0 | — | 54.9 | 75.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 46 | 27 | 41 | — | 127 | 107 | — | — |
| Edges on loops from a border to a border, % (L) | 47.0 | 32.4 | 22.0 | — | 45.1 | 21.3 | — | — |
| Off budget, % (N) | 9.4 | 69.4 | 40.5 | 19.4 | 0.7 | 0.5 | — | — |
| Time, seconds (T) | 17.72 | 2.22 | 7.20 | 93.73 | 30.36 | 39.86 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1325 | 1680 | 2070 | 1810 | 3215 | — | — |
| Irregular vertices (S) | — | 422 | 171 | 79 | 477 | 490 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1286 | — | 568 | — | — | — |
| Alignment, degrees (A) | — | 9.34 | 10.21 | 7.86 | 14.32 | 8.52 | — | — |
| Mean angle deviation, degrees (F) | — | 32.26 | 9.58 | 9.29 | 26.16 | 24.81 | — | — |
| Angle deviation, 95th percentile (F) | — | 85.28 | 33.49 | 30.88 | 77.38 | 78.62 | — | — |
| Mean aspect ratio (F) | — | 5.235 | 1.467 | 1.934 | 64.721 | 131.902 | — | — |
| Area variation (F) | — | 1.443 | 0.352 | 0.539 | 1.968 | 2.758 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 44.66 | 49.79 | 29.42 | 58.43 | 58.43 | — | — |
| Mean distance, ‰ (H) | — | 5.890 | 7.421 | 3.753 | 4.326 | 4.271 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 46.8 | 70.2 | 34.8 | 35.7 | 35.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 7.2 | 1.4 | 12.9 | 31.1 | 31.4 | — | — |
| Edges on closed loops, % (L) | — | — | 7.9 | 28.2 | 12.0 | — | — | — |
| Edges on spiralling loops, % (L) | — | — | 11.1 | 2.2 | 73.2 | — | — | — |
| Spiralling loops (L) | — | — | 31 | 4 | 751 | — | — | — |
| Edges on helices of any pitch, % (L) | — | — | 5.3 | 0.0 | 45.3 | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 75.3 | 50.6 | 9.3 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 70 | 95 | 8 | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 6.6 | 19.0 | 5.6 | — | — | — |
| Off budget, % (N) | — | 55.8 | 44.0 | 31.0 | 39.7 | 7.2 | — | — |
| Time, seconds (T) | — | 1.38 | 4.78 | 54.42 | 161.87 | 217.93 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 870 | 1246 | — | 8231 | 7908 | — | — |
| Irregular vertices (S) | — | 858 | 605 | — | 6101 | 5973 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1191 | — | 6431 | — | — | — |
| Alignment, degrees (A) | — | 18.06 | 19.30 | — | 11.09 | 10.45 | — | — |
| Mean angle deviation, degrees (F) | — | 52.53 | 23.75 | — | 19.31 | 19.14 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.72 | 61.64 | — | 68.87 | 68.41 | — | — |
| Mean aspect ratio (F) | — | 14.155 | 2.891 | — | 11.605 | 61.920 | — | — |
| Area variation (F) | — | 7.748 | 0.797 | — | 3.336 | 2.827 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 9.99 | 7.30 | — | 5.64 | 7.13 | — | — |
| Mean distance, ‰ (H) | — | 2.873 | 1.821 | — | 0.425 | 0.425 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.1 | 57.3 | — | 12.0 | 11.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 3.5 | 10.6 | — | 59.7 | 61.1 | — | — |
| Edges on closed loops, % (L) | — | — | 3.8 | — | 12.8 | — | — | — |
| Edges on spiralling loops, % (L) | — | — | 16.8 | — | 48.1 | — | — | — |
| Spiralling loops (L) | — | — | 188 | — | 6723 | — | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | — | 2.2 | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | — | 0 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 78.4 | — | 38.5 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 12 | — | 22 | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 1.0 | — | 0.6 | — | — | — |
| Off budget, % (N) | — | 71.0 | 58.5 | — | 174.4 | 163.6 | — | — |
| Time, seconds (T) | — | 1.14 | 2.62 | — | 131.98 | 202.59 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 10 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4122 | 1500 | 2101 | 4480 | 4300 | — | — |
| Irregular vertices (S) | — | 2217 | 403 | 227 | 1982 | 1980 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1329 | — | 2956 | 3086 | — | — |
| Alignment, degrees (A) | — | 14.85 | 12.61 | 7.24 | 7.11 | 8.24 | — | — |
| Mean angle deviation, degrees (F) | — | 41.32 | 19.76 | 11.56 | 15.27 | 16.12 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.07 | 64.46 | 38.33 | 64.53 | 61.73 | — | — |
| Mean aspect ratio (F) | — | 7.957 | 2.173 | 1.828 | 60.033 | 65.278 | — | — |
| Area variation (F) | — | 2.547 | 0.539 | 0.428 | 2.154 | 2.042 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 87.83 | 95.94 | 96.86 | 38.56 | 31.15 | — | — |
| Mean distance, ‰ (H) | — | 9.067 | 20.293 | 16.167 | 1.377 | 1.372 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 62.6 | 143.8 | 121.1 | 10.1 | 10.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 5.7 | 26.6 | 30.2 | 58.7 | 53.5 | — | — |
| Edges on closed loops, % (L) | — | — | 3.4 | — | 17.2 | 13.9 | — | — |
| Edges on spiralling loops, % (L) | — | — | 12.8 | — | 44.1 | 41.4 | — | — |
| Spiralling loops (L) | — | — | 152 | — | 2040 | 1928 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 1.1 | — | 6.9 | 9.5 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 75.8 | — | 37.4 | 42.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 34 | — | 35 | 43 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 8.4 | — | 1.3 | 1.8 | — | — |
| Off budget, % (N) | — | 37.4 | 50.0 | 30.0 | 49.3 | 43.3 | — | — |
| Time, seconds (T) | — | 1.88 | 4.08 | 80.01 | 44.88 | 62.49 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 2222 | 2216 | 1113 | 2878 | 3075 | — | — |
| Irregular vertices (S) | — | 338 | 265 | 81 | 159 | 159 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1675 | — | 529 | 674 | — | — |
| Alignment, degrees (A) | — | 7.28 | 8.44 | 8.17 | 5.23 | 4.64 | — | — |
| Mean angle deviation, degrees (F) | — | 22.14 | 11.96 | 13.02 | 15.21 | 16.72 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.37 | 45.38 | 48.92 | 73.32 | 73.50 | — | — |
| Mean aspect ratio (F) | — | 4.205 | 1.771 | 2.915 | 17.807 | 23.193 | — | — |
| Area variation (F) | — | 2.439 | 0.430 | 0.718 | 1.128 | 1.194 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 50.21 | 35.63 | 37.83 | 33.19 | 33.28 | — | — |
| Mean distance, ‰ (H) | — | 5.701 | 4.253 | 4.790 | 1.819 | 1.683 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 25.0 | 19.2 | 19.7 | 7.7 | 7.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 11.9 | 20.2 | 24.7 | 64.4 | 70.9 | — | — |
| Edges on closed loops, % (L) | — | — | 9.1 | 37.6 | 56.2 | 48.8 | — | — |
| Edges on spiralling loops, % (L) | — | — | 9.4 | 5.5 | 5.9 | 9.7 | — | — |
| Spiralling loops (L) | — | — | 47 | 18 | 56 | 72 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 6.3 | 0.0 | 2.2 | 5.2 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 75.7 | 30.8 | 24.5 | 28.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 62 | 30 | 71 | 62 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 7.3 | 26.0 | 13.4 | 13.4 | — | — |
| Off budget, % (N) | — | 25.9 | 26.1 | 62.9 | 4.1 | 2.5 | — | — |
| Time, seconds (T) | — | 0.78 | 1.21 | 91.04 | 9.58 | 9.89 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 625 | 2774 | 2269 | 2501 | 2504 | — | — |
| Irregular vertices (S) | — | 125 | 430 | 224 | 264 | 270 | — | — |
| … of which above the minimum | — | — | 387 | — | — | — | — | — |
| Base complex patches (C) | — | — | 2517 | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.57 | 14.83 | 15.42 | 13.98 | 13.79 | — | — |
| Mean angle deviation, degrees (F) | — | 16.08 | 11.44 | 16.54 | 14.43 | 14.46 | — | — |
| Angle deviation, 95th percentile (F) | — | 44.86 | 37.21 | 44.67 | 39.10 | 39.07 | — | — |
| Mean aspect ratio (F) | — | 2.516 | 1.550 | 1.921 | 2.001 | 1.922 | — | — |
| Area variation (F) | — | 0.933 | 0.323 | 0.568 | 0.382 | 0.389 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 89.72 | 44.20 | 46.18 | 45.74 | 45.74 | — | — |
| Mean distance, ‰ (H) | — | 9.165 | 3.465 | 3.408 | 2.411 | 2.432 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 125.4 | 88.0 | 82.5 | 56.6 | 57.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 2.7 | 17.2 | 37.3 | 35.6 | 35.4 | — | — |
| Edges on closed loops, % (L) | — | — | 5.0 | 10.7 | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | 5.0 | 16.3 | — | — | — | — |
| Spiralling loops (L) | — | — | 25 | 24 | — | — | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.9 | 14.0 | — | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 91.2 | 73.0 | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 54 | 86 | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 0.0 | 0.0 | — | — | — | — |
| Off budget, % (N) | — | 79.2 | 7.5 | 24.4 | 16.6 | 16.5 | — | — |
| Time, seconds (T) | — | 1.32 | 0.33 | 37.56 | 38.96 | 37.59 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3107 | 2086 | 3084 | 3032 | 2990 | 2990 | — | — |
| Irregular vertices (S) | 8 | 10 | 84 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 2 | 76 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 2730 | 2004 | 3084 | 84 | 6 | 6 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 7.65 | 3.91 | 3.04 | 1.23 | 1.23 | — | — |
| Angle deviation, 95th percentile (F) | 12.67 | 19.88 | 12.98 | 8.59 | 3.57 | 3.58 | — | — |
| Mean aspect ratio (F) | 1.125 | 1.326 | 1.108 | 1.129 | 1.108 | 1.108 | — | — |
| Area variation (F) | 0.135 | 0.341 | 0.136 | 0.115 | 0.142 | 0.142 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 1.54 | 0.72 | 0.38 | 0.35 | 0.35 | — | — |
| Mean distance, ‰ (H) | 0.216 | 0.408 | 0.225 | 0.167 | 0.191 | 0.191 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.59 | 99.79 | 100.01 | 100.01 | 100.01 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 6.1 | 2.0 | 0.0 | 59.3 | 95.5 | 95.5 | — | — |
| Edges on spiralling loops, % (L) | 70.1 | 74.4 | 9.2 | 34.4 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 5 | 6 | 4 | 2 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 93.0 | 81.6 | 0.0 | 34.4 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 2 | 1 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 15.4 | 23.6 | 90.8 | 6.3 | 4.5 | 4.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 276 | 298 | 164 | 92 | 23 | 23 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 30.5 | 2.8 | 1.1 | 0.3 | 0.3 | — | — |
| Time, seconds (T) | 1.46 | 0.62 | 0.36 | 3.29 | 1.84 | 2.71 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2976 | 2672 | 3122 | 2720 | 2952 | 2952 | — | — |
| Irregular vertices (S) | 30 | 0 | 138 | 0 | 0 | 0 | — | — |
| … of which above the minimum | 30 | 0 | 138 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 2976 | 1 | 3088 | 1 | 1 | 1 | — | — |
| Alignment, degrees (A) | 10.14 | 10.37 | 13.88 | 0.66 | 0.62 | 0.62 | — | — |
| Mean angle deviation, degrees (F) | 10.25 | 4.17 | 5.54 | 1.46 | 1.43 | 1.43 | — | — |
| Angle deviation, 95th percentile (F) | 26.29 | 11.07 | 18.12 | 2.91 | 2.35 | 2.35 | — | — |
| Mean aspect ratio (F) | 1.239 | 2.545 | 1.165 | 1.351 | 1.323 | 1.323 | — | — |
| Area variation (F) | 0.223 | 0.256 | 0.167 | 0.281 | 0.307 | 0.307 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.50 | 1.68 | 0.60 | 0.58 | 0.58 | — | — |
| Mean distance, ‰ (H) | 0.299 | 0.465 | 0.291 | 0.208 | 0.191 | 0.191 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.46 | 99.09 | 99.45 | 100.02 | 100.02 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | 0.0 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 43.5 | 100.0 | 13.4 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 7 | 6 | 11 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 63.1 | 100.0 | 5.9 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 7 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 56.5 | 0.0 | 86.1 | 0.0 | 0.0 | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 411 | 0 | 116 | 0 | 0 | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.8 | 10.9 | 4.1 | 9.3 | 1.6 | 1.6 | — | — |
| Time, seconds (T) | 1.32 | 1.37 | 0.36 | 1.66 | 1.97 | 3.15 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 6 | 0 | 8 | 1 | 1 | 0 | 0 |
| Base complex patches (C) | 2 | 3 | 0 | 1 | 5 | 4 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 1 | 13 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 2 | 1 | 7 | 3 | 0 | 2 |
| Mean aspect ratio (F) | 2 | 0 | 10 | 2 | 0 | 0 | 0 | 1 |
| Valid meshes (V) | 36/75 | 4/15 | 52/75 | 9/15 | 14/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 3 | 0 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 1 | 3 | 4 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 0 | 3 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 1 | 2 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| Valid meshes (V) | 18/20 | 3/4 | 20/20 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
