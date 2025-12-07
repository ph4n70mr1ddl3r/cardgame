# Data Model

## Enums

### Suit
- `HEARTS`, `DIAMONDS`, `CLUBS`, `SPADES`

### Rank
- `TWO` (2) ... `TEN` (10), `JACK` (11), `QUEEN` (12), `KING` (13), `ACE` (14)

### GameState
- `WAITING_FOR_PLAYERS`: < 2 players connected/active
- `PREFLOP`: Blinds posted, hole cards dealt
- `FLOP`: 3 community cards dealt
- `TURN`: 4th community card dealt
- `RIVER`: 5th community card dealt
- `SHOWDOWN`: Hands revealed, winner determined
- `HAND_END`: Pot distributed, waiting for next hand

### PlayerStatus
- `ACTIVE`: Connected and playing
- `DISCONNECTED`: Connection lost, grace period active
- `SITTING_OUT`: Grace period expired or manually sat out (folded)

### ActionType
- `FOLD`
- `CHECK`
- `CALL`
- `BET`
- `RAISE`
- `ALL_IN`

## Entities

### Card
- `rank`: Rank
- `suit`: Suit

### Deck
- `cards`: List<Card>
- Methods: `shuffle()`, `draw()`

### Player
- `id`: String (UUID or Name)
- `stack`: Integer (Chips/BB)
- `hole_cards`: List<Card> (2 cards, empty if folded)
- `current_bet`: Integer (Amount bet in current street)
- `status`: PlayerStatus
- `disconnect_timestamp`: Timestamp (nullable, set when connection lost)
- `is_folded`: Boolean

### Pot
- `amount`: Integer
- `contributors`: List<PlayerID> (For handling side pots if we expand, main pot for now)

### Table
- `id`: String
- `players`: Map<Position, Player> (Position 0 (SB) and 1 (BB) for Heads Up)
- `board`: List<Card> (0 to 5 community cards)
- `pot`: Integer (Total pot size)
- `dealer_pos`: Integer (0 or 1)
- `current_turn`: Integer (Position of player to act)
- `state`: GameState
- `min_raise`: Integer
- `big_blind_amount`: Integer (Default 2)
