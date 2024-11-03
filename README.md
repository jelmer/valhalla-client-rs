# Rust API client for Valhalla

This crate contains the types and functions for interacting with the Valhalla API.

At the moment, only the routing API is implemented.

## Examples

```rust
use valhalla_client::Valhalla;
use valhalla_client::route::{Location, Manifest};
use valhalla_client::route::costing::{Costing};

let valhalla = Valhalla::default();

let manifest = Manifest {
    locations: vec![Location::new(4.9041, 52.3676), Location::new(5.1214, 52.0907)],
    ..Default::default()
}
.costing(Costing::Bicycle(Default::default()));

let response = valhalla.route(manifest).unwrap();

println!("{:#?}", response);

// If the gpx feature is enabled, you can convert the response to a gpx::Gpx object
// let gpx = response.trip.into();
```
