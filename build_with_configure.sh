# 赋予脚本执行的权限
sudo chmod 777 ./configure ./autogen.sh

# autogen.sh脚本内部会执行.configure : CPPFLAGS="-fPIC" LIBS=-lxml2 $srcdir/configure $EXTRA_ARGS "$@" --disable-shared
./autogen.sh
make clean
# 执行编译，在.libs下生成libxml2.a静态库
make

# 将生成的.a文件放置在项目根目录，使得cargo build能够搜索到
cp -r .libs/libxml2.a .

# rust编译，生成.a静态库
cd rust_ffi 
cargo clean 
cargo build 
cd ../rust
cargo clean
cargo build --release -v
cd ..

rm -rf libxml2.a

# 开启COMPILE_WITH_RUST宏，并且链接librust_project.a
# 在.libs下生成libxml2.a和libxml2.so
./configure CPPFLAGS="-DCOMPILE_WITH_RUST" LDFLAGS="-lrust_project"

make clean
make

# 可以在.libs中看到生成的libxml2.a和libxml2.so
ls .libs