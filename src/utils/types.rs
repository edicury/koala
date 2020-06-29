use regex::Regex;

pub fn get_type_from_vec<'a>(vec: &Vec<&'a str>) -> &'a str {
    let val_type = get_type(vec[0]);
    val_type
}

pub fn get_type(val: &str) -> &str {
    let numeric = r#"[0-9]+.?([0-9]+)?"#;
    let bool = r#"true|false"#;

    let numeric_re = Regex::new(numeric).unwrap();
    let bool_re = Regex::new(bool).unwrap();

    let is_numeric = numeric_re.is_match(val);
    let is_bool = bool_re.is_match(val);
    if is_numeric {
        "numeric"
    } else if is_bool {
        "bool"
    } else {
        "str"
    }
}