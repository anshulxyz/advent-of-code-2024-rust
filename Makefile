.PHONY: all
all: test run

.PHONY: scaffold
scaffold:
	@echo "Creating scaffold for $(ARG)..."
	@mkdir -p src/bin
	@touch src/bin/$(ARG).rs data/examples/$(ARG).txt data/inputs/$(ARG).txt
	@echo "Files created:"
	@echo "- src/bin/$(ARG).rs:"
	@echo "- data/examples/$(ARG).txt"
	@echo "- data/inputs/$(ARG).txt"

.PHONY: run
run:
	cargo run --bin $(ARG)

.PHONY: test
test:
	cargo test --bin $(ARG)
