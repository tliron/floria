*Work in progress, not ready for general use*

[![crates.io](https://img.shields.io/crates/v/floria?color=%23227700)](https://crates.io/crates/floria)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/floria)

Floria
======

A dual-paradigm data system for cloud orchestration.

It consists of a data model (the declarative paradigm) integrated with an event-driven plugin execution mechanism (the imperative paradigm), with first-class support for [Wasm](https://webassembly.org) plugins.

This project comprises:

* Storage backends for graph, relational, and "no-SQL" databases
* CLI tools
* A rich terminal UI
* A rich web UI with graphical visualization of topologies
* A library for working with Floria data and plugins
* An SDK for building Wasm plugins for Floria

For a Floria-based orchestrator, see [Khutulun](https://khutulun.org).

For a [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) frontend for Floria, see [Puccini](https://puccini.cloud).

Documentation
-------------

* [Design Choices](https://floria.khutulun.org/documentation/design)
* [Frequently Asked Questions](https://floria.khutulun.org/documentation/faq)
* [floria API Documentation](https://docs.rs/floria)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/floria/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/floria/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
