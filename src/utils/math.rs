use crate::utils::vec::non_na_vec;
use std::ops::Deref;

pub fn mean(values: &Vec<&str>, mean: &mut f64) -> f64 {
    let non_empty : Vec<&str> = non_na_vec(values);

    if non_empty.len() > 0 {
        for val in 0..non_empty.len() {
            let value = non_empty[val];
            if value != "" {
                let fval :f64 =  value.parse().expect("Value is not a number");
                *mean += fval;
            }
        }
        *mean / non_empty.len() as f64
    }
    else {
        panic!("Values are empty");
    }
}

pub fn max(values: &Vec<&str>, max: &mut f64) -> f64 {
    let non_empty : Vec<&str> = non_na_vec(values);
    for val in 0..non_empty.len() {
        let value = non_empty[val];
        if value != "" {
            let fval :f64 = value.parse().expect("Value is not a number");
            if fval > *max {
                *max = fval;
            }
        }
    }
    *max
}

pub fn min(values: &Vec<&str>, min: &mut f64) -> f64 {
    let non_empty : Vec<&str> = non_na_vec(values);

    for val in 0..non_empty.len() {
        let value = non_empty[val];
        let fval :f64 = value.parse().expect("Value is not a number");
        if fval < *min {
            *min = fval;
        }
    }
    *min
}

pub fn sum(values: &Vec<&str>) -> f64 {
    let non_empty: Vec<&str> = non_na_vec(values);

    let mut total: f64 = 0 as f64;
    for val in non_empty.iter() {
        let fval: f64 = val.deref().parse().expect("Value is not a number");
        total += fval;
    }
    total
}