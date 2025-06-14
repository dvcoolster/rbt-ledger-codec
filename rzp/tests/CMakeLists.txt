# Tests for rzp CLI

add_executable(rzp_cli_test cli_roundtrip.cpp)

add_executable(rzp_integrity_test integrity_check.cpp)

target_compile_definitions(rzp_cli_test PRIVATE RZP_BIN_PATH="$<TARGET_FILE:rzp>")

target_link_libraries(rzp_cli_test PRIVATE ledgerizer)

add_test(NAME rzp_cli_roundtrip COMMAND rzp_cli_test)
target_compile_definitions(rzp_integrity_test PRIVATE RZP_BIN_PATH="$<TARGET_FILE:rzp>")
target_link_libraries(rzp_integrity_test PRIVATE ledgerizer)
add_test(NAME rzp_integrity_check COMMAND rzp_integrity_test)

file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/png_1x1.hex "89504E470D0A1A0A0000000D49484452000000010000000108060000001F15C4890000000A49444154789C63600000020001E221BC330000000049454E44AE426082")

# Convert hex to binary file using CMake -E
add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/cat.png
    COMMAND ${CMAKE_COMMAND} -E echo "Converting hex to png"
    COMMAND ${CMAKE_COMMAND} -E rm -f ${CMAKE_CURRENT_BINARY_DIR}/cat.png
    COMMAND python3 ${CMAKE_CURRENT_LIST_DIR}/hex2bin.py ${CMAKE_CURRENT_BINARY_DIR}/png_1x1.hex ${CMAKE_CURRENT_BINARY_DIR}/cat.png
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/png_1x1.hex
)

add_custom_target(cat_png ALL DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/cat.png)

# Test encode/decode roundtrip
add_test(NAME rzp_cli_roundtrip_script
    COMMAND ${CMAKE_COMMAND} -DRZP_EXE=$<TARGET_FILE:rzp> -DTEST_DIR=${CMAKE_CURRENT_BINARY_DIR} -P ${CMAKE_CURRENT_LIST_DIR}/run_test.cmake
) 