#compiler definitions
add_compile_definitions(
  DEBUG_MESSAGES
  DEBUG_AST
)

#source files
set(SOURCES
  main.cpp
  ast.cpp
)

#find packages
find_package(BISON REQUIRED)
find_package(FLEX REQUIRED)

if(NOT BISON_FOUND)
  message (FATAL_ERROR "BISON WAS NOT FOUND!!!!!")
else()
  message("bison was found :)")
endif()

if(NOT FLEX_FOUND)
  message (FATAL_ERROR "FLEX WAS NOT FOUND!!!!!")
else()
  message("flex was found :)")
endif()

BISON_TARGET(OCEAN_PARSER ocean.y ${CMAKE_CURRENT_BINARY_DIR}/ocean.tab.cpp COMPILE_FLAGS -Wcounterexamples VERBOSE ${CMAKE_CURRENT_BINARY_DIR}/ocean.output)
FLEX_TARGET(OCEAN_SCANNER ocean.l ${CMAKE_CURRENT_BINARY_DIR}/lex.yy.cpp)
ADD_FLEX_BISON_DEPENDENCY(OCEAN_SCANNER OCEAN_PARSER)

add_executable(ocean
  ${SOURCES}
  ${BISON_OCEAN_PARSER_OUTPUTS}
  ${FLEX_OCEAN_SCANNER_OUTPUTS}
)
