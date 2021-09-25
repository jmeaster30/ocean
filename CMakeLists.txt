cmake_minimum_required(VERSION 3.19)

#set project name
project(ocean VERSION 1.0)

set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set (CMAKE_BUILD_TYPE RelWithDebInfo)


add_subdirectory(src)
