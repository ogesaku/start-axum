use cfg_if::cfg_if;
use lazy_static::lazy_static;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[component(transparent)]
pub fn Blog(cx: Scope, prefix: &'static str) -> impl IntoView {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let _ = GetPost::register();
            let _ = ListPostMetadata::register();
        }
    }
    view! { cx,
        <Route path=prefix view=|cx| view!{cx, <Outlet />} >
            <Route path="" view=move |cx| view! {cx, <PostsListing prefix=prefix.clone()/> } />
            <Route path="/:id" view=|cx| view! { cx, <Post/> } />
        </Route>
    }
}

#[component]
fn Wrapper(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <p>"HEADER"</p>
        {children(cx)}
        <p>"FOOTER"</p>
    }
}

#[component]
fn PostsListing(cx: Scope, prefix: &'static str) -> impl IntoView {
    // load the posts
    let posts = create_resource(cx, || (), |_| async { list_post_metadata().await });
    let posts_view = move |cx| {
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
        <Wrapper>
            <h1>"My Great Blog"</h1>
            <Suspense fallback=move || view! { cx, <p>"Loading posts..."</p> }>
                <ul>{posts_view(cx)}</ul>
            </Suspense>
        </Wrapper>
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
    let post = create_blocking_resource(cx, id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(id)
                .await
                .map(|data| data.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });
    let post_view = move |cx| {
        post.with(cx, |post| {
            post.clone().map(|post| {
                view! { cx,
                    <h1>{&post.title}</h1>
                    <p>{&post.content}</p>
                    <Title text=post.title/>
                    <Meta name="description" content=post.content/>
                }
            })
        })
    };

    let comments = create_resource(cx, id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_comments(id)
                .await
                .map(|data| data.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });
    let comments_view = move |cx| {
        comments.with(cx, |comments| {
            comments.clone().map(|comments| {
                view! { cx,
                    <h3>"Comments:"</h3>
                    <p>{&comments.content}</p>
                }
            })
        })
    };

    view! { cx,
        <Suspense fallback=|| ()>
            {post_view(cx)}
        </Suspense>
        <Suspense fallback=|| "Loading comments...">
            {comments_view(cx)}
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    id: usize,
    content: String,
}

#[server(ListPostMetadata, "/api")]
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

#[server(GetPost, "/api")]
pub async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(POSTS.iter().find(|post| post.id == id).cloned())
}

#[server(GetComments, "/api")]
pub async fn get_comments(id: usize) -> Result<Option<Comment>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(Some(Comment {
        id: 1,
        content: format!("Comment for post: {}", id),
    }))
}
