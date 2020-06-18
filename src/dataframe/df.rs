use std::fs::File;
use std::io::{BufReader, Read};
use crate::structures::csv::CSV;
use std::collections::HashMap;


fn into_matrix<'a>(lines: Vec<&'a str>, matrix: &mut Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>>  {
    let mut local_matrix = matrix.clone();

    for line_idx in 1..lines.len() {
        let row : Vec<&'a str> = lines[line_idx].split(";").collect();
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


pub fn read_csv<'a>(path: &'a str, content: &'a mut String) -> CSV<'a> {
    let file = File::open(path).expect("unable to open file");
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(content).expect("Unable to transform file into string");

    let lines : Vec<&'a str> = content.split('\n').collect();
    let headers : Vec<&'a str> = lines[0].split(';').collect();

    let mut raw_matrix : Vec<Vec<&'a str>> = Vec::new();

    let matrix = into_matrix(lines, &mut raw_matrix);

    let hashmap : HashMap<&str, Vec<&'a str>> = HashMap::new();

    let values = into_hashmap(&headers, &matrix, &hashmap);

    CSV { headers, matrix, values }
}