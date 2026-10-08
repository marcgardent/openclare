# Shortcuts for the common tasks. `make help` lists them.

.PHONY: help build test blenderkit

help:
	@echo "make build       build the retopo binary (release)"
	@echo "make test        run the test suite"
	@echo "make blenderkit  download the BlenderKit models of the bench that cannot be"
	@echo "                 redistributed, into bench/corpus-large (needs blender)"

build:
	cargo build --release

test:
	cargo test

# Their licence does not allow keeping them in the repository: see CREDITS.md.
blenderkit:
	python3 bench/fetch_blenderkit.py
