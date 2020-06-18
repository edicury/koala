pub fn mean(values: &Vec<&str>, mean: &mut f64) -> f64 {
    if values.len() > 0 {
        for val in 0..values.len() {
            let fval :f64 =  values[val].parse().expect("Value is not a number");
            *mean += fval;
        }
        *mean / values.len() as f64
    }
    else {
        panic!("Values are empty");
    }
}

pub fn max(values: &Vec<&str>, max: &mut f64) -> f64 {
    for val in 0..values.len() {
        let fval :f64 = values[val].parse().expect("Value is not a number");
        if fval > *max {
            *max = fval;
        }
    }
    *max
}

pub fn min(values: &Vec<&str>, min: &mut f64) -> f64 {
    for val in 0..values.len() {
        let fval :f64 = values[val].parse().expect("Value is not a number");
        if fval < *min {
            *min = fval;
        }
    }
    *min
}