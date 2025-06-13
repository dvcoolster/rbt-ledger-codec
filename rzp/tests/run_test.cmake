# CMake script to run rzp CLI roundtrip test

# Use the TEST_DIR variable passed from CMake
if(NOT TEST_DIR)
    message(FATAL_ERROR "TEST_DIR not defined")
endif()

# Strip quotes from TEST_DIR if present
string(REGEX REPLACE "^\"(.*)\"$" "\\1" TEST_DIR_CLEAN "${TEST_DIR}")

set(INPUT_PNG "${TEST_DIR_CLEAN}/cat.png")
set(OUTPUT_RBT "${TEST_DIR_CLEAN}/test.rbt")
set(OUTPUT_PNG "${TEST_DIR_CLEAN}/test_out.png")

# Use the RZP_EXE variable passed from CMake
if(NOT RZP_EXE)
    message(FATAL_ERROR "RZP_EXE not defined")
endif()

# Strip quotes from RZP_EXE if present
string(REGEX REPLACE "^\"(.*)\"$" "\\1" RZP_EXE_CLEAN "${RZP_EXE}")



# Check if input file exists
if(NOT EXISTS "${INPUT_PNG}")
    message(FATAL_ERROR "Input PNG file does not exist: ${INPUT_PNG}")
endif()

# Encode PNG to RBT
execute_process(
    COMMAND "${RZP_EXE_CLEAN}" encode "${INPUT_PNG}" "${OUTPUT_RBT}"
    RESULT_VARIABLE ENCODE_RESULT
    OUTPUT_VARIABLE ENCODE_OUTPUT
    ERROR_VARIABLE ENCODE_ERROR
)

if(NOT ENCODE_RESULT EQUAL 0)
    message(FATAL_ERROR "Encode failed with exit code ${ENCODE_RESULT}: ${ENCODE_ERROR}")
endif()

# Decode RBT to PNG
execute_process(
    COMMAND "${RZP_EXE_CLEAN}" decode "${OUTPUT_RBT}" "${OUTPUT_PNG}"
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