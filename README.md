# Flyx Signaling Server

A WebSocket-based signaling server for the Flyx CLI file transfer tool. This server facilitates WebRTC peer-to-peer connection establishment between sender and receiver clients using standardized signaling messages.

## Overview

The Flyx signaling server enables secure peer-to-peer file transfers by coordinating WebRTC signaling between Flyx CLI instances. The server manages rooms where one sender can connect with multiple receivers to exchange offers, answers, and ICE candidates before establishing direct connections.

## Features

- **Automatic Room Creation**: Generates unique 6-character room IDs
- **Role-based Architecture**: First client becomes sender, others become receivers
- **WebRTC Signaling**: Supports offer/answer/ICE candidate exchange
- **Multi-receiver Support**: One sender can coordinate with multiple receivers
- **Client Management**: Automatic cleanup on disconnection

## Message Types

The server handles the following WebRTC signaling messages:

```json
{
  "type": "join",
  "data": { "client_id": "unique-id" }
}

{
  "type": "offer", 
  "data": { "client_id": "sender-id", "sdp": "..." }
}

{
  "type": "answer",
  "data": { "client_id": "receiver-id", "sdp": "..." }
}

{
  "type": "ice_candidate",
  "data": { "client_id": "client-id", "candidate": "..." }
}

{
  "type": "disconnect",
  "data": { "client_id": "client-id" }
}
```

## Quick Start

### Prerequisites

- Rust 1.70+
- Cargo

### Running the Server

```bash
cargo run
```

The server starts on `0.0.0.0:8000` and automatically creates a test room on startup.

### API

#### WebSocket Connection
```
WS /ws/{room_id}
```

Connect to a room via WebSocket. The first client becomes the sender, subsequent clients become receivers with auto-generated UUIDs.

## Architecture

```
┌─────────────┐                 ┌──────────────────┐
│  Flyx CLI   │    WebSocket    │  Signaling       │
│ (Sender)    │ <──────────────>│  Server          │
└─────────────┘                 │                  │
      │                         │  Room: ABC123    │
      │ WebRTC Offer/Answer     │  ├─ Sender       │
      │ ICE Candidates          │  ├─ Receiver 1   │
      ▼                         │  └─ Receiver 2   │
┌─────────────┐    WebSocket    └──────────────────┘
│  Flyx CLI   │ <──────────────────────┘
│(Receiver 1) │
└─────────────┘
      │
      │ Direct P2P Connection
      │ (File Transfer)
      ▼
┌─────────────┐
│  Flyx CLI   │
│ (Sender)    │
└─────────────┘
```

## Project Structure

```
src/
├── main.rs          # Server setup and WebSocket handling
├── app_state.rs     # Application state and room management
├── room.rs          # Room logic and message routing
├── types.rs         # Message types and data structures
└── handler.rs       # Message processing handlers
```

## Core Components

### AppState
- Manages room creation and unique ID generation
- Thread-safe room storage with Arc<Mutex<HashMap>>
- Ensures room ID uniqueness across concurrent access

### Room
- Handles sender/receiver client management
- Routes messages between clients based on role
- Supports broadcast to all receivers and targeted messaging
- Automatic cleanup of disconnected clients

### SignalingMessage
- Enum-based message system with serde serialization
- WebRTC-compliant message types (offer, answer, ICE candidate)
- Client identification and role management

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run with debug output
RUST_LOG=debug cargo run
```

## TODO

### Core Functionality
- [ ] Fix message processing in reader task (undefined `message` variable)
- [ ] Implement proper message routing logic in `received_handle_message_processing`
- [ ] Add client disconnection cleanup in writer task
- [ ] Handle sender disconnection and room state management

### Message Handling
- [ ] Complete WebRTC signaling flow implementation
- [ ] Add message validation and error responses
- [ ] Implement proper offer/answer/ICE candidate routing
- [ ] Add support for multiple concurrent offers

### API Improvements
- [ ] Add REST endpoint for room creation (`POST /room`)
- [ ] Implement room existence validation
- [ ] Add room metadata and status endpoints
- [ ] Support for room configuration (max clients, timeout)

### Error Handling
- [ ] Add comprehensive error handling for WebSocket operations
- [ ] Implement proper error responses for invalid messages
- [ ] Handle malformed JSON and unknown message types
- [ ] Add timeout handling for inactive connections

### Code Quality
- [ ] Remove debug print statements and add structured logging
- [ ] Fix compilation warnings and unused imports
- [ ] Add proper documentation for public APIs
- [ ] Implement comprehensive unit tests

### Features
- [ ] Add heartbeat/keepalive mechanism
- [ ] Implement room expiration and automatic cleanup
- [ ] Add connection limits and rate limiting
- [ ] Support for persistent rooms and reconnection

## Current Status

⚠️ **Work in Progress**: The core architecture is implemented but message processing logic is incomplete. The server can handle WebSocket connections and role assignment but needs completion of the signaling message flow.

## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.