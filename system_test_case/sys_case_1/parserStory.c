#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <libxml/xmlmemory.h>
#include <libxml/parser.h>
 
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
 
int main(int argc, char **argv){
	char *docname;
	if(argc <= 1){
		printf("Usage: %s docname\n", argv[0]);
		return 0;
	}
	docname=argv[1];
	parseDoc(docname);
	return 1;
}