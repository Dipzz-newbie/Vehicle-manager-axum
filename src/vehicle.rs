use axum::{Json};

#[derive(Debug, serde::Serialize)]
pub struct Vehicle {
    id: Option<String>,
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
        id: Some(uuid::Uuid::new_v4().to_string()),
        name: "Koenigsegg".to_string(),
        model: "Agera RS".to_string(),
        manufacturer: "Koenigsegg Automotive AB".to_string(),
        year: 2015,
        color: "Red On Black".to_string(),

    })
    
}

pub async fn vehicle_get_post(Json(mut v): Json<Vehicle>) {
   //println!("Received a POST request to /vehicle with payload: {:?}", v);
    println!("name{0}, model{1}, manufacturer{2}, year{3}, color{4}", v.name, v.model, v.manufacturer, v.year, v.color);

    v.id = Some(uuid::Uuid::new_v4().to_string());
    println!("Vehicle created with ID: {:?}", v.id);
    Json::from(v);


}