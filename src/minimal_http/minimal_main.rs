use warp::Filter;

pub async fn minimal_http_svr() {
    let hello = warp::get()
        .map(|| format!("Hello, World!"));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 8083))
        .await;
}