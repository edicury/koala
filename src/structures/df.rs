use std::ops::{Index, IndexMut};
use std::collections::HashMap;

use crate::utils::math;

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

impl<'a> DataFrame<'a> {
    /// Adds new value to the end of the col
    pub fn push(&mut self, column: &'a str, value: &'a str) {
        let values = self.values.get_mut(column).expect("Column does not exist");
        values.push(value);
    }

    /// Removes last val from col
    pub fn pop(&mut self, column: &'a str) -> &'a str {
        let values = self.values.get_mut(column).expect("Column does not exist");
        values.pop().expect("no value to pop")
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
            let mut max : f64 = values[0].parse().expect("Value is not a number");
            return math::max(values, &mut max)
        }
        panic!("Values are empty")
    }

    /// Returns min as f64 from all values inside given column
    pub fn min(&self, column: &'a str) -> f64 {
        let values = &self.values[column];
        if values.len() > 0 {
            let mut min : f64 = values[0].parse().expect("Value is not a number");
            return math::min(values, &mut min)
        }
        panic!("Values are empty")
    }

    /// Returns matrix with boolean meaning if value is N/A
    pub fn is_na(&self, na_matrix: &'a mut Vec<Vec<bool>>) -> &'a mut Vec<Vec<bool>> {
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
    pub fn is_na_col(&self, column: &'a str, is_na: &'a mut bool) -> &'a mut bool {
        let values = self.values.get(column).expect("Column not found");
        for row_idx in 0..values.len() {
            if values[row_idx] == "" {
                *is_na = true;
                break;
            }
        }
        is_na
    }
}