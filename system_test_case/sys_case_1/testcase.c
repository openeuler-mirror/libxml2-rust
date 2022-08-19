#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>
#include <libxml/xmlmemory.h>
#include <libxml/parser.h>

/* 解析节点，打印节点内容 */
static void parseStory(xmlDocPtr doc, xmlNodePtr cur)
{
	xmlChar *key;
	cur = cur->xmlChildrenNode;
	while (cur != NULL)
	{
		/* 找到keyword子节点 */
		if (!xmlStrcmp(cur->name, (const xmlChar *)"keyword"))
		{
			key = xmlNodeListGetString(doc, cur->xmlChildrenNode, 1);
			// printf("keyword: %s\n", key);
			xmlFree(key);
		}
		cur = cur->next; /* 下一个子节点 */
	}

	return;
}

/* 解析文档 */
static int parseDoc(char *docname)
{
	/* 定义文档和节点指针 */
	xmlDocPtr doc;
	xmlNodePtr cur;

	/* 进行解析，若未成功，显示一个错误并停止 */
	doc = xmlParseFile(docname);
	if (doc == NULL)
	{
		fprintf(stderr, "Document not parse successfully. \n");
		// printf("case %s failed\n", docname);
		return 0;
	}

	/* 获取文档根节点，若无内容则释放文档树并返回 */
	cur = xmlDocGetRootElement(doc);
	if (cur == NULL)
	{
		fprintf(stderr, "empty document\n");
		xmlFreeDoc(doc);
		// printf("case %s failed\n", docname);
		return 0;
	}

	/* 确定根节点名是否为目标节点，不是则返回 */
	if (xmlStrcmp(cur->name, (const xmlChar *)"story"))
	{
		fprintf(stderr, "document of the wrong type, root node != story");
		xmlFreeDoc(doc);
		// printf("case %s failed\n", docname);
		return 0;
	}

	/* 遍历文档树 */
	cur = cur->xmlChildrenNode;
	while (cur != NULL)
	{
		/* 找到子节点 */
		if (!xmlStrcmp(cur->name, (const xmlChar *)"storyinfo"))
		{
			parseStory(doc, cur); /* 解析子节点 */
		}
		cur = cur->next; /* 下一个子节点 */
	}

	xmlFreeDoc(doc); /* 释放文档树 */
	// printf("case %s pass\n", docname);
	return 1;
}

int main(int argc, char **argv)
{
	int N = 6;
	int i = 0;
	int cnt = 0;
	char docname[][20] = {"base.xml", "large.xml", "blank.xml", "no_exist.xml", "form_error1.xml", "form_error2.xml"};
	int result[] = {1, 1, 0, 0, 0, 0};
	double timeConsuming[N];
	double totalTime = 0;
	clock_t start, end;

	//测试
	for (i = 0; i < N; i++)
	{
		start = clock();
		if (parseDoc(docname[i]) == result[i])
		{
			printf("case %s passed, ", docname[i]);
			cnt++;
		}
		else
		{
			printf("case %s failed, ", docname[i]);
		}
		end = clock();
		timeConsuming[i] = ((double)(end - start)) / CLOCKS_PER_SEC * 1000;
		totalTime += timeConsuming[i];
		printf("spend %.0lf ms\n", timeConsuming[i]);
	}

	// 结果输出
	printf("%d of %d cases passed, total spend %.0lf ms\n", cnt, N, totalTime);

	return 1;
}