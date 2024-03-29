add_library(automation STATIC
  blind.cpp)

target_include_directories(automation PUBLIC ${CMAKE_CURRENT_LIST_DIR})

target_link_libraries(automation PUBLIC
  common
  spdlog::spdlog)

add_library(mqtt STATIC
  mqtt.cpp)

target_include_directories(mqtt PUBLIC ${CMAKE_CURRENT_LIST_DIR})

target_link_libraries(mqtt PUBLIC ${CONAN_LIBS})

if (NOT BUILD_RUN_TESTS)
  return ()
endif()

add_executable(blind_state_test blind_state_test.cpp)
target_link_libraries(blind_state_test PRIVATE
  Catch2::Catch2WithMain
  automation)

catch_discover_tests(blind_state_test)

add_executable(blind_test blind_test.cpp)
target_link_libraries(blind_test PRIVATE
  Catch2::Catch2WithMain
  automation)

catch_discover_tests(blind_test)

add_executable(trigger_test trigger_test.cpp)
target_link_libraries(trigger_test PRIVATE
  Catch2::Catch2WithMain
  automation)

catch_discover_tests(trigger_test)
