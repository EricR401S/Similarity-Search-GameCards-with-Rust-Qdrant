install:
	# assumes you installed rust
	cd ygenvmp7 &&\
		cargo clean &&\
			cargo build &&\
				cargo run

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

local-run: dockersetup install

dockersetup:
	cd ygenvmp7 &&\
		cargo add qdrant-client anyhow tonic tokio serde-json --features tokio/rt-multi-thread &&\
			docker run -p 6333:6333 -p 6334:6334 -e QDRANT__SERVICE__GRPC_PORT="6334" qdrant/qdrant

all: format lint test run