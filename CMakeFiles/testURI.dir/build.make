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
CMAKE_SOURCE_DIR = /root/code/01/10/libxml2-rust

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /root/code/01/10/libxml2-rust

# Include any dependencies generated for this target.
include CMakeFiles/testURI.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/testURI.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/testURI.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/testURI.dir/flags.make

CMakeFiles/testURI.dir/testURI.c.o: CMakeFiles/testURI.dir/flags.make
CMakeFiles/testURI.dir/testURI.c.o: testURI.c
CMakeFiles/testURI.dir/testURI.c.o: CMakeFiles/testURI.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/root/code/01/10/libxml2-rust/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/testURI.dir/testURI.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/testURI.dir/testURI.c.o -MF CMakeFiles/testURI.dir/testURI.c.o.d -o CMakeFiles/testURI.dir/testURI.c.o -c /root/code/01/10/libxml2-rust/testURI.c

CMakeFiles/testURI.dir/testURI.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/testURI.dir/testURI.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /root/code/01/10/libxml2-rust/testURI.c > CMakeFiles/testURI.dir/testURI.c.i

CMakeFiles/testURI.dir/testURI.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/testURI.dir/testURI.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /root/code/01/10/libxml2-rust/testURI.c -o CMakeFiles/testURI.dir/testURI.c.s

# Object files for target testURI
testURI_OBJECTS = \
"CMakeFiles/testURI.dir/testURI.c.o"

# External object files for target testURI
testURI_EXTERNAL_OBJECTS =

testURI: CMakeFiles/testURI.dir/testURI.c.o
testURI: CMakeFiles/testURI.dir/build.make
testURI: libxml2.a
testURI: /usr/lib64/libc.so
testURI: /usr/lib64/liblzma.so
testURI: /usr/lib64/libz.so
testURI: CMakeFiles/testURI.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/root/code/01/10/libxml2-rust/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C executable testURI"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/testURI.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/testURI.dir/build: testURI
.PHONY : CMakeFiles/testURI.dir/build

CMakeFiles/testURI.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/testURI.dir/cmake_clean.cmake
.PHONY : CMakeFiles/testURI.dir/clean

CMakeFiles/testURI.dir/depend:
	cd /root/code/01/10/libxml2-rust && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /root/code/01/10/libxml2-rust /root/code/01/10/libxml2-rust /root/code/01/10/libxml2-rust /root/code/01/10/libxml2-rust /root/code/01/10/libxml2-rust/CMakeFiles/testURI.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/testURI.dir/depend

