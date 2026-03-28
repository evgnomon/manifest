.PHONY: all build clean

rust_target := target/debug/manifest
cpp_target := cpp_manifest

all: build

build: $(cpp_target) $(rust_target)

$(cpp_target): 
	g++ -std=c++23 -pthread src/main.cc src/run.cc -I src -o $(cpp_target)

$(rust_target):
	cargo build

clean:
	rm -f $(cpp_target)
	zig clean
	cargo clean
