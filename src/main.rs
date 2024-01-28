use rgb::{RGB, RGB8};
use textplots::{Chart, ColorPlot, LabelBuilder, LabelFormat, Plot, Shape};

use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
enum WeatherResponse {
    Current {temp_c : f32},
    Forecast {temp_vec : Vec<(String,f32)>, percipitation_mm_vec : Vec<(String,f32)>},
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
    precip_mm : f32,
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
        let mut temp_vec = vec![];
        let mut percipitation_vec = vec![];

        let _temp = res.forecast.forecastday[0].hour.iter().for_each(|x| temp_vec.push((x.time.to_owned(),x.temp_c)));
        let _percipitation = res.forecast.forecastday[0].hour.iter().for_each(|x: &ForecastHour| percipitation_vec.push((x.time.to_owned(),x.precip_mm)));
        
        Chart::new(180,60,0.0,24.0)
        .linecolorplot(&Shape::Lines(&temp_vec.iter().enumerate().map(|(index ,&(_,temp))| (index as f32,temp)).collect::<Vec<_>>()),RGB8 { r: 255, g: 0, b: 0 })
        .linecolorplot(&Shape::Lines(&percipitation_vec.iter().enumerate().map(|(index ,&(_,precip))| (index as f32,precip)).collect::<Vec<_>>()),RGB8 { r: 0, g: 0, b: 255 })
        .x_label_format(LabelFormat::Custom(Box::new(move |val| {
            format!("{}", val)
        }))).nice();
        Ok(WeatherResponse::Forecast {temp_vec, percipitation_mm_vec: percipitation_vec})
        
    
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

