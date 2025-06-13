set(ANSX_LIB_PATH ${CMAKE_SOURCE_DIR}/target/release/libansx.a)

# Build ansx staticlib using cargo
add_custom_command(
    OUTPUT ${ANSX_LIB_PATH}
    COMMAND cargo build --package ansx --release
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building ansx static library via cargo"
)
add_custom_target(ansx_lib ALL DEPENDS ${ANSX_LIB_PATH})

add_executable(rzp src/rzp.cpp)

add_dependencies(rzp ansx_lib)

target_include_directories(rzp PRIVATE ${CMAKE_SOURCE_DIR}/ansx/include)

target_link_libraries(rzp PRIVATE ledgerizer ${ANSX_LIB_PATH})

install(TARGETS rzp RUNTIME DESTINATION bin) 