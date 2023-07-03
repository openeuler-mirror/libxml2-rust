# libxml2-rust

#### 介绍
libxml2-rust是一个libxml2的rust重写项目，该项目对原有libxml2中漏洞相对较多的HTMLparser.c、parser.c、xpath.c、parserInternals.c文件使用rust进行重写，接口和功能保持同原有的libxml2完全一致。
该软件的重构过程参见WIKI：[libxml2-rust重构介绍](https://gitee.com/openeuler/libxml2-rust/wikis/%E9%A1%B9%E7%9B%AE%E4%BB%8B%E7%BB%8D)

#### 使用说明
1. 安装依赖
```
yum install -y python3-devel rust  cargo rust-packaging gcc make cmake autoconf automake libtool zlib-devel xz-devel pkgconfig
```
2. 下载对应的发行版文件或者通过如下命令下载最新源码
```
git clone --depth=10 git@gitee.com:openeuler/libxml2-rust.git
```
3. 进入libxml2-rust目录，执行``sh build_with_configure.sh``执行构建
4. 编译成功后，执行``make install``进行安装

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
