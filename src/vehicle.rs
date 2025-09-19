use axum::{Json};

#[derive(Debug, serde::Serialize)]
pub struct Vehicle {
    id: String,
    name: String,
    model: String,
    manufacturer: String,
    year: u16,
    color: String,
}


pub async fn vehicle_get() -> Json<Vehicle> {
    println!("Received a GET request to /vehicle");
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

pub async fn vehicle_get_post() {
    // handle post request
    println!("Received a POST request to /vehicle");
}