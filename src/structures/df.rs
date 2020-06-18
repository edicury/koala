pub struct DataFrame<'a> {
    pub columns : &'a Vec<&'a str>,
    pub dataset : &'a Vec<Vec<&'a str>>
}