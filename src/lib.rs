#![feature(in_band_lifetimes)]
#![feature(allocator_api)]

pub mod dataframe;
pub mod structures;
pub mod utils;
pub mod globals;

pub fn test() {
    let mut content = String::new();
    let mut csv = dataframe::df::read_csv("/Users/edisoncury/Documents/Development/Rust/koala/src/dataset.csv", &mut content, Some(","));
    let mut df = csv.get_df();



    println!("df.columns {:?} columns", df.columns);

    let mean = df.mean("x");
    let mean_str: String =  format!("{}", mean);

    df.fillna("x", mean_str.as_str());

    println!("df x {:?}", df.is_na_col("x"));

}


