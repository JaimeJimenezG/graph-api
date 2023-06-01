#[allow(dead_code)]
use super::functions_math_utils::{average, standard_deviation, max, min, covariance, correlation};


pub fn linear_regression(values1: Vec<f32>, values2: Vec<f32>) -> (f32, f32) {
    let avg1 = average(values1.clone());
    let avg2 = average(values2.clone());
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    for i in 0..values1.len() {
        sum1 += (values1[i] - avg1) * (values2[i] - avg2);
        sum2 += (values1[i] - avg1).powf(2.0);
    }
    let b1 = sum1 / sum2;
    let b0 = avg2 - b1 * avg1;
    (b0, b1)
}

pub fn linear_regression_prediction(values1: Vec<f32>, values2: Vec<f32>, value: f32) -> f32 {
    let (b0, b1) = linear_regression(values1.clone(), values2.clone());
    b0 + b1 * value
}

pub fn linear_regression_error(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    let (b0, b1) = linear_regression(values1.clone(), values2.clone());
    let mut sum = 0.0;
    for i in 0..values1.len() {
        sum += (values2[i] - (b0 + b1 * values1[i])).powf(2.0);
    }
    sum / values1.len() as f32
}

pub fn linear_regression_error_percentage(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error(values1.clone(), values2.clone()) / average(values2.clone())
}

pub fn linear_regression_error_percentage_max(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error_percentage(values1.clone(), values2.clone()) * 100.0 / max(values2.clone())
}

pub fn linear_regression_error_percentage_min(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error_percentage(values1.clone(), values2.clone()) * 100.0 / min(values2.clone())
}

pub fn linear_regression_error_percentage_avg(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error_percentage(values1.clone(), values2.clone()) * 100.0 / average(values2.clone())
}

pub fn linear_regression_error_percentage_std(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error_percentage(values1.clone(), values2.clone()) * 100.0 / standard_deviation(values2.clone())
}

pub fn linear_regression_error_percentage_cov(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error_percentage(values1.clone(), values2.clone()) * 100.0 / covariance(values1.clone(), values2.clone())
}

pub fn linear_regression_error_percentage_cor(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error_percentage(values1.clone(), values2.clone()) * 100.0 / correlation(values1.clone(), values2.clone())
}

pub fn linear_regression_error_percentage_lin(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    linear_regression_error_percentage(values1.clone(), values2.clone()) * 100.0 / linear_regression_error(values1.clone(), values2.clone())
}

