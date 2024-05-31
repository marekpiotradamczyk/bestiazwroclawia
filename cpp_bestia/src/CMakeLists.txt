add_library(uci uci.cpp)
add_library(heuristic heuristic.cpp)
add_library(engine engine.cpp)
add_library(tt tt.cpp)

target_include_directories(uci PUBLIC ../include)
target_include_directories(heuristic PUBLIC ../include)
target_include_directories(engine PUBLIC ../include)
target_include_directories(tt PUBLIC ../include)

target_compile_features(uci PUBLIC cxx_std_20)
target_compile_features(heuristic PUBLIC cxx_std_20)
target_compile_features(engine PUBLIC cxx_std_20)
target_compile_features(tt PUBLIC cxx_std_20)
