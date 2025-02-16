# Rust API client for Valhalla

This crate contains the types and functions for interacting with the Valhalla API.

These APIs are implemented:
- [x] [Turn-by-Turn Route](https://valhalla.github.io/valhalla/api/turn-by-turn/overview/)
- [ ] [Optimized Route](https://valhalla.github.io/valhalla/api/optimized/api-reference/)
- [x] [Time-Distance Matrix](https://valhalla.github.io/valhalla/api/matrix/api-reference/)
- [ ] [Isochrone & Isodistance](https://valhalla.github.io/valhalla/api/isochrone/api-reference/)
- [ ] [Map Matching](https://valhalla.github.io/valhalla/api/map-matching/api-reference/)
- [ ] [Valhalla locate](https://valhalla.github.io/valhalla/api/locate/api-reference/)
- [x] [Elevation](https://valhalla.github.io/valhalla/api/elevation/api-reference/)
- [ ] [Expansion](https://valhalla.github.io/valhalla/api/expansion/api-reference/)
- [x] [Status](https://valhalla.github.io/valhalla/api/status/api-reference/)

## Features and usage

We provide two clients:
- async: [`valhalla_client::Valhalla`](https://docs.rs/valhalla-client/latest/valhalla_client/struct.Valhalla.html) and
- sync: [`valhalla_client::blocking::Valhalla`](https://docs.rs/valhalla-client/latest/valhalla_client/blocking/struct.Valhalla.html) using the [tokyo runtime](https://tokio.rs/) internally to call the async version

The second one is behind the (default-enabled) `blocking` feature, so if you don't need it, you can disable it via `default-features = false`.

We also offer the (default-enabled) `gpx` feature.
This enables [reading and writing GPX (GPS Exchange Format) files](https://docs.rs/gpx/latest/gpx/) for APIs where we have the needed context.

## Example

```rust
// an async version is available at valhalla_client::Valhalla
use valhalla_client::blocking::Valhalla;
use valhalla_client::route::{Location, Manifest};
use valhalla_client::costing::{Costing};

let valhalla = Valhalla::default();

let amsterdam = Location::new(4.9041, 52.3676);
let utrecht = Location::new(5.1214, 52.0907);
let manifest = Manifest::builder()
    .locations([amsterdam, utrecht])
    .costing(Costing::Motorcycle(Default::default()));

let response = valhalla.route(manifest).unwrap();

println!("{:#?}", response);

// If the gpx feature is enabled, you can convert the response to a gpx::Gpx object
// let gpx = response.trip.into();
```

For further examples, please see the different clients:
- async: [`valhalla_client::Valhalla`](https://docs.rs/valhalla-client/latest/valhalla_client/struct.Valhalla.html) and
- sync: [`valhalla_client::blocking::Valhalla`](https://docs.rs/valhalla-client/latest/valhalla_client/blocking/struct.Valhalla.html)

