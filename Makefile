# Shortcuts for the common tasks. `make help` lists them.

.PHONY: help build test blenderkit scores

help:
	@echo "make build       build the retopo binary (release)"
	@echo "make test        run the test suite"
	@echo "make blenderkit  download the BlenderKit models of the bench that cannot be"
	@echo "                 redistributed, into bench/corpus-large (needs blender)"
	@echo "make scores      write the comparative report, marks from 0 to 100 for every model"
	@echo "                 and remesher (bench/results/SCORES.md)"
	@echo "make scores MODEL=ogre [TARGET=8000]"
	@echo "                 print the marks of one model instead"

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
