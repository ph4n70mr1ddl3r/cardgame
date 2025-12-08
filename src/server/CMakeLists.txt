add_executable(poker_server
    main.cpp
    server.cpp
    game_manager.cpp
    table.cpp
)
target_link_libraries(poker_server PRIVATE 
    poker_common 
    Boost::system 
    Boost::thread
    nlohmann_json::nlohmann_json
)
target_include_directories(poker_server PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
