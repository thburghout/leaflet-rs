# Leaflet for Rust

High level Rust interface for [Leaflet.js](https://leafletjs.com/).

## About
The interface exported by [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
is wrapped into native rust types for a more Rusty interface.

The goal of this project is to provide a full rust interface to Leaflet.
It will also provide easy to use wrappers for Rust front-end frameworks.
These can be enabled separately through `features`.

## Usage
Right now, the only way to use this is through cargo's git support:

```
[dependencies]
leaflet = { git = "https://gitlab.com/thburghout/leaflet-rs.git"}
```

Or with support for the front-end framework [Yew](https://yew.rs/docs/en/):

```
[dependencies]
leaflet = { git = "https://gitlab.com/thburghout/leaflet-rs.git", features = ["front-end-yew"] }
```

Each supported interface comes with a example which can be directly run and viewed using [Trunk](https://github.com/thedodd/trunk):

- [Rust only](leaflet/examples/bare/src/main.rs)
- [Yew](leaflet/examples/yew/src/main.rs)

## Features

Currently, this project supports a pure Rust interface and Yew. 
Other frameworks will be added in the future. 
Contributions welcome.

| Feature | Bare Rust | Yew |
| ------- | --------- | --- |
| Map |  :white_check_mark: | :white_check_mark: |
| TileLayer | | |
| Marker | :white_check_mark: |  :white_check_mark: |
| Polyline | :white_check_mark: | :white_check_mark: |
