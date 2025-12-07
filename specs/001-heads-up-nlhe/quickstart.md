# Quickstart

## Prerequisites
- **C++ Compiler**: GCC 10+ or Clang 11+ (C++20 support)
- **CMake**: 3.20+
- **Make** or **Ninja**

## Dependencies (Fetched automatically via CMake)
- Boost (Beast, Asio)
- nlohmann/json
- Google Test

## Building
```bash
mkdir build
cd build
cmake ..
make -j4
```

## Running the Server
```bash
./bin/poker_server --port 8080
```

## Running the Bot Client
```bash
./bin/poker_client --server ws://localhost:8080 --name "Bot_1"
```
(Run a second instance for the second player)
```bash
./bin/poker_client --server ws://localhost:8080 --name "Bot_2"
```

## Testing
```bash
cd build
ctest --output-on-failure
```
