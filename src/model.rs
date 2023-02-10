use std::collections::HashMap;

use chrono::NaiveDate;
use log::debug;

use crate::rate_data::RateTimeSeries;

pub struct LinearRegression {
    pub rate_data: RateTimeSeries,
    pub features: Vec<Vec<f64>>,
    pub dependent_var: Vec<f64>,

}

impl LinearRegression {
    pub fn builder(rate_data: RateTimeSeries) -> Self {
        let timestamps: Vec<f64> = rate_data
            .dates
            .iter()
            .map(|date| convert_to_timestamp(*date))
            .collect();
        // let mean_timestamp: f64 = timestamps.iter().sum() / timestamps.len();
        let features = process_feature(&timestamps);
        let rates = rate_data.rates.clone();
        LinearRegression {
            rate_data,
            features,
            dependent_var: rates,
        }
    }

    pub fn estimate_for(&self, target_dates: Vec<NaiveDate>) -> RateTimeSeries {
        let initial_theta = vec![0., 0.];
        let learning_rate = 0.1;
        let max_iterations = 1000;
        let min_tolerance = 1e-3;
        let updated_theta = Self::gradient_descent(
            &self.features,
            &self.dependent_var,
            &initial_theta,
            learning_rate,
            max_iterations,
            min_tolerance,
        );

        let target_timeseries = target_dates
            .iter()
            .map(|date| convert_to_timestamp(*date))
            .collect();
        let target_feature = process_feature(&target_timeseries);
        let estimated_rates = Self::estimate_rates(&updated_theta, &target_feature);
        debug!("Estimated Rates: {:?}", estimated_rates);
        // Building the return object
        let mut rates_map = HashMap::new();
        for (rate, date) in estimated_rates.iter().zip(target_dates.iter()) {
            rates_map.insert(*date, *rate);
        }
        RateTimeSeries {
            rates_map,
            rates: estimated_rates,
            dates: target_dates,
        }
    }

    fn estimate_rates(theta: &Vec<f64>, features: &Vec<Vec<f64>>) -> Vec<f64> {
        features
            .iter()
            .map(|feature| feature.iter().zip(theta.iter()).map(|(a, b)| a * b).sum())
            .collect()
    }

    fn estimate_cost(features: &Vec<Vec<f64>>, observed: &Vec<f64>, theta: &Vec<f64>) -> f64 {
        let estimate = Self::estimate_rates(&theta, &features);
        let error_aggr: f64 = estimate
            .iter()
            .zip(observed.iter())
            .map(|(a, b)| (a - b).powf(2.0))
            .sum();

        error_aggr / (2. * observed.len() as f64)
    }

    fn gradient_descent(
        features: &Vec<Vec<f64>>,
        observed: &Vec<f64>,
        coeff: &Vec<f64>,
        learning_rate: f64,
        num_epochs: u32,
        tolerance: f64,
    ) -> Vec<f64> {
        let mut previous_cost = 100000.;
        let mut cost_diff = tolerance;
        let size = &observed.len();
        let mut updated_coeff = coeff.clone();
        let mut num_iterations = 0;
        while num_iterations <= num_epochs && cost_diff >= tolerance {
            let estimate = Self::estimate_rates(&updated_coeff, &features);
            let error: Vec<f64> = estimate
                .iter()
                .zip(observed.iter())
                .map(|(a, b)| a - b)
                .collect();
            // Updating model coefficients here by performing a dot product
            for (i, param) in updated_coeff.iter_mut().enumerate() {
                let correction: f64 = error
                    .iter()
                    .zip(features.iter())
                    .map(|(val, feature)| val * feature[i])
                    .sum();
                *param -= correction * learning_rate / (*size as f64);
            }
            let cost = Self::estimate_cost(&features, &observed, &updated_coeff);
            num_iterations += 1;
            cost_diff = (previous_cost - cost).abs();
            previous_cost = cost;
        }

        debug!("Iteration: {:?}", num_iterations);
        debug!("Cost: {:?}", previous_cost);
        debug!("Model Coefficients: {:?}", updated_coeff);
        updated_coeff
    }
}

// Helper Functions

fn process_feature(feature: &Vec<f64>) -> Vec<Vec<f64>> {
    feature.iter().map(|val| vec![1., *val]).collect()
}

fn convert_to_timestamp(date: NaiveDate) -> f64 {
    date.and_hms_opt(0, 0, 0).unwrap().timestamp() as f64 * 1e-9
}
