# CMake generated Testfile for 
# Source directory: /root/code/01/01/libxml2-rust
# Build directory: /root/code/01/01/libxml2-rust
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(testapi "/root/code/01/01/libxml2-rust/testapi")
set_tests_properties(testapi PROPERTIES  _BACKTRACE_TRIPLES "/root/code/01/01/libxml2-rust/CMakeLists.txt;605;add_test;/root/code/01/01/libxml2-rust/CMakeLists.txt;0;")
add_test(testchar "/root/code/01/01/libxml2-rust/testchar")
set_tests_properties(testchar PROPERTIES  _BACKTRACE_TRIPLES "/root/code/01/01/libxml2-rust/CMakeLists.txt;607;add_test;/root/code/01/01/libxml2-rust/CMakeLists.txt;0;")
add_test(testdict "/root/code/01/01/libxml2-rust/testdict")
set_tests_properties(testdict PROPERTIES  _BACKTRACE_TRIPLES "/root/code/01/01/libxml2-rust/CMakeLists.txt;608;add_test;/root/code/01/01/libxml2-rust/CMakeLists.txt;0;")
add_test(testrecurse "/root/code/01/01/libxml2-rust/testrecurse")
set_tests_properties(testrecurse PROPERTIES  WORKING_DIRECTORY "/root/code/01/01/libxml2-rust" _BACKTRACE_TRIPLES "/root/code/01/01/libxml2-rust/CMakeLists.txt;609;add_test;/root/code/01/01/libxml2-rust/CMakeLists.txt;0;")
add_test(runtest "/root/code/01/01/libxml2-rust/runtest" "--out" "/root/code/01/01/libxml2-rust")
set_tests_properties(runtest PROPERTIES  WORKING_DIRECTORY "/root/code/01/01/libxml2-rust" _BACKTRACE_TRIPLES "/root/code/01/01/libxml2-rust/CMakeLists.txt;623;add_test;/root/code/01/01/libxml2-rust/CMakeLists.txt;0;")
add_test(testThreads "/root/code/01/01/libxml2-rust/testThreads")
set_tests_properties(testThreads PROPERTIES  WORKING_DIRECTORY "/root/code/01/01/libxml2-rust" _BACKTRACE_TRIPLES "/root/code/01/01/libxml2-rust/CMakeLists.txt;624;add_test;/root/code/01/01/libxml2-rust/CMakeLists.txt;0;")
