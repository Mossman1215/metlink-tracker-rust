//extern crate postgres;
use metlink_tracker_lib;
use serde::{Serialize, Deserialize};
use std::fs;
use std::{thread, time};


//use postgres::{Connection, TlsMode};

fn main() {
    //fetch list of routes from a toml file?
    //fetch postgres connection info from toml
    //fail if toml file is mising
    //fetch data (iterate over list of routes)
    //if no param or param = cli print to cli via table?
    //if param = db
    // check db or crash 
    //  push data to postgres db (retry 3x then crash)
    let conf = load_config();
    let delay = time::Duration::from_millis(500);
    let token = conf.api_key;
    println!("route\tvehicle\tlong\tlat\tbearing");
    let service = metlink_tracker_lib::fetch_vehicles_v1(token);
    thread::sleep(delay);
    for vehicle in service.iter() {
        println!("{}\t{}\t{}\t{}\t{}",vehicle.trip_id,vehicle.vehicle_id,vehicle.longitude, vehicle.latitude, vehicle.bearing)
    }

}

#[derive(Deserialize)]
struct Config{
    hostname: String,
    port: Option<u16>,
    routes: Vec<String>,
    api_key: String,
}
fn load_config() -> Config{
//load file
let contents = fs::read_to_string("./metlink-tracker.toml")
        .expect("Something went wrong reading the config file");
//convert to a config struct
toml::from_str(contents.as_str()).expect("error parsing toml config")
}
fn pushtoDB(){

}