#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../assets/wit/floria-plugins.wit",
    with: {
        "floria:plugins/floria/list-resource": super::host::List,
        "floria:plugins/floria/map-resource": super::host::Map,
        "floria:plugins/floria/custom-resource": super::host::Custom,
        "floria:plugins/floria/call-resource": super::host::Call,
    },
    imports: { default: trappable },
});

// Used to be:
// trappable_imports: true,
