#![feature(in_band_lifetimes)]
#![feature(allocator_api)]

pub mod dataframe;
pub mod structures;
pub mod utils;
pub mod globals;

pub fn test() {
    let mut content = String::new();
    let mut csv = dataframe::df::read_csv("/Users/edisoncury/Documents/Development/Rust/koala/src/test.csv", &mut content, None);

    let df = csv.get_df();

    println!("Spent Sum ${:?}", df.sum("spent"));
}


