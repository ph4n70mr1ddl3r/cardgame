#pragma once
#include <iostream>
#include <string>
#include <chrono>
#include <iomanip>
#include <sstream>
#include <nlohmann/json.hpp>

namespace poker {

class Logger {
public:
    static void log(const std::string& msg) {
        log_json("INFO", msg);
    }
    
    static void error(const std::string& msg) {
        log_json("ERROR", msg);
    }

private:
    static void log_json(const std::string& level, const std::string& msg) {
        auto now = std::chrono::system_clock::now();
        auto in_time_t = std::chrono::system_clock::to_time_t(now);
        std::stringstream ss;
        ss << std::put_time(std::localtime(&in_time_t), "%Y-%m-%d %X");
        
        nlohmann::json j;
        j["timestamp"] = ss.str();
        j["level"] = level;
        j["message"] = msg;
        
        if (level == "ERROR") {
            std::cerr << j.dump() << std::endl;
        } else {
            std::cout << j.dump() << std::endl;
        }
    }
};

}
