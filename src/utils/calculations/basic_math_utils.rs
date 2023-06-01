#[allow(dead_code)]


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