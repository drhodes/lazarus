EXE=target/debug/deps/lazarus-7159901fdd293aad

profile-perf:
	perf record -g $(EXE)
	perf script | stackcollapse-perf.pl | rust-unmangle | flamegraph.pl > flame.svg

profile-valgrind:
	valgrind --tool=massif $(EXE)

heaptrack: 
	heaptrack $(EXE)

debug: ## debug with gdb
	rust-gdb $(EXE)

build: ## build
	cargo build		

test: ## test
	cargo test

#test-watch: export RUST_BACKTRACE = 1
test-watch: ## test on file change
	cargo watch -x test


run: ## run with backtrace
	RUST_BACKTRACE=1 cargo run

clean: ## clean all the things
	bash clean.bash

work: ## open all files in editor
	emacs -nw src/*.rs Makefile

# http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk \
	'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

FORCE:

