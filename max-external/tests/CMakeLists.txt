add_executable(note_merge_engine_tests note_merge_engine_tests.cpp)
target_compile_features(note_merge_engine_tests PRIVATE cxx_std_17)
target_include_directories(
    note_merge_engine_tests
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../source/common
)

if(MSVC)
    target_compile_options(note_merge_engine_tests PRIVATE /W4 /WX /permissive-)
else()
    target_compile_options(note_merge_engine_tests PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()

add_test(NAME note_merge_engine_tests COMMAND note_merge_engine_tests)
