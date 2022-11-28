use yew::prelude::*;
use yew::Properties;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq, Debug)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[derive(Properties, Eq, PartialEq, Debug)]
pub struct VideoDetailsProps {
    pub video: Video,
}
