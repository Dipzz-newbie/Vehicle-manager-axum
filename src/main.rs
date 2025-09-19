use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // create a router 
    let router1 = Router::new().route("/vehicle", get(vehicle_get).post(vehicle_get_post));

    //2. define the address and port tcp
    let addres = "localhost:8888";
    let listener =  tokio::net::TcpListener::bind(addres).await.unwrap();

    //3. start the server
    axum::serve(listener, router1).await.unwrap();
}


async fn vehicle_get() {
    // handle get request
    println!("Received a GET request to /vehicle");
}

async fn vehicle_get_post() {
    // handle post request
    println!("Received a POST request to /vehicle");
}