use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Properties, PartialEq)]
struct PostsListProps {
    posts: Vec<Post>,
    on_click: Callback<Post>,
}

#[function_component(PostsList)]
fn posts_list(
    PostsListProps { posts, on_click }: &PostsListProps,
) -> Html {
    posts
        .iter()
        .map(|post| {
            let on_post_select = {
                let on_click = on_click.clone();
                let post = post.clone();
                Callback::from(move |_| {
                    on_click.emit(post.clone())
                })
            };

            html! {
                <p onclick={on_post_select}>{
                    format!("{}: {}",
                    post.id, post.title)
                }</p>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct PostsDetailProps {
    post: Post,
}

#[function_component(PostDetail)]
fn post_detail(
    PostsDetailProps { post }: &PostsDetailProps,
) -> Html {
    html! {
        <div>
            <h3>{ post.title.clone() }</h3>
            <p>{ post.body.clone() }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let posts = use_state(|| vec![]);
    {
        let posts = posts.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(
                    async move {
                        let fetched_posts: Vec<Post> =
                            Request::get("/posts")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                        posts.set(fetched_posts);
                    },
                );
                || ()
            },
            (),
        );
    }

    let selected_post = use_state(|| None);
    let on_post_select = {
        let selected_post = selected_post.clone();
        Callback::from(move |post: Post| {
            selected_post.set(Some(post))
        })
    };
    let detail = selected_post.as_ref().map(|post| {
        html! {
            <PostDetail post={post.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "My blog" }</h1>
            <div>
                <h3>{ "posts list" }</h3>
                <PostsList posts={(*posts).clone()}
                 on_click={on_post_select.clone()} />
            </div>
            { for detail }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
