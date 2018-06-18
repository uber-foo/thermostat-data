extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate prost_types;

// Include the sensor elements, which are generated from sensor.proto.
include!(concat!(env!("OUT_DIR"), "/sensor.rs"));
include!(concat!(env!("OUT_DIR"), "/thermostat.rs"));
