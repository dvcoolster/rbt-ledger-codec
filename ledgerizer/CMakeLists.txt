add_library(ledgerizer STATIC
    src/ledgerizer.cpp
)

target_include_directories(ledgerizer
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

# Dependencies (vendored or system) - none for now
# If fmt or range-v3 are available, you can link them like:
# find_package(fmt REQUIRED)
# target_link_libraries(ledgerizer PUBLIC fmt::fmt)

add_executable(ledgerizer-bin src/main.cpp)

target_link_libraries(ledgerizer-bin PRIVATE ledgerizer)

install(TARGETS ledgerizer EXPORT ledgerizerTargets
        ARCHIVE DESTINATION lib
        LIBRARY DESTINATION lib
        RUNTIME DESTINATION bin)

install(DIRECTORY include/ DESTINATION include)

export(EXPORT ledgerizerTargets FILE ledgerizerTargets.cmake) 