#项目名称：libxml2-rust
使用Rust语言重写libxml2高危模块

#### 项目目标
libxml2历史安全漏洞较多，其中内存漏洞在两个项目漏洞中分别占比74%，都比较高，产生这些安全漏洞或与这些安全漏洞相关的模块称为高危模块。本项目拟通过Rust语言重写libxml2库的这些高危模块，利用Rust语言自身的内存安全机制修复这些安全漏洞，增加两个库的安全性和可靠性。

#### 项目实施方案
整体方案：
按照以下步骤，对libxml2部分文件进行改写并通过测试：
- 设计并实现Rust&C混合编译
- 自动改写C代码
- 改写C宏定义
- 缩小unsafe范围

(1)Step1：混合编译
使用CMake工具，通过编写CMakeLists.txt文件，将Rust项目编译过程插入到C项目的编译过程和链接过程中间。理论过程如下：
- C工程实现编译过程（包括预处理、编译、汇编），生成库文件（.a文件/.so文件）；
- Rust工程借助C工程的库文件实现编译过程，并生成自身的库文件（.a文件/.so文件）；
- C工程将自身的库文件与Rust工程的库文件进行链接；

具体文件改写方式如下：
- 改写CMakeLists.txt文件完成C项目的编译与链接：设置条件变量STEP，当STEP为build时，执行CMakeLists.txt中的编译部分，当STEP为link时，执行CMakeLists.txt中的连接部分。在调用cmake命令时传入该参数，第一步时参数值为build，第三步时参数值为link。
- 编写build.rs和cargo.toml完成rust项目的编译。其中，build.rs中定位c项目中的库文件并提供给项目使用；cargo.toml生成rust项目本身的库文件。

改写上述文件后，可通过如下顺序完成整个项目的混合编译过程：
- 在C工程目录下，输入cmake . -D STEP=build命令。
- 在C工程目录下，输入make命令。
- 在Rust工程目录下，输入cargo build命令。
- 在C工程目录下，输入cmake . -D STEP=link命令。
- 在C工程目录下，输入make命令。

(2)Step2：自动改写C代码
项目中使用成熟的C转Rust工具C2Rust进行自动改写，该工具可以自动转换得到与C代码功能相同的Rust代码。

改写过程遵循以下规范：
- 对于要改写的C函数，在C工程中，保留其函数声明，在函数定义内调用对应Rust版本的函数，并在C文件头部使用extern声明需要调用的Rust版本的函数；在Rust工程中，在对应的Rust版本函数体内填入转换得到的Rust代码，使用extern "C"将该函数声明为可被C工程调用。
- 对于Rust工程要调用的C函数，使用link关键字声明用到的模块，并使用extern声明其函数定义。

对每个函数改写完成后，使用ctest工具执行执行系统测试，并针对出现的问题进行代码手动修复，保证每次改写都可通过所有测试用例。

(3)Step3：改写宏定义
宏定义在C工程中主要有宏作为值进行访问和宏作为编译条件进行访问两种情况。针对这两种情况，分别制定了不同的改写方案。

宏作为值时，在C工程中，将所有方法中使用的宏在方法之前进行定义，并对每个方法使用的宏通过一个结构体来保存。在C工程中编写函数，负责返回该结构体中各个属性的值，以供Rust工程调用。在Rust工程中定义结构相同的结构体，并构造单例全局变量，调用C工程中传递结构体值的函数，赋值给单例全局变量，以供Rust工程使用。Rust工程调用C工程中传递结构体值函数的过程符合上文所述Rust改写规范。

宏作为编译条件时，在C工程中，与宏作为值类似，建立方法返回条件宏的值。在Rust工程中，建立专门用于处理条件宏值的rust工具包，即rust_ffi，在工具包中编写cfg设置方法，该方法作用是根据C工程提供的条件宏值打印对应宏命令行，以建立Rust的条件宏。在存放改写代码的rust工程包，即rust_project，通过调用rust_ffi工具包的cfg设置方法添加对应cfg属性。此时，Rust工程即可根据cfg属性情况进行条件编译。

在该宏作为编译条件的解决方案下，Rust工程中rust_ffi工具包需要依赖可执行的c库，因此需要在CMakeLists.txt添加全局宏定义，再拟造出一个无需链接Rust工程即可执行的C库，添加方式如下。同时C工程中，当进行编译时，需要提供可直接运行的空壳方法，以生成可执行C库。

```
if(STEP STREQUAL "link")
  ADD_DEFINITIONS(-DCOMPILE_WITH_RUST) #当链接rust时，添加全局宏定义COMPILE_WITH_RUST
endif()
```

值得一提的是，在自动改写过程中，如果环境不满足条件宏，则对应代码无法自动翻译。因此需要首先将条件宏删除，获得完整代码，再根据条件宏分别添加对应代码。

(4)Step4：缩小unsafe范围

在进行自动代码改写后，Rust版本函数都使用了unsafe进行包裹，需要进行范围缩小。针对以下情况，代码进行了unsafe范围调整：
- Rust工程调用C工程函数时，将Rust工程中的每个C函数调用都使用一个unsafe代码块进行包裹，并返回一个新的无unsafe函数。改写的代码调用新的无unsafe函数。
- 部分函数的入参或返回值为裸指针，对裸指针操作属于unsafe操作，使用解引用的方式将裸指针转化为引用。
- 对于以指针形式提供的c语言字符串和数组，使用unsafe代码块进行小范围包裹。

按照上述方式进行unsafe调整后，即可去除Rust版本函数的整体unsafe标识符，缩小其unsafe范围。

#### 推进计划
- 第一阶段：确定原项目版本；
- 第二阶段：构建交叉编译环境、整理和添加Rust接口；
- 第三阶段：分批完成Rust接口的具体实现；
- 第四阶段：分批次完成宏定义改写和合并；
- 第五阶段：分批完成Rust代码仲的unsafe范围缩小。


#### libxml2-rust库的安装教程
准备make,cmake,rust等环境。版本推荐：
- cmake:3.22 包地址：http://repo.openeuler.org/openEuler-22.03-LTS/everything/x86_64/Packages/cmake-3.22.0-4.oe2203.x86_64.rpm
- make: 4.3 包地址：http://repo.openeuler.org/openEuler-22.03-LTS/everything/x86_64/Packages/make-4.3-2.oe2203.x86_64.rpm
- rust：1.57.0 包地址：http://repo.openeuler.org/openEuler-22.03-LTS/everything/x86_64/Packages/rust-1.57.0-1.oe2203.x86_64.rpm

安装步骤如下：

1. 通过gitee 获取代码 git clone git@gitee.com:openeuler/libxml2-rust.git

2. 运行build.sh编译代码

3. 运行make install 将代码安装到系统中

#### libxml2-rust库的使用方法

使用时，引入libxml包下的头文件parser.h和xmlmemory.h进行xml文档解析。例如，针对一个文档名为story.xml的文档，其解析代码如下：
```
/* 解析storyinfo节点，打印keyword节点的内容 */
void parseStory(xmlDocPtr doc, xmlNodePtr cur){
	xmlChar* key;
	cur=cur->xmlChildrenNode;
	while(cur != NULL){
		/* 找到keyword子节点 */
		if(!xmlStrcmp(cur->name, (const xmlChar *)"keyword")){
			key = xmlNodeListGetString(doc, cur->xmlChildrenNode, 1);
			printf("keyword: %s\n", key);
			xmlFree(key);
		}
		cur=cur->next; /* 下一个子节点 */
	}
 
	return;
}
 
/* 解析文档 */
static void parseDoc(char *docname){
	/* 定义文档和节点指针 */
	xmlDocPtr doc;
	xmlNodePtr cur;
	
	/* 进行解析，如果没成功，显示一个错误并停止 */
	doc = xmlParseFile(docname);
	if(doc == NULL){
		fprintf(stderr, "Document not parse successfully. \n");
		return;
	}
 
	/* 获取文档根节点，若无内容则释放文档树并返回 */
	cur = xmlDocGetRootElement(doc);
	if(cur == NULL){
		fprintf(stderr, "empty document\n");
		xmlFreeDoc(doc);
		return;
	}
 
	/* 确定根节点名是否为story，不是则返回 */
	if(xmlStrcmp(cur->name, (const xmlChar *)"story")){
		fprintf(stderr, "document of the wrong type, root node != story");
		xmlFreeDoc(doc);
		return;
	}
 
	/* 遍历文档树 */
	cur = cur->xmlChildrenNode;
	while(cur != NULL){
		/* 找到storyinfo子节点 */
		if(!xmlStrcmp(cur->name, (const xmlChar *)"storyinfo")){
			parseStory(doc, cur); /* 解析storyinfo子节点 */
		}
		cur = cur->next; /* 下一个子节点 */
	}
 
	xmlFreeDoc(doc); /* 释放文档树 */
	return;
}
```

rust库作为libxml2的依赖不单独对外提供服务。libxml2-rust的使用方式与libxml2完全相同，更多使用方式可以参考gitlab上的wiki：https://gitlab.gnome.org/GNOME/libxml2/-/wikis/home。

#### 特别说明

libxml2-2.9.12_openEuler_version目录下的是原始代码，但在openEuler操作系统下有部分ctest测试用例无法通过，故我们将libxml2官方库的代码放在libxml2-2.9.12_github_version目录下一并提交。所有对libxml2-2.9.12_openEuler_version的改写操作也会同步到libxml2-2.9.12_github_version，通过后者能否通过所有ctest测试用例来判断改写内容的正确性。



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
