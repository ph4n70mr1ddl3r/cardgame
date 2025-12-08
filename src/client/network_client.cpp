#include "network_client.hpp"
#include "strategy.hpp"
#include "../common/protocol.hpp"
#include <iostream>
#include <random>

namespace beast = boost::beast;         // from <boost/beast.hpp>
namespace http = beast::http;           // from <boost/beast/http.hpp>
namespace websocket = beast::websocket; // from <boost/beast/websocket.hpp>
namespace net = boost::asio;            // from <boost/asio.hpp>
using tcp = boost::asio::ip::tcp;       // from <boost/asio/ip/tcp.hpp>

namespace poker {

NetworkClient::NetworkClient(net::io_context& ioc, std::string host, std::string port, std::string player_id)
    : resolver_(net::make_strand(ioc)),
      ws_(net::make_strand(ioc)),
      host_(std::move(host)),
      port_(std::move(port)),
      player_id_(std::move(player_id)),
      timer_(ioc),
      state_()
{
    state_.my_id = player_id_;
}

void NetworkClient::run() {
    resolver_.async_resolve(host_, port_,
        beast::bind_front_handler(&NetworkClient::on_resolve, this));
}

void NetworkClient::on_resolve(beast::error_code ec, tcp::resolver::results_type results) {
    if(ec) {
        std::cerr << "Resolve failed: " << ec.message() << std::endl;
        return;
    }
    
    beast::get_lowest_layer(ws_).async_connect(results,
        beast::bind_front_handler(&NetworkClient::on_connect, this));
}

void NetworkClient::on_connect(beast::error_code ec, tcp::resolver::results_type::endpoint_type ep) {
    if(ec) {
        std::cerr << "Connect failed: " << ec.message() << std::endl;
        return;
    }
    
    ws_.set_option(websocket::stream_base::timeout::suggested(beast::role_type::client));
    ws_.async_handshake(host_ + ":" + std::to_string(ep.port()), "/",
        beast::bind_front_handler(&NetworkClient::on_handshake, this));
}

void NetworkClient::on_handshake(beast::error_code ec) {
    if(ec) {
        std::cerr << "Handshake failed: " << ec.message() << std::endl;
        return;
    }
    
    // Send Login
    LoginPayload p;
    p.player_id = player_id_;
    Message m;
    m.type = MSG_LOGIN;
    m.payload = p; 
    json j = m;
    send(j.dump());
    
    do_read();
}

void NetworkClient::do_read() {
    ws_.async_read(buffer_, beast::bind_front_handler(&NetworkClient::on_read, this));
}

void NetworkClient::on_read(beast::error_code ec, std::size_t bytes_transferred) {
    boost::ignore_unused(bytes_transferred);
    if(ec) {
         if (ec != websocket::error::closed) {
             std::cerr << "Read failed: " << ec.message() << std::endl;
         }
         return;
    }
    
    std::string data = beast::buffers_to_string(buffer_.data());
    buffer_.consume(buffer_.size());
    
    processMessage(data);
    do_read();
}

void NetworkClient::processMessage(const std::string& data) {
    try {
        auto j = json::parse(data);
        Message msg = j.get<Message>();
        
        if (msg.type == MSG_GAME_STATE) {
            auto pl = msg.payload.get<GameStatePayload>();
            state_.game_state = pl.state;
            state_.pot = pl.pot;
            state_.board = pl.board;
            state_.current_bet = 0; 
            // Find my stack
            for (const auto& p : pl.players) {
                if (p.id == player_id_) {
                    state_.my_stack = p.stack;
                }
            }
        } else if (msg.type == MSG_HOLE_CARDS) {
            auto pl = msg.payload.get<HoleCardsPayload>();
            state_.my_hole_cards = pl.cards;
            std::cout << "Dealt: " << state_.my_hole_cards[0] << " " << state_.my_hole_cards[1] << std::endl;
        } else if (msg.type == MSG_REQUEST_ACTION) {
            auto pl = msg.payload.get<RequestActionPayload>();
            state_.valid_actions = pl.valid_actions;
            state_.min_raise = pl.min_raise;
            state_.current_bet = pl.current_bet;
            state_.call_amount = pl.call_amount;
            state_.is_my_turn = true;
            
            makeMove();
        } else if (msg.type == MSG_ERROR) {
             auto pl = msg.payload.get<ErrorPayload>();
             std::cerr << "Error from server: " << pl.message << std::endl;
        }
    } catch (std::exception& e) {
        std::cerr << "JSON Error: " << e.what() << std::endl;
    }
}

void NetworkClient::makeMove() {
    // Random delay 1-5s (FR-005)
    static std::random_device rd;
    static std::mt19937 g(rd());
    std::uniform_int_distribution<> dist(1000, 5000);
    
    timer_.expires_after(std::chrono::milliseconds(dist(g)));
    timer_.async_wait(beast::bind_front_handler(&NetworkClient::on_timer, this));
}

void NetworkClient::on_timer(boost::system::error_code ec) {
    if (ec) return;
    
    ActionPayload action = Strategy::decide(state_);
    Message m;
    m.type = MSG_ACTION;
    m.payload = action;
    json j = m;
    send(j.dump());
    
    state_.is_my_turn = false; // Turn done
}

void NetworkClient::send(const std::string& msg) {
    write_queue_.push_back(msg);
    if(write_queue_.size() > 1) return;
    do_write();
}

void NetworkClient::do_write() {
    ws_.async_write(net::buffer(write_queue_.front()),
        beast::bind_front_handler(&NetworkClient::on_write, this));
}

void NetworkClient::on_write(beast::error_code ec, std::size_t bytes_transferred) {
    boost::ignore_unused(bytes_transferred);
    if(ec) {
        std::cerr << "Write failed: " << ec.message() << std::endl;
        return;
    }
    write_queue_.pop_front();
    if(!write_queue_.empty()) do_write();
}

} // namespace poker
