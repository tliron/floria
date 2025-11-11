[⇐ to main site](https://floria.khutulun.org)

Design Choices
==============

Data
----

Floria is a [graph database](https://en.wikipedia.org/wiki/Graph_database). We start with the assumption that clouds are quintessentially about relationships between entities, from dependencies between infrastructure components, through network connections and secure routes, to the composition of complex multi-sited workloads.

In Floria, graph vertexes and edges are both first-class citizens with custom data and metadata. If vertexes are the bones and muscles, then edges are the connective tissue.

Vertexes can represent software or hardware components at any level from infrastructure to application, as well as logical configurations that exist purely as data (and metadata).

Edges can represent actual connections, such as network routes, ports, and secure channels, as well as logical dependencies.

| Floria allows vertexes to be nested within other vertexes recursively. Notably, this feature is could also be handled via edges, i.e. a "contains" edge between the containing vertex and a contained vertex. However, because this particular kind of relationship is very commonly used for composition and namespacing it is optimized as a feature.

Both vertexes and edges can be assigned to any number of "classes". Classes can be used to associate metadata, send events, and apply policies to any number of entities at once. Classes are intentionally simple, designed for metadata, not data, and do not participate in the graph.

Finally, Floria can represent templates for these topologies. Though you can design templates directly in Floria, higher levels of abstraction are possible and useful. For example, you can compile [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) to Floria templates via [Puccini](https://puccini.cloud).

All the above entities live in nested directories that can be used to organize them hierarchically. Directories function as namespaces and are gated with role-based permissions, which allows a single running Floria service to support multiple tenants. Note that edges can connect vertexes between different directories should the permissions allow it.

Floria data is designed to be portable and communicable. Entities can be dumped into and imported from CBOR, MessagePack, JSON, and YAML formats.

Code
----

Floria is not just a data model. It also defines a plugin interface. Plugins are used for event handling, data retrieval and validation, and for instantiation hooks.

That last feature allows templates to be "self-adaptive" to their target clouds. For example, when instantiating a general-purpose template in a constrained edge site your hooks can create optimized configurations, choose components supported by the hardware vendor, inject different security credentials for the remote environment, etc. (This feature is inspired by the [Nephio](https://nephio.org) project.)

Though Wasm Component Model plugins get preferential support in Floria, Wasm is not a requirement. Plugins can be implemented in anything that can be executed on your cloud computers.

Both!
-----

Floria's data values might look like the familiar JSON-like ["composite primitive schema"](https://github.com/tliron/compris/blob/main/CPS.md), but the groundbreaking difference is that you can embed calls to functions (that are implemented by plugins).

This powerful expression language (we call it "FlorEx": Floria Expressions) is itself an elegant expression of Floria's dual-paradigm design philosophy. Here data and code are united as one. Moreover, the ephemeral character of cloud state is not swept under the carpet: every time you access a property you could be talking to infrastructure management, accessing orchestration systems, reconfiguring components, and finally getting a result contingent on that composite moment. And errors also count as results because clouds are anything but reliable. Failure is to be expected and Floria provides a framework for dealing with it.

FlorEx is representable in an efficient binary format that can be stored in any database and transmitted over the network. Of course you will need Floria to evaluate that data, as it provides the runtime environment for safely loading plugins, dispatching functions, accumulating errors, etc. Data needs code; code needs data.

An Ecosystem
------------

Even though a single function can do all the heavy lifting, FlorEx's design promotes composability. You can use the result of a function call as an argument to another function call, pass a function call itself as a value, and even return function calls from function calls. Because functions can be used together in these ways, individual functions may be designed with such cooperation and reusabality in mind. Thus Floria encourages an ecosystem of libraries.

Indeed that's one role of [Khutulun](https://khutulun.org/). It's not just "an orchestrator" as a system, it's also a repository of Floria plugins that provide composable buildings blocks for *your* orchestration.
