#pragma once
#include <boost/asio.hpp>
#include <boost/beast.hpp>
#include <string>
#include <memory>
#include "game_manager.hpp"

namespace poker {

class Server {
public:
    Server(boost::asio::io_context& ioc, unsigned short port);
    // run is handled by io_context.run() in main

private:
    boost::asio::ip::tcp::acceptor acceptor_;
    GameManager game_manager_;
    
    void do_accept();
};

} // namespace poker
