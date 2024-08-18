init: ## Run initial setting
	cargo install cargo-watch
	cargo install --path .

run: ## Run salvo server
	RUST_LOG=DEBUG cargo watch -x run

ping: ## Run ping check
	curl http://localhost:5800

test: ## Run test
	cargo test -- --nocapture

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-10s\033[0m %s\n", $$1, $$2}'

.PHONY: help init run ping test
.DEFAULT_GOAL := help
