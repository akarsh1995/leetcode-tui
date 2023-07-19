.PHONY: debug build-release release-linux-musl 

build-linux-musl-debug:
	cargo build --target=x86_64-unknown-linux-musl

build-linux-musl-release:
	cargo build --release --target=x86_64-unknown-linux-musl

test-linux-musl:
	cargo test --workspace --target=x86_64-unknown-linux-musl

release-linux-arm: build-linux-arm-release
	mkdir -p release

	aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/leetui
	arm-linux-gnueabihf-strip target/armv7-unknown-linux-gnueabihf/release/leetui
	arm-linux-gnueabihf-strip target/arm-unknown-linux-gnueabihf/release/leetui

	tar -C ./target/aarch64-unknown-linux-gnu/release/ -czvf ./release/leetui-linux-aarch64.tar.gz ./leetui
	tar -C ./target/armv7-unknown-linux-gnueabihf/release/ -czvf ./release/leetui-linux-armv7.tar.gz ./leetui
	tar -C ./target/arm-unknown-linux-gnueabihf/release/ -czvf ./release/leetui-linux-arm.tar.gz ./leetui

build-linux-arm-debug:
	cargo build --target=aarch64-unknown-linux-gnu
	cargo build --target=armv7-unknown-linux-gnueabihf
	cargo build --target=arm-unknown-linux-gnueabihf

build-linux-arm-release:
	cargo build --release --target=aarch64-unknown-linux-gnu
	cargo build --release --target=armv7-unknown-linux-gnueabihf
	cargo build --release --target=arm-unknown-linux-gnueabihf

