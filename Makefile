
run:
	WINIT_UNIX_BACKEND=x11 cargo run

rrun:
	WINIT_UNIX_BACKEND=x11 cargo run --release

rrrun:
	WINIT_UNIX_BACKEND=x11 cargo run --release --features no-slow-safety-checks

bundle: ./assets ./config ./src
	rm pong.zip
	cargo build --release
	mkdir -p pong
	cp target/release/amethyst-pong pong/pong
	strip pong/pong
	cp -r assets pong/assets
	cp -r config pong/config
	zip -r pong.zip pong
	rm -r pong
