# Shortcuts for the common tasks. `make help` lists them.

.PHONY: help build test blenderkit scores pages view

help:
	@echo "make build       build the retopo binary (release)"
	@echo "make test        run the test suite"
	@echo "make blenderkit  download the BlenderKit models of the bench that cannot be"
	@echo "                 redistributed, into bench/corpus-large (needs blender)"
	@echo "make scores      write the comparative report, marks from 0 to 100 for every model"
	@echo "                 and remesher (bench/results/SCORES.md)"
	@echo "make scores MODEL=ogre [TARGET=8000]"
	@echo "                 print the marks of one model instead"
	@echo "make pages       write one page per model, with rendered pictures and marks"
	@echo "                 (docs/models; needs blender). MODELS=ogre,blob for some only"
	@echo "make view MODEL=ogre [QUADS=3000] [OPTIONS='--preset hard-surface']"
	@echo "                 remesh a model of the bench, recording what each stage makes, and"
	@echo "                 open the recording in the Rerun viewer (long to build the first"
	@echo "                 time). FILE=path/to/mesh.obj for a mesh of your own"

build:
	cargo build --release

test:
	cargo test

# Their licence does not allow keeping them in the repository: see CREDITS.md.
blenderkit:
	python3 bench/fetch_blenderkit.py

# Marks from 0 to 100, overall and per criterion, from what the bench measured.
scores:
ifdef MODEL
	python3 scripts/scores.py model $(MODEL) $(if $(TARGET),--target $(TARGET))
else
	python3 scripts/scores.py report
endif

# One page per model: pictures of what each remesher makes of it, and its marks.
pages: scores
	python3 scripts/pages.py $(if $(MODELS),--models $(MODELS))

# What each stage of the chain makes of a model, in the Rerun viewer. Both the
# recorder and the viewer are built on demand only: nothing else needs Rerun.
VIEW_SOURCE = $(or $(FILE),$(firstword $(wildcard bench/corpus/$(MODEL).obj bench/corpus-large/$(MODEL).obj)))
VIEW_NAME = $(basename $(notdir $(VIEW_SOURCE)))
view:
	@test -n "$(VIEW_SOURCE)" || { echo "which model? make view MODEL=ogre, or FILE=mesh.obj"; exit 2; }
	cargo build --release -p retopo-cli --features rerun
	cargo build --release -p retopo-viewer
	mkdir -p bench/out/view
	target/release/retopo remesh $(VIEW_SOURCE) bench/out/view/$(VIEW_NAME).obj --quads $(or $(QUADS),3000) \
		$(OPTIONS) --record bench/out/view/$(VIEW_NAME).rrd
	target/release/retopo-view bench/out/view/$(VIEW_NAME).rrd
