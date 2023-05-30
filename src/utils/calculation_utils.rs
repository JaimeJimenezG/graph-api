pub fn sum(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value;
    }
    sum
}

pub fn minus(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum -= value;
    }
    sum
}

pub fn product(values: Vec<f32>) -> f32 {
    let mut product = 1.0;
    for value in values.clone() {
        product *= value;
    }
    product
}

pub fn division(values: Vec<f32>) -> f32 {
    let mut division = values[0];
    for i in 1..values.len() {
        division /= values[i];
    }
    division
}

pub fn sum_of_squares(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value.powf(2.0);
    }
    sum
}

pub fn minus_of_squares(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum -= value.powf(2.0);
    }
    sum
}

pub fn product_of_squares(values: Vec<f32>) -> f32 {
    let mut product = 1.0;
    for value in values.clone() {
        product *= value.powf(2.0);
    }
    product
}

pub fn division_of_squares(values: Vec<f32>) -> f32 {
    let mut division = values[0].powf(2.0);
    for i in 1..values.len() {
        division /= values[i].powf(2.0);
    }
    division
}

pub fn sum_of_cubes(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value.powf(3.0);
    }
    sum
}

pub fn minus_of_cubes(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum -= value.powf(3.0);
    }
    sum
}

pub fn product_of_cubes(values: Vec<f32>) -> f32 {
    let mut product = 1.0;
    for value in values.clone() {
        product *= value.powf(3.0);
    }
    product
}

pub fn division_of_cubes(values: Vec<f32>) -> f32 {
    let mut division = values[0].powf(3.0);
    for i in 1..values.len() {
        division /= values[i].powf(3.0);
    }
    division
}

pub fn sum_of_powers(values: Vec<f32>, power: f32) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value.powf(power);
    }
    sum
}

pub fn minus_of_powers(values: Vec<f32>, power: f32) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum -= value.powf(power);
    }
    sum
}

pub fn product_of_powers(values: Vec<f32>, power: f32) -> f32 {
    let mut product = 1.0;
    for value in values.clone() {
        product *= value.powf(power);
    }
    product
}

pub fn division_of_powers(values: Vec<f32>, power: f32) -> f32 {
    let mut division = values[0].powf(power);
    for i in 1..values.len() {
        division /= values[i].powf(power);
    }
    division
}

pub fn sum_of_powers_of_squares(values: Vec<f32>, power: f32) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value.powf(2.0 * power);
    }
    sum
}

pub fn minus_of_powers_of_squares(values: Vec<f32>, power: f32) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum -= value.powf(2.0 * power);
    }
    sum
}

pub fn product_of_powers_of_squares(values: Vec<f32>, power: f32) -> f32 {
    let mut product = 1.0;
    for value in values.clone() {
        product *= value.powf(2.0 * power);
    }
    product
}

pub fn division_of_powers_of_squares(values: Vec<f32>, power: f32) -> f32 {
    let mut division = values[0].powf(2.0 * power);
    for i in 1..values.len() {
        division /= values[i].powf(2.0 * power);
    }
    division
}

pub fn sum_of_powers_of_powers(values: Vec<f32>, power1: f32, power2: f32) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum += value.powf(power1).powf(power2);
    }
    sum
}

pub fn minus_of_powers_of_powers(values: Vec<f32>, power1: f32, power2: f32) -> f32 {
    let mut sum = 0.0;
    for value in values.clone() {
        sum -= value.powf(power1).powf(power2);
    }
    sum
}

pub fn product_of_powers_of_powers(values: Vec<f32>, power1: f32, power2: f32) -> f32 {
    let mut product = 1.0;
    for value in values.clone() {
        product *= value.powf(power1).powf(power2);
    }
    product
}

pub fn division_of_powers_of_powers(values: Vec<f32>, power1: f32, power2: f32) -> f32 {
    let mut division = values[0].powf(power1).powf(power2);
    for i in 1..values.len() {
        division /= values[i].powf(power1).powf(power2);
    }
    division
}

pub fn ponderated_sum(values: Vec<f32>, ponderations: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for i in 0..values.len() {
        sum += values[i] * ponderations[i];
    }
    sum
}

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

