pub fn find_index(value: &str, vec: &Vec<&str>) -> Option<usize> {
    let mut found = false;
    let mut idx: usize = 0;
    for val in 0..vec.len() {
        if value == vec[val] {
            found = true;
            idx = val;
        }
    }
    if found {
        return Some(idx)
    }
    None
}

pub fn uniques<'a>(vec: &Vec<&'a str>) -> Vec<&'a  str> {
    let mut uniques : Vec<&str> = Vec::new();
    for idx in 0..vec.len() {
        if !uniques.contains(&vec[idx]) {
            uniques.push(vec[idx]);
        }
    }
    uniques
}
