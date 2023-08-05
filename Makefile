
.PHONY: dev
dev:
	cargo run -- \
		override \
		--override-toml sample_override.toml \
		< sample_base.toml

# 'plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc.options'
# cargo run -- override < sample.toml

.PHONY: build
build:
	cargo build --release

.PHONY: doc
doc:
	cargo doc

.PHONY: publish
publish:
	cargo publish
