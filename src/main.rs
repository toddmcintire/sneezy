use sneezy::{ airquality, location };

fn main() {
    let mut loc = location::Location::get_location(get_ip());
    let data = location::Location::test_request(&mut loc);

    //let v: Value = serde_json::from_str(&data).unwrap();
    //println!("{}",v);
    //println!("{}", v["current"]["air_quality"]);

    let res = airquality::AirQuality::get_aq(data);
    println!("{}", res.us_epa_index);

}

fn get_ip() -> String {
    let resp = reqwest::blocking::get("https://ifconfig.me").expect("reason").text();
    match resp {
        Ok(res) => res,
        Err(_error) => panic!("error getting ip"),
    }
}