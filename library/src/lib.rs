// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
A dual-paradigm data system for cloud orchestration.

It consists of a data model (the declarative paradigm) integrated with an event-driven plugin
execution mechanism (the imperative paradigm), with first-class support for
[Wasm](https://webassembly.org) plugins.

This project comprises:

* Storage backends for graph, relational, and "no-SQL" databases
* CLI tools
* A rich terminal UI
* A rich web UI with graphical visualization of topologies
* A library for working with Floria data and plugins
* An SDK for building Wasm plugins for Floria

For a Floria-based orchestrator, see [Khutulun](https://khutulun.org).

For a [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) frontend for Floria,
see [Puccini](https://puccini.cloud).

For documentation and usage examples see the
[Floria site](https://floria.khutulun.org).
*/

mod data;
mod entities;
mod store;

/// Plugins.
#[cfg(feature = "plugins")]
pub mod plugins;

#[allow(unused_imports)]
pub use {data::*, entities::*, store::*};
