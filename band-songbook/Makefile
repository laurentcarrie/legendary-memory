help: ## Show this help
	@grep -E '^[a-zA-Z0-9_-]+:.*##' $(MAKEFILE_LIST) | awk -F ':.*## ' '{printf "  %-10s %s\n", $$1, $$2}'

clean: ## Remove delivery and sandbox directories
	rm -rf delivery sandbox

run: ## Build songs from local test data
	cargo run --bin band-songbook -- \
		--srcdir tests/data \
		--settings tests/data/settings.yml \
		--sandbox sandbox \
		--delivery delivery \
		--drum-patterns-dir /Users/laurent/work/songbook/drums

open: ## Open strudel HTML for ca_me_vexe
	open sandbox/songs/mademoiselle_K/ca_me_vexe/strudel.html

s3: ## Build rasmus song from S3
	cargo run --bin band-songbook -- \
		--srcdir s3://zik-laurent/songs \
		--settings s3://zik-laurent/songs/settings.yml \
		--sandbox sandbox \
		--delivery delivery \
		--pattern rasmus

run-all: ## Build songs from local
	cargo run --bin band-songbook -- \
		--srcdir ../../songbook/songs \
		--settings ../../songbook/songs/settings.yml \
		--sandbox sandbox \
		--delivery delivery \
		--pattern u2