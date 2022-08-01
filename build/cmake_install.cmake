# Install script for directory: /root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/usr/local")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/x86_64-linux-gnu-objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/libxml2/libxml" TYPE FILE FILES
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/c14n.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/catalog.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/chvalid.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/debugXML.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/dict.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/DOCBparser.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/encoding.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/entities.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/globals.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/hash.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/HTMLparser.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/HTMLtree.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/list.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/nanoftp.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/nanohttp.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/parser.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/parserInternals.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/pattern.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/relaxng.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/SAX.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/SAX2.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/schemasInternals.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/schematron.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/threads.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/tree.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/uri.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/valid.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xinclude.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xlink.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlIO.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlautomata.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlerror.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlexports.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlmemory.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlmodule.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlreader.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlregexp.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlsave.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlschemas.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlschemastypes.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlstring.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlunicode.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xmlwriter.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xpath.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xpathInternals.h"
    "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/include/libxml/xpointer.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xruntimex" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so.2.9.10" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so.2.9.10")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so.2.9.10"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/libxml2.so.2.9.10")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so.2.9.10" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so.2.9.10")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so.2.9.10"
         OLD_RPATH "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/rust/target/debug:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/x86_64-linux-gnu-strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so.2.9.10")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/libxml2.so")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so"
         OLD_RPATH "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/rust/target/debug:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/x86_64-linux-gnu-strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libxml2.so")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man3" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/libxml.3")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man1" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/doc/xmlcatalog.1")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man1" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/doc/xmllint.1")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/libxml2" TYPE DIRECTORY FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/doc/" REGEX "/Makefile\\.[^/]*$" EXCLUDE)
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/libxml2-config.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/libxml2-config-version.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10/libxml2-export.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10/libxml2-export.cmake"
         "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/CMakeFiles/Export/lib/cmake/libxml2-2.9.10/libxml2-export.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10/libxml2-export-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10/libxml2-export.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/CMakeFiles/Export/lib/cmake/libxml2-2.9.10/libxml2-export.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/CMakeFiles/Export/lib/cmake/libxml2-2.9.10/libxml2-export-debug.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/libxml2/libxml" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/libxml/xmlversion.h")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/libxml-2.0.pc")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/xml2-config")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE FILE FILES "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/xml2Conf.sh")
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/root/codes/libxml2-nju-huawei/libxml2-2.9.12_new_version/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
