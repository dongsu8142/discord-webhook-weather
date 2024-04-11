use serde::Deserialize;

#[derive(Deserialize)]
pub struct Local {
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Deserialize)]
pub struct Data {
    pub timezone: String,
    // pub current: Current,
    pub daily: Vec<Daily>,
}

// #[derive(Deserialize)]
// pub struct Current {
//     pub temp: f32,
//     pub feels_like: f32,
//     pub pressure: u16,
//     pub humidity: u8,
//     pub dew_point: f32,
//     pub uvi: u8,
//     pub clouds: u8,
//     pub wind_speed: f32,
//     pub wind_deg: u8,
//     pub wind_gust: f32,
//     pub weather: Vec<Weather>,
// }

#[derive(Deserialize)]
pub struct Weather {
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct Daily {
    pub summary: String,
    pub temp: Temp,
    pub feels_like: FeelsLike,
    pub pressure: u16,
    pub humidity: u8,
    pub dew_point: f32,
    pub wind_speed: f32,
    pub wind_gust: f32,
    pub wind_deg: u16,
    pub weather: Vec<Weather>,
    pub clouds: u8,
    pub pop: f32,
    pub uvi: f32,
    pub rain: Option<f32>,
    pub snow: Option<f32>,
}

impl Daily {
    pub fn get_rain(&self) -> String {
        match self.rain {
            Some(rain) => format!("{}mm", rain),
            None => "0mm".to_string()
        }
    }

    pub fn get_snow(&self) -> String {
        match self.snow {
            Some(snow) => format!("{}mm", snow),
            None => "0mm".to_string()
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Temp {
    pub morn: f32,
    pub day: f32,
    pub eve: f32,
    pub night: f32,
    pub min: f32,
    pub max: f32
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FeelsLike {
    pub morn: f32,
    pub day: f32,
    pub eve: f32,
    pub night: f32
}