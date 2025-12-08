#pragma once
#include "types.hpp"
#include <vector>
#include <iostream>

namespace poker {

struct Card {
    Rank rank;
    Suit suit;

    bool operator==(const Card& other) const = default;
};

class Deck {
public:
    Deck();
    void shuffle();
    Card draw();
    bool isEmpty() const;
    void reset();

private:
    std::vector<Card> cards_;
    size_t current_index_;
};

// Stream operator for printing
std::ostream& operator<<(std::ostream& os, const Card& card);

} // namespace poker
