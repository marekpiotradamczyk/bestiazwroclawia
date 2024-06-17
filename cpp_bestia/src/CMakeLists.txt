add_library(uci uci.cpp)
add_library(heuristic heuristic.cpp)
add_library(search search.cpp)

target_include_directories(uci PUBLIC ../include)
target_include_directories(heuristic PUBLIC ../include)
target_include_directories(search PUBLIC ../include)

target_compile_features(uci PUBLIC cxx_std_20)
target_compile_features(heuristic PUBLIC cxx_std_20)
target_compile_features(search PUBLIC cxx_std_20)
