# Shortcuts for the common tasks. `make help` lists them.

.PHONY: help build test scenes blenderkit scores pages view addon addon-test

help:
	@echo "make build       build the openclare binary (release)"
	@echo "make test        run the test suite"
	@echo "make addon       build the Blender add-on: dist/openclare-<version>-<system>.zip, to install"
	@echo "                 from Blender (Preferences, Get Extensions, Install from Disk)"
	@echo "make addon-test  run the add-on in a Blender without interface (needs blender)"
	@echo "make scenes      make the scenes of the bench that are made from a file as it is"
	@echo "                 (bench/scenes; needs blender). MODELS=cube,pig for some only,"
	@echo "                 REFRESH=1 to make again those already there"
	@echo "make blenderkit  download the BlenderKit models of the bench that cannot be"
	@echo "                 redistributed, into bench/corpus-large, and make their scenes"
	@echo "                 (needs blender)"
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

addon-test: addon
	ulimit -v $(MEMORY_CAP); blender -b --factory-startup --python blender/test_addon.py
