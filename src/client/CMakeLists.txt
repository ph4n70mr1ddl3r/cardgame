add_executable(poker_client
    main.cpp
    network_client.cpp
)
target_link_libraries(poker_client PRIVATE 
    poker_common 
    Boost::system 
    Boost::thread
    nlohmann_json::nlohmann_json
)
target_include_directories(poker_client PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
