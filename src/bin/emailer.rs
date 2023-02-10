use exchanger::configuration::{get_env_var, RATE_THRESHOLD_ENV, DEFAULT_RATE_THRESHOLD, CRONJOB_INTERVAL_DAYS_ENV, DEFAULT_CRONJOB_INTERVAL_DAYS, DAYS_TO_SECONDS};
use exchanger::email_api::send_email;
use log::info;
use std::thread::sleep;
use std::time::{Duration, Instant};



fn main() {
    env_logger::init();
    let rate_threshold: f64 = get_env_var(RATE_THRESHOLD_ENV, DEFAULT_RATE_THRESHOLD);
    let num_days: f64 = get_env_var(CRONJOB_INTERVAL_DAYS_ENV, DEFAULT_CRONJOB_INTERVAL_DAYS);
    info!("Executing cronjob with rate threshold {} with an interval of {} days.", rate_threshold, num_days);    
    
    let interval = Duration::from_secs((num_days * DAYS_TO_SECONDS) as u64);
    let mut next_time = Instant::now() + interval;
    loop {
        info!("Cron job running.");
        send_email(rate_threshold);
        info!("Cron job executed.");
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}