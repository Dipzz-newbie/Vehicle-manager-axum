use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // create a router 
    let router1 = Router::new().route("/vehicle", get(vehicle_get));

    //2. define the address and port tcp
    let addres = "127.0.0.1:8080";
    let listener =  tokio::net::TcpListener::bind(addres).await.unwrap();

    //3. start the server
    axum::serve(listener, router1).await.unwrap();
}


async fn vehicle_get() {

}