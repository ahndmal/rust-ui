prepare:
	sudo apt update && sudo apt install \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libglib2.0-dev \
    libssl-dev \
    libgtk-3-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libappindicator-dev \
    libgdk3.0-cil libatk1.0-dev
	rustup update \
    rustup update nightly \
#	rustup target add wasm32-unknown-unknown --toolchain nightly
run:
	cargo run --color=always --package rust-features --bin rust-features
run_nightly:
	cargo +nightly run
release:
	cargo +nightly run --release