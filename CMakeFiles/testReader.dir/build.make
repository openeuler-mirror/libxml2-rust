# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.20

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
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /root/code/01/08/libxml2-rust

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /root/code/01/08/libxml2-rust

# Include any dependencies generated for this target.
include CMakeFiles/testReader.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/testReader.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/testReader.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/testReader.dir/flags.make

CMakeFiles/testReader.dir/testReader.c.o: CMakeFiles/testReader.dir/flags.make
CMakeFiles/testReader.dir/testReader.c.o: testReader.c
CMakeFiles/testReader.dir/testReader.c.o: CMakeFiles/testReader.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/root/code/01/08/libxml2-rust/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/testReader.dir/testReader.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/testReader.dir/testReader.c.o -MF CMakeFiles/testReader.dir/testReader.c.o.d -o CMakeFiles/testReader.dir/testReader.c.o -c /root/code/01/08/libxml2-rust/testReader.c

CMakeFiles/testReader.dir/testReader.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/testReader.dir/testReader.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /root/code/01/08/libxml2-rust/testReader.c > CMakeFiles/testReader.dir/testReader.c.i

CMakeFiles/testReader.dir/testReader.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/testReader.dir/testReader.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /root/code/01/08/libxml2-rust/testReader.c -o CMakeFiles/testReader.dir/testReader.c.s

# Object files for target testReader
testReader_OBJECTS = \
"CMakeFiles/testReader.dir/testReader.c.o"

# External object files for target testReader
testReader_EXTERNAL_OBJECTS =

testReader: CMakeFiles/testReader.dir/testReader.c.o
testReader: CMakeFiles/testReader.dir/build.make
testReader: libxml2.a
testReader: /usr/lib64/libc.so
testReader: /usr/lib64/liblzma.so
testReader: /usr/lib64/libz.so
testReader: CMakeFiles/testReader.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/root/code/01/08/libxml2-rust/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C executable testReader"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/testReader.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/testReader.dir/build: testReader
.PHONY : CMakeFiles/testReader.dir/build

CMakeFiles/testReader.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/testReader.dir/cmake_clean.cmake
.PHONY : CMakeFiles/testReader.dir/clean

CMakeFiles/testReader.dir/depend:
	cd /root/code/01/08/libxml2-rust && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /root/code/01/08/libxml2-rust /root/code/01/08/libxml2-rust /root/code/01/08/libxml2-rust /root/code/01/08/libxml2-rust /root/code/01/08/libxml2-rust/CMakeFiles/testReader.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/testReader.dir/depend

