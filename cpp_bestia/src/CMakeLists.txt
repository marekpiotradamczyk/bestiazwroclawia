add_library(uci position.cpp)
target_include_directories(uci PUBLIC ../include)
target_compile_features(uci PUBLIC cxx_std_11)