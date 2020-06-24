use std::collections::HashMap;
use super::df::DataFrame;
use std::ops::Index;

#[derive(Debug)]
pub struct CSV<'a> {
    pub headers: Vec<&'a str>,
    pub matrix: Vec<Vec<&'a str>>,
    pub values : HashMap<&'a str, Vec<&'a str>>,
    pub types: HashMap<&'a str, &'a str>,
    pub separator: &'a str
}


impl<'a> CSV<'a> {
    pub fn get_df(&'a mut self) -> DataFrame {
        DataFrame { columns: &mut self.headers, dataset: &mut self.matrix, values: &mut self.values, dtypes: &mut self.types }
    }
}

impl<'a> Index<&'a str> for CSV<'a> {
    type Output = Vec<&'a str>;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self.values.get(index) {
            Some(v) => v,
            _ => {panic!("key not present on csv")}
        }
    }
}