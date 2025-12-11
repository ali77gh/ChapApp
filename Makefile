
release:
	rm -rf ./docs
	mkdir ./docs
	dx bundle --out-dir docs
	mv docs/public/* docs
	cp docs/index.html docs/404.html
	cp assets/* docs/assets/
# 	wasm-opt ./docs/wasm/chap-app_bg.wasm -o ./docs/wasm/chap-app_bg.wasm -Oz
