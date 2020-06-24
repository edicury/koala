use std::ops::Deref;
use crate::globals::constants::NA;
use std::collections::HashMap;

pub fn find_index(value: &str, vec: &Vec<&str>) -> Option<usize> {
    let mut found = false;
    let mut idx: usize = 0;
    for val in 0..vec.len() {
        if value == vec[val] {
            found = true;
            idx = val;
        }
    }
    if found {
        return Some(idx)
    }
    None
}

pub fn uniques<'a>(vec: &Vec<&'a str>) -> Vec<&'a  str> {
    let mut uniques : Vec<&str> = Vec::new();
    for idx in 0..vec.len() {
        if !uniques.contains(&vec[idx]) && vec[idx] != "" {
            uniques.push(vec[idx]);
        }
    }
    uniques
}


pub fn non_na_vec(vec: &'a Vec<&'a str>) -> Vec<&'a str> {
    let non_na: Vec<&'a str> = vec.iter()
        .filter(|x| x.to_string() != NA)
        .map(|x| x.deref())
        .collect();

    non_na
}

pub fn to_hashmap<'a>(columns: &Vec<&'a str>, dataset: &Vec<Vec<&'a str>>, map: &mut HashMap<&'a str, Vec<&'a str>>) {
    for header_idx in 0..columns.len() {
        let header : &'a str = columns[header_idx];
        let mut row : Vec<&'a str> = Vec::new();

        for row_idx in 0..dataset.len() {
            let value = dataset[row_idx][header_idx];
            row.push(value);
        }
        map.insert(header, row);
    }
}