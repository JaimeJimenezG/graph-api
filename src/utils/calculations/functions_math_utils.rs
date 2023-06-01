#[allow(dead_code)]


pub fn max(values: Vec<f32>) -> f32 {
    let mut max = values[0];
    for value in values.clone() {
        if value > max {
            max = value;
        }
    }
    max
}

pub fn min(values: Vec<f32>) -> f32 {
    let mut min = values[0];
    for value in values.clone() {
        if value < min {
            min = value;
        }
    }
    min
}

pub fn average(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value;
    }
    sum / values.len() as f32
}

pub fn variance(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    let avg = average(values.clone());
    for value in values.clone() {
        sum += (value - avg).powf(2.0);
    }
    sum / values.len() as f32
}

pub fn standard_deviation(values: Vec<f32>) -> f32 {
    variance(values.clone()).sqrt()
}

pub fn covariance(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    let avg1 = average(values1.clone());
    let avg2 = average(values2.clone());
    for i in 0..values1.len() {
        sum += (values1[i] - avg1) * (values2[i] - avg2);
    }
    sum / values1.len() as f32
}

pub fn correlation(values1: Vec<f32>, values2: Vec<f32>) -> f32 {
    covariance(values1.clone(), values2.clone()) / (standard_deviation(values1.clone()) * standard_deviation(values2.clone()))
}
