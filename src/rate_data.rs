use std::collections::HashMap;

use chrono::NaiveDate;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct InrRateDataPoint {
    pub inr: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RateTimeSeriesBuilder {
    pub rates: HashMap<String, InrRateDataPoint>,
}

#[derive(Debug)]
pub struct RateTimeSeries {
    pub rates_map: HashMap<NaiveDate, f64>,
    pub rates: Vec<f64>,
    pub dates: Vec<NaiveDate>,
}

impl RateTimeSeries {
    pub fn new(rates: Vec<f64>, dates: Vec<NaiveDate>, rates_map: HashMap<NaiveDate, f64>) -> Self {
        RateTimeSeries {
            rates_map,
            rates,
            dates,
        }
    }
}

impl From<RateTimeSeriesBuilder> for RateTimeSeries {
    fn from(rate_timeseries: RateTimeSeriesBuilder) -> Self {
        let mut rate_data = HashMap::new();
        let mut rates = Vec::new();
        let mut dates: Vec<NaiveDate> = rate_timeseries
            .rates
            .keys()
            .map(|date| NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").unwrap())
            .collect();
        dates.sort_by(|a, b| a.cmp(b));
        for date in dates.iter() {
            let date_format = "%Y-%m-%d";
            let rate = rate_timeseries
                .rates
                .get(&date.format(date_format).to_string())
                .unwrap();
            rates.push(rate.inr);
            rate_data.insert(*date, rate.inr);
        }
        RateTimeSeries::new(rates, dates, rate_data)
    }
}
