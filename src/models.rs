use yew::Properties;
#[derive(Clone, PartialEq, Debug)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq, Debug)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
}
