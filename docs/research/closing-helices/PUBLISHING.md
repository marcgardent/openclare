# Publishing the note

How "Closing Edge Loops at Quantization Time" goes from the repository to a
citable preprint: first a DOI from Zenodo for a release of the repository,
then, if wanted, HAL and arXiv. The steps below that need an account or a
decision are for the author to take; the rest is already in the repository.

## Already prepared

- **Author block** (`paper.tex`): Marc Gardent, independent researcher, contact
  through the repository; no e-mail address in the PDF.
- **Licence of the text**: CC BY 4.0, stated in a footnote of the first page
  and in [`LICENSE.md`](LICENSE.md). The Lean proofs, like the engine, are under
  the GPL-3.0-or-later of the repository.
- **The commit described**: `make paper` writes `version.tex` (ignored by git)
  with `\enginecommit`, the short hash of `HEAD`, followed by `-dirty` when
  tracked files other than the PDFs differ from it. The first page and the
  paragraph "Reproducibility" name it.
- **Proofs and PDF on GitHub**: [`.github/workflows/proofs.yml`](../../../.github/workflows/proofs.yml)
  checks the Lean proofs (no `sorry`, `admit` or `axiom`, then `lake build` on
  the pinned Mathlib) and builds `paper.pdf`, kept with the run as an
  artifact, on every pull request and push to `main` that touches the note.
- **Citation metadata**: [`CITATION.cff`](../../../CITATION.cff) (GitHub's
  "Cite this repository"; the note is listed under `references`) and
  [`.zenodo.json`](../../../.zenodo.json), which Zenodo reads in place of
  `CITATION.cff` when both are present. Keep the two in step.

## Before the first preprint

1. **Confirm** the licence (CC BY 4.0) and the author block. Choosing another
   licence means changing the footnote of `paper.tex`, `LICENSE.md`, and
   `license` in the reference of `CITATION.cff`.
2. **ORCID**: done, 0009-0000-6466-3098 (<https://orcid.org/0009-0000-6466-3098>), in `CITATION.cff`,
   `.zenodo.json` and under the name in `paper.tex`. Zenodo, HAL and arXiv all
   link it. On the ORCID record, the published name is "Marc Gardent".
3. **Title block**: it reads "preprint, version 1" since the three reviews
   were applied; set the date at release. Commit the sources, then run `make paper` on a
   clean working tree and commit the PDF on its own: the PDF then names the
   commit that holds its sources, without `-dirty`.

## A DOI from Zenodo

1. Sign in to <https://zenodo.org> with the GitHub account, open
   **Account › GitHub**, and switch on `marcgardent/openclare` (it must be
   public, which it is). Zenodo then watches the repository for releases.
2. Make a GitHub release as usual (`make release PUSH=1` on `main`: the
   tag starts `.github/workflows/release.yml`, which publishes the release with
   the add-on). Zenodo archives the source of the tag, PDF included, and gives
   it a DOI. Check the record it made: title, creator, licence and keywords
   come from `.zenodo.json`.
3. Zenodo gives two DOIs: one for this version, and a "concept" DOI that always
   resolves to the latest. Add the concept DOI to `CITATION.cff`
   (`doi: 10.5281/zenodo.N`, or under `identifiers`) and, if wanted, as a badge
   in the README and a line in the paragraph "Reproducibility" of the note.

The Zenodo record archives the software with the note in it; it is not a
separate record for the note. A record of its own for the PDF (upload type
"publication", subtype "preprint", licence CC BY 4.0) can be made by hand on
Zenodo, linked to the software record by "is supplemented by".

## HAL, then arXiv

- **HAL** (<https://hal.science>): deposit the PDF with its metadata, domain
  "Computer Science › Computational Geometry" and "Graphics", licence CC BY 4.0.
  An account is enough, no endorsement. HAL can pass a deposit on to arXiv,
  which still asks for the endorsement below.
- **arXiv** (<https://arxiv.org>): primary category `cs.CG` (Computational
  Geometry), cross-listed to `cs.GR` (Graphics); licence CC BY 4.0. A first
  submission to a `cs` category needs an **endorsement**: an author already
  established in that category endorses the new submitter through the code
  arXiv sends. arXiv compiles the LaTeX itself: upload the bundle that
  `make paper-arxiv NOTE=closing-helices` writes, `arxiv.tar.gz` beside the
  note. It holds `paper.tex`, the `paper.bbl` (arXiv does not run BibTeX on
  `refs.bib`) and `version.tex`, without which `\enginecommit` reads
  "unknown". Make it on a clean tree at the commit the PDF names. The note
  states the use of AI in its section "AI assistance", as arXiv's policy asks;
  no AI system is listed as an author.
- Each new version (v2, ...) on arXiv or HAL: a new commit, `make paper`, a new
  release for its DOI, and the new commit named in the PDF.
