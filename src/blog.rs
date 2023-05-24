use cfg_if::cfg_if;
use lazy_static::lazy_static;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[component(transparent)]
pub fn Blog(cx: Scope, prefix: String) -> impl IntoView {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let _ = GetPost::register();
            let _ = ListPostMetadata::register();
        }
    }
    view! { cx,
        <Route path=prefix.clone() view=|cx| view!{cx, <Outlet clone:prefix/>} clone:prefix>
            <Route path="" clone:prefix view=|cx| view! {cx, <PostsListing prefix=prefix.clone()/> } />
            <Route
                path="/:id"
                view=|cx| view! { cx, <Post/> }
            />
        </Route>
    }
}

#[component]
fn PostsListing(cx: Scope, prefix: String) -> impl IntoView {
    // load the posts
    let posts = create_resource(cx, || (), |_| async { list_post_metadata().await });
    let posts_view = move || {
        posts.with(cx, |posts| posts
            .clone()
            .map(|posts| {
                posts.iter()
                .map(|post| view! { cx, <li><a href=format!("{}/{}", prefix, post.id)>{&post.title}</a></li>})
                .collect_view(cx)
            })
        )
    };

    view! { cx,
        <h1>"My Great Blog"</h1>
        <Suspense fallback=move || view! { cx, <p>"Loading posts..."</p> }>
            <ul>{posts_view}</ul>
        </Suspense>
    }
}

#[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: usize,
}

#[component]
fn Post(cx: Scope) -> impl IntoView {
    let query = use_params::<PostParams>(cx);
    let id = move || query.with(|q| q.as_ref().map(|q| q.id).map_err(|_| PostError::InvalidId));
    let post = create_resource(cx, id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(id)
                .await
                .map(|data| data.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });

    let post_view = move || {
        post.with(cx, |post| {
            post.clone().map(|post| {
                view! { cx,
                    // render content
                    <h1>{&post.title}</h1>
                    <p>{&post.content}</p>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    <Title text=post.title/>
                    <Meta name="description" content=post.content/>
                }
            })
        })
    };

    view! { cx,
        <Suspense fallback=move || view! { cx, <p>"Loading post..."</p> }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx,
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, error)| view! { cx, <li>{error.to_string()} </li> })
                            .collect_view(cx)
                        }
                        </ul>
                    </div>
                }
            }>
                {post_view}
            </ErrorBoundary>
        </Suspense>
    }
}

// Dummy API
lazy_static! {
    static ref POSTS: Vec<Post> = vec![
        Post {
            id: 0,
            title: "My first post".to_string(),
            content: "This is my first post".to_string(),
        },
        Post {
            id: 1,
            title: "My second post".to_string(),
            content: "This is my second post".to_string(),
        },
        Post {
            id: 2,
            title: "My third post".to_string(),
            content: "This is my third post".to_string(),
        },
    ];
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidId,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    id: usize,
    title: String,
    content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostMetadata {
    id: usize,
    title: String,
}

#[server(ListPostMetadata, "/api/csr")]
pub async fn list_post_metadata() -> Result<Vec<PostMetadata>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(POSTS
        .iter()
        .map(|data| PostMetadata {
            id: data.id,
            title: data.title.clone(),
        })
        .collect())
}

#[server(GetPost, "/api/csr")]
pub async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(POSTS.iter().find(|post| post.id == id).cloned())
}