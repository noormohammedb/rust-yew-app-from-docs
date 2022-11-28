use crate::models::Video;
use yew::prelude::*;
use yew::Properties;

// #[derive(Properties)]
#[derive(Properties, PartialEq, Debug)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            html! {
            <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}
