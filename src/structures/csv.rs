use std::collections::HashMap;
use super::df::DataFrame;
use std::ops::Index;

#[derive(Debug)]
pub struct CSV<'a> {
    pub headers: Vec<&'a str>,
    pub matrix: Vec<Vec<&'a str>>,
    pub values : HashMap<&'a str, Vec<&'a str>>
}


impl<'a> CSV<'a> {
    pub fn get_df(&'a self) -> DataFrame {
        DataFrame { columns: &self.headers, dataset: &self.matrix }
    }
}

impl<'a> Index<&'a str> for CSV<'a> {
    type Output = Vec<&'a str>;

    fn index(&self, index: &'a str) -> &Self::Output {
        self.values.get(index).unwrap()
    }
}