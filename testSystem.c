/*
 * testSystem.c : a system tester program for libxml2
 *
 * See Copyright for the status of this software.
 */

#include <stdio.h>
#include <stdlib.h>

int system_test(){
    int res = -1;
    res = system("cd system_test_case && /bin/bash sys_test.sh");
    return res;
}

int main(){
    int test_res = system_test();
    if(test_res == 0){
        printf("\nSystem_test success\n");
    }else{
        printf("\nSystem_test error\n");
    }
    return 0;
}