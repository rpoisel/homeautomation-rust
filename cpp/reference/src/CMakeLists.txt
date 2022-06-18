add_subdirectory(common)
add_subdirectory(system)
add_subdirectory(scheduler)
add_subdirectory(components)

add_executable(roof roof.cpp)
target_link_libraries(roof PRIVATE
  components
  scheduler
  CLI11_SINGLE)
if (RASPBERRY)
  target_link_libraries(roof PUBLIC -static)
endif()
if(MSVC)
  target_compile_options(roof PRIVATE /W4 /WX)
else()
  target_compile_options(roof PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()
target_compile_features(roof PRIVATE cxx_std_20)
