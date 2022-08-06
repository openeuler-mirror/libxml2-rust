# Install script for directory: /root/code/01/08/libxml2-rust

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
    set(CMAKE_INSTALL_CONFIG_NAME "")
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
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/libxml2/libxml" TYPE FILE FILES
    "/root/code/01/08/libxml2-rust/include/libxml/c14n.h"
    "/root/code/01/08/libxml2-rust/include/libxml/catalog.h"
    "/root/code/01/08/libxml2-rust/include/libxml/chvalid.h"
    "/root/code/01/08/libxml2-rust/include/libxml/debugXML.h"
    "/root/code/01/08/libxml2-rust/include/libxml/dict.h"
    "/root/code/01/08/libxml2-rust/include/libxml/DOCBparser.h"
    "/root/code/01/08/libxml2-rust/include/libxml/encoding.h"
    "/root/code/01/08/libxml2-rust/include/libxml/entities.h"
    "/root/code/01/08/libxml2-rust/include/libxml/globals.h"
    "/root/code/01/08/libxml2-rust/include/libxml/hash.h"
    "/root/code/01/08/libxml2-rust/include/libxml/HTMLparser.h"
    "/root/code/01/08/libxml2-rust/include/libxml/HTMLtree.h"
    "/root/code/01/08/libxml2-rust/include/libxml/list.h"
    "/root/code/01/08/libxml2-rust/include/libxml/nanoftp.h"
    "/root/code/01/08/libxml2-rust/include/libxml/nanohttp.h"
    "/root/code/01/08/libxml2-rust/include/libxml/parser.h"
    "/root/code/01/08/libxml2-rust/include/libxml/parserInternals.h"
    "/root/code/01/08/libxml2-rust/include/libxml/pattern.h"
    "/root/code/01/08/libxml2-rust/include/libxml/relaxng.h"
    "/root/code/01/08/libxml2-rust/include/libxml/SAX.h"
    "/root/code/01/08/libxml2-rust/include/libxml/SAX2.h"
    "/root/code/01/08/libxml2-rust/include/libxml/schemasInternals.h"
    "/root/code/01/08/libxml2-rust/include/libxml/schematron.h"
    "/root/code/01/08/libxml2-rust/include/libxml/threads.h"
    "/root/code/01/08/libxml2-rust/include/libxml/tree.h"
    "/root/code/01/08/libxml2-rust/include/libxml/uri.h"
    "/root/code/01/08/libxml2-rust/include/libxml/valid.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xinclude.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xlink.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlIO.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlautomata.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlerror.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlexports.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlmemory.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlmodule.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlreader.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlregexp.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlsave.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlschemas.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlschemastypes.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlstring.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlunicode.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xmlwriter.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xpath.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xpathInternals.h"
    "/root/code/01/08/libxml2-rust/include/libxml/xpointer.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64" TYPE STATIC_LIBRARY FILES "/root/code/01/08/libxml2-rust/libxml2.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xprogramsx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmlcatalog" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmlcatalog")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmlcatalog"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES "/root/code/01/08/libxml2-rust/xmlcatalog")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmlcatalog" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmlcatalog")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmlcatalog"
         OLD_RPATH "/root/code/01/08/libxml2-rust/rust/target/debug:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmlcatalog")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xprogramsx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmllint" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmllint")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmllint"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES "/root/code/01/08/libxml2-rust/xmllint")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmllint" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmllint")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmllint"
         OLD_RPATH "/root/code/01/08/libxml2-rust/rust/target/debug:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/xmllint")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xruntimex" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10"
         RPATH "")
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/usr/lib64/python3.6/site-packages" TYPE SHARED_LIBRARY FILES "/root/code/01/08/libxml2-rust/libxml2mod.so.2.9.10")
  if(EXISTS "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10"
         OLD_RPATH "/root/code/01/08/libxml2-rust/rust/target/debug:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so.2.9.10")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so"
         RPATH "")
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/usr/lib64/python3.6/site-packages/libxml2mod.so")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/usr/lib64/python3.6/site-packages" TYPE SHARED_LIBRARY FILES "/root/code/01/08/libxml2-rust/libxml2mod.so")
  if(EXISTS "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so"
         OLD_RPATH "/root/code/01/08/libxml2-rust/rust/target/debug:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" "$ENV{DESTDIR}/usr/lib64/python3.6/site-packages/libxml2mod.so")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xruntimex" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/usr/lib64/python3.6/site-packages/drv_libxml2.py")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/usr/lib64/python3.6/site-packages" TYPE FILE FILES "/root/code/01/08/libxml2-rust/python/drv_libxml2.py")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xruntimex" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/usr/lib64/python3.6/site-packages/libxml2.py")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/usr/lib64/python3.6/site-packages" TYPE FILE FILES "/root/code/01/08/libxml2-rust/libxml2.py")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man3" TYPE FILE FILES "/root/code/01/08/libxml2-rust/libxml.3")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man1" TYPE FILE FILES "/root/code/01/08/libxml2-rust/doc/xmlcatalog.1")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man1" TYPE FILE FILES "/root/code/01/08/libxml2-rust/doc/xmllint.1")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdocumentationx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/libxml2" TYPE DIRECTORY FILES "/root/code/01/08/libxml2-rust/doc/" REGEX "/Makefile\\.[^/]*$" EXCLUDE)
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/code/01/08/libxml2-rust/libxml2-config.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/code/01/08/libxml2-rust/libxml2-config-version.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10/libxml2-export.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10/libxml2-export.cmake"
         "/root/code/01/08/libxml2-rust/CMakeFiles/Export/lib64/cmake/libxml2-2.9.10/libxml2-export.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10/libxml2-export-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10/libxml2-export.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/code/01/08/libxml2-rust/CMakeFiles/Export/lib64/cmake/libxml2-2.9.10/libxml2-export.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^()$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/cmake/libxml2-2.9.10" TYPE FILE FILES "/root/code/01/08/libxml2-rust/CMakeFiles/Export/lib64/cmake/libxml2-2.9.10/libxml2-export-noconfig.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/libxml2/libxml" TYPE FILE FILES "/root/code/01/08/libxml2-rust/libxml/xmlversion.h")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64/pkgconfig" TYPE FILE FILES "/root/code/01/08/libxml2-rust/libxml-2.0.pc")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "/root/code/01/08/libxml2-rust/xml2-config")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xdevelopmentx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64" TYPE FILE FILES "/root/code/01/08/libxml2-rust/xml2Conf.sh")
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/root/code/01/08/libxml2-rust/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
