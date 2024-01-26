
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
enum WeatherResponse {
    Current {temp_c : f32},
    Forecast {temp_vec : Vec<(String,f32)>},
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Get the curent weather for a location
    #[arg(short, long)]
    current: Option<String>,

    #[arg(short, long)]
    forecast : Option<String>,


}


#[derive(Deserialize, Debug)]
struct CurrentWeather{
    current: CurrentDetails
}
#[derive(Deserialize, Debug)]
struct CurrentDetails{
    temp_c : f32,
}
#[derive(Deserialize, Debug)]
struct ForecastHour{
    temp_c : f32,
    time : String
}
#[derive(Deserialize, Debug)]
struct ForecastDay{
    hour : Vec<ForecastHour>
}
#[derive(Deserialize, Debug)]
struct ForecastDays{
    forecastday : Vec<ForecastDay>
}
#[derive(Deserialize, Debug)]
struct WeatherForecast {
    forecast: ForecastDays
}

impl WeatherResponse {
    async fn fetch_current_weather(location: &String, api_key: &String) -> Result<Self, ExitFailure> {
        const BASE_URL: &str = "http://api.weatherapi.com/v1/current.json";
        let url = Url::parse_with_params(BASE_URL, &[("key", api_key), ("q", location)]).unwrap();   
        let res = reqwest::get(url.as_str()).await?.json::<CurrentWeather>().await?;
        let temp = res.current.temp_c;
        Ok(WeatherResponse::Current {temp_c: temp})
    }

    async fn fetch_forecast(location: &String, api_key: &String) -> Result<Self, ExitFailure> {
        const BASE_URL: &str = "http://api.weatherapi.com/v1/forecast.json";
        let url = Url::parse_with_params(BASE_URL, &[("key", api_key), ("q", location)]).unwrap(); 
        let res = reqwest::get(url.as_str()).await?.json::<WeatherForecast>().await?;
        let mut tempVec = vec![];
        let temp = res.forecast.forecastday[0].hour.iter().for_each(|x| tempVec.push((x.time.to_owned(),x.temp_c)));
        Ok(WeatherResponse::Forecast {temp_vec: tempVec})
        
    
    }

}


const API_KEY:&'static str = "fa8e4f9240a04e5fa32202525241601";

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    
    let args = Args::parse();
    if let Some(current) = &args.current {
        let res = WeatherResponse::fetch_current_weather(current,&API_KEY.to_owned()).await?;
        println!("{}'s current temperature: {:?}c", current, res);
    }
    if let Some(forecast) = &args.forecast {
        let res = WeatherResponse::fetch_forecast(forecast,&API_KEY.to_owned()).await?;
        println!("{}'s current temperature: {:?}c", forecast, res);
    }   

    Ok(())    
}

#[tokio::test]
async fn test_get_current() {
    let res = WeatherResponse::fetch_current_weather(&String::from("nyc"),&API_KEY.to_owned()).await;
    assert!(res.is_ok());
}

