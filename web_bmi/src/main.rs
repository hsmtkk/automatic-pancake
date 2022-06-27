use axum::{
    routing::{get, post},
    response::Html,
    Form,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/calc", post(calc_bmi));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    Html("<html><body><h1>BMI判定</h1>
    <form action='calc'>
    身長: <input name'height' value='160'><br>
    体重: <input name'weight' value='70'><br>
    <input type='submit' value='送信'>
    </form></body></html>")
}

async fn calc_bmi(Form(height_weight): Form<HeightWeight>) -> Html<String> {
    let h = height_weight.height / 100.0;
    let bmi = height_weight.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    Html(format!("<h3>BMI={:.1}, 乖離率={:.0}%</h3>", bmi, per))
}

// the output to our `create_user` handler
#[derive(Deserialize)]
struct HeightWeight {
    height: f64,
    weight: f64,
}