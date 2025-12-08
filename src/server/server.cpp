#include "server.hpp"
#include "player.hpp"
#include <iostream>
#include <deque>

namespace beast = boost::beast;         // from <boost/beast.hpp>
namespace http = beast::http;           // from <boost/beast/http.hpp>
namespace websocket = beast::websocket; // from <boost/beast/websocket.hpp>
namespace net = boost::asio;            // from <boost/asio.hpp>
using tcp = boost::asio::ip::tcp;       // from <boost/asio/ip/tcp.hpp>

namespace poker {

// WebSocket Session
class WebSocketSession : public GameSession, public std::enable_shared_from_this<WebSocketSession> {
    websocket::stream<beast::tcp_stream> ws_;
    beast::flat_buffer buffer_;
    GameManager& game_manager_;
    std::string player_id_;
    std::deque<std::string> write_queue_;

public:
    WebSocketSession(tcp::socket&& socket, GameManager& gm)
        : ws_(std::move(socket)), game_manager_(gm) {}

    void run() {
        net::dispatch(ws_.get_executor(),
            beast::bind_front_handler(&WebSocketSession::on_run, shared_from_this()));
    }

    void on_run() {
        ws_.set_option(websocket::stream_base::timeout::suggested(beast::role_type::server));
        ws_.async_accept(beast::bind_front_handler(&WebSocketSession::on_accept, shared_from_this()));
    }

    void on_accept(beast::error_code ec) {
        if(ec) return fail(ec, "accept");
        do_read();
    }

    void do_read() {
        ws_.async_read(buffer_, beast::bind_front_handler(&WebSocketSession::on_read, shared_from_this()));
    }

    void on_read(beast::error_code ec, std::size_t bytes_transferred) {
        boost::ignore_unused(bytes_transferred);
        if(ec == websocket::error::closed) return;
        if(ec) return fail(ec, "read");

        // Process message
        std::string data = beast::buffers_to_string(buffer_.data());
        buffer_.consume(buffer_.size());

        try {
            auto j = json::parse(data);
            Message msg = j.get<Message>();
            
            if (msg.type == MSG_LOGIN) {
                LoginPayload p = msg.payload.get<LoginPayload>();
                player_id_ = p.player_id;
                
                auto player = std::make_shared<Player>(player_id_, shared_from_this());
                game_manager_.onPlayerJoin(player);
            } else if (msg.type == MSG_ACTION) {
                ActionPayload p = msg.payload.get<ActionPayload>();
                game_manager_.onPlayerAction(player_id_, p);
            } else if (msg.type == MSG_TOP_UP) {
                game_manager_.onPlayerTopUp(player_id_);
            }
        } catch (const std::exception& e) {
            std::cerr << "JSON Error: " << e.what() << std::endl;
        }

        do_read();
    }

    void send(const std::string& message) override {
        net::post(ws_.get_executor(), beast::bind_front_handler(&WebSocketSession::on_send, shared_from_this(), message));
    }
    
    void on_send(std::string message) {
        write_queue_.push_back(message);
        if(write_queue_.size() > 1) return; // Already writing
        do_write();
    }

    void do_write() {
        ws_.async_write(net::buffer(write_queue_.front()), 
            beast::bind_front_handler(&WebSocketSession::on_write, shared_from_this()));
    }

    void on_write(beast::error_code ec, std::size_t bytes_transferred) {
        boost::ignore_unused(bytes_transferred);
        if(ec) return fail(ec, "write");
        write_queue_.pop_front();
        if(!write_queue_.empty()) do_write();
    }

    void close() override {
        // ws_.async_close(websocket::close_code::normal, ...);
    }

    void fail(beast::error_code ec, char const* what) {
        std::cerr << what << ": " << ec.message() << "\n";
        if (!player_id_.empty()) {
            game_manager_.onPlayerDisconnect(player_id_);
        }
    }
};

Server::Server(net::io_context& ioc, unsigned short port)
    : acceptor_(ioc, {tcp::v4(), port})
{
    // Setup callbacks
    game_manager_.sendToPlayer = [this](const std::string& pid, const Message& msg) {
        auto p = game_manager_.table.getPlayer(pid);
        if (p) {
            if (auto s = p->session.lock()) {
                json j = msg;
                s->send(j.dump());
            }
        }
    };
    
    game_manager_.broadcast = [this](const Message& msg) {
        json j = msg;
        std::string s_msg = j.dump();
        for (auto& p : game_manager_.table.seats) {
            if (p) {
                if (auto s = p->session.lock()) {
                    s->send(s_msg);
                }
            }
        }
    };

    do_accept();
}

void Server::do_accept() {
    acceptor_.async_accept(net::make_strand(acceptor_.get_executor()),
        [this](beast::error_code ec, tcp::socket socket) {
            if (!ec) {
                std::make_shared<WebSocketSession>(std::move(socket), game_manager_)->run();
            }
            do_accept();
        });
}

} // namespace poker
