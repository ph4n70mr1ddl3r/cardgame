#pragma once
#include <boost/asio.hpp>
#include <boost/beast.hpp>
#include "bot_state.hpp"
#include <string>
#include <deque>

namespace poker {

class NetworkClient {
public:
    NetworkClient(boost::asio::io_context& ioc, std::string host, std::string port, std::string player_id);
    void run();

private:
    void on_resolve(boost::beast::error_code ec, boost::asio::ip::tcp::resolver::results_type results);
    void on_connect(boost::beast::error_code ec, boost::asio::ip::tcp::resolver::results_type::endpoint_type ep);
    void on_handshake(boost::beast::error_code ec);
    void do_read();
    void on_read(boost::beast::error_code ec, std::size_t bytes_transferred);
    void on_write(boost::beast::error_code ec, std::size_t bytes_transferred);
    void do_write();

    void processMessage(const std::string& data);
    void send(const std::string& msg);
    void makeMove();
    void on_timer(boost::system::error_code ec);
    
    // Reconnection
    void wait_and_reconnect();
    void on_reconnect_timer(boost::system::error_code ec);

    boost::asio::ip::tcp::resolver resolver_;
    boost::beast::websocket::stream<boost::beast::tcp_stream> ws_;
    boost::beast::flat_buffer buffer_;
    std::string host_;
    std::string port_;
    std::string player_id_;
    std::deque<std::string> write_queue_;
    
    boost::asio::steady_timer timer_;
    boost::asio::steady_timer reconnect_timer_;
    BotState state_;
};

} // namespace poker
