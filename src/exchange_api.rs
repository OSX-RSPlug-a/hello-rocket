use std::collections::HashMap;
use std::io::{Read, Result};
use std::{io::Write, net::TcpStream};

use chrono::{Duration, NaiveDate, Utc};
use log::debug;
use regex::Regex;
use serde_derive::Serialize;

use crate::configuration::{
    BASE_CURRENCY, CRLF, EXCHANGE_API_HOST, MODEL_DATA_QUERY_DURATION, TARGET_CURRENCY,
};
use crate::model::LinearRegression;
use crate::rate_data::{RateTimeSeries, RateTimeSeriesBuilder};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RateResponse {
    pub current_rate: f64,
    pub future_rates: HashMap<String, f64>,
}

impl RateResponse {
    pub fn new(current_rate: f64, future_rates: HashMap<String, f64>) -> Self {
        Self {
            current_rate,
            future_rates,
        }
    }
}

pub fn generate_rate_response(estimation_days: i64) -> RateResponse {
    let latest_rates = fetch_exchange_rates(MODEL_DATA_QUERY_DURATION).unwrap();
    let last_date = *latest_rates.dates.last().unwrap();
    let current_rate = *latest_rates.rates.clone().last().unwrap();
    let target_dates = get_future_dates(last_date, estimation_days);
    let model = LinearRegression::builder(latest_rates);
    let estimated_rates = model.estimate_for(target_dates);

    let mut futures_rates = HashMap::new();
    for (date, rate) in estimated_rates.rates_map {
        futures_rates.insert(format_date(date), rate);
    }
    RateResponse::new(current_rate, futures_rates)
}

pub fn fetch_exchange_rates(duration: i64) -> Result<RateTimeSeries> {
    let mut stream = TcpStream::connect(format!("{EXCHANGE_API_HOST}:80"))?;

    let request_data = generate_request(BASE_CURRENCY, TARGET_CURRENCY, duration);
    stream.write_all(request_data.as_bytes())?;

    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;
    let rates = extract_timeseries(buf)?;
    Ok(rates)
}

fn extract_timeseries(buf: String) -> Result<RateTimeSeries> {
    let body = capture_response_body(buf);
    let rates_builder: RateTimeSeriesBuilder = serde_json::from_str(body.as_str())?;
    let rates: RateTimeSeries = rates_builder.into();
    debug!("Rates: {:?}", rates.rates);
    debug!("Dates: {:?}", rates.dates);

    Ok(rates)
}

fn generate_request(base_currency: &str, target_currency: &str, duration_days: i64) -> String {
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(duration_days);
    let mut request = String::new();
    let date_format = "%Y-%m-%d";
    let formatted_start_date = start_date.format(date_format).to_string();
    let formatted_end_date = end_date.format(date_format).to_string();
    let header = format!("GET /timeseries?base={base_currency}&symbols={target_currency}&start_date={formatted_start_date}&end_date={formatted_end_date} HTTP/1.1");
    request.push_str(header.as_str());
    request.push_str(CRLF);
    request.push_str(&format!("Host: {EXCHANGE_API_HOST}"));
    request.push_str(CRLF);
    request.push_str("Connection: close");
    request.push_str(CRLF);
    request.push_str(CRLF);

    request
}

fn capture_response_body(response: String) -> String {
    let body: Vec<String> = Regex::new(r"(\r\n){2}(\d*\r\n)?")
        .unwrap()
        .split(&response)
        .map(|x| x.to_string())
        .collect();
    let corrected_body = Regex::new(r"\{.*\}")
        .unwrap()
        .captures(body[1].as_str())
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();

    corrected_body.to_string()
}

fn get_future_dates(start_date: NaiveDate, num_days: i64) -> Vec<NaiveDate> {
    let mut target_dates = Vec::new();
    for i in 1..num_days {
        target_dates.push(start_date + Duration::days(i));
    }
    target_dates
}

fn format_date(date: NaiveDate) -> String {
    let date_format = "%Y-%m-%d";
    date.format(date_format).to_string()
}
