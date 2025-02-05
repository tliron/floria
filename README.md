*Work in progress, not ready for general use*

Floria
======

A dual-paradigm data system for cloud orchestration.

It consists of a data model (the declarative paradigm) integrated with an event-driven plugin execution mechanism (the imperative paradigm), with first-class support for [Wasm](https://webassembly.org) plugins.

This project comprises:

* Storage backends for graph, relational, and "no-SQL" databases
* CLI tools
* A rich terminal UI
* A rich web UI
* A library for working with Floria data and plugins
* An SDK for building Wasm plugins for Floria

For a Floria-based orchestrator, see [Khutulun](https://khutulun.org).

For a [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) frontend for Floria, see [Puccini](https://puccini.cloud).

Data
----

Floria topologies are graphs of nested vertexes with edges between them. Both vertexes and edges are first-class citizens with custom data and metadata. If vertexes are the bones and muscles, then edges are the connective tissue.

Vertexes can represent software or hardware components at any level from infrastructure to application, as well as logical configurations that exist purely as data (and metadata). Vertexes can be nested within other vertexes recursively.

Edges can represent actual connections, such as network routes, ports, and secure channels, as well as logical dependencies.

Both vertexes and edges can be assigned to any number of "classes", which can be organized hierarchically. Classes can be used to associate metadata, run operations, and apply policies to any number of entities at once.

Additionally, Floria can represent templates for these topologies. Though you can design templates directly in Floria, higher levels of abstraction are possible. For example, you can compile [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) to Floria with [Puccini](https://puccini.cloud).

All the above entities live in nested directories that can be used to organize them hierarchically. Directories function as both namespaces and for access permissions. Note that edges can connect vertexes between different directories.

Floria data is designed to be portable and communicable. Entities can be converted to and from CBOR, MessagePack, JSON, and YAML formats.

Code
----

Floria is not just a data model. It also defines interfaces for interaction with plugins. Plugins are used for event handling, data retrieval and validation, and for topology template instantiation. The latter is an orchestration scalability feature that allows generic templates to be "self-adaptive" to their target clouds, e.g. optimizing for constrained edge sites, choosing components per hardware vendor, etc. This feature is inspired by the [Nephio](https://nephio.org) project.

Though Wasm Component Model plugins get preferential support in Floria, Wasm is not a requirement. Plugins can be implemented in anything that can be executed on your cloud computers.

FAQ
---

### Why is it called "Floria"?

Named after Floria Tosca, the protagonist of Victorien Sardou's play, [*La Tosca*](https://en.wikipedia.org/wiki/La_Tosca). She was written for and first portrayed by actor Sarah Bernhardt.

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/floria/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/floria/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
