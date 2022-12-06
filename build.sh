rm -rf CMakeCache.txt
make clean
cmake -DSTEP="build"
make
cp libxml2.a ./libxml2_static.a
cd rust
cargo clean
cargo build --release -v
cd ../
cmake -DSTEP="link"
make
