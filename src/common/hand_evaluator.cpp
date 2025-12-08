#include "hand_evaluator.hpp"
#include <algorithm>
#include <map>
#include <set>

namespace poker {

bool HandResult::operator>(const HandResult& other) const {
    if (rank != other.rank) {
        return rank > other.rank;
    }
    return kickers > other.kickers;
}

bool HandResult::operator==(const HandResult& other) const {
    return rank == other.rank && kickers == other.kickers;
}

bool HandResult::operator<(const HandResult& other) const {
    return !(*this > other) && !(*this == other);
}

HandResult HandEvaluator::evaluate(const std::vector<Card>& hole_cards, const std::vector<Card>& board) {
    std::vector<Card> all_cards = hole_cards;
    all_cards.insert(all_cards.end(), board.begin(), board.end());

    if (all_cards.empty()) return {HandRank::HIGH_CARD, {}}; // Should not happen

    // Sort by rank descending
    std::sort(all_cards.begin(), all_cards.end(), [](const Card& a, const Card& b) {
        return a.rank > b.rank;
    });

    // Check Flush
    std::map<Suit, std::vector<Card>> suits;
    for (const auto& c : all_cards) suits[c.suit].push_back(c);

    bool flush = false;
    Suit flush_suit = Suit::HEARTS;
    for (const auto& [s, cards] : suits) {
        if (cards.size() >= 5) {
            flush = true;
            flush_suit = s;
            break;
        }
    }

    // Check Straight
    // Helper to find straight in a sorted list of ranks
    auto find_straight = [](const std::vector<Rank>& sorted_ranks) -> std::pair<bool, Rank> {
        if (sorted_ranks.size() < 5) return {false, Rank::TWO};
        
        // Remove duplicates for straight check
        std::vector<Rank> unique_ranks;
        if (!sorted_ranks.empty()) {
            unique_ranks.push_back(sorted_ranks[0]);
            for (size_t i = 1; i < sorted_ranks.size(); ++i) {
                if (sorted_ranks[i] != sorted_ranks[i-1]) {
                    unique_ranks.push_back(sorted_ranks[i]);
                }
            }
        }

        if (unique_ranks.size() < 5) return {false, Rank::TWO};

        // Check normal straights
        for (size_t i = 0; i <= unique_ranks.size() - 5; ++i) {
            if (static_cast<int>(unique_ranks[i]) - static_cast<int>(unique_ranks[i+4]) == 4) {
                return {true, unique_ranks[i]};
            }
        }

        // Check Ace low straight (A 5 4 3 2) -> A is at 0 (14), then we need 5,4,3,2
        // unique_ranks contains 14 ... 5 4 3 2
        bool has_ace = unique_ranks[0] == Rank::ACE;
        bool has_5432 = false;
        if (has_ace) {
             // check if 5,4,3,2 exist
             int count_low = 0;
             for (auto r : unique_ranks) {
                 int val = static_cast<int>(r);
                 if (val >= 2 && val <= 5) count_low++;
             }
             if (count_low == 4) has_5432 = true;
        }

        if (has_ace && has_5432) {
            return {true, Rank::FIVE}; // High card is 5
        }

        return {false, Rank::TWO};
    };

    // Straight Flush
    if (flush) {
        std::vector<Rank> flush_ranks;
        for (const auto& c : suits[flush_suit]) flush_ranks.push_back(c.rank); // already sorted desc
        auto sf_res = find_straight(flush_ranks);
        if (sf_res.first) {
            return {HandRank::STRAIGHT_FLUSH, {sf_res.second}};
        }
    }

    // Count ranks
    std::map<Rank, int> rank_counts;
    for (const auto& c : all_cards) rank_counts[c.rank]++;

    std::vector<Rank> quads, trips, pairs, singles;
    for (auto it = rank_counts.rbegin(); it != rank_counts.rend(); ++it) {
        if (it->second == 4) quads.push_back(it->first);
        else if (it->second == 3) trips.push_back(it->first);
        else if (it->second == 2) pairs.push_back(it->first);
        else singles.push_back(it->first);
    }

    // 4 of a Kind
    if (!quads.empty()) {
        std::vector<Rank> kickers;
        kickers.push_back(quads[0]);
        // Get highest kicker
        for (const auto& c : all_cards) {
            if (c.rank != quads[0]) {
                kickers.push_back(c.rank);
                break;
            }
        }
        return {HandRank::FOUR_OF_A_KIND, kickers};
    }

    // Full House
    if (!trips.empty() && (!pairs.empty() || trips.size() > 1)) {
        std::vector<Rank> kickers;
        kickers.push_back(trips[0]);
        if (trips.size() > 1) {
            kickers.push_back(trips[1]); // e.g. AAA KKK -> Full House A over K
        } else {
            kickers.push_back(pairs[0]);
        }
        return {HandRank::FULL_HOUSE, kickers};
    }

    // Flush
    if (flush) {
        std::vector<Rank> kickers;
        int count = 0;
        for (const auto& c : suits[flush_suit]) {
            kickers.push_back(c.rank);
            if (++count == 5) break;
        }
        return {HandRank::FLUSH, kickers};
    }

    // Straight
    std::vector<Rank> all_ranks;
    for (const auto& c : all_cards) all_ranks.push_back(c.rank);
    auto st_res = find_straight(all_ranks);
    if (st_res.first) {
        return {HandRank::STRAIGHT, {st_res.second}};
    }

    // 3 of a Kind
    if (!trips.empty()) {
        std::vector<Rank> kickers;
        kickers.push_back(trips[0]);
        int count = 0;
        for (const auto& c : all_cards) {
            if (c.rank != trips[0]) {
                kickers.push_back(c.rank);
                if (++count == 2) break;
            }
        }
        return {HandRank::THREE_OF_A_KIND, kickers};
    }

    // Two Pair
    if (pairs.size() >= 2) {
        std::vector<Rank> kickers;
        kickers.push_back(pairs[0]);
        kickers.push_back(pairs[1]);
        int count = 0;
        for (const auto& c : all_cards) {
            if (c.rank != pairs[0] && c.rank != pairs[1]) {
                kickers.push_back(c.rank);
                if (++count == 1) break;
            }
        }
        return {HandRank::TWO_PAIR, kickers};
    }

    // Pair
    if (!pairs.empty()) {
        std::vector<Rank> kickers;
        kickers.push_back(pairs[0]);
        int count = 0;
        for (const auto& c : all_cards) {
            if (c.rank != pairs[0]) {
                kickers.push_back(c.rank);
                if (++count == 3) break;
            }
        }
        return {HandRank::PAIR, kickers};
    }

    // High Card
    std::vector<Rank> kickers;
    int count = 0;
    for (const auto& c : all_cards) {
        kickers.push_back(c.rank);
        if (++count == 5) break;
    }
    return {HandRank::HIGH_CARD, kickers};
}

} // namespace poker
