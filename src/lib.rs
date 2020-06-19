#![feature(in_band_lifetimes)]

mod dataframe;
mod structures;
mod utils;

pub fn test() {
    let mut content = String::new();
    let mut csv = dataframe::df::read_csv("/Users/edisoncury/Documents/Development/Rust/koala/src/test.csv", &mut content);

    let mut df = csv.get_df();

    println!("Age mean {:?}", df.mean("age"));

    println!("Age max {:?}", df.max("age"));

    println!("Age min {:?}", df.min("age"));

    df.push("age", "hello");
    println!("VALUES {:?}", df.values);

    df.pop("age");
}


