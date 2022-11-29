use gloo_net::http::Request;
use yew::prelude::*;
mod models;
use models::Video;
mod video_list;
use video_list::*;
mod video_details;
use video_details::*;

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("https://api.jsonstorage.net/v1/json/ce1635ff-17f5-4a0e-ac9b-4905478273fe/c06d3dd4-97ba-458c-905e-3c5617424d5f")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()}/>
        }
    });

    /*
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breeaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];
     */

    html! {
        <>
            <h1>{"RustConf Explorer"}</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select} />
            </div>
            {for details}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
