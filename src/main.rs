//extern crate postgres;
use metlink_tracker_lib;
use serde::{Serialize, Deserialize};
use std::fs;
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
    println!("route\tvehicle\tlong\tlat\tbearing");
    if conf.routes.len()>0 {
        for route in conf.routes.iter(){
            let service = metlink_tracker_lib::fetch_vehicles_v1(String::from(route));
            for vehicle in service.iter() {
                println!("{}\t{}\t{}\t{}\t{}",vehicle.service_id,vehicle.vehicle_id,vehicle.longitude, vehicle.latitude, vehicle.bearing)
            }
        }
    }
}

#[derive(Deserialize)]
struct Config{
    hostname: String,
    port: Option<u16>,
    routes: Vec<String>,
}
fn load_config() -> Config{
//load file
let contents = fs::read_to_string("./metlink-tracker.toml")
        .expect("Something went wrong reading the file");
//convert to a config struct
toml::from_str(contents.as_str()).expect("error parsing toml config")
}
fn pushtoDB(){

}