# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.23

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /snap/cmake/1088/bin/cmake

# The command to remove a file.
RM = /snap/cmake/1088/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version

# Include any dependencies generated for this target.
include CMakeFiles/testXPath.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/testXPath.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/testXPath.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/testXPath.dir/flags.make

CMakeFiles/testXPath.dir/testXPath.c.o: CMakeFiles/testXPath.dir/flags.make
CMakeFiles/testXPath.dir/testXPath.c.o: testXPath.c
CMakeFiles/testXPath.dir/testXPath.c.o: CMakeFiles/testXPath.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/testXPath.dir/testXPath.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/testXPath.dir/testXPath.c.o -MF CMakeFiles/testXPath.dir/testXPath.c.o.d -o CMakeFiles/testXPath.dir/testXPath.c.o -c /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/testXPath.c

CMakeFiles/testXPath.dir/testXPath.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/testXPath.dir/testXPath.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/testXPath.c > CMakeFiles/testXPath.dir/testXPath.c.i

CMakeFiles/testXPath.dir/testXPath.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/testXPath.dir/testXPath.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/testXPath.c -o CMakeFiles/testXPath.dir/testXPath.c.s

# Object files for target testXPath
testXPath_OBJECTS = \
"CMakeFiles/testXPath.dir/testXPath.c.o"

# External object files for target testXPath
testXPath_EXTERNAL_OBJECTS =

testXPath: CMakeFiles/testXPath.dir/testXPath.c.o
testXPath: CMakeFiles/testXPath.dir/build.make
testXPath: libxml2.a
testXPath: /usr/lib/x86_64-linux-gnu/liblzma.so
testXPath: /usr/lib/x86_64-linux-gnu/libz.so
testXPath: CMakeFiles/testXPath.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C executable testXPath"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/testXPath.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/testXPath.dir/build: testXPath
.PHONY : CMakeFiles/testXPath.dir/build

CMakeFiles/testXPath.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/testXPath.dir/cmake_clean.cmake
.PHONY : CMakeFiles/testXPath.dir/clean

CMakeFiles/testXPath.dir/depend:
	cd /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles/testXPath.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/testXPath.dir/depend

