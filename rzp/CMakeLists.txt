add_executable(rzp src/rzp.cpp)

target_link_libraries(rzp PRIVATE ledgerizer)

install(TARGETS rzp RUNTIME DESTINATION bin) 