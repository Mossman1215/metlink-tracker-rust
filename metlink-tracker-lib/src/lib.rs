use std::vec;

pub struct MetlinkVehicle {
    pub report_time: String,
    pub vehicle_id: String,
    pub service_id: String,
    pub longitude: f64, 
    pub latitude: f64,
    pub bearing: u32,
    pub late: bool,
    pub service_direction: String,
    pub destination_id: u32,
    pub delayed_sec: u64,
}

pub fn fetch_vehicles_v1(service: String) -> Vec<MetlinkVehicle>{
    let met_response = reqwest::blocking::get("https://www.metlink.org.nz/api/v1/ServiceLocation/1").unwrap();
    let body = met_response.text().unwrap();
    
    println!("body = {:?}", body);
    let vehicle = MetlinkVehicle {
        report_time: "some time".to_string(),
        vehicle_id: "1".to_string(),
        service_id: "JVL".to_string(),
        longitude: 1.23,
        latitude: 201.765,
        bearing: 100,
        late: true,
        service_direction: "Incoming".to_string(),
        destination_id: 7777,
        delayed_sec: 100,
    };
    vec!(vehicle)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
