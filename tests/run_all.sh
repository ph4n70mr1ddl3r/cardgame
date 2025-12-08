#!/bin/bash

echo "Running Unit Tests..."
# Assuming `ctest` is configured to run unit tests in the build directory
# Change directory to build/ and run ctest
(cd build && ctest --output-on-failure)
if [ $? -ne 0 ]; then
    echo "Unit tests failed!"
    exit 1
fi
echo "Unit Tests Passed."

echo "Running Integration Tests..."
python3 tests/integration/test_core_gameplay.py
if [ $? -ne 0 ]; then
    echo "Core gameplay integration test failed!"
    exit 1
fi

python3 tests/integration/test_disconnect.py
if [ $? -ne 0 ]; then
    echo "Disconnect integration test failed!"
    exit 1
fi

echo "All Tests Passed!"
