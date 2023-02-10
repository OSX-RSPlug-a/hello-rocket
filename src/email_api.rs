use log::info;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::{
    configuration::{
        get_env_var, DEFUALT_NUM_DAYS, SENDGRID_API_KEY_ENV, SENDGRID_API_URI, SOURCE_EMAIL_ENV,
        USER_EMAIL_ENV,
    },
    exchange_api::{generate_rate_response, RateResponse},
};

pub fn send_email(rate_threshold: f64) {
    let api_key: String = get_env_var(SENDGRID_API_KEY_ENV, "invalid_key");
    let source_email: String = get_env_var(SOURCE_EMAIL_ENV, "invalid_email");
    let destination_email: String = get_env_var(USER_EMAIL_ENV, "invalid_email");
    let rate_response = generate_rate_response(5);
    if rate_response.current_rate >= rate_threshold {
        let body = generate_json_string(rate_response, &source_email, &destination_email);
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(SENDGRID_API_URI)
            .header(AUTHORIZATION, format!("Bearer {api_key}"))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(body)
            .send()
            .unwrap();
        info!("Request sent to the email API.");
        let status = response.status();
        info!("Status of the automated email: {:?}", status);
    } else {
        info!("Rate threshold not passed; Email not sent to the user.");
    }
}

fn generate_json_string(rate: RateResponse, source: &str, destination: &str) -> String {
    let latest_rate = rate.current_rate;
    let future_rates: Vec<f64> = rate.future_rates.values().cloned().collect();
    format!(
        "{{
        \"personalizations\": [
          {{
            \"to\": [
              {{
                \"email\": \"{destination}\"
              }}
            ]
          }}
        ],
        \"from\": {{
          \"email\": \"{source}\"
        }},
        \"subject\": \"Automated Exchanger Alert\",
        \"content\": [
          {{
            \"type\": \"text/plain\",
            \"value\": \"Note that the current rate for EUR to INR is {latest_rate}. The rate is estimated to be the following in the upcoming {DEFUALT_NUM_DAYS} days: {:?}.\"
          }}
        ]
      }}",
        future_rates
    )
}
