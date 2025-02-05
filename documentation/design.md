[⇐ to main site](https://floria.khutulun.org)

Design Choices
==============

Data
----

Floria is a [graph database](https://en.wikipedia.org/wiki/Graph_database). We start with the assumption that clouds are quintessentially about relationships between entities, from dependencies between infrastructure components, through network connections and secure routes, to the composition of complex multi-sited workloads.

In Floria, graph vertexes and edges are both first-class citizens with custom data and metadata. If vertexes are the bones and muscles, then edges are the connective tissue.

Vertexes can represent software or hardware components at any level from infrastructure to application, as well as logical configurations that exist purely as data (and metadata). Vertexes can be nested within other vertexes recursively.

Edges can represent actual connections, such as network routes, ports, and secure channels, as well as logical dependencies.

Both vertexes and edges can be assigned to any number of "classes", which can be organized hierarchically. Classes can be used to associate metadata, run operations, and apply policies to any number of entities at once.

Additionally, Floria can represent templates for these topologies. Though you can design templates directly in Floria, higher levels of abstraction are possible and useful. For example, you can compile [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) to Floria with [Puccini](https://puccini.cloud).

All the above entities live in nested directories that can be used to organize them hierarchically. Directories function as namespaces and are gated with role-based permissions, which allows a single Floria service to support multiple tenants. Note that edges can connect vertexes between different directories should the permissions allow it.

Floria data is designed to be portable and communicable. Entities can be dumped into and imported from CBOR, MessagePack, JSON, and YAML formats.

Code
----

Floria is not just a data model. It also defines interfaces for interaction with plugins. Plugins are used for event handling, data retrieval and validation, and for template instantiation.

That last feature allows templates to be "self-adaptive" to their target clouds, e.g. you can create an optimized instance for constrained edge sites, choose components per licensed hardware vendor, inject different credentials for staging vs. production environments, etc. The feature is inspired by the [Nephio](https://nephio.org) project.

Though Wasm Component Model plugins get preferential support in Floria, Wasm is not a requirement. Plugins can be implemented in anything that can be executed on your cloud computers.

Both!
-----

Floria property values use an expression language that allows you to embed function calls. Thus, every time you access a property you could be running custom code. This powerful feature elegantly epitomizes Floria's dual-paradigm design.

The expression language is stored in an efficient binary format. Furthermore, it is extensible using custom types to allow special handling in code without resorting to cumbersome pre-processing.
