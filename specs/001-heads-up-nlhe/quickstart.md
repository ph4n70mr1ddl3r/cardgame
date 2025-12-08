# Quickstart

## Prerequisites
*   Linux
*   C++20 Compiler (GCC 10+ or Clang 10+)
*   CMake 3.15+
*   Boost 1.70+ (`libboost-system`, `libboost-thread` if needed by older beast)
*   nlohmann-json (likely vendored or system installed)

## Build

```bash
mkdir build && cd build
cmake ..
make -j$(nproc)
```

## Run Server

```bash
# Defaults: port 8080
./bin/poker_server --port 8080
```

## Run Bot Client

```bash
# Start bot 1
./bin/poker_client --server localhost --port 8080 --name bot1

# Start bot 2
./bin/poker_client --server localhost --port 8080 --name bot2
```

## Testing

```bash
cd build
ctest --output-on-failure
```