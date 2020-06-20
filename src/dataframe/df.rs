use std::fs::File;
use std::io::{BufReader, Read};
use crate::structures::csv::CSV;
use std::collections::HashMap;
use regex::Regex;

fn into_matrix<'a>(lines: Vec<&'a str>, matrix: &mut Vec<Vec<&'a str>>, separator: &'a str) -> Vec<Vec<&'a str>>  {
    let mut local_matrix = matrix.clone();

    for line_idx in 1..lines.len() {
        let row : Vec<&'a str> = lines[line_idx].split(separator).collect();
        local_matrix.push(row);
    }
    local_matrix.to_vec()
}


fn into_hashmap<'a>(headers: &Vec<&'a str>, matrix: &Vec<Vec<&'a str>>, hashmap: &HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map = hashmap.clone();

    for header_idx in 0..headers.len() {
        let header : &'a str = headers[header_idx];
        let mut row : Vec<&'a str> = Vec::new();

        for row_idx in 0..matrix.len() {
            let value = matrix[row_idx][header_idx];
            row.push(value);
        }
        map.insert(header, row);
    }
    map
}

fn is_csv_valid(file: &str, separator: &str) -> bool {
    let lines: Vec<&str> = file.split("\n").collect();
    let columns : Vec<&str> = lines[0].split(separator).collect();

    let number_of_lines : usize = lines.len();
    let number_of_cols : usize = columns.len();

    let regex : Regex = Regex::new(separator).unwrap();

    let separators = regex.find_iter(file);

    let number_of_separators : usize = separators.count();

    let valid_qty = (number_of_cols - 1) * number_of_lines;

    number_of_separators == valid_qty
}


pub fn read_csv<'a>(path: &'a str, content: &'a mut String, separator: Option<&'a str>) -> CSV<'a> {
    let sep = separator.unwrap_or(";");
    let file = File::open(path).expect("unable to open file");
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(content).expect("Unable to transform file into string");

    if is_csv_valid(&content,sep) {
        let lines : Vec<&'a str> = content.split('\n').collect();

        let headers : Vec<&'a str> = lines[0].split(sep).collect();

        let mut raw_matrix : Vec<Vec<&'a str>> = Vec::new();

        let matrix = into_matrix(lines, &mut raw_matrix, sep);

        let hashmap : HashMap<&str, Vec<&'a str>> = HashMap::new();

        let values = into_hashmap(&headers, &matrix, &hashmap);

        return CSV { headers, matrix, values, separator: sep }
    }
    panic!("CSV is invalid, check separators");
}