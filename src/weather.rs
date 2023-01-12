pub use std::collections::HashMap;
pub use serde::{Serialize, Deserialize};
pub use std::fmt::{self, Display, Formatter};

// 'daily' object in APIResponse
#[derive(Deserialize, Debug)]
pub struct APIDaily
{
    time: Vec<String>,
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
    sunrise: Vec<String>,
    sunset: Vec<String>,
    precipitation_sum: Vec<f32>,
    windspeed_10m_max: Vec<f32>
}

// format of the weather api's json response
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct APIResponse
{
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f32,
    daily_units: HashMap<String, String>,
    daily: APIDaily
}

// necessary information for each day's weather
#[derive(Clone, Debug)]
pub struct DailyWeather
{
    pub date: String,
    pub max_temp: f32,
    pub min_temp: f32,
    pub sunrise: String,
    pub sunset: String,
    pub precipitation: f32,
    pub max_windspeed: f32
}

// to be able to conver to string and cleanly output
impl Display for DailyWeather
{
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error>
    {
        let date_split: Vec<&str> = self.date.split("-")
            .collect();

        return fmt.write_fmt
        (
            format_args!
            (
                "date: {}\nmax temp: {}\nmin temp: {}\nsunrise: {}\nsunset: {}\nprecipitation: {}\nmax windspeed: {}",
                format!("{}/{}/{}", date_split[1], date_split[2], date_split[0]),
                self.max_temp,
                self.min_temp,
                self.sunrise,
                self.sunset,
                self.precipitation,
                self.max_windspeed
            )
        );
    }
}

// ease of use for converting DailyWeather object to a hashmap
impl From<DailyWeather> for HashMap<String, String>
{
    fn from(daily_weather: DailyWeather) -> HashMap<String, String>
    {
        let weather_string: String = daily_weather.to_string();
        let weather_pairs: Vec<(String, String)> = weather_string.split("\n")
            .collect::< Vec<&str> >()
            .iter()
            .map(|n| n.split(": ").collect::< Vec<&str> >())
            .collect::< Vec< Vec<&str> > >()
            .iter()
            .map(|n| (n[0].to_string(), n[1].to_string()))
            .collect();

        let mut result_map: HashMap<String, String> = HashMap::new();

        for pair in weather_pairs.into_iter()
        {
            result_map.insert(pair.0, pair.1);
        }

        return result_map;
    }
}

// container to hold daily forwcasts
#[derive(Debug)]
pub struct WeeklyForecast
{
    pub days: Vec<DailyWeather>
}

impl Display for WeeklyForecast
{
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error>
    {
        for day in self.days.iter()
        {
            fmt.write_fmt(format_args!("{}\n", day.to_string()))?;
        }

        return Ok(());
    }
}

// for ease of use in converting APIResponse to a weekly forecase
impl From<APIResponse> for WeeklyForecast
{
    fn from(api_response: APIResponse) -> WeeklyForecast
    {
        let mut days: Vec<DailyWeather> = Vec::with_capacity(api_response.daily.time.len());

        for i in 0..api_response.daily.time.len()
        {
            let sunrise: String = api_response.daily.sunrise[i]
                .split("T")
                .map(|n| n.to_string())
                .collect::< Vec<String> >()[1]
                .to_string();

            let sunset: String = api_response.daily.sunset[i]
                .split("T")
                .map(|n| n.to_string())
                .collect::< Vec<String> >()[1]
                .to_string();

            days.push
            (
                DailyWeather
                {
                    date: api_response.daily.time[i].clone(),
                    max_temp: api_response.daily.temperature_2m_max[i],
                    min_temp: api_response.daily.temperature_2m_min[i],
                    sunrise,
                    sunset,
                    precipitation: api_response.daily.precipitation_sum[i],
                    max_windspeed: api_response.daily.windspeed_10m_max[i]
                }
            );
        }

        return WeeklyForecast { days };
    }
}

// generates reqwest request to weather api and returns the response
pub async fn generate_request() -> Result<APIResponse, reqwest::Error>
{
    return Ok(reqwest::get("https://api.open-meteo.com/v1/forecast?latitude=42.64&longitude=-82.96&daily=temperature_2m_max,temperature_2m_min,sunrise,sunset,precipitation_sum,windspeed_10m_max&temperature_unit=fahrenheit&windspeed_unit=mph&precipitation_unit=inch&timezone=America%2FNew_York")
        .await?
        .json::<APIResponse>()
        .await?);
}
