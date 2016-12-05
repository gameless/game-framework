all: build package-assets
.PHONY : all

build: 
	cargo build
.PHONY : clean
clean:
	cargo clean
package-assets:
ifeq ($(OS), Windows_NT)
	powershell .\package_assets.ps1
else
	./package_assets.sh
endif
package:
ifeq ($(OS),Windows_NT)
	powershell .\package.ps1
else
	./package.sh
endif

.PHONY : run
run:
ifeq ($(OS),Windows_NT)
		cd target && .\debug\framework && cd ..
else
	cd target; ./debug/framework; cd ..
endif
