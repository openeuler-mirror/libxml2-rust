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
include CMakeFiles/testchar.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/testchar.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/testchar.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/testchar.dir/flags.make

CMakeFiles/testchar.dir/testchar.c.o: CMakeFiles/testchar.dir/flags.make
CMakeFiles/testchar.dir/testchar.c.o: testchar.c
CMakeFiles/testchar.dir/testchar.c.o: CMakeFiles/testchar.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/testchar.dir/testchar.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/testchar.dir/testchar.c.o -MF CMakeFiles/testchar.dir/testchar.c.o.d -o CMakeFiles/testchar.dir/testchar.c.o -c /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/testchar.c

CMakeFiles/testchar.dir/testchar.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/testchar.dir/testchar.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/testchar.c > CMakeFiles/testchar.dir/testchar.c.i

CMakeFiles/testchar.dir/testchar.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/testchar.dir/testchar.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/testchar.c -o CMakeFiles/testchar.dir/testchar.c.s

# Object files for target testchar
testchar_OBJECTS = \
"CMakeFiles/testchar.dir/testchar.c.o"

# External object files for target testchar
testchar_EXTERNAL_OBJECTS =

testchar: CMakeFiles/testchar.dir/testchar.c.o
testchar: CMakeFiles/testchar.dir/build.make
testchar: libxml2.a
testchar: /usr/lib/x86_64-linux-gnu/liblzma.so
testchar: /usr/lib/x86_64-linux-gnu/libz.so
testchar: CMakeFiles/testchar.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C executable testchar"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/testchar.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/testchar.dir/build: testchar
.PHONY : CMakeFiles/testchar.dir/build

CMakeFiles/testchar.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/testchar.dir/cmake_clean.cmake
.PHONY : CMakeFiles/testchar.dir/clean

CMakeFiles/testchar.dir/depend:
	cd /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles/testchar.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/testchar.dir/depend

