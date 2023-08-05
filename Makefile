
.PHONY: dev
dev:
	cargo run -- \
		override \
		--override-toml sample/sample_override.toml \
		< sample/sample_base.toml

.PHONY: build
build:
	cargo build --release

.PHONY: doc
doc:
	cargo doc

.PHONY: publish
publish:
	cargo publish
