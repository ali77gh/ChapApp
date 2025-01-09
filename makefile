

release:
	dx build --release
	rm -rf ./docs
	mkdir ./docs
	cp -r ./target/dx/chap-app/release/web/public/* ./docs