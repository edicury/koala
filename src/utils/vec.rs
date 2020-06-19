pub fn uniques<'a>(vec: &Vec<&'a str>) -> Vec<&'a  str> {
    let mut uniques : Vec<&str> = Vec::new();
    for idx in 0..vec.len() {
        if !uniques.contains(&vec[idx]) {
            uniques.push(vec[idx]);
        }
    }
    uniques
}