#include "card.hpp"
#include <algorithm>
#include <random>
#include <stdexcept>

namespace poker {

Deck::Deck() : current_index_(0) {
    reset();
}

void Deck::reset() {
    cards_.clear();
    for (int s = 0; s <= 3; ++s) {
        for (int r = 2; r <= 14; ++r) {
            cards_.push_back({static_cast<Rank>(r), static_cast<Suit>(s)});
        }
    }
    current_index_ = 0;
}

void Deck::shuffle() {
    static std::random_device rd;
    static std::mt19937 g(rd());
    std::shuffle(cards_.begin(), cards_.end(), g);
    current_index_ = 0;
}

Card Deck::draw() {
    if (current_index_ >= cards_.size()) {
        throw std::out_of_range("Deck is empty");
    }
    return cards_[current_index_++];
}

bool Deck::isEmpty() const {
    return current_index_ >= cards_.size();
}

std::ostream& operator<<(std::ostream& os, const Card& card) {
    const char* rankStr[] = {"", "", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"};
    const char* suitStr[] = {"H", "D", "C", "S"}; // Hearts, Diamonds, Clubs, Spades
    
    int r = static_cast<int>(card.rank);
    int s = static_cast<int>(card.suit);
    
    if (r >= 2 && r <= 14) os << rankStr[r];
    if (s >= 0 && s <= 3) os << suitStr[s];
    
    return os;
}

} // namespace poker
