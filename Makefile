# Unified build orchestrator for aider-rs components
.PHONY: build build-rust build-go build-flutter release clean

build: build-rust build-go build-flutter

build-rust:
	cargo build --manifest-path aider-cli/Cargo.toml

build-go:
	mkdir -p build/go-shell
	go build -o build/go-shell/go-shell ./go-shell

build-flutter:
	cd flutter_app && flutter build linux --release
	cd flutter_app && flutter build macos --release
	cd flutter_app && flutter build windows --release

release: build
	scripts/release.sh

clean:
	cargo clean --manifest-path aider-cli/Cargo.toml
	go clean ./go-shell
	cd flutter_app && flutter clean
	rm -rf build dist
