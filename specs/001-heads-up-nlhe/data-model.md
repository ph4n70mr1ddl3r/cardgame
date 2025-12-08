# Data Model: Heads Up NLHE

## Core Entities

### Card
*   **Rank**: 2-10, J, Q, K, A
*   **Suit**: Spades, Hearts, Diamonds, Clubs
*   **Representation**: Integer (0-51) or String (e.g., "Ah", "Kd")

### Player
*   **ID**: String (Unique identifier)
*   **Name**: String
*   **Stack**: Integer (Chips in cents or smallest unit)
*   **Status**: Enum { ACTIVE, SITTING_OUT, DISCONNECTED }
*   **Hole Cards**: [Card, Card] (Private)
*   **Current Bet**: Integer (In current street)
*   **Has Folded**: Boolean

### Table
*   **ID**: String
*   **Players**: List[Player] (Max 2)
*   **Button Position**: Integer (Index of dealer)
*   **Pot**: Integer
*   **Community Cards**: List[Card] (0 to 5)
*   **Deck**: List[Card] (Remaining)
*   **Config**: GameConfig

### GameConfig
*   **Small Blind**: Integer
*   **Big Blind**: Integer
*   **Turn Timeout**: Integer (Seconds)
*   **Disconnect Timeout**: Integer (Seconds)

## Game States (Finite State Machine)

1.  **WAITING_FOR_PLAYERS**: < 2 active players.
2.  **STARTING_HAND**: 2 players ready. Posts blinds. Deals cards.
3.  **PRE_FLOP**: Betting round 1.
4.  **FLOP**: Deal 3 community cards. Betting round 2.
5.  **TURN**: Deal 1 community card. Betting round 3.
6.  **RIVER**: Deal 1 community card. Betting round 4.
7.  **SHOWDOWN**: Reveal cards. Evaluate hands. Award pot.
8.  **HAND_END**: Cleanup. Check stacks (Top-up). Rotate button.

## Relationships
*   Server 1--1 Table
*   Table 1--N Player (N<=2)
*   Player 1--1 Connection (Session)