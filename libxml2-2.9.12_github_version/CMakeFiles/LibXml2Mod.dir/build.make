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
include CMakeFiles/LibXml2Mod.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/LibXml2Mod.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/LibXml2Mod.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/LibXml2Mod.dir/flags.make

CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o: CMakeFiles/LibXml2Mod.dir/flags.make
CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o: libxml2-py.c
CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o: CMakeFiles/LibXml2Mod.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o -MF CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o.d -o CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o -c /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/libxml2-py.c

CMakeFiles/LibXml2Mod.dir/libxml2-py.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/LibXml2Mod.dir/libxml2-py.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/libxml2-py.c > CMakeFiles/LibXml2Mod.dir/libxml2-py.c.i

CMakeFiles/LibXml2Mod.dir/libxml2-py.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/LibXml2Mod.dir/libxml2-py.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/libxml2-py.c -o CMakeFiles/LibXml2Mod.dir/libxml2-py.c.s

CMakeFiles/LibXml2Mod.dir/python/libxml.c.o: CMakeFiles/LibXml2Mod.dir/flags.make
CMakeFiles/LibXml2Mod.dir/python/libxml.c.o: python/libxml.c
CMakeFiles/LibXml2Mod.dir/python/libxml.c.o: CMakeFiles/LibXml2Mod.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building C object CMakeFiles/LibXml2Mod.dir/python/libxml.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/LibXml2Mod.dir/python/libxml.c.o -MF CMakeFiles/LibXml2Mod.dir/python/libxml.c.o.d -o CMakeFiles/LibXml2Mod.dir/python/libxml.c.o -c /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/python/libxml.c

CMakeFiles/LibXml2Mod.dir/python/libxml.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/LibXml2Mod.dir/python/libxml.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/python/libxml.c > CMakeFiles/LibXml2Mod.dir/python/libxml.c.i

CMakeFiles/LibXml2Mod.dir/python/libxml.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/LibXml2Mod.dir/python/libxml.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/python/libxml.c -o CMakeFiles/LibXml2Mod.dir/python/libxml.c.s

CMakeFiles/LibXml2Mod.dir/python/types.c.o: CMakeFiles/LibXml2Mod.dir/flags.make
CMakeFiles/LibXml2Mod.dir/python/types.c.o: python/types.c
CMakeFiles/LibXml2Mod.dir/python/types.c.o: CMakeFiles/LibXml2Mod.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building C object CMakeFiles/LibXml2Mod.dir/python/types.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/LibXml2Mod.dir/python/types.c.o -MF CMakeFiles/LibXml2Mod.dir/python/types.c.o.d -o CMakeFiles/LibXml2Mod.dir/python/types.c.o -c /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/python/types.c

CMakeFiles/LibXml2Mod.dir/python/types.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/LibXml2Mod.dir/python/types.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/python/types.c > CMakeFiles/LibXml2Mod.dir/python/types.c.i

CMakeFiles/LibXml2Mod.dir/python/types.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/LibXml2Mod.dir/python/types.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/python/types.c -o CMakeFiles/LibXml2Mod.dir/python/types.c.s

# Object files for target LibXml2Mod
LibXml2Mod_OBJECTS = \
"CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o" \
"CMakeFiles/LibXml2Mod.dir/python/libxml.c.o" \
"CMakeFiles/LibXml2Mod.dir/python/types.c.o"

# External object files for target LibXml2Mod
LibXml2Mod_EXTERNAL_OBJECTS =

libxml2mod.so.2.9.10: CMakeFiles/LibXml2Mod.dir/libxml2-py.c.o
libxml2mod.so.2.9.10: CMakeFiles/LibXml2Mod.dir/python/libxml.c.o
libxml2mod.so.2.9.10: CMakeFiles/LibXml2Mod.dir/python/types.c.o
libxml2mod.so.2.9.10: CMakeFiles/LibXml2Mod.dir/build.make
libxml2mod.so.2.9.10: libxml2.a
libxml2mod.so.2.9.10: /usr/lib/x86_64-linux-gnu/libpython2.7.so
libxml2mod.so.2.9.10: /usr/lib/x86_64-linux-gnu/liblzma.so
libxml2mod.so.2.9.10: /usr/lib/x86_64-linux-gnu/libz.so
libxml2mod.so.2.9.10: CMakeFiles/LibXml2Mod.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Linking C shared library libxml2mod.so"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/LibXml2Mod.dir/link.txt --verbose=$(VERBOSE)
	$(CMAKE_COMMAND) -E cmake_symlink_library libxml2mod.so.2.9.10 libxml2mod.so.2.9.10 libxml2mod.so

libxml2mod.so: libxml2mod.so.2.9.10
	@$(CMAKE_COMMAND) -E touch_nocreate libxml2mod.so

# Rule to build all files generated by this target.
CMakeFiles/LibXml2Mod.dir/build: libxml2mod.so
.PHONY : CMakeFiles/LibXml2Mod.dir/build

CMakeFiles/LibXml2Mod.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/LibXml2Mod.dir/cmake_clean.cmake
.PHONY : CMakeFiles/LibXml2Mod.dir/clean

CMakeFiles/LibXml2Mod.dir/depend:
	cd /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version /root/pr/20220711_01/libxml2-rust_1/libxml2-2.9.12_github_version/CMakeFiles/LibXml2Mod.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/LibXml2Mod.dir/depend

