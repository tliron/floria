[⇐ to main site](https://floria.khutulun.org)

Design Choices
==============

Data
----

Floria is a [graph database](https://en.wikipedia.org/wiki/Graph_database). We start with the assumption that clouds are quintessentially about relationships between entities, from dependencies between infrastructure components, through network connections and secure routes, to the composition of complex multi-sited workloads.

In Floria, graph vertexes and edges are both first-class citizens with custom data and metadata.

If vertexes are the bones and muscles, then edges are the connective tissue. Vertexes can represent software or hardware components at any level from infrastructure to application, as well as logical configurations that exist purely as data (and metadata). Edges can represent actual connections, such as network routes, ports, and secure channels, as well as logical dependencies.

| Floria allows vertexes to be nested within other vertexes recursively. Notably, this feature could also be handled via edges, i.e. a "contains" edge between the containing vertex and a contained vertex. However, because this particular kind of relationship is very commonly used for composition and namespacing it is optimized as a feature.

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

Floria's data values might look like the familiar JSON-like ["composite primitive schema"](https://github.com/tliron/compris/blob/main/CPS.md), but the innovative difference is that you can embed calls to functions provided by plugins. We call this expression language "FlorEx" (Floria Expressions).

| Metadata (for vertexes, edges, and classes) can be modified like data, but by design it *cannot* embed function calls. It is thus guaranteed to be literal and portable. This restriction draws the line between data and metadata in Floria.

FlorEx is the most explicit application of Floria's dual-paradigm design philosophy, which is driven by the ephemerality of cloud state. Every time you access a property you might have to talk to an infrastructure management layer, access remote orchestration systems, reconfigure components, all to finally get a result that is ultimately contingent on a loosely composite state. Floria even counts errors as valid results, because clouds are anything but reliable. Failure is to be expected and Floria provides ways for dealing with it.

Data needs tight integration with code—sophisticated, robust code—to do all that heavy lifting. In other words, cloud data is not only stored but also *delivered*. 

FlorEx is representable in an efficient binary format that can be stored in any database and transmitted over the network. Of course you will need Floria to evaluate that data, as it provides the runtime environment for safely loading plugins, dispatching functions, accumulating errors, etc.

An Ecosystem
------------

Even though a single function can perform a complete orchestration workflow, FlorEx's design promotes composability. You can use the result of a function call as an argument to another function call, pass a function call itself as a value, and even return function calls from function calls. Thus Floria encourages an ecosystem: libraries of functions, both generic and specialized, that are designed to work together.

That's one role of [Khutulun](https://khutulun.org). It's not just an orchestrator, it's also a repository of Floria plugins that provide composable buildings blocks for your specific orchestration needs.
