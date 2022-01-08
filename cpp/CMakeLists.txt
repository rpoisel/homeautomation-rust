cmake_policy(SET CMP0048 NEW)
project(homeautomation-cpp VERSION 0.1.0 LANGUAGES CXX)
cmake_minimum_required(VERSION 3.11)

add_executable(main main.cpp)

target_include_directories(main PRIVATE
  ${CMAKE_CURRENT_LIST_DIR}/libmodbuspp/include
  /usr/include/modbus
)

find_library(modbuspp modbuspp PATHS ${CMAKE_CURRENT_LIST_DIR}/build/libmodbuspp/lib NO_DEFAULT_PATH)
target_link_libraries(main ${modbuspp} modbus pthread)
