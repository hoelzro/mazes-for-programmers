CRATES=$(shell ls */Cargo.toml | xargs -n1 dirname)

build-all:
	@for dir in $(CRATES) ; do \
		echo "(cd \"$$dir\" ; cargo build)" ; \
		(cd "$$dir" ; cargo build) \
	done
