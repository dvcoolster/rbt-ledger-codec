add_executable(ledgerizer_tests test_basic.cpp)

target_link_libraries(ledgerizer_tests PRIVATE ledgerizer)

add_test(NAME ledgerizer_basic COMMAND ledgerizer_tests) 