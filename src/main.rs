//extern crate postgres;
use metlink_tracker_lib;
use serde::{Serialize, Deserialize};
use std::fs;
use std::{thread, time};
use geojson::{Feature, GeoJson, Geometry, Value, FeatureCollection};
use geo::Point;
use serde_json::{Map, to_value};
use clap::{App,Arg};


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

   let args = App::new("metlink-tracker")
   .version("0.1").about("fetch gtfs real time data from metlink").arg(Arg::with_name("geojson")
   .help("use geojson format output").takes_value(false).required(false))
   .get_matches();
    let conf = load_config();
    let token = conf.api_key;
    let service = metlink_tracker_lib::fetch_vehicles_v1(token);
    if args.is_present("geojson") {
        //do a map over service making a new geojson geometry and feature
        let mut geometry: Vec<Feature> = Vec::new();
        for vehic in service.iter() {
            let mut properties = Map::new();
            properties.insert(String::from("Vehicle ID"),to_value(vehic.vehicle_id.clone()).unwrap());
            properties.insert(String::from("Schedule"),to_value(vehic.schedule_relation).unwrap());
            properties.insert(String::from("Start_time"),to_value(vehic.start_time.clone()).unwrap());
            properties.insert(String::from("Trip Id"),to_value(vehic.trip_id.clone()).unwrap());
            geometry.push(Feature{
                bbox: None,
                geometry: Some(Geometry::new(Value::Point(vec![vehic.longitude,vehic.latitude]))),
                id: None,
                properties: Some(properties),
                foreign_members: None
            })
        }
        //collect into a vector of geojson features
        let feat_collection = FeatureCollection {
            bbox: None,
            features: geometry,
            foreign_members: None,
        };
        println!("{}",GeoJson::from(feat_collection).to_string());
        //convert to feature collection
    }else{
        println!("route,vehicle,long,lat,bearing");
        for vehicle in service.iter() {
            println!("{},{},{},{},{}",vehicle.trip_id,vehicle.vehicle_id,vehicle.longitude, vehicle.latitude, vehicle.bearing)
        }
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
        .expect("Error while reading the config file");
//convert to a config struct
toml::from_str(contents.as_str()).expect("error parsing toml config")
}
fn pushtoDB(){

}