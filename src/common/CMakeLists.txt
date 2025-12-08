add_library(poker_common
    card.cpp
    hand_evaluator.cpp
)
target_link_libraries(poker_common PUBLIC nlohmann_json::nlohmann_json)
target_include_directories(poker_common PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})