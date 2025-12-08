#pragma once
#include "card.hpp"
#include <vector>

namespace poker {

enum class HandRank {
    HIGH_CARD,
    PAIR,
    TWO_PAIR,
    THREE_OF_A_KIND,
    STRAIGHT,
    FLUSH,
    FULL_HOUSE,
    FOUR_OF_A_KIND,
    STRAIGHT_FLUSH
};

struct HandResult {
    HandRank rank;
    std::vector<Rank> kickers; // Main rank values then kickers
    
    bool operator>(const HandResult& other) const;
    bool operator==(const HandResult& other) const;
    bool operator<(const HandResult& other) const;
};

class HandEvaluator {
public:
    static HandResult evaluate(const std::vector<Card>& hole_cards, const std::vector<Card>& board);
};

}
