use serde_json::{Result, Value};

pub enum VehicleMethod{
    Bus,
    Train,
    Other
}
pub struct MetlinkVehicle {
    pub last_modified: String,
    pub report_time: String,
    pub vehicle_id: String,
    pub service_id: String,
    pub longitude: String, 
    pub latitude: String,
    pub bearing: String,
    pub late: bool,
    pub service_direction: String,
    pub destination_id: String,
    pub delayed_sec: i64,
    pub method: VehicleMethod,
}

pub fn fetch_vehicles_v1(route: String) -> Vec<MetlinkVehicle>{
    let url = String::from(format!("{}{}","https://www.metlink.org.nz/api/v1/ServiceLocation/",route.as_str()));
    let met_response = reqwest::blocking::get(&url).unwrap();
    let body = met_response.text().unwrap();

    let v: Value = serde_json::from_str(&body).unwrap();
    let timestamp = v["LastModified"].as_str().unwrap();
    //if v["Services"] > 0
    let services = v["Services"].as_array().unwrap();
    let mut vehicles: Vec<MetlinkVehicle> =  Vec::new();
    if services.len() > 0 {
        for vehicle_json in services.iter(){
            let service_json = vehicle_json["Service"].as_object().unwrap();
            let method_str = service_json["Mode"].as_str().unwrap();
            let method = match method_str {
                "Bus" => VehicleMethod::Bus,
                "Train" => VehicleMethod::Train,
                _ => VehicleMethod::Other,
            };
            vehicles.push(MetlinkVehicle {
                last_modified: String::from(timestamp),
                report_time: String::from(vehicle_json["RecordedAtTime"].as_str().unwrap()),
                vehicle_id: String::from(vehicle_json["VehicleRef"].as_str().unwrap()),
                service_id: String::from(vehicle_json["ServiceID"].as_str().unwrap()),
                longitude: String::from(vehicle_json["Long"].as_str().unwrap()),
                latitude: String::from(vehicle_json["Lat"].as_str().unwrap()),
                bearing: String::from(vehicle_json["Bearing"].as_str().unwrap()),
                late: vehicle_json["BehindSchedule"].as_bool().unwrap(),
                service_direction: String::from(vehicle_json["Bearing"].as_str().unwrap()),
                destination_id: String::from(vehicle_json["DestinationStopID"].as_str().unwrap()),
                delayed_sec: vehicle_json["DelaySeconds"].as_i64().unwrap(),
                method: method,
            });
        }
    }
    
    vehicles
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
