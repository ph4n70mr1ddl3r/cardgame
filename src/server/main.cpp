#include "server.hpp"
#include <iostream>
#include <cstdlib>

int main(int argc, char* argv[]) {
    try {
        unsigned short port = 8080;
        if (argc > 1) {
            port = static_cast<unsigned short>(std::atoi(argv[1]));
        }

        boost::asio::io_context ioc{1};
        poker::Server server(ioc, port);
        
        std::cout << "Poker Server running on port " << port << std::endl;
        
        ioc.run();
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
