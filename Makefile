.PHONY: all build

all: build

build: 
	g++ -std=c++23 -pthread main.cc
