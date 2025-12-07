# Research & Technical Decisions

**Feature**: Heads Up NLHE Server & Bot
**Date**: 2025-12-08

## Technical Decisions

### 1. WebSocket Library
**Decision**: `Boost.Beast` (referencing `Boost.Asio`)
**Rationale**: 
- `Boost.Beast` is a robust, HTTP/WebSocket library built on top of `Boost.Asio`, the standard for C++ asynchronous I/O.
- It provides the low-level control needed for handling connection states (grace periods, timeouts) accurately.
- It is widely adopted and ensures long-term maintainability compared to smaller standalone libraries.
**Alternatives Considered**: 
- `uWebSockets`: Extremely performant but has a more unique API style and external dependencies (uSockets). Overkill for a simple 2-player server.
- `websocketpp`: A solid choice, but `Beast` is generally preferred for new projects using modern Boost.

### 2. JSON Library
**Decision**: `nlohmann/json`
**Rationale**:
- Offers the most intuitive "modern C++" API, treating JSON as a first-class data type.
- Simplifies serialization/deserialization logic in the `common` library.
- Performance impact is negligible for a turn-based card game.
**Alternatives Considered**:
- `RapidJSON`: Faster, but the API is more verbose and complex. Not necessary for this scale.

### 3. Testing Framework
**Decision**: `Google Test` (gtest)
**Rationale**:
- The industry standard for C++ unit testing.
- Excellent integration with CMake (via `FetchContent` or installed packages).
- Supports powerful mocking (`gmock`) if needed for testing client/server interactions in isolation.
**Alternatives Considered**:
- `Catch2`: Excellent header-only library, but `gtest` structure fits better with a formal `tests/` directory layout.

### 4. Build System
**Decision**: `CMake`
**Rationale**:
- The universal build system for C++.
- Allows easy dependency management (finding Boost, fetching gtest/json).
