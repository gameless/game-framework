all: build package-assets
.PHONY : all

build: 
	cargo build
.PHONY : clean
clean:
	cargo clean
package-assets:
	./package_assets.sh
package:
	./package.sh

.PHONY : run
run:
	cd target; ./debug/framework; cd ..

