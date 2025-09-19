use axum::{routing::get, Router};

mod vehicle;

#[tokio::main]
async fn main() {
    // create a router 
    let app = Router::new().route("/vehicle", get(vehicle::vehicle_get).post(vehicle::vehicle_get_post));

    //2. define the address and port tcp
    let addres = "localhost:8888";
    let listener =  tokio::net::TcpListener::bind(addres).await.unwrap();

    //3. start the server
    axum::serve(listener, app).await.unwrap();
}

