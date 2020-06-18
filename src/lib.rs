#![feature(in_band_lifetimes)]

mod dataframe;
mod structures;

pub fn test() {
    let mut content = String::new();
    let csv = dataframe::df::read_csv("./test.csv", &mut content);
    println!("Headers {:?}", csv.headers);
    println!("Matrix {:?}", csv.matrix);

    println!("{:?}", csv.values);
    println!("Indexed {:?}", csv["name"]);

    let df = csv.get_df();

    println!("DF dataset {:?}", df.dataset);
    println!("DF columns {:?}", df.columns);

}