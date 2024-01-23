use std::env;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {    
    temp_c : f32
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Get the curent weather for a location
    #[arg(short, long)]
    fetch_current: String,

}

impl WeatherResponse {
    async fn fetch_current_weather(location: &String, api_key: &String) -> Result<Self, ExitFailure> {
        const BASE_URL: &str = "http://api.weatherapi.com/v1/current.json";
        let url = Url::parse_with_params(BASE_URL, &[("key", api_key), ("q", location)]).unwrap();   
        let res = reqwest::get(url.as_str()).await?.json::<serde_json::Value>().await?;
        //unwrap current field of res or handle error   
        match res.get("current") {
            
            Some(x) => {
                match x.get("temp_c") {  
                    Some(y) => {
                        let weather_res = WeatherResponse {temp_c: y.as_f64().unwrap() as f32};
                        Ok(weather_res)
                    },
                    None => {
                        // let weather_res = WeatherResponse {temp_c: 0.0};
                        Err(ExitFailure::from(std::io::Error::new(std::io::ErrorKind::Other, "Temperature not found")))
                    }

                }
            },
            None => {
                Err(ExitFailure::from(std::io::Error::new(std::io::ErrorKind::Other, "Location not found")))
            }

        } 

        
    }
    
}


const API_KEY:&'static str = "fa8e4f9240a04e5fa32202525241601";

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    
    let args = Args::parse();
    let res = WeatherResponse::fetch_current_weather(&args.fetch_current,&API_KEY.to_owned()).await?;
    println!("{}'s current temperature: {:}c", &args.fetch_current, res.temp_c);
    Ok(())
}

#[tokio::test]
async fn test_get_current() {
    let res = WeatherResponse::fetch_current_weather(&String::from("nyc"),&API_KEY.to_owned()).await;
    assert!(res.is_ok());
}
