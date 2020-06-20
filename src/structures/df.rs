use std::ops::{Index, IndexMut, Range, Deref};
use std::collections::HashMap;

use crate::utils::math;
use crate::utils::parser::parse_to_usize;
use crate::utils::vec::{uniques, find_index};

pub struct DataFrame<'a> {
    pub columns : &'a mut Vec<&'a str>,
    pub dataset : &'a mut Vec<Vec<&'a str>>,
    pub values : &'a mut HashMap<&'a str, Vec<&'a str>>
}

/// Accessor for rows inside dataframe.
impl<'a> Index<usize> for DataFrame<'a> {
    type Output = Vec<&'a str>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.dataset[index as usize]
    }
}

/// Accessor for rows inside dataframe, returns mutable row.
impl<'a> IndexMut<usize> for DataFrame<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.dataset[index as usize]
    }
}

/// Accessor for columns inside dataframe.
impl<'a> Index<&'a str> for DataFrame<'a> {
    type Output = Vec<&'a str>;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self.values.get(index) {
            Some(v) => v,
            None => panic!("key not present on dataframe")
        }
    }
}

/// Accessor for columns inside dataframe, returns mutable column.
impl<'a> IndexMut<&'a str> for DataFrame<'a> {
    fn index_mut(&mut self, index: &'a str) -> &mut Self::Output {
        match self.values.get_mut(index) {
            Some(v) => v,
            None => panic!("key not present on dataframe")
        }
    }
}



/// Implementation methods
#[allow(dead_code)]
impl<'a> DataFrame<'a> {
    /// Adds new value to the end of the col
    pub fn push(&mut self, value: Vec<&'a str>) {
        // TODO: change &self.values after push
        if value.len() == self.columns.len() {
            &self.dataset.push(value);
        }
        else {
            panic!(format!("Value is not valid, number of cols trying to be inserted {:?}, and should be {:?}", value.len(), self.columns.len()))
        }
    }

    /// Removes last val from col
    pub fn pop(&mut self) -> Vec<&'a str> {
        // TODO: change &self.values after pop
        self.dataset.pop().expect("no values to pop from dataset")
    }

    /// Returns mean as f64 from all values inside given column
    pub fn mean(&self, column: &'a str) -> f64 {
        let mut mean : f64 = 0 as f64;
        let values = &self.values[column];
        math::mean(values, &mut mean)
    }

    /// Returns max as f64 from all values inside given column
    pub fn max(&self, column: &'a str) -> f64 {
        let values = &self.values[column];
        if values.len() > 0 {
            let mut max : f64 = parse_to_usize(values[0]) as f64;
            return math::max(values, &mut max)
        }
        panic!("Values are empty")
    }

    /// Returns min as f64 from all values inside given column
    pub fn min(&self, column: &'a str) -> f64 {
        let values = &self.values[column];
        if values.len() > 0 {
            let mut min : f64 = parse_to_usize(values[0]) as f64;
            return math::min(values, &mut min)
        }
        panic!("Values are empty")
    }

    /// Returns matrix with boolean meaning if value is N/A
    pub fn is_na(&self) -> Vec<Vec<bool>> {
        let mut na_matrix: Vec<Vec<bool>> = Vec::new();

        if self.dataset.len() > 0 {
            let col_size = self.dataset[0].len();
            for row_idx in 0..self.dataset.len() {
                na_matrix.push(Vec::new());
                for col_idx in 0..col_size {
                    na_matrix[row_idx].push(self.dataset[row_idx][col_idx] == "")
                }
            }
            return na_matrix
        }
        panic!("Dataset is empty")
    }
    /// Returns if col has any N/A value ( true or false )
    pub fn is_na_col(&self, column: &str) -> bool {
        let mut is_na = false;
        let values = self.values.get(column).expect("Column not found");
        for row_idx in 0..values.len() {
            if values[row_idx] == "" {
                is_na = true;
                break;
            }
        }
        is_na
    }
    /// Returns sliced dataset as new owned Vec.
    // TODO: it would be nice to return a DataFrame instance here.
    pub fn iloc(&self, range: Vec<Range<usize>>) -> Vec<Vec<&'a str>> {
        let matrix = &self.dataset.clone();

        if range.len() > 2 || range.len() == 0 {
            panic!("Invalid range to loc");
        } else if range.len() == 2 {
            let rows = &range[0];
            let cols = &range[1];
            if rows.end > matrix.len() || cols.end > matrix[0].len() {
                panic!("Invalid ranges for row or column");
            }
            let mut sliced_matrix: Vec<Vec<&str>> = Vec::new();

            for i in rows.start..rows.end {
                sliced_matrix.push(Vec::new());
                for j in cols.start..cols.end {
                    sliced_matrix[i].push(matrix[i][j]);
                }
            }

            sliced_matrix
        } else {
            let rows = &range[0];
            return matrix[rows.start..rows.end].to_vec();
        }
    }

    // Return number of unique occurrences on given DataFrame column
    pub fn n_uniques(&self, column: &str) -> usize {
        let values = self.values.get(column).expect("Column does not exist");
        let uniques = uniques(values);
        uniques.len()
    }

    // Return unique occurrences on given DataFrame column
    pub fn uniques(&self, column: &str) -> Vec<&str> {
        let values = self.values.get(column).expect("Column does not exist");
        let uniques = uniques(values);
        uniques
    }

    // Applies closure function on given column and changes dataset to new value
    pub fn apply(&mut self, column: &str, function: for<'r> fn(&'r str) -> &'a str) {
        let col_idx = find_index(column, self.columns).expect("Column does not exist");
        let values : Vec<&str> = self.values.get_mut(column).expect("Column does not exist").to_vec().iter().map(|x| x.deref()).map(function).collect();

        for i in 0..self.dataset.len() {
            for j in 0..self.dataset[i].len() {
                if j == col_idx {
                    self.dataset[i][j] = values[i];
                }
            }
        }
    }

    // Applies value to any na value inside column
    pub fn fillna(&mut self, column: &str, value: &'a str) {
        let col_idx = find_index(column, self.columns).expect("Column does not exist");

        for i in 0..self.dataset.len() {
            for j in 0..self.dataset[i].len() {
                if j == col_idx {
                    if self.dataset[i][j] == "" {
                        self.dataset[i][j] = value;
                    }
                }
            }
        }
    }
}