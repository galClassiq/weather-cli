use std::env;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    temp_c : f32,
    forecast: Option<Vec<f32>>
}

impl WeatherResponse {
    async fn get(location: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            location, api_key);

        let url = Url::parse(&*url)?;
        println!("{}", url);
        let res = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        
        let temp = res.get("current").and_then(|x| x.get("temp_c")).and_then(|x| x.as_f64());
        match temp {
            Some(x) => Ok(WeatherResponse{temp_c: x as f32, forecast: None}),
            None => Err(ExitFailure::from(std::io::Error::new(std::io::ErrorKind::Other, "API Error")))
        }
    }
    async fn get_today(location: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=1&aqi=no&alerts=no",
            location, api_key);

        let url = Url::parse(&*url)?;
        // println!("{}", url);
        let res = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        let forcastday = res.get("forecast").and_then(|x| x.get("forecastday"));
        let hours = forcastday.and_then(|x| x[0].get("hour")).unwrap().to_owned();
        if let Some(hours_array) = hours.as_array(){
            let mut temps = vec![];
            for hour in hours_array {
                let temp = hour.get("temp_c").and_then(|x| x.as_f64());
                match temp {
                    Some(x) => temps.push(x as f32),
                    None => continue
                }
            }
            return Ok(WeatherResponse {
                temp_c: 0.0,
                forecast: Some(temps)})
        }
       
        Ok(WeatherResponse {
            temp_c: 0.0,
            forecast: None})

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
    // let res = WeatherResponse::get(&api_key,&location).await?;
    // println!("{}'s current temperature: {:?}", location, res);
    let forecast = WeatherResponse::get_today(&api_key,&location).await?;
    println!("{}'s forecast: {:?}", location, forecast);
    Ok(())
}
