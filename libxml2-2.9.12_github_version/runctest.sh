#!/bin/bash
rm -rf CMakeCache.txt
rm -rf cmake_install.cmake
rm -rf CMakeFiles
rm -rf Makefile


make clean
make distclean

cmake -DSTEP="build"
make

cd rust_ffi
cargo clean
cargo build
cd ../

cd rust
cargo clean
cargo build
cd ../

cmake -DSTEP="link"
make 
ctest
