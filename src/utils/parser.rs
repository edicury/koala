#[allow(dead_code)]
pub fn parse_to_usize(value: &str) -> usize {
    let val : usize = value.parse().expect("could not parse value into number");
    val
}

#[allow(dead_code)]
pub fn parse_to_bool(value: &str) -> bool {
    let val : bool = value.parse().expect("could not parse value into bool");
    val
}