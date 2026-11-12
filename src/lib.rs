// Copyright 2026 Sigma Prime Pty Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Implementation of the [Waggle](https://github.com/libp2p/specs/pull/732) protocol.
//!
//! Waggle is a P2P pubsub protocol designed as a successor to Gossipsub, focused on
//! the dissemination of incrementally reconstructed objects. Objects are split into
//! pieces that peers progressively exchange until they converge on the complete content.
//!
//! # Design Highlights
//!
//! - **Per-topic streams** — Each topic uses dedicated streams. Subscribing opens a stream;
//!   unsubscribing closes it.
//! - **Application-driven dissemination** — Subscribing to a topic hands the application one
//!   receiver for the pieces all peers send on it. Each remote subscription hands it a sender for
//!   one peer and topic. The application decides which pieces go to which peers.
//! - **Application-defined scoring** — Peer scoring is left entirely to the application. The
//!   protocol surfaces relevant events but does not bake in scoring.
//! - **Opaque identifiers** — Topics, objects, and piece metadata are opaque to the protocol;
//!   semantics are defined by the application.
//!
//! # Using Waggle
//!
//! See [`Config`](config::Config) for configuration and
//! [`Behaviour`](behaviour::Behaviour) for the
//! [`NetworkBehaviour`](libp2p::swarm::NetworkBehaviour) implementation.

mod protocol;

pub(crate) mod substream;

pub mod behaviour;
pub mod config;
pub mod handler;
