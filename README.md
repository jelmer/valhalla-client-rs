# Rust API client for Valhalla

This crate contains the types and functions for interacting with the Valhalla API.

These APIs are implemented:
- [x] [Turn-by-Turn Route](https://valhalla.github.io/valhalla/api/turn-by-turn/overview/)
- [ ] [Optimized Route](https://valhalla.github.io/valhalla/api/optimized/api-reference/)
- [x] [Time-Distance Matrix](https://valhalla.github.io/valhalla/api/matrix/api-reference/)
- [ ] [Isochrone & Isodistance](https://valhalla.github.io/valhalla/api/isochrone/api-reference/)
- [ ] [Map Matching](https://valhalla.github.io/valhalla/api/map-matching/api-reference/)
- [ ] [Valhalla locate](https://valhalla.github.io/valhalla/api/locate/api-reference/)
- [ ] [Elevation](https://valhalla.github.io/valhalla/api/elevation/api-reference/)
- [ ] [Expansion](https://valhalla.github.io/valhalla/api/expansion/api-reference/)
- [ ] [Status](https://valhalla.github.io/valhalla/api/status/api-reference/)

## Examples

```rust
use valhalla_client::Valhalla;
use valhalla_client::route::{Location, Manifest};
use valhalla_client::costing::{Costing};

let valhalla = Valhalla::default();

let manifest = Manifest::builder()
    .locations([Location::new(4.9041, 52.3676), Location::new(5.1214, 52.0907)])
    .costing(Costing::Bicycle(Default::default()));

let response = valhalla.route(manifest).unwrap();

println!("{:#?}", response);

// If the gpx feature is enabled, you can convert the response to a gpx::Gpx object
// let gpx = response.trip.into();
```
