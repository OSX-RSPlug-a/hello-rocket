use std::env;
use std::fmt::Debug;
use std::str::FromStr;

// Application constants
pub const DAYS_TO_SECONDS: f64 = 24. * 60. * 60.;
pub const CRLF: &str = "\r\n";
pub const WEBSERVER_PORT: i64 = 7878;
pub const MAX_WORKERS_ENV: &str = "MAX_WORKERS";
pub const DEFAULT_MAX_WORKERS: &str = "4";

// Cron job
pub const RATE_THRESHOLD_ENV: &str = "RATE_THRESHOLD";
pub const DEFAULT_RATE_THRESHOLD: &str = "90.0";
pub const CRONJOB_INTERVAL_DAYS_ENV: &str = "EMAILER_INTERVAL";
pub const DEFAULT_CRONJOB_INTERVAL_DAYS: &str = "1.";

// Email API
pub const USER_EMAIL_ENV: &str = "USER_EMAIL";
pub const SOURCE_EMAIL_ENV: &str = "SOURCE_EMAIL";
pub const SENDGRID_API_KEY_ENV: &str = "SENDGRID_API_KEY";
pub const SENDGRID_API_URI: &str = "https://api.sendgrid.com/v3/mail/send";
pub const DEFUALT_NUM_DAYS: i64 = 3;

// Exchange API
pub const EXCHANGE_API_HOST: &str = "api.exchangerate.host";
pub const BASE_CURRENCY: &str = "EUR";
pub const TARGET_CURRENCY: &str = "INR";
pub const MODEL_DATA_QUERY_DURATION: i64 = 15;
pub const SERVER_DATA_QUERY_DURATION: i64 = 4;

pub fn get_env_var<T>(env_var_name: &str, default_value: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let var = env::var(env_var_name).unwrap_or(default_value.to_string());
    var.parse::<T>().unwrap()
}
