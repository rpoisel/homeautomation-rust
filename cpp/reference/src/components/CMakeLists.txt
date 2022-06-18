add_library(components STATIC
  blind.cpp)

target_include_directories(components PUBLIC ${CMAKE_CURRENT_LIST_DIR})

target_link_libraries(components PUBLIC
  common
  spdlog::spdlog)

if (NOT BUILD_RUN_TESTS)
  return ()
endif()

add_executable(blind_state_test blind_state_test.cpp)
target_link_libraries(blind_state_test PRIVATE
  Catch2::Catch2WithMain
  components)

catch_discover_tests(blind_state_test)

add_executable(blind_test blind_test.cpp)
target_link_libraries(blind_test PRIVATE
  Catch2::Catch2WithMain
  components)

catch_discover_tests(blind_test)

add_executable(trigger_test trigger_test.cpp)
target_link_libraries(trigger_test PRIVATE
  Catch2::Catch2WithMain
  components)

catch_discover_tests(trigger_test)
