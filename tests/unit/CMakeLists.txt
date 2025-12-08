add_executable(unit_tests
    test_card.cpp
    test_hand_evaluator.cpp
)
target_link_libraries(unit_tests PRIVATE GTest::gtest_main poker_common)
include(GoogleTest)
gtest_discover_tests(unit_tests)