# Acceptance

Chain under test: `quad-layout`. Organic models of the corpus: blob, rock, creature, cage, suzanne, ogre, spot, bob, blub, pig, jerry, bridge.
The thresholds are those of ALGORITHME.md, section 0, against QuadriFlow at the same budget.

## 1000 quads

Medians over the organic models each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 4 | 6 | 8 | 11 | 11 | 8 |
| Irregular vertices (S) | 62 | 88 | 40 | 42 | 72 | 61 |
| Base complex patches (C) | 891 | 886 | 598 | 553 | 1079 | 437 |
| Alignment, degrees (A) | 18.0 | 17.8 | 10.3 | 9.1 | 6.4 | 8.1 |
| Mean angle deviation, degrees (F) | 9.7 | 8.1 | 8.8 | 5.3 | 8.2 | 8.8 |
| Hausdorff, per mille of the diagonal (H) | 38.9 | 41.1 | 27.2 | 18.5 | 7.9 | 18.3 |
| Off budget, % (N) | 5.9 | 3.5 | 9.5 | 0.8 | 71.4 | 11.3 |
| Time, seconds (T) | 1.3 | 1.1 | 3.9 | 1.4 | 13.5 | 12.9 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 4/5 | 5.3 / 18.0 | 24 / 36 | 664 / 941 | 6.4 / 9.3 | 13.8 / 14.5 | 2.6 % | — |
| rock | 1/1 / 5/5 | 17.0 / 23.1 | 26 / 8 | 767 / 941 | 5.6 / 6.2 | 9.6 / 9.4 | 2.2 % | S |
| creature | 1/1 / 4/5 | 8.0 / 14.8 | 26 / 16 | 773 / 812 | 4.9 / 12.6 | 8.0 / 40.5 | 3.5 % | S |
| cage | 1/1 / 5/5 | 4.1 / 7.7 | 48 / 64 | 263 / 716 | 4.6 / 7.0 | 29.7 / 21.4 | 0.1 % | H |
| suzanne | 1/1 / 0/5 | 11.2 / 15.7 | 44 / 67 | 544 / 995 | 4.7 / 9.7 | 19.6 / 44.8 | 0.1 % | — |
| ogre | 1/1 / 3/5 | 10.5 / 20.9 | 46 / 73 | 562 / 891 | 7.0 / 14.2 | 28.6 / 95.0 | 0.2 % | — |
| spot | 1/1 / 5/5 | 9.5 / 18.2 | 64 / 62 | 770 / 965 | 7.2 / 7.6 | 12.5 / 20.4 | 1.0 % | — |
| bob | 1/1 / 5/5 | 4.3 / 20.0 | 18 / 28 | 237 / 958 | 4.5 / 7.9 | 8.1 / 12.6 | 0.7 % | — |
| blub | 1/1 / 3/5 | 8.6 / 16.3 | 40 / 44 | 321 / 785 | 4.7 / 10.0 | 21.6 / 38.9 | 1.7 % | — |
| pig | 0/1 / 0/5 | 11.7 / 17.3 | 60 / 92 | 541 / 848 | 6.0 / 15.5 | 23.6 / 130.1 | 0.9 % | V |
| jerry | 1/1 / 4/5 | 14.8 / 18.5 | 51 / 69 | 605 / 724 | 7.5 / 10.2 | 35.8 / 58.1 | 0.0 % | — |
| bridge | 1/1 / 0/5 | 7.0 / — | 36 / — | 279 / — | 4.0 / — | 17.4 / — | 0.3 % | — |

Thresholds met, out of 12 models: V 11, A 12, S 10, C 12, F 12, H 11, N 12.

## 3000 quads

Medians over the organic models each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 5 | 8 | 9 | 12 | 8 | 10 |
| Irregular vertices (S) | 66 | 156 | 50 | 50 | 80 | 68 |
| Base complex patches (C) | 2697 | 2917 | 951 | 1886 | 2409 | 2009 |
| Alignment, degrees (A) | 17.5 | 16.6 | 8.8 | 8.4 | 6.4 | 7.7 |
| Mean angle deviation, degrees (F) | 8.6 | 6.0 | 8.0 | 5.4 | 6.5 | 7.2 |
| Hausdorff, per mille of the diagonal (H) | 17.7 | 17.5 | 9.3 | 8.1 | 5.9 | 9.6 |
| Off budget, % (N) | 4.1 | 2.1 | 11.1 | 1.4 | 36.6 | 10.3 |
| Time, seconds (T) | 1.9 | 1.2 | 6.5 | 2.2 | 14.5 | 12.3 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 3/5 | 5.4 / 17.5 | 24 / 38 | 2361 / 2697 | 6.2 / 8.7 | 4.4 / 8.5 | 0.7 % | — |
| rock | 1/1 / 5/5 | 16.4 / 23.0 | 50 / 8 | 2968 / 2855 | 5.3 / 5.8 | 5.7 / 5.4 | 1.1 % | SC |
| creature | 1/1 / 5/5 | 6.9 / 12.9 | 40 / 24 | 1301 / 2667 | 5.7 / 6.8 | 2.4 / 8.0 | 1.4 % | S |
| cage | 1/1 / 5/5 | 3.1 / 8.1 | 48 / 66 | 689 / 1943 | 2.9 / 5.9 | 13.7 / 11.5 | 1.3 % | — |
| suzanne | 1/1 / 0/5 | 10.6 / 15.3 | 53 / 78 | 2475 / 2884 | 4.4 / 8.6 | 9.2 / 42.3 | 0.7 % | — |
| ogre | 1/1 / 1/5 | 12.2 / 19.3 | 76 / 106 | 2727 / 2857 | 6.5 / 12.0 | 27.6 / 36.6 | 2.0 % | — |
| spot | 1/1 / 5/5 | 8.7 / 18.1 | 70 / 68 | 1730 / 2902 | 5.6 / 7.2 | 6.3 / 17.7 | 1.6 % | — |
| bob | 1/1 / 5/5 | 4.0 / 19.4 | 20 / 32 | 1290 / 2903 | 3.3 / 6.4 | 2.5 / 6.8 | 1.4 % | — |
| blub | 1/1 / 3/5 | 7.9 / 16.0 | 44 / 52 | 1287 / 2584 | 5.2 / 10.7 | 10.4 / 40.7 | 0.2 % | — |
| pig | 1/1 / 0/5 | 10.6 / 17.4 | 78 / 122 | 2401 / 2612 | 5.9 / 14.3 | 18.1 / 44.6 | 1.6 % | — |
| jerry | 1/1 / 4/5 | 14.3 / 17.9 | 86 / 104 | 2043 / 2211 | 6.3 / 11.3 | 31.6 / 38.8 | 0.0 % | — |
| bridge | 1/1 / 0/5 | 8.0 / — | 50 / — | 1066 / — | 3.8 / — | 6.9 / — | 1.6 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 10, C 11, F 12, H 12, N 12.

## 8000 quads

Medians over the organic models each tool was run on.

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 3 | 9 | 7 | 12 | 11 | 9 |
| Irregular vertices (S) | 70 | 278 | 66 | 67 | 72 | 70 |
| Base complex patches (C) | 7342 | 7812 | 3344 | 4436 | 4605 | 3789 |
| Alignment, degrees (A) | 15.8 | 15.8 | 8.8 | 8.7 | 7.5 | 8.6 |
| Mean angle deviation, degrees (F) | 6.4 | 4.5 | 8.7 | 6.1 | 6.4 | 6.8 |
| Hausdorff, per mille of the diagonal (H) | 6.9 | 6.5 | 5.7 | 4.7 | 4.3 | 5.9 |
| Off budget, % (N) | 3.0 | 1.2 | 10.5 | 0.8 | 12.7 | 11.9 |
| Time, seconds (T) | 4.1 | 1.1 | 30.0 | 6.8 | 15.5 | 13.0 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 3/5 | 5.8 / 14.8 | 24 / 40 | 6808 / 7412 | 6.3 / 7.8 | 2.5 / 5.2 | 0.9 % | — |
| rock | 1/1 / 5/5 | 16.6 / 22.4 | 116 / 10 | 7943 / 7342 | 5.9 / 6.2 | 3.1 / 7.0 | 0.7 % | SC |
| creature | 1/1 / 4/5 | 6.0 / 13.7 | 40 / 22 | 2864 / 8170 | 6.7 / 6.0 | 1.5 / 3.2 | 1.5 % | SF |
| cage | 1/1 / 5/5 | 2.5 / 8.2 | 48 / 72 | 1654 / 7283 | 2.1 / 5.8 | 4.6 / 6.6 | 1.4 % | — |
| suzanne | 1/1 / 5/5 | 10.0 / 14.3 | 64 / 55 | 3543 / 6194 | 5.5 / 6.1 | 5.0 / 6.9 | 0.4 % | — |
| ogre | 1/1 / 0/5 | 11.3 / 17.8 | 202 / 162 | 6252 / 7352 | 6.3 / 11.6 | 13.0 / 27.3 | 0.4 % | — |
| spot | 1/1 / 4/5 | 7.9 / 16.8 | 80 / 76 | 5329 / 7772 | 6.5 / 6.2 | 3.0 / 5.3 | 0.4 % | F |
| bob | 1/1 / 3/5 | 3.6 / 17.3 | 20 / 36 | 1279 / 8263 | 3.6 / 6.4 | 0.9 / 3.2 | 0.4 % | — |
| blub | 1/1 / 0/5 | 7.0 / 15.2 | 48 / 70 | 3338 / 6275 | 6.6 / 10.3 | 4.7 / 21.9 | 0.1 % | — |
| pig | 1/1 / 0/5 | 9.4 / 15.8 | 88 / 136 | 5985 / 6841 | 7.0 / 11.4 | 8.3 / 56.2 | 1.2 % | — |
| jerry | 1/1 / 4/5 | 12.9 / 17.6 | 166 / 112 | 5332 / 4991 | 5.3 / 7.8 | 15.0 / 31.3 | 1.6 % | C |
| bridge | 1/1 / 0/5 | 9.9 / — | 70 / — | 2987 / — | 4.8 / — | 6.7 / — | 1.5 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 10, C 10, F 10, H 12, N 12.
