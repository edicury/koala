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

    df.push(["edison", "26", "BR"].to_vec());
    println!("VALUES {:?}", df.dataset);

    df.pop();

    let selected = df.iloc([0..2, 0..1].to_vec());

    println!("ILoc {:?}", selected);

    let n_uniques = df.n_uniques("age");
    println!("N Unique Ages {:?}", n_uniques);
    let uniques = df.uniques("age");
    println!("Unique ages {:?}", uniques);
}


