# Game Protocol (WebSocket/JSON)

Communication is event-driven via WebSockets. All messages are JSON objects.

## Message Structure

Every message has a `type` field.

```json
{
  "type": "MESSAGE_TYPE",
  "payload": { ... }
}
```

## Client -> Server Messages

### 1. Login
Sent immediately after connection to identify the player.
```json
{
  "type": "LOGIN",
  "payload": {
    "player_id": "Bot_Alice"
  }
}
```

### 2. Action
Sent when it is the player's turn (`REQUEST_ACTION` received).
```json
{
  "type": "ACTION",
  "payload": {
    "action": "BET", // FOLD, CHECK, CALL, BET, RAISE
    "amount": 100    // Optional, required for BET/RAISE. Total amount (not increment).
  }
}
```

### 3. Top Up
Request to refill stack to 100BB (only valid between hands or if supported).
```json
{
  "type": "TOP_UP",
  "payload": {}
}
```

## Server -> Client Messages

### 1. Game State Update
Broadcasted whenever public state changes (cards dealt, action made, player joined).
```json
{
  "type": "GAME_STATE",
  "payload": {
    "state": "FLOP",
    "pot": 150,
    "board": [{"rank": "ACE", "suit": "SPADES"}, ...],
    "players": [
      { "id": "Bot_Alice", "stack": 950, "bet": 50, "is_active": true, "has_folded": false },
      { "id": "Bot_Bob", "stack": 1000, "bet": 0, "is_active": true, "has_folded": false }
    ],
    "dealer_pos": 0,
    "current_turn": 1
  }
}
```

### 2. Request Action
Sent specifically to the player whose turn it is.
```json
{
  "type": "REQUEST_ACTION",
  "payload": {
    "valid_actions": ["FOLD", "CALL", "RAISE"],
    "min_raise": 20,
    "current_bet": 50,
    "call_amount": 50
  }
}
```

### 3. Hole Cards
Sent privately to a player when cards are dealt.
```json
{
  "type": "HOLE_CARDS",
  "payload": {
    "cards": [
      {"rank": "TEN", "suit": "HEARTS"},
      {"rank": "NINE", "suit": "HEARTS"}
    ]
  }
}
```

### 4. Error
Sent when an invalid action occurs.
```json
{
  "type": "ERROR",
  "payload": {
    "code": "INVALID_MOVE",
    "message": "Cannot Check when facing a bet."
  }
}
```
