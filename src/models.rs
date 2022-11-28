// pub struct Video<'a> {
// use yew::Properties;
#[derive(Clone, PartialEq, Debug)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
    // title: &'a str,
    // speaker: &'a str,
    // url: &'a str,
}
