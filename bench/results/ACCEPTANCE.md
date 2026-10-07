# Acceptance

Chain under test: `quad-layout-adaptive`.
The thresholds are those of ALGORITHME.md, section 0, against QuadriFlow at the same budget.
They were set for the organic models; the mechanical ones are held to the same for reference.

## 1000 quads, organic models

Models: blob, rock, creature, cage, suzanne, ogre, spot, bob, blub, pig, jerry, bridge. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 4 | 6 | 8 | 11 | 12 | 11 | 8 |
| Irregular vertices (S) | 62 | 88 | 40 | 42 | 42 | 72 | 61 |
| Base complex patches (C) | 891 | 886 | 598 | 584 | 530 | 1079 | 437 |
| Alignment, degrees (A) | 18.0 | 17.8 | 10.3 | 9.1 | 8.4 | 6.4 | 8.1 |
| Mean angle deviation, degrees (F) | 9.7 | 8.1 | 8.8 | 5.3 | 5.3 | 8.2 | 8.8 |
| Hausdorff, per mille of the diagonal (H) | 38.9 | 41.1 | 27.2 | 17.6 | 14.1 | 7.9 | 18.3 |
| Off budget, % (N) | 5.9 | 3.5 | 9.5 | 0.9 | 0.5 | 71.4 | 11.3 |
| Area variation | 0.27 | 0.25 | 0.31 | 0.35 | 0.48 | 0.55 | 0.33 |
| Mean distance, per mille of the diagonal | 1.55 | 1.95 | 1.70 | 1.44 | 1.44 | 0.90 | 1.65 |
| Time, seconds (T) | 1.3 | 1.1 | 3.9 | 2.0 | 1.9 | 13.5 | 12.9 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 4/5 | 4.7 / 18.0 | 24 / 36 | 802 / 941 | 5.7 / 9.3 | 12.3 / 14.5 | 0.2 % | — |
| rock | 1/1 / 5/5 | 16.8 / 23.1 | 26 / 8 | 981 / 941 | 4.8 / 6.2 | 9.1 / 9.4 | 1.9 % | SC |
| creature | 1/1 / 4/5 | 5.7 / 14.8 | 40 / 16 | 355 / 812 | 5.5 / 12.6 | 5.0 / 40.5 | 0.1 % | S |
| cage | 1/1 / 5/5 | 3.2 / 7.7 | 48 / 64 | 208 / 716 | 5.0 / 7.0 | 15.1 / 21.4 | 0.1 % | — |
| suzanne | 1/1 / 0/5 | 10.3 / 15.7 | 48 / 67 | 648 / 995 | 5.2 / 9.7 | 13.1 / 44.8 | 0.6 % | — |
| ogre | 1/1 / 3/5 | 10.0 / 20.9 | 44 / 73 | 436 / 891 | 7.5 / 14.2 | 31.5 / 95.0 | 0.4 % | — |
| spot | 1/1 / 5/5 | 8.2 / 18.2 | 66 / 62 | 681 / 965 | 7.5 / 7.6 | 10.2 / 20.4 | 0.3 % | — |
| bob | 1/1 / 5/5 | 3.6 / 20.0 | 20 / 28 | 185 / 958 | 4.5 / 7.9 | 5.5 / 12.6 | 2.8 % | — |
| blub | 1/1 / 3/5 | 8.6 / 16.3 | 40 / 44 | 406 / 785 | 5.0 / 10.0 | 19.4 / 38.9 | 0.2 % | — |
| pig | 1/1 / 0/5 | 10.9 / 17.3 | 56 / 92 | 625 / 848 | 5.9 / 15.5 | 33.3 / 130.1 | 2.5 % | — |
| jerry | 1/1 / 4/5 | 15.4 / 18.5 | 50 / 69 | 714 / 724 | 7.5 / 10.2 | 33.9 / 58.1 | 1.1 % | — |
| bridge | 1/1 / 0/5 | 6.6 / — | 36 / — | 303 / — | 4.7 / — | 17.5 / — | 1.8 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 10, C 11, F 12, H 12, N 12.

## 1000 quads, mechanical models

Models: part, cube, lblock, cylinder. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Models valid on every run (V) | 4 | 3 | 4 | 4 | 4 | 4 | 4 |
| Irregular vertices (S) | 10 | 28 | 10 | 10 | 10 | 10 | 10 |
| Base complex patches (C) | 156 | 375 | 283 | 10 | 41 | 76 | 76 |
| Alignment, degrees (A) | 2.7 | 3.1 | 1.9 | 0.9 | 1.2 | 1.1 | 1.1 |
| Mean angle deviation, degrees (F) | 2.5 | 3.6 | 3.5 | 1.1 | 1.3 | 2.2 | 2.2 |
| Hausdorff, per mille of the diagonal (H) | 6.0 | 7.7 | 18.4 | 0.7 | 0.8 | 0.8 | 1.1 |
| Off budget, % (N) | 3.8 | 6.7 | 2.5 | 1.2 | 1.6 | 1.4 | 4.5 |
| Area variation | 0.09 | 0.14 | 0.12 | 0.12 | 0.15 | 0.17 | 0.16 |
| Mean distance, per mille of the diagonal | 0.23 | 0.12 | 0.47 | 0.12 | 0.08 | 0.07 | 0.09 |
| Time, seconds (T) | 0.4 | 0.1 | 2.7 | 0.7 | 0.7 | 2.9 | 2.8 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| part | 1/1 / 5/5 | 10.8 / 13.4 | 24 / 30 | 68 / 227 | 2.2 / 3.8 | 1.7 / 8.2 | 1.8 % | — |
| cube | 1/1 / 5/5 | 0.3 / 0.2 | 8 / 8 | 6 / 6 | 0.3 / 0.1 | 0.0 / 2.3 | 1.4 % | AF |
| lblock | 1/1 / 5/5 | 0.7 / 1.4 | 12 / 12 | 14 / 85 | 0.4 / 1.3 | 0.0 / 9.7 | 0.6 % | A |
| cylinder | 1/1 / 5/5 | 1.8 / 3.9 | 8 / 8 | 186 / 376 | 3.7 / 4.5 | 1.7 / 3.9 | 3.2 % | — |

Thresholds met, out of 4 models: V 4, A 2, S 4, C 4, F 3, H 4, N 4.

## 3000 quads, organic models

Models: blob, rock, creature, cage, suzanne, ogre, spot, bob, blub, pig, jerry, bridge. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 5 | 8 | 9 | 12 | 12 | 8 | 10 |
| Irregular vertices (S) | 66 | 156 | 50 | 50 | 54 | 80 | 68 |
| Base complex patches (C) | 2697 | 2917 | 951 | 1886 | 1769 | 2409 | 2009 |
| Alignment, degrees (A) | 17.5 | 16.6 | 8.8 | 8.4 | 7.6 | 6.4 | 7.7 |
| Mean angle deviation, degrees (F) | 8.6 | 6.0 | 8.0 | 5.4 | 5.0 | 6.5 | 7.2 |
| Hausdorff, per mille of the diagonal (H) | 17.7 | 17.5 | 9.3 | 7.6 | 7.2 | 5.9 | 9.6 |
| Off budget, % (N) | 4.1 | 2.1 | 11.1 | 1.4 | 1.1 | 36.6 | 10.3 |
| Area variation | 0.24 | 0.18 | 0.27 | 0.31 | 0.43 | 0.43 | 0.28 |
| Mean distance, per mille of the diagonal | 0.73 | 0.73 | 0.77 | 0.60 | 0.58 | 0.41 | 0.70 |
| Time, seconds (T) | 1.9 | 1.2 | 6.5 | 3.0 | 2.3 | 14.5 | 12.3 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 3/5 | 4.9 / 17.5 | 24 / 38 | 2045 / 2697 | 5.7 / 8.7 | 3.5 / 8.5 | 0.2 % | — |
| rock | 1/1 / 5/5 | 15.7 / 23.0 | 66 / 8 | 2984 / 2855 | 5.0 / 5.8 | 5.6 / 5.4 | 0.5 % | SC |
| creature | 1/1 / 5/5 | 5.7 / 12.9 | 40 / 24 | 2839 / 2667 | 5.1 / 6.8 | 2.4 / 8.0 | 1.5 % | SC |
| cage | 1/1 / 5/5 | 2.3 / 8.1 | 48 / 66 | 304 / 1943 | 3.2 / 5.9 | 14.2 / 11.5 | 1.1 % | H |
| suzanne | 1/1 / 0/5 | 9.8 / 15.3 | 56 / 78 | 1872 / 2884 | 4.5 / 8.6 | 7.0 / 42.3 | 1.1 % | — |
| ogre | 1/1 / 1/5 | 12.3 / 19.3 | 88 / 106 | 2109 / 2857 | 5.9 / 12.0 | 25.3 / 36.6 | 0.5 % | — |
| spot | 1/1 / 5/5 | 7.9 / 18.1 | 70 / 68 | 1663 / 2902 | 5.6 / 7.2 | 7.5 / 17.7 | 0.7 % | — |
| bob | 1/1 / 5/5 | 3.6 / 19.4 | 20 / 32 | 237 / 2903 | 3.2 / 6.4 | 2.1 / 6.8 | 1.7 % | — |
| blub | 1/1 / 3/5 | 7.1 / 16.0 | 52 / 52 | 1036 / 2584 | 4.8 / 10.7 | 8.7 / 40.7 | 0.3 % | — |
| pig | 1/1 / 0/5 | 9.0 / 17.4 | 80 / 122 | 1666 / 2612 | 5.4 / 14.3 | 22.2 / 44.6 | 1.8 % | — |
| jerry | 1/1 / 4/5 | 13.8 / 17.9 | 102 / 104 | 2384 / 2211 | 6.0 / 11.3 | 22.3 / 38.8 | 1.1 % | C |
| bridge | 1/1 / 0/5 | 7.4 / — | 46 / — | 764 / — | 3.9 / — | 6.7 / — | 1.4 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 10, C 9, F 12, H 11, N 12.

## 3000 quads, mechanical models

Models: part, cube, lblock, cylinder. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Models valid on every run (V) | 2 | 4 | 4 | 4 | 4 | 4 | 4 |
| Irregular vertices (S) | 10 | 58 | 10 | 10 | 10 | 10 | 10 |
| Base complex patches (C) | 352 | 1123 | 372 | 46 | 148 | 211 | 211 |
| Alignment, degrees (A) | 2.6 | 3.7 | 1.2 | 1.0 | 1.2 | 1.1 | 1.1 |
| Mean angle deviation, degrees (F) | 1.7 | 2.9 | 1.7 | 0.7 | 1.4 | 1.7 | 1.7 |
| Hausdorff, per mille of the diagonal (H) | 5.2 | 1.9 | 1.0 | 0.5 | 0.3 | 0.4 | 0.4 |
| Off budget, % (N) | 3.3 | 2.7 | 1.9 | 0.8 | 1.1 | 4.5 | 4.5 |
| Area variation | 0.07 | 0.12 | 0.09 | 0.08 | 0.17 | 0.11 | 0.11 |
| Mean distance, per mille of the diagonal | 0.07 | 0.03 | 0.05 | 0.03 | 0.03 | 0.03 | 0.03 |
| Time, seconds (T) | 1.2 | 0.3 | 5.3 | 1.4 | 2.0 | 2.8 | 2.9 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| part | 1/1 / 4/5 | 10.5 / 13.0 | 24 / 28 | 281 / 619 | 2.3 / 2.7 | 0.6 / 5.5 | 1.9 % | — |
| cube | 1/1 / 5/5 | 0.4 / 0.3 | 8 / 8 | 6 / 6 | 0.4 / 0.2 | 0.0 / 4.8 | 0.3 % | AF |
| lblock | 1/1 / 4/5 | 0.9 / 1.1 | 12 / 12 | 14 / 86 | 0.5 / 0.8 | 0.0 / 10.3 | 0.9 % | A |
| cylinder | 1/1 / 5/5 | 1.6 / 4.2 | 8 / 8 | 463 / 1003 | 2.7 / 4.3 | 0.9 / 2.1 | 1.3 % | — |

Thresholds met, out of 4 models: V 4, A 2, S 4, C 4, F 3, H 4, N 4.

## 8000 quads, organic models

Models: blob, rock, creature, cage, suzanne, ogre, spot, bob, blub, pig, jerry, bridge. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 3 | 9 | 7 | 12 | 12 | 11 | 9 |
| Irregular vertices (S) | 70 | 278 | 66 | 67 | 78 | 72 | 70 |
| Base complex patches (C) | 7342 | 7812 | 3344 | 4334 | 4736 | 4605 | 3789 |
| Alignment, degrees (A) | 15.8 | 15.8 | 8.8 | 8.7 | 7.8 | 7.5 | 8.6 |
| Mean angle deviation, degrees (F) | 6.4 | 4.5 | 8.7 | 6.1 | 5.6 | 6.4 | 6.8 |
| Hausdorff, per mille of the diagonal (H) | 6.9 | 6.5 | 5.7 | 4.7 | 3.5 | 4.3 | 5.9 |
| Off budget, % (N) | 3.0 | 1.2 | 10.5 | 1.0 | 0.8 | 12.7 | 11.9 |
| Area variation | 0.21 | 0.14 | 0.28 | 0.32 | 0.42 | 0.37 | 0.28 |
| Mean distance, per mille of the diagonal | 0.27 | 0.26 | 0.28 | 0.25 | 0.22 | 0.24 | 0.31 |
| Time, seconds (T) | 4.1 | 1.1 | 30.0 | 6.0 | 6.4 | 15.5 | 13.0 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 3/5 | 5.5 / 14.8 | 24 / 40 | 6819 / 7412 | 5.8 / 7.8 | 2.2 / 5.2 | 0.3 % | — |
| rock | 1/1 / 5/5 | 16.4 / 22.4 | 130 / 10 | 7993 / 7342 | 5.4 / 6.2 | 3.0 / 7.0 | 0.1 % | SC |
| creature | 1/1 / 4/5 | 5.6 / 13.7 | 42 / 22 | 3225 / 8170 | 5.6 / 6.0 | 1.5 / 3.2 | 2.2 % | S |
| cage | 1/1 / 5/5 | 2.2 / 8.2 | 48 / 72 | 556 / 7283 | 2.5 / 5.8 | 3.7 / 6.6 | 0.4 % | — |
| suzanne | 1/1 / 5/5 | 10.0 / 14.3 | 78 / 55 | 4489 / 6194 | 5.9 / 6.1 | 4.2 / 6.9 | 1.3 % | — |
| ogre | 1/1 / 0/5 | 10.8 / 17.8 | 228 / 162 | 6062 / 7352 | 6.2 / 11.6 | 7.9 / 27.3 | 0.7 % | — |
| spot | 1/1 / 4/5 | 7.5 / 16.8 | 78 / 76 | 5007 / 7772 | 6.5 / 6.2 | 2.4 / 5.3 | 1.2 % | F |
| bob | 1/1 / 3/5 | 3.8 / 17.3 | 20 / 36 | 2898 / 8263 | 3.8 / 6.4 | 1.3 / 3.2 | 0.2 % | — |
| blub | 1/1 / 0/5 | 6.4 / 15.2 | 56 / 70 | 2980 / 6275 | 6.0 / 10.3 | 3.3 / 21.9 | 0.6 % | — |
| pig | 1/1 / 0/5 | 8.1 / 15.8 | 94 / 136 | 4984 / 6841 | 5.6 / 11.4 | 7.0 / 56.2 | 1.8 % | — |
| jerry | 1/1 / 4/5 | 12.3 / 17.6 | 182 / 112 | 7034 / 4991 | 5.7 / 7.8 | 11.5 / 31.3 | 1.0 % | SC |
| bridge | 1/1 / 0/5 | 8.9 / — | 86 / — | 2286 / — | 4.3 / — | 6.5 / — | 1.8 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 9, C 10, F 11, H 12, N 12.

## 8000 quads, mechanical models

Models: part, cube, lblock, cylinder. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 2 | 4 | 4 | 4 | 4 | 4 | 4 |
| Models valid on every run (V) | 1 | 3 | 4 | 4 | 4 | 4 | 4 |
| Irregular vertices (S) | 23 | 94 | 10 | 10 | 10 | 10 | 10 |
| Base complex patches (C) | 588 | 2692 | 737 | 46 | 53 | 372 | 372 |
| Alignment, degrees (A) | 6.9 | 2.3 | 1.0 | 1.0 | 1.1 | 1.1 | 1.1 |
| Mean angle deviation, degrees (F) | 1.5 | 2.0 | 1.3 | 0.5 | 0.8 | 1.8 | 1.8 |
| Hausdorff, per mille of the diagonal (H) | 3.0 | 1.0 | 0.5 | 0.2 | 0.2 | 0.3 | 0.3 |
| Off budget, % (N) | 0.6 | 1.5 | 1.5 | 0.8 | 0.8 | 4.5 | 4.5 |
| Area variation | 0.05 | 0.08 | 0.08 | 0.07 | 0.15 | 0.13 | 0.13 |
| Mean distance, per mille of the diagonal | 0.02 | 0.01 | 0.03 | 0.01 | 0.01 | 0.02 | 0.02 |
| Time, seconds (T) | 5.3 | 1.1 | 12.3 | 4.4 | 4.5 | 3.3 | 3.0 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| part | 1/1 / 3/5 | 10.4 / 13.3 | 24 / 38 | 92 / 1171 | 1.0 / 2.6 | 0.4 / 5.1 | 0.7 % | — |
| cube | 1/1 / 5/5 | 0.3 / 0.5 | 8 / 8 | 6 / 6 | 0.2 / 0.4 | 0.0 / 0.8 | 0.8 % | A |

Thresholds met, out of 2 models: V 2, A 1, S 2, C 2, F 2, H 2, N 2.
