
release:
	rm -rf ./docs
	mkdir ./docs
	dx bundle --release --out-dir docs
	mv docs/public/* docs
	cp docs/index.html docs/404.html
	cp assets/* docs/assets/
	
