#[allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, Read};
use crate::structures::csv::CSV;
use std::collections::HashMap;
use regex::{Regex, Matches};
use std::ops::Deref;
use std::borrow::{Cow, Borrow, BorrowMut};
use std::cell::Cell;
use crate::utils::types::get_type_from_vec;

fn into_matrix<'a>(lines: Vec<&'a str>, matrix: &mut Vec<Vec<&'a str>>, headers: &Vec<&'a str>, separator: &'a str) -> Vec<Vec<&'a str>>  {
    let mut local_matrix = matrix.clone();

    for line_idx in 1..lines.len() {
        let mut row : Vec<&'a str> = lines[line_idx].split(separator).collect();
        if row.len() != headers.len()  {
            for _ in 0..headers.len() {
                row.insert(0, "")
            }
        }
        local_matrix.push(row);
    }
    local_matrix.to_vec()
}


fn into_hashmap<'a>(headers: &Vec<&'a str>, matrix: &Vec<Vec<&'a str>>, hashmap: &HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map = hashmap.clone();
    for header_idx in 0..headers.len() {
        let mut row : Vec<&'a str> = Vec::new();
        let header = headers[header_idx];

        for row_idx in 0..matrix.len() {
            let value = matrix[row_idx][header_idx];
            row.push(value);
        }
        map.insert(header, row);
    }
    map
}

fn is_csv_valid(file: &str, separator: &str) -> bool {
    let lines: Vec<&str> = file.lines().collect();
    let columns : Vec<&str> = lines[0].split(separator).collect();

    let number_of_lines : usize = lines.len();
    let number_of_cols : usize = columns.len();

    let regex : Regex = Regex::new(separator).unwrap();

    let separators = regex.find_iter(file);

    let number_of_separators : usize = separators.count();

    let valid_qty = (number_of_cols - 1) * number_of_lines;

    number_of_separators == valid_qty
}

fn get_types<'a>(values: &HashMap<&'a str, Vec<&'a str>>, columns: &Vec<&'a str>, types: &mut HashMap<&'a str, &'a str>) {
    for col in columns.iter() {
        let val_type = get_type_from_vec(values.get(col).expect("Column does not exist"));

        types.insert(col, val_type);
    }
}


pub fn read_csv<'a>(path: &'a str, content: &'a mut String, separator: Option<&'a str>) -> CSV<'a> {
    let sep = separator.unwrap_or(";");
    let file = File::open(path).expect("unable to open file");
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(content).expect("Unable to transform file into string");

    let mut lines : Vec<&'a str> = content.lines().collect();
    let headers : Vec<&'a str> = lines[0].split(sep).collect();
    let mut raw_matrix : Vec<Vec<&'a str>> = Vec::new();

    let matrix = into_matrix(lines, &mut raw_matrix, &headers, sep);
    let hashmap : HashMap<&str, Vec<&'a str>> = HashMap::new();
    let mut types: HashMap<&'a str, &'a str> = HashMap::new();

    let values = into_hashmap(&headers, &matrix, &hashmap);
    get_types(&values, &headers, &mut types);

    return CSV { headers, matrix, values, separator: sep, types }
}