cmake_minimum_required(VERSION 3.18)
project(rbt_ledger_codec VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Options
option(BUILD_TESTS "Build unit tests" ON)

add_subdirectory(ledgerizer)
add_subdirectory(rzp)

if(BUILD_TESTS)
    enable_testing()
    add_subdirectory(ledgerizer/tests)
    add_subdirectory(rzp/tests)
endif() 