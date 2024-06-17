include(FetchContent)

FetchContent_Declare(
  googletest
  DOWNLOAD_EXTRACT_TIMESTAMP TRUE
  URL https://github.com/google/googletest/archive/refs/tags/v1.14.0.zip
)

FetchContent_MakeAvailable(googletest)

# Prevent GoogleTest from overriding our compiler/linker options
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

add_executable(tt_test tt_test.cpp)
target_link_libraries(tt_test tt GTest::gmock_main)

add_executable(heur_test heur_test.cpp)
target_link_libraries(heur_test heuristic GTest::gmock_main)

include(GoogleTest)
gtest_discover_tests(tt_test)