#[allow(dead_code)]


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