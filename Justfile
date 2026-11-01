# user_name        := env("USER")
# current_location := justfile()
current_dir      := justfile_directory()
module_name      := file_name(current_dir)
target_dir       := `cargo metadata --no-deps --format-version=1 | jq -r '.target_directory'`

default: build

clear:
    tmux clear-history || true
    clear

pull: clear
    python3 build.py github_pull

build: clear
    python3 build.py build-debug

build-release: clear
    python3 build.py build-release

gbuild: pull build

rbuild: pull build-release

clean:
    @cargo clean -p {{module_name}}

clean-all:
    @rm target -rf
    @cargo clean

targetlist:
    rustup target list

rpi:
    cargo build --target aarch64-unknown-linux-gnu


cover-setup:
    cargo install cargo-llvm-cov
    rustup component add llvm-tools-preview

cover:
    cargo llvm-cov --all-features --workspace --html

cover-lcov:
    cargo llvm-cov --all-features --workspace --lcov --output-path {{target_dir}}/coverage/lcov.info

cover-text:
    cargo llvm-cov --all-features --workspace

