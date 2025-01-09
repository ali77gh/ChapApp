

release:
	dx build --release
	rm -rf ./docs
	mkdir ./docs
	cp -r ./target/dx/chap-app/release/web/public/* ./docs
	wasm-opt ./docs/wasm/chap-app_bg.wasm -o ./docs/wasm/chap-app_bg.wasm -Oz