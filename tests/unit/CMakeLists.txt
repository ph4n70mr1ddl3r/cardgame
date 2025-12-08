add_executable(unit_tests
    test_card.cpp
    test_hand_evaluator.cpp
    test_table.cpp
    test_game_manager.cpp
    ../../src/server/table.cpp
    ../../src/server/game_manager.cpp
)
target_link_libraries(unit_tests PRIVATE GTest::gtest_main poker_common)
target_include_directories(unit_tests PRIVATE ../../src/server)
include(GoogleTest)
gtest_discover_tests(unit_tests)