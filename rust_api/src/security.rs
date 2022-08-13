use warp::Filter;

const HEADER_AUTH: &str = "token";

pub fn check_auth() -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(HEADER_AUTH))
        .and_then(|token: String| async move {
            if !token.ends_with("secret") {
                return Err(warp::reject::custom(FailAuth));
            }
            Ok::<(), warp::Rejection>(())
        })
}

#[derive(Debug)]
pub struct FailAuth;

impl warp::reject::Reject for FailAuth {}
