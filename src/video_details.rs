use yew::prelude::*;

use crate::models::VideoDetailsProps;

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
      <div>
        <h2>{video.title.clone()}</h2>
        <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
      </div>
    }
}
