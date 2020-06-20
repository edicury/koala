#![feature(in_band_lifetimes)]
#![feature(allocator_api)]

pub mod dataframe;
pub mod structures;
pub mod utils;
pub mod globals;

fn test() {
    let mut content = String::new();
    let mut csv = dataframe::df::read_csv("/Users/edisoncury/Documents/Development/Rust/koala/src/Salary_Data.csv", &mut content, Some(","));

    let df = csv.get_df();

    println!("df.columns {:?} columns", df.columns);

    println!("YearsExperience Mean: {:?}", df.mean("YearsExperience"));
    println!("Salary Mean: {:?}", df.mean("Salary"));
}


