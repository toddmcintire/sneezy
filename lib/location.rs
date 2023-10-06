use geolocation;
use std::fs;

#[derive(Debug)]
pub struct Location {
    pub latitude: String,
    pub longitude: String,
}

impl Location {
    pub fn get_location(address: String) -> Location {
        let info = geolocation::find(&address).unwrap();
        Location {
            latitude: info.latitude,
            longitude: info.longitude,
        }
    }

    pub fn test_request(&mut self) -> String {
        let file = fs::read(".env");
        let buf = match file {
            Ok(v) => v,
            Err(e) => panic!("err{e}"),
        };
        let api_key = match String::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("err{e}"),
        };
        let url = format!(
            "https://api.weatherapi.com/v1/current.json?key={}&q={},{}&aqi=yes",
            api_key, self.latitude, self.longitude
        );

        let resp = reqwest::blocking::get(url).expect("reason").text();
        match resp {
            Ok(res) => res,
            Err(_error) => panic!("error getting api"),
        }
    }
}

