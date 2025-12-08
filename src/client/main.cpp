#include "network_client.hpp"
#include <iostream>
#include <cstdlib>

int main(int argc, char* argv[]) {
    try {
        if (argc != 4) {
            std::cerr << "Usage: poker_client <host> <port> <name>\n";
            return 1;
        }
        std::string host = argv[1];
        std::string port = argv[2];
        std::string name = argv[3];

        boost::asio::io_context ioc;
        poker::NetworkClient client(ioc, host, port, name);
        client.run();
        
        ioc.run();
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}

