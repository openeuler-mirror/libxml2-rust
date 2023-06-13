# 清除上一次编译数据
CUR_PATH=$(cd `dirname $0`; pwd)
cd $CUR_PATH
rm .libs -rf
rm autom4te.cache -rf
rm rust/target -rf

# 赋予脚本执行的权限
sudo chmod 777 ./configure ./autogen.sh

# autogen.sh脚本内部会执行.configure : CPPFLAGS="-fPIC" LIBS=-lxml2 $srcdir/configure $EXTRA_ARGS "$@" --disable-shared
./autogen.sh
if [ $? -ne 0 ];then
    echo "autogen.sh failed."
    exit 1
fi
make clean
# 执行编译，在.libs下生成libxml2.a静态库
make
if [ $? -ne 0 ];then
    echo "first make failed."
    exit 1
fi

cd .libs
# 将生成的.a文件放置在项目根目录，使得cargo build能够搜索到
ar x libxml2.a
ar r libxml2_static.a *.o
if [ $? -ne 0 ];then
    echo "ar r libxml2_static.a *.o failed."
    exit 1
fi
cp -r ./libxml2_static.a ../
cd ..
# rust编译，生成.a静态库
cd rust
cargo clean
cargo build --release -v
if [ $? -ne 0 ];then
    echo "cargo build --release -v failed."
    exit 1
fi
cd ..

mv libxml2_static.a ./.libs

#cp rust/target/release/librust_project.a /usr/lib

# 开启COMPILE_WITH_RUST宏，并且链接librust_project.a
# 在.libs下生成libxml2.a和libxml2.so
./configure CPPFLAGS="-DCOMPILE_WITH_RUST" LDFLAGS="-L$CUR_PATH/rust/target/release/ -lrust_project" with_xmllint="yes" with_xmlcatalog="yes"
if [ $? -ne 0 ];then
    echo "configure failed."
    exit 1
fi

make clean
make
if [ $? -ne 0 ];then
    echo "second make failed."
    exit 1
fi
# 可以在.libs中看到生成的libxml2.a和libxml2.so
ls .libs
