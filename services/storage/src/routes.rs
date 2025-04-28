use axum::{Json, extract::Path};
use common::models::invoice::{Company}

pub async fn get_companies() -> Json<Vec<Company>> {
    Json(vec![]) // TODO: Load from DB
}

pub async fn get_company(Path(id): Path<i64>) -> Json<Company> {
    Json(Company {

    })
}

pub async fn create_company(Json(_payload): Json<Company>) -> Json<&'static str> {
    Json("Company created")
}

pub async fn update_company(Path(_id): Path<i64>, Json(_payload): Json<Company>) -> Json<&'static str> {
    Json("Company updated")
}

pub async fn delete_company(Path(_id): Path<i64>) -> Json<&'static str> {
    Json("Company deleted")
}

pub async fn get_customers() -> Json<Vec<Company>> {
    Json(vec![]) // TODO: Load from DB
}

pub async fn get_customer(Path(id): Path<i64>) -> Json<Company> {
    Json(Company {
        nazwa: String,
        nip: String,
        enip: String,
        numer_konta: String,
        adres: Address,
    })
}

pub async fn create_customer(Json(_payload): Json<Company>) -> Json<&'static str> {
    Json("Company created")
}

pub async fn update_customer(Path(_id): Path<i64>, Json(_payload): Json<Company>) -> Json<&'static str> {
    Json("Company updated")
}

pub async fn delete_customer(Path(_id): Path<i64>) -> Json<&'static str> {
    Json("Company deleted")
}
