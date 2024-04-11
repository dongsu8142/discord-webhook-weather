mod structs;

use crate::structs::{Data, Local};
use chrono::{Datelike, Utc};
use reqwest::Client;
use serenity::builder::{CreateEmbed, ExecuteWebhook};
use serenity::model::Color;
use std::env;
use tzfile::Tz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY").expect("API 키를 찾을 수 없습니다.");
    let city = env::var("CITY").expect("City를 찾을 수 없습니다.");
    let client = Client::new();
    let params = [("q", city), ("appid", api_key.clone())];
    let local = client
        .get("http://api.openweathermap.org/geo/1.0/direct")
        .query(&params)
        .send()
        .await?
        .json::<Vec<Local>>()
        .await?;
    let params = [
        ("lat", local[0].lat.to_string()),
        ("lon", local[0].lon.to_string()),
        ("exclude", "current,minutely,hourly".to_string()),
        ("lang", "kr".to_string()),
        ("units", "metric".to_string()),
        ("appid", api_key),
    ];
    let data = client
        .get("https://api.openweathermap.org/data/3.0/onecall")
        .query(&params)
        .send()
        .await?
        .json::<Data>()
        .await?;
    webhook_send(local, data).await;
    Ok(())
}

async fn webhook_send(local: Vec<Local>, data: Data) {
    let webhook_url = env::var("WEBHOOK_URL").expect("webhook 주소를 찾을 수 없습니다.");
    let tz = Tz::named(data.timezone.as_str()).unwrap();
    let time = Utc::now().with_timezone(&&tz);
    let content = format!(
        "{}년 {}월 {}일의 {} 날씨를 알려드립니다.",
        time.year(),
        time.month(),
        time.day(),
        local[0].name
    );
    let body = ExecuteWebhook::new()
        .username("날씨 알림")
        .avatar_url("https://e7.pngegg.com/pngimages/225/586/png-clipart-react-weather-forecasting-computer-icons-weather-text-weather-forecasting.png")
        .embed(
            CreateEmbed::new()
                .color(Color::from_rgb(153, 204, 255))
                .thumbnail(format!("https://openweathermap.org/img/wn/{}@2x.png", data.daily[0].weather[0].icon))
                .title(content)
                .description(data.daily[0].summary.clone())
                .field("날씨", data.daily[0].weather[0].description.clone(), true)
                .field("온도", format!("{}°C", data.daily[0].temp.eve), true)
                .field("최소/최고 온도", format!("{}°C / {}°C", data.daily[0].temp.min, data.daily[0].temp.max), true)
                .field("체감온도", format!("{}°C", data.daily[0].feels_like.eve), true)
                .field("기압", format!("{}hPa", data.daily[0].pressure), true)
                .field("구름", format!("{}%", data.daily[0].clouds), true)
                .field("습도", format!("{}%", data.daily[0].humidity), true)
                .field("이슬점", format!("{}°C", data.daily[0].dew_point), true)
                .field("강수 확률", format!("{}%", data.daily[0].pop * 100.), true)
                .field("풍속", format!("{}m/s", data.daily[0].wind_speed), true)
                .field("돌풍", format!("{}m/s", data.daily[0].wind_gust), true)
                .field("풍향", data.daily[0].wind_deg.to_string(), true)
                .field("자외선 지수", data.daily[0].uvi.to_string(), true)
                .field("강수량", data.daily[0].get_rain(), true)
                .field("강설량", data.daily[0].get_snow(), true)
        );
    Client::new()
        .post(webhook_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
        .unwrap();
}
