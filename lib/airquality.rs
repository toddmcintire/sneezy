use serde_json::Value;

pub struct AirQuality {
    pub co: f64,
    pub o3: f64,
    pub no2: f64,
    pub so2: f64,
    pub pm2_5: f64,
    pub pm10: f64,
    pub us_epa_index: u64,
}

impl AirQuality {
    pub fn get_aq(data: String) -> AirQuality {
        let v: Value = serde_json::from_str(&data).unwrap();
        let co = v["current"]["air_quality"]["co"].as_f64().unwrap();
        let o3 = v["current"]["air_quality"]["o3"].as_f64().unwrap();
        let no2 = v["current"]["air_quality"]["no2"].as_f64().unwrap();
        let so2 = v["current"]["air_quality"]["so2"].as_f64().unwrap();
        let pm2_5 = v["current"]["air_quality"]["pm2_5"].as_f64().unwrap();
        let pm10 = v["current"]["air_quality"]["pm10"].as_f64().unwrap();
        let us_epa_index = v["current"]["air_quality"]["us-epa-index"].as_u64().unwrap();
        AirQuality {
            co,
            o3,
            no2,
            so2,
            pm2_5,
            pm10,
            us_epa_index,
        }
    }
}