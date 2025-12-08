#pragma once
#include <iostream>
#include <string>
#include <chrono>
#include <iomanip>

namespace poker {

class Logger {
public:
    static void log(const std::string& msg) {
        auto now = std::chrono::system_clock::now();
        auto in_time_t = std::chrono::system_clock::to_time_t(now);

        std::cout << std::put_time(std::localtime(&in_time_t), "%Y-%m-%d %X") << " [INFO] " << msg << std::endl;
    }
    
    static void error(const std::string& msg) {
        auto now = std::chrono::system_clock::now();
        auto in_time_t = std::chrono::system_clock::to_time_t(now);

        std::cerr << std::put_time(std::localtime(&in_time_t), "%Y-%m-%d %X") << " [ERROR] " << msg << std::endl;
    }
};

}
