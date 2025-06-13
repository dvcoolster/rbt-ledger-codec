# CMake script to run rzp CLI roundtrip test

set(INPUT_PNG "${CMAKE_CURRENT_BINARY_DIR}/cat.png")
set(OUTPUT_RBT "${CMAKE_CURRENT_BINARY_DIR}/test.rbt")
set(OUTPUT_PNG "${CMAKE_CURRENT_BINARY_DIR}/test_out.png")

# Encode PNG to RBT
execute_process(
    COMMAND rzp encode "${INPUT_PNG}" "${OUTPUT_RBT}"
    RESULT_VARIABLE ENCODE_RESULT
    OUTPUT_VARIABLE ENCODE_OUTPUT
    ERROR_VARIABLE ENCODE_ERROR
)

if(NOT ENCODE_RESULT EQUAL 0)
    message(FATAL_ERROR "Encode failed: ${ENCODE_ERROR}")
endif()

# Decode RBT to PNG
execute_process(
    COMMAND rzp decode "${OUTPUT_RBT}" "${OUTPUT_PNG}"
    RESULT_VARIABLE DECODE_RESULT
    OUTPUT_VARIABLE DECODE_OUTPUT
    ERROR_VARIABLE DECODE_ERROR
)

if(NOT DECODE_RESULT EQUAL 0)
    message(FATAL_ERROR "Decode failed: ${DECODE_ERROR}")
endif()

# Compare files
execute_process(
    COMMAND ${CMAKE_COMMAND} -E compare_files "${INPUT_PNG}" "${OUTPUT_PNG}"
    RESULT_VARIABLE COMPARE_RESULT
)

if(NOT COMPARE_RESULT EQUAL 0)
    message(FATAL_ERROR "Files are not identical")
endif()

message("Roundtrip test passed successfully") 