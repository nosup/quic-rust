# Project Roadmap

> This is a learning project focused on understanding QUIC protocol internals and Rust systems programming. Milestones are flexible and will evolve as I learn.

## Milestone 0: UDP Foundations ✅

**Goal**: Get comfortable with Rust and basic UDP networking

**Completed**:
- UDP echo server and client
- Buffer management and slicing
- Binary data visualization (hex dumps)
- Understanding of Rust ownership and borrowing

**Key concepts**: UDP sockets, byte arrays, slices, mutability, string ↔ byte conversion

---

## Milestone 1: Binary Protocol Basics

**Goal**: Learn binary serialization and packet encoding

### Phase 1: Custom Binary Protocol
Build a simple packet format to understand:
- Binary serialization (struct → bytes)
- Big-endian vs little-endian byte order
- Fixed-width vs variable-length fields
- Error handling for malformed data

**Deliverables**:
- `SimplePacket` with encode/decode methods
- Packet type constants (PING, PONG, MESSAGE, ACK)
- Unit tests for round-trip encoding
- Client/server using binary packets

### Phase 2: Simplified QUIC Headers
Implement a subset of QUIC's packet format:
- Long header parsing (Initial packets only)
- Connection ID handling
- Packet number encoding/decoding
- Wireshark packet inspection

**Note**: No TLS encryption, limited frame types, focusing on understanding over completeness.

---

## Milestone 2: Connection Management

**Goal**: Implement QUIC connection lifecycle

**Deliverables**:
- Connection state machine
- Connection ID management
- Simplified handshake (no TLS initially)
- Multi-connection support

**Key concepts**: State machines, connection routing, packet spaces

---

## Milestone 3: Stream Multiplexing

**Goal**: Implement QUIC's core feature - independent streams

**Deliverables**:
- Stream creation (bidirectional/unidirectional)
- Stream state management
- Stream ID allocation
- Independent data flow per stream

**Demo idea**: Multi-stream chat (control, messages, files on separate streams)

---

## Milestone 4: Reliability & Flow Control

**Goal**: Build reliable delivery on top of UDP

**Deliverables**:
- Packet acknowledgments (ACK frames)
- Loss detection and retransmission
- Stream-level and connection-level flow control
- RTT estimation
- Basic congestion control

**Key concepts**: Reliable transport, flow control, congestion avoidance

---

## Milestone 5: WebTransport Integration

**Goal**: Connect browser to Rust QUIC server

**Deliverables**:
- WebTransport session establishment
- Browser client using native WebTransport API
- Bidirectional streaming from browser
- React visualization dashboard

**Key concepts**: HTTP/3, WebTransport API, browser networking

---

## Milestone 6: Polish & Demos

**Goal**: Create presentable demos and documentation

**Deliverables**:
- Performance benchmarks (QUIC vs TCP)
- Head-of-line blocking demonstration
- Network condition simulator
- Comprehensive documentation
- Video walkthrough

---

## Out of Scope

To keep this project focused on learning, I'm explicitly **not** implementing:
- Full TLS 1.3 encryption (would be 50% of the project)
- All QUIC frame types (focusing on core frames)
- Connection migration
- 0-RTT resumption
- Production-level error handling
- RFC-9000 compliance (simplified for learning)

## Resources

- [RFC 9000: QUIC Transport Protocol](https://datatracker.ietf.org/doc/html/rfc9000)
- [RFC 9002: QUIC Loss Detection](https://datatracker.ietf.org/doc/html/rfc9002)
- [WebTransport Specification](https://w3c.github.io/webtransport/)
- [Quinn](https://github.com/quinn-rs/quinn) - Reference Rust implementation
- [Cloudflare Quiche](https://github.com/cloudflare/quiche) - Another reference
