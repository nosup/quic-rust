# QUIC Protocol Implementation (Learning Project)

A simplified QUIC protocol implementation in Rust -- built from scratch to understand modern network protocols and systems programming.

> **Note**: This is an educational project focused on learning fundamentals. NOT a production-ready implementation.

## What is QUIC

QUIC (Quick UDP Internet Connections) is the transport protocol used by HTTP/3. It provides TCP-like reliability over UDP while solving head-of-line blocking through independent stream multiplexing.

## Current Status

**Completed**: Milestone 0: UDP echo between server and client
**In Progress**: Binary packet serialization

[See detailed roadmap](ROADMAP.md)

## What I'm Learning

* **Rust fundamentals**: Ownership, borrowing, lifetimes
* **Network programming**: UDP sockets, binary protocols, packet parsing
* **Protocol design**: Binary serialization, state machines, flow control
* **Systems thinking**: Low-latency networking, memory management

## Quick Start

```bash
# Run server
cargo run --bin server

# Run client (in another terminal)
cargo run --bin client

# Run tests
cargo test
```

## Project Structure
```
src/
├── lib.rs           # Library entry point
└── bin/
    ├── server.rs    # UDP echo server
    └── client.rs    # Interactive UDP client
```

**Coming next**: `src/packet.rs` for binary packet serialization

## Milestones

- [x] **M0**: UDP echo server and client
- [ ] M1: Binary packet protocol + simplified QUIC headers
- [ ] M2: Connection management
- [ ] M3: Stream multiplexing
- [ ] M4: Reliability & flow control
- [ ] M5: WebTransport integration
- [ ] M6: Performance benchmarking

[Detailed milestone breakdown](ROADMAP.md)

## Why This Project?

As a frontend developer working on video streaming, I wanted to understand the protocols that power modern internet infrastructure. Building QUIC from scratch forces a deeper understanding of:
* How bytes become packets become connections
* Why HTTP/3 is faster than HTTP/2
* How video streaming handles packet loss
* Tradeoffs in protocol design

## Resources

* [RFC 9000: QUIC Transport](https://datatracker.ietf.org/doc/html/rfc9000)
* [QUIC Working Group](https://quicwg.org/)
* [The Rust Book](https://doc.rust-lang.org/book/)

## License
MIT