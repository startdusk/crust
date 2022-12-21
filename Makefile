.PHONY: check
check:
	@cargo check

.PHONY: expand
expand:
	@cargo expand -p declarative_macros --lib --tests

.PHONY: test
test:
	@cargo test --lib

.PHONY: bench
bench:
	@cargo bench
