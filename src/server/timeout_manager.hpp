#pragma once
#include <boost/asio.hpp>
#include <functional>
#include <map>
#include <string>
#include <memory>
#include <chrono>

namespace poker {

class TimeoutManager {
public:
    TimeoutManager(boost::asio::io_context& ioc) : ioc_(ioc) {}

    void startTimer(const std::string& key, int seconds, std::function<void()> callback) {
        cancelTimer(key);
        auto timer = std::make_shared<boost::asio::steady_timer>(ioc_, std::chrono::seconds(seconds));
        timers_[key] = timer;
        timer->async_wait([this, key, callback, timer](const boost::system::error_code& ec) {
            if (!ec) {
                callback();
                // Don't erase here immediately if recursive logic, but okay for now
                // Actually erasing from map inside callback might be unsafe if map is modified?
                // std::map iterators are stable.
            }
        });
    }

    void cancelTimer(const std::string& key) {
        auto it = timers_.find(key);
        if (it != timers_.end()) {
            it->second->cancel();
            timers_.erase(it);
        }
    }

private:
    boost::asio::io_context& ioc_;
    std::map<std::string, std::shared_ptr<boost::asio::steady_timer>> timers_;
};

} // namespace poker
