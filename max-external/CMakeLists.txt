cmake_minimum_required(VERSION 3.25)

include(FetchContent)

FetchContent_Declare(
    max-sdk-base
    GIT_REPOSITORY https://github.com/Cycling74/max-sdk-base.git
    GIT_TAG c03a2922a2a8ff149165a1e2ea134e9321d6e202
    GIT_SHALLOW FALSE
)
FetchContent_MakeAvailable(max-sdk-base)

project(OrganologicMerge LANGUAGES C CXX)

enable_testing()

add_subdirectory(source/projects/organologic.merge)
add_subdirectory(tests)
