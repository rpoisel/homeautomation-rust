add_library(system STATIC
  i2c_bus.cpp
  signal.cpp)
target_include_directories(system PUBLIC ${CMAKE_CURRENT_LIST_DIR})
target_link_libraries(system PRIVATE
  common
  spdlog::spdlog)
if(MSVC)
  target_compile_options(system PRIVATE /W4 /WX)
else()
  target_compile_options(system PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()
target_compile_features(system PRIVATE cxx_std_20)

