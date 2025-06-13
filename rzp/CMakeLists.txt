# Set portable path for ansx static library
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)
set(ANSX_LIB_PATH
    "${CARGO_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}ansx${CMAKE_STATIC_LIBRARY_SUFFIX}"
)

message(STATUS "Using ansx static lib: ${ANSX_LIB_PATH}")

# Build ansx staticlib using cargo
add_custom_command(
    OUTPUT ${ANSX_LIB_PATH}
    COMMAND cargo build --package ansx --release
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building ansx static library via cargo"
    BYPRODUCTS ${ANSX_LIB_PATH}
)
add_custom_target(ansx_lib ALL DEPENDS ${ANSX_LIB_PATH})

add_executable(rzp src/rzp.cpp)

add_dependencies(rzp ansx_lib)

target_include_directories(rzp PRIVATE ${CMAKE_SOURCE_DIR}/ansx/include)

if (MSVC)
    target_link_libraries(rzp PRIVATE
        ledgerizer
        ${ANSX_LIB_PATH}
        ws2_32
        userenv
        ntdll
    )
else()
    target_link_libraries(rzp PRIVATE ledgerizer ${ANSX_LIB_PATH})
endif()

install(TARGETS rzp RUNTIME DESTINATION bin) 