.PHONY: all build

rust_target := target/debug/manifest
cpp_target := cpp_manifest

all: build

build: $(cpp_target) $(rust_target)

$(cpp_target): 
	g++ -std=c++23 -pthread main.cc -o $(cpp_target)

$(rust_target):
	cargo build
