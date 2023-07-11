# Rust API client for Valhalla

This crate contains the types and functions for interacting with the Valhalla API.

At the moment, only the routing API is implemented.

## Examples

```rust
use valhalla::Valhalla;
let valhalla = Valhalla::default();

let manifest = valhalla::Manifest {
    locations: vec![valhalla::Location::new(52.3676, 4.9041), Point::new(52.0907, 5.1214)],
    costing: valhalla::Costing::Bicycle,
};

let response = valhalla.route(manifest).unwrap();

println!("{:#?}", response);

let gpx = response.into();
```
