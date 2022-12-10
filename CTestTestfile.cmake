# CMake generated Testfile for 
# Source directory: /root/yhm/libxml2-rust
# Build directory: /root/yhm/libxml2-rust
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(testapi "/root/yhm/libxml2-rust/testapi")
set_tests_properties(testapi PROPERTIES  _BACKTRACE_TRIPLES "/root/yhm/libxml2-rust/CMakeLists.txt;636;add_test;/root/yhm/libxml2-rust/CMakeLists.txt;0;")
add_test(testchar "/root/yhm/libxml2-rust/testchar")
set_tests_properties(testchar PROPERTIES  _BACKTRACE_TRIPLES "/root/yhm/libxml2-rust/CMakeLists.txt;638;add_test;/root/yhm/libxml2-rust/CMakeLists.txt;0;")
add_test(testdict "/root/yhm/libxml2-rust/testdict")
set_tests_properties(testdict PROPERTIES  _BACKTRACE_TRIPLES "/root/yhm/libxml2-rust/CMakeLists.txt;639;add_test;/root/yhm/libxml2-rust/CMakeLists.txt;0;")
add_test(testrecurse "/root/yhm/libxml2-rust/testrecurse")
set_tests_properties(testrecurse PROPERTIES  WORKING_DIRECTORY "/root/yhm/libxml2-rust" _BACKTRACE_TRIPLES "/root/yhm/libxml2-rust/CMakeLists.txt;640;add_test;/root/yhm/libxml2-rust/CMakeLists.txt;0;")
add_test(runtest "/root/yhm/libxml2-rust/runtest" "--out" "/root/yhm/libxml2-rust")
set_tests_properties(runtest PROPERTIES  WORKING_DIRECTORY "/root/yhm/libxml2-rust" _BACKTRACE_TRIPLES "/root/yhm/libxml2-rust/CMakeLists.txt;655;add_test;/root/yhm/libxml2-rust/CMakeLists.txt;0;")
add_test(testThreads "/root/yhm/libxml2-rust/testThreads")
set_tests_properties(testThreads PROPERTIES  WORKING_DIRECTORY "/root/yhm/libxml2-rust" _BACKTRACE_TRIPLES "/root/yhm/libxml2-rust/CMakeLists.txt;656;add_test;/root/yhm/libxml2-rust/CMakeLists.txt;0;")
