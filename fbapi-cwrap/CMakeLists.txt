cmake_minimum_required(VERSION 3.5)

project(fbapi-cwrapper)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pedantic-errors -ferror-limit=1")

file(GLOB sources "fbapi-cwrapper.cpp")

add_library(${PROJECT_NAME} STATIC ${sources})
target_link_libraries(${PROJECT_NAME} "fbclient")