# Shortcuts for the common tasks. `make help` lists them.

.PHONY: help build test scenes blenderkit essentials remeasure autoremesher scores quick pages dashboard view addon install addon-windows test-windows addon-test release paper paper-version paper-arxiv paper-clean proofs site site-serve

help:
	@echo "make build       build the openclare binary (release)"
	@echo "make test        run the test suite"
	@echo "make addon       build the Blender add-on: dist/openclare-<version>-<system>.zip, to install"
	@echo "                 from Blender (Preferences, Get Extensions, Install from Disk)"
	@echo "make install     build the add-on and put it straight into the extensions of the"
	@echo "                 Blender on the PATH (~/.config/blender/<version>/extensions/user_default);"
	@echo "                 BLENDER_VERSION=5.2 for another, EXTENSIONS=<dir> for another place"
	@echo "make addon-windows  build the same add-on for Windows, from here:"
	@echo "                 dist/openclare-<version>-windows-x64.zip (needs the Rust target"
	@echo "                 x86_64-pc-windows-gnu and MinGW-w64)"
	@echo "make test-windows WINE=/path/to/wine64"
	@echo "                 run the tests of the engine built for Windows, under Wine"
	@echo "make addon-test  run the add-on in a Blender without interface (needs blender)"
	@echo "make autoremesher download and install the AutoRemesher challenger into ~/.local/bin"
	@echo "make release version=MAJOR|MINOR|FIX"
	@echo "                 raise that number of the version, commit and tag it on main; PUSH=1"
	@echo "                 to send the tag, from which GitHub builds the add-on for Linux and"
	@echo "                 for Windows"
	@echo "make scenes      make the scenes of the bench that are made from a file as it is"
	@echo "                 (bench/scenes; needs blender). MODELS=cube,pig for some only,"
	@echo "                 REFRESH=1 to make again those already there"
	@echo "make blenderkit  download the BlenderKit models of the bench that cannot be"
	@echo "                 redistributed, into bench/corpus-large, and make their scenes"
	@echo "                 (needs blender)"
	@echo "make remeasure   measure again the meshes of the results at the three budgets, without"
	@echo "                 remeshing: after a change to the measures"
	@echo "make essentials  the human bodies of Blender's Essentials library, sculpt and mesh made by"
	@echo "                 hand, from Blender's cache (add them once from the Asset Browser)"
	@echo "make scores      write the comparative report, marks from 0 to 100 for every model"
	@echo "                 and remesher (bench/results/SCORES.md)"
	@echo "make scores MODEL=ogre [TARGET=8000]"
	@echo "                 print the marks of one model instead"
	@echo "make quick       one run of the add-on per model next to what the bench stored, in a"
	@echo "                 couple of minutes. MODELS=camera,wolf TARGET=8000 to choose"
	@echo "make pages       write one page per model, with rendered pictures and marks"
	@echo "                 (docs/models; needs blender). MODELS=ogre,blob for some only"
	@echo "make dashboard   the pictures and the chart of the README (docs/images/dashboard; needs"
	@echo "                 blender and AutoRemesher)"
	@echo "make view MODEL=ogre [QUADS=3000] [OPTIONS='--preset hard-surface']"
	@echo "                 remesh a model of the bench, recording what each stage makes, and"
	@echo "                 open the recording in the Rerun viewer (long to build the first"
	@echo "                 time). FILE=path/to/mesh.obj for a mesh of your own"
	@echo "make paper       compile the research notes (docs/research/*/paper.tex) into PDFs"
	@echo "                 next to them (needs latexmk and TeX Live). NOTE=closing-helices for one"
	@echo "                 Each PDF names the commit it describes (HEAD, \"-dirty\" if changed)"
	@echo "make paper-arxiv the bundle arXiv compiles, beside each note: paper.tex, paper.bbl and"
	@echo "                 version.tex (arxiv.tar.gz), so that the PDF arXiv builds names the commit"
	@echo "make paper-clean remove what LaTeX leaves next to them, the PDFs kept"
	@echo "make proofs      check the machine-checked proofs of the research notes"
	@echo "                 (docs/research/*/lean, Lean and Mathlib; the first run downloads"
	@echo "                 the toolchain and Mathlib's compiled cache, about 10 GB in all)."
	@echo "                 NOTE=closing-helices for one"
	@echo "make site        build the website, English and French, into site/_build (plain"
	@echo "                 Python; Pillow, if installed, makes the pictures WebP), then check"
	@echo "                 its links"
	@echo "make site-serve  build it and serve it on http://localhost:8000 (PORT=8001 for another)"

# The most memory, in kilobytes, a run of the engine started from here may
# ask for (`ulimit -v`): beyond it the run fails instead of taking the
# machine down. The bench has the same cap (MEMORY_CAP in bench/run.py).
MEMORY_CAP = 12000000

build:
	cargo build --release

test:
	cargo test

# The scenes the bench runs on, for the models that are a plain mesh: one
# object and the settings of the add-on, from bench/corpus/<model>.obj or, for
# a BlenderKit model, from the file downloaded.
scenes:
	ulimit -v $(MEMORY_CAP); blender -b --factory-startup --python bench/make_scenes.py -- \
		$(if $(MODELS),--models $(MODELS)) $(if $(REFRESH),--refresh)

# Download and install AutoRemesher (huxingyi/autoremesher) into ~/.local/bin
autoremesher:
	@mkdir -p $(HOME)/.local/bin
	curl -sL https://github.com/huxingyi/autoremesher/releases/download/1.2.0/autoremesher-1.2.0.AppImage -o $(HOME)/.local/bin/autoremesher
	chmod +x $(HOME)/.local/bin/autoremesher
	@echo "installed: $(HOME)/.local/bin/autoremesher"

# Their licence does not allow keeping them in the repository: see CREDITS.md.
blenderkit:
	python3 bench/fetch_blenderkit.py

# The human bodies of Blender's Essentials library, from Blender's cache.
essentials:
	python3 bench/fetch_essentials.py

# Measure again the meshes the results were made of, at the three budgets,
# without remeshing (after a change to the measures).
remeasure:
	for target in 3000 1000 8000; do python3 bench/run.py --remeasure --target $$target; done

# The research notes: LaTeX sources in docs/research/<note>/paper.tex, their
# bibliography next to them, the PDF written beside.
NOTES = $(if $(NOTE),docs/research/$(NOTE),$(patsubst %/paper.tex,%,$(wildcard docs/research/*/paper.tex)))
# The commit a note describes, its \enginecommit: the short hash of HEAD, with
# "-dirty" when tracked files other than the PDFs differ from it. Written into
# version.tex beside each note (ignored by git), and only when it changes, so
# that latexmk does not compile again for nothing. For a PDF that names a
# commit holding its sources: commit them, `make paper`, then commit the PDF.
PAPER_COMMIT = $(or $(shell git rev-parse --short=12 HEAD 2>/dev/null),unknown)$(shell \
	git status --porcelain --untracked-files=no -- . ':(exclude,glob)docs/research/*/paper.pdf' \
	2>/dev/null | grep -q . && echo -dirty)
paper-version:
	@for note in $(NOTES); do \
		printf '%s\n' '\newcommand{\enginecommit}{$(PAPER_COMMIT)}' > $$note/version.tex.new; \
		if cmp -s $$note/version.tex.new $$note/version.tex; then rm $$note/version.tex.new; \
		else mv $$note/version.tex.new $$note/version.tex; fi; \
	done

paper: paper-version
	@for note in $(NOTES); do \
		echo "== $$note"; \
		(cd $$note && latexmk -pdf -interaction=nonstopmode -halt-on-error paper.tex >/dev/null) \
			|| { echo "failed: see $$note/paper.log"; exit 1; }; \
		echo "written: $$note/paper.pdf"; \
	done

# arXiv compiles paper.tex itself and runs no BibTeX: it needs the .bbl, and
# version.tex, without which \enginecommit reads "unknown".
paper-arxiv: paper
	@for note in $(NOTES); do \
		tar -czf $$note/arxiv.tar.gz -C $$note paper.tex paper.bbl version.tex \
			&& echo "written: $$note/arxiv.tar.gz, commit $$(sed 's/.*{\(.*\)}$$/\1/' $$note/version.tex)"; \
	done

paper-clean:
	@for note in $(NOTES); do (cd $$note && latexmk -c paper.tex >/dev/null && rm -f version.tex arxiv.tar.gz); done

# The proofs checked by Lean: a Lake project per note in docs/research/<note>/lean,
# on the Mathlib release its lakefile pins. elan installs the toolchain the project
# names; `lake exe cache get` fetches Mathlib compiled, so that only the note's own
# files are built.
LAKE = $(or $(shell command -v lake 2>/dev/null),$(HOME)/.elan/bin/lake)
PROOFS = $(if $(NOTE),docs/research/$(NOTE)/lean,$(patsubst %/lakefile.toml,%,$(wildcard docs/research/*/lean/lakefile.toml)))
proofs:
	@for project in $(PROOFS); do \
		echo "== $$project"; \
		(cd $$project && PATH="$(dir $(LAKE)):$$PATH" && export PATH && \
			$(LAKE) exe cache get && $(LAKE) build) || { echo "failed: $$project"; exit 1; }; \
		echo "checked: $$project"; \
	done

# Marks from 0 to 100, overall and per criterion, from what the bench measured.
scores:
ifdef MODEL
	python3 scripts/scores.py model $(MODEL) $(if $(TARGET),--target $(TARGET))
else
	python3 scripts/scores.py report
endif

# What the engine as it is built gives next to what the bench has stored: one
# run of the add-on per model, several at a time, in a couple of minutes.
quick: addon
	python3 bench/quick.py $(if $(MODELS),--models $(MODELS)) $(if $(TARGET),--target $(TARGET))

# One page per model: pictures of what each remesher makes of it, and its marks.
pages: scores
	python3 scripts/pages.py $(if $(MODELS),--models $(MODELS))

# The dashboard of the README: the remeshers at the same number of quads on
# a few models, in pictures, and the average marks of the bench as a chart.
dashboard:
	python3 scripts/gallery.py $(if $(MODELS),--models $(MODELS))
	python3 scripts/dashboard.py --gallery

# What each stage of the chain makes of a model, in the Rerun viewer. Both the
# recorder and the viewer are built on demand only: nothing else needs Rerun.
# A model of the bench is a scene: what is remeshed here is its object as the
# add-on sees it, exported as an OBJ (bench/run.py --source). Not its curves
# nor its painted density, which only a run in Blender takes.
VIEW_SOURCE = $(or $(FILE),$(if $(MODEL),$(shell python3 bench/run.py --source $(MODEL))))
VIEW_NAME = $(if $(FILE),$(basename $(notdir $(FILE))),$(MODEL))
view:
	@test -n "$(VIEW_SOURCE)" || { echo "which model? make view MODEL=ogre, or FILE=mesh.obj"; exit 2; }
	cargo build --release -p openclare-cli --features rerun
	cargo build --release -p openclare-viewer
	mkdir -p bench/out/view
	ulimit -v $(MEMORY_CAP); target/release/openclare remesh $(VIEW_SOURCE) bench/out/view/$(VIEW_NAME).obj \
		--quads $(or $(QUADS),3000) $(OPTIONS) --record bench/out/view/$(VIEW_NAME).rrd
	target/release/openclare-view bench/out/view/$(VIEW_NAME).rrd

# The Blender add-on: the Python package of blender/openclare with the engine,
# built for this system, in its lib/ folder.
ADDON_VERSION = $(shell sed -n 's/^version = "\(.*\)"/\1/p' blender/openclare/blender_manifest.toml)
ADDON_SYSTEM = $(shell sed -n 's/^platforms = \["\(.*\)"\]/\1/p' blender/openclare/blender_manifest.toml)
addon:
	cargo build --release -p openclare-capi
	mkdir -p blender/openclare/lib dist
	cp target/release/libopenclare_capi.so blender/openclare/lib/
	strip blender/openclare/lib/libopenclare_capi.so
	rm -f dist/openclare-$(ADDON_VERSION)-$(ADDON_SYSTEM).zip
	cd blender/openclare && zip -q -r ../../dist/openclare-$(ADDON_VERSION)-$(ADDON_SYSTEM).zip . -x '__pycache__/*' '*.pyc'
	@echo "written: dist/openclare-$(ADDON_VERSION)-$(ADDON_SYSTEM).zip"

# Where Blender keeps the extensions installed by hand, for the version of the
# blender on the PATH. Only the folder openclare there is replaced.
BLENDER_VERSION ?= $(shell blender --version 2>/dev/null | sed -n '1s/^Blender \([0-9]*\.[0-9]*\).*/\1/p')
EXTENSIONS ?= $(or $(XDG_CONFIG_HOME),$(HOME)/.config)/blender/$(BLENDER_VERSION)/extensions/user_default

install: addon
	@test -n "$(BLENDER_VERSION)" || { echo "no blender on the PATH: say which, BLENDER_VERSION=5.2"; exit 1; }
	mkdir -p "$(EXTENSIONS)"
	rm -rf "$(EXTENSIONS)/openclare"
	mkdir "$(EXTENSIONS)/openclare"
	cd "$(EXTENSIONS)/openclare" && unzip -q "$(CURDIR)/dist/openclare-$(ADDON_VERSION)-$(ADDON_SYSTEM).zip"
	@echo "installed: $(EXTENSIONS)/openclare (restart Blender, or disable and enable the add-on)"

# The same add-on for Windows, built here: the engine for that system (the
# Rust target x86_64-pc-windows-gnu and the MinGW-w64 linker are needed:
# `rustup target add x86_64-pc-windows-gnu`, and the package mingw-w64),
# with the files of the add-on and a manifest that says windows-x64, put
# together in a folder of their own. blender/openclare/lib keeps the engine
# of this system, which the tests load. Nothing here runs the result: it has
# to be tried on Windows.
WINDOWS_TARGET = x86_64-pc-windows-gnu
WINDOWS_STAGE = dist/stage-windows-x64
addon-windows:
	CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \
		cargo build --release -p openclare-capi --target $(WINDOWS_TARGET)
	rm -rf $(WINDOWS_STAGE)
	mkdir -p $(WINDOWS_STAGE)/lib
	cp blender/openclare/__init__.py blender/openclare/engine.py $(WINDOWS_STAGE)/
	sed 's/^platforms = .*/platforms = ["windows-x64"]/' blender/openclare/blender_manifest.toml \
		> $(WINDOWS_STAGE)/blender_manifest.toml
	cp target/$(WINDOWS_TARGET)/release/openclare_capi.dll $(WINDOWS_STAGE)/lib/
	x86_64-w64-mingw32-strip $(WINDOWS_STAGE)/lib/openclare_capi.dll
	rm -f dist/openclare-$(ADDON_VERSION)-windows-x64.zip
	cd $(WINDOWS_STAGE) && zip -q -r ../openclare-$(ADDON_VERSION)-windows-x64.zip .
	@echo "written: dist/openclare-$(ADDON_VERSION)-windows-x64.zip"

# The tests of the engine and of its C interface, built for Windows and run
# under Wine: `make test-windows WINE=/path/to/wine64`. It says nothing of
# the Python of the add-on, which needs a Blender for Windows.
WINE ?= wine64
test-windows:
	CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \
	CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER="$(WINE)" WINEDEBUG=-all \
		cargo test --release --target $(WINDOWS_TARGET) -p openclare -p openclare-capi

addon-test: addon
	ulimit -v $(MEMORY_CAP); blender -b --factory-startup --python blender/test_addon.py

# The next version: MAJOR, MINOR or FIX says which of its three numbers is
# raised. Committed and tagged here; the tag pushed (PUSH=1, or by hand
# afterwards), the add-on is built on GitHub for Linux and for Windows and
# attached to a release (.github/workflows/release.yml).
release:
	@test -n "$(version)" || { echo "which number? make release version=MAJOR, MINOR or FIX"; exit 2; }
	python3 scripts/release.py $(version) $(if $(PUSH),--push)

# The website: site/content and the documents of the repository, made into
# static pages by site/build.py, its links checked. GitHub Pages publishes it
# from main (.github/workflows/pages.yml), once Pages is turned on.
PORT ?= 8000
site:
	python3 site/build.py
	python3 site/check_links.py

site-serve: site
	@echo "serving site/_build on http://localhost:$(PORT) (Ctrl+C to stop)"
	python3 -m http.server $(PORT) --bind 127.0.0.1 --directory site/_build
