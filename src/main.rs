use axum::{routing::get, Json, Router};

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

#[derive(Debug, serde::Serialize)]
struct Vehicle {
    id: String,
    name: String,
    model: String,
    manufacturer: String,
    year: u16,
    color: String,
}


async fn vehicle_get() -> Json<Vehicle> {
    
    Json::from(
        Vehicle {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Koenigsegg".to_string(),
        model: "Agera RS".to_string(),
        manufacturer: "Koenigsegg Automotive AB".to_string(),
        year: 2015,
        color: "Red On Black".to_string(),

    })
    
}

async fn vehicle_get_post() {
    // handle post request
    println!("Received a POST request to /vehicle");
}