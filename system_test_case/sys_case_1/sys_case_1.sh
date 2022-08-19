#!/bin/bash
rm -rf testcase
gcc testcase.c -o testcase -I ../../include/ -L ../../rust/target/debug/ -lxml2 -lrust_project -lpthread -ldl -lz -llzma -lm
echo "sys_test_1 begin"
./testcase
echo "sys_test_1 end"

