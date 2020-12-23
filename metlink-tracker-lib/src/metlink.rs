
struct MetlinkVehicle {
    reportTime: String,
    vehicleID: String,
    serviceID: String,
    longitude: f64, 
    latitude: f64,
    bearing: u32,
    late: bool,
    serviceDirection: String,
    destinationID: u32,
    delayedSec: u64,
}