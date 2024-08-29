use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the directory to serve files from
    let file_dir = warp::fs::dir("static");

    // Start the warp server
    warp::serve(file_dir)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
