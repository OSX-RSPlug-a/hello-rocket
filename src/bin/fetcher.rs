use std::io::Result;

use chrono::{Duration, NaiveDate};
use exchanger::{exchange_api::fetch_exchange_rates, model::LinearRegression};

fn main() -> Result<()> {
    let latest_rates = fetch_exchange_rates(360)?;
    let last_date = *latest_rates.dates.last().unwrap();
    let target_dates = get_future_dates(last_date, 4);
    let model = LinearRegression::builder(latest_rates);
    let _estimated_rates = model.estimate_for(target_dates);

    Ok(())
}

fn get_future_dates(start_date: NaiveDate, num_days: i64) -> Vec<NaiveDate> {
  let mut target_dates = Vec::new();
  for i in 1..num_days {
    target_dates.push(start_date + Duration::days(i));
  }
  target_dates
}