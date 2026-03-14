# Build, run and test scripts.
CLIENT_TARGET = client/
CLIENT_SOURCE = ./src/client/

release:
	cargo build --release
	$(call build-client,./target/release/)
	- mkdir ./release
	npx copyfiles -u 1 ./target/release/**/*.{html,css,frag,vert,js,js.map} ./release
	cp ./target/release/space_game ./release/

debug:
	cargo build
	$(call build-client,./target/debug/)
	./target/debug/space_game

test:
	echo "No tests aviable yet"

define build-client
	rm -rf $(1)$(CLIENT_TARGET)*
	- mkdir $(1)$(CLIENT_TARGET)
	npx tsc --rootDir $(CLIENT_SOURCE) --outDir $(1)$(CLIENT_TARGET)
	npx copyfiles -u 2 $(CLIENT_SOURCE)**/*.{html,css,frag,vert} $(1)$(CLIENT_TARGET)
endef
