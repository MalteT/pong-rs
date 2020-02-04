
run:
	WINIT_UNIX_BACKEND=x11 cargo run

rrun:
	WINIT_UNIX_BACKEND=x11 cargo run --release

rrrun:
	WINIT_UNIX_BACKEND=x11 cargo run --release --features no-slow-safety-checks

bundle: ./assets ./config ./src
	@if [ -f pong.tar.gz ]; then\
		rm pong.tar.gz;\
	fi
	cargo build --release --features no-slow-safety-checks
	mkdir -p pong
	cp target/release/amethyst-pong pong/pong
	strip pong/pong
	chmod +x pong/pong
	cp -r assets pong/assets
	cp -r config pong/config
	tar czf pong.tar.gz pong -p
	rm -r pong
