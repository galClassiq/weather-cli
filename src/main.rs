use std::env;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {    
    temp_c : f32
}

impl WeatherResponse {
    async fn get(location: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            location, api_key);

        let url = Url::parse(&*url)?;
        println!("{}", url);
        let res = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        //unwrap current field of res or handle error   
        match res.get("current") {
            Some(x) => {
                match _x.get("temp_c") {  
                    Some(y) => {
                        println!("current field found");
                        let weather_res = WeatherResponse {temp_c: y.as_f64().unwrap() as f32};
                        Ok(weather_res)
                    },
                    None => {
                        println!("temp_c field not found");
                        let weather_res = WeatherResponse {temp_c: 0.0};
                        Ok(weather_res)
                    }

                }
            },
            None => {
                println!("current field not found");
                let weather_res = WeatherResponse {temp_c: 0.0};
                Ok(weather_res)
            }

        } 

        
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let api_key = "fa8e4f9240a04e5fa32202525241601".to_string();
    let args: Vec<String> = env::args().collect();
    let mut location = String::from("nyc");
    if args.len() < 2 {
        println!("enter a weather query location");
    } else {
        location = args[1].clone();
    }
    println!("{}",location);
    let res = WeatherResponse::get(&api_key,&location).await?;
    println!("{}'s current temperature: {:?}", location, res);
    println!("{}", location);
    Ok(())
}
