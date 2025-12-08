# Game Protocol (JSON over WebSocket)

## Message Format

All messages are JSON objects.

```json
{
  "type": "MESSAGE_TYPE",
  "payload": { ... }
}
```

## Client -> Server

### LOGIN
Identifies the player.
```json
{
  "type": "LOGIN",
  "payload": {
    "player_id": "bot-1"
  }
}
```

### ACTION
Performs a game action.
```json
{
  "type": "ACTION",
  "payload": {
    "action": "CHECK" | "CALL" | "BET" | "RAISE" | "FOLD",
    "amount": 100 // Optional, required for BET/RAISE. Absolute total bet amount? Or incremental?
                  // Standard is usually "total amount put in pot this round" or "incremental".
                  // Let's specify: Amount to add to the pot (incremental) or Total wager?
                  // For simplicity: Total amount for this street (e.g. Raise to X).
                  // But standard poker is "Raise To".
    "amount": 200 // "Raise to 200"
  }
}
```

## Server -> Client

### GAME_STATE
Sent on any state change.
```json
{
  "type": "GAME_STATE",
  "payload": {
    "state": "PRE_FLOP",
    "pot": 150,
    "community_cards": ["Ah", "Kd", "2s"],
    "players": [
      {
        "id": "bot-1",
        "stack": 9850,
        "bet": 50,
        "active": true,
        "has_folded": false,
        "is_turn": false
      },
      {
        "id": "bot-2",
        "stack": 9900,
        "bet": 100,
        "active": true,
        "has_folded": false,
        "is_turn": true
      }
    ]
  }
}
```

### DEAL_CARDS
Sent privately to each player at start of hand.
```json
{
  "type": "DEAL_CARDS",
  "payload": {
    "cards": ["Th", "Tc"]
  }
}
```

### REQUEST_ACTION
Sent when it is a specific player's turn.
```json
{
  "type": "REQUEST_ACTION",
  "payload": {
    "timeout_ms": 30000,
    "valid_actions": ["CALL", "RAISE", "FOLD"],
    "min_raise": 200
  }
}
```

### ERROR
Sent when an action is invalid.
```json
{
  "type": "ERROR",
  "payload": {
    "code": "INVALID_BET",
    "message": "Bet amount below minimum raise."
  }
}
```