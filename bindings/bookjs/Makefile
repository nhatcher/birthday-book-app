all:
	mkdir -p build
	mkdir -p build/pkg
	wasm-pack build --target web --release
	cp -r pkg/* build/pkg/
	cp ts/index.html build/
	tsc

serve:
	cd build
	python3 -m http.server -b 127.0.0.1 8000

clean:
	rm -f -r build/
	rm -f -r pkg/
	rm -f -r target/
