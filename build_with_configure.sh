# 赋予脚本执行的权限
sudo chmod 777 ./configure ./autogen.sh

# autogen.sh脚本内部会执行.configure : CPPFLAGS="-fPIC" LIBS=-lxml2 $srcdir/configure $EXTRA_ARGS "$@" --disable-shared
./autogen.sh
make clean
# 执行编译，在.libs下生成libxml2.a静态库
make

cd .libs
# 将生成的.a文件放置在项目根目录，使得cargo build能够搜索到
ar x libxml2.a
ar r libxml2_static.a *.o
cp -r ./libxml2_static.a ../
cd ..
# rust编译，生成.a静态库
cd rust
cargo clean
cargo build --release -v
cd ..

mv libxml2_static.a ./.libs

cp rust/target/release/librust_project.a /usr/lib

# 开启COMPILE_WITH_RUST宏，并且链接librust_project.a
# 在.libs下生成libxml2.a和libxml2.so
./configure CPPFLAGS="-DCOMPILE_WITH_RUST" LDFLAGS="-L/usr/lib -lrust_project" with_xmllint="yes" with_xmlcatalog="yes"

make clean
make
# 可以在.libs中看到生成的libxml2.a和libxml2.so
ls .libs
