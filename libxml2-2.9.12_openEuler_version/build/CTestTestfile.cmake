# CMake generated Testfile for 
# Source directory: /home/code/libxml2-2.9.12
# Build directory: /home/code/libxml2-2.9.12/build
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(testapi "/home/code/libxml2-2.9.12/build/testapi")
set_tests_properties(testapi PROPERTIES  _BACKTRACE_TRIPLES "/home/code/libxml2-2.9.12/CMakeLists.txt;582;add_test;/home/code/libxml2-2.9.12/CMakeLists.txt;0;")
add_test(testchar "/home/code/libxml2-2.9.12/build/testchar")
set_tests_properties(testchar PROPERTIES  _BACKTRACE_TRIPLES "/home/code/libxml2-2.9.12/CMakeLists.txt;584;add_test;/home/code/libxml2-2.9.12/CMakeLists.txt;0;")
add_test(testdict "/home/code/libxml2-2.9.12/build/testdict")
set_tests_properties(testdict PROPERTIES  _BACKTRACE_TRIPLES "/home/code/libxml2-2.9.12/CMakeLists.txt;585;add_test;/home/code/libxml2-2.9.12/CMakeLists.txt;0;")
add_test(testrecurse "/home/code/libxml2-2.9.12/build/testrecurse")
set_tests_properties(testrecurse PROPERTIES  WORKING_DIRECTORY "/home/code/libxml2-2.9.12" _BACKTRACE_TRIPLES "/home/code/libxml2-2.9.12/CMakeLists.txt;586;add_test;/home/code/libxml2-2.9.12/CMakeLists.txt;0;")
add_test(runtest "/home/code/libxml2-2.9.12/build/runtest" "--out" "/home/code/libxml2-2.9.12/build")
set_tests_properties(runtest PROPERTIES  WORKING_DIRECTORY "/home/code/libxml2-2.9.12" _BACKTRACE_TRIPLES "/home/code/libxml2-2.9.12/CMakeLists.txt;600;add_test;/home/code/libxml2-2.9.12/CMakeLists.txt;0;")
add_test(testThreads "/home/code/libxml2-2.9.12/build/testThreads")
set_tests_properties(testThreads PROPERTIES  WORKING_DIRECTORY "/home/code/libxml2-2.9.12" _BACKTRACE_TRIPLES "/home/code/libxml2-2.9.12/CMakeLists.txt;601;add_test;/home/code/libxml2-2.9.12/CMakeLists.txt;0;")
