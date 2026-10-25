set_property(DIRECTORY PROPERTY max::external YES)
project(organologic.merge)

target_sources(${PROJECT_NAME} PRIVATE organologic.merge.cpp)
target_include_directories(
    ${PROJECT_NAME}
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../../common
)

if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4 /WX /permissive-)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()
