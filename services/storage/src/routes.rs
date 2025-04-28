use axum::{Json, extract::Path};
use common::models::{Company, Customer};

pub async fn get_companies() -> Json<Vec<Company>> {
    Json(vec![]) // TODO: Load from DB
}

pub async fn get_company(Path(id): Path<i64>) -> Json<Company> {
    Json(Company {
        id,
        nazwa: "Example".into(),
        nip: "1234567890".into(),
        enip: Some("PL1234567890".into()),
        numer_konta: "12345678901234567890123456".into(),
        email: Some("firma@example.com".into()),
        tel: Some("+48123456789".into()),
        adres_ulica: "Ulica".into(),
        adres_kod_pocztowy: "00-000".into(),
        adres_miasto: "Miasto".into(),
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

pub async fn get_customers() -> Json<Vec<Customer>> {
    Json(vec![]) // TODO: Load from DB
}

pub async fn get_customer(Path(id): Path<i64>) -> Json<Customer> {
    Json(Customer {
        id,
        nazwa: "Klient".into(),
        nip: "9876543210".into(),
        email: Some("klient@example.com".into()),
        tel: Some("+48111222333".into()),
        adres_ulica: "Klienta 5".into(),
        adres_kod_pocztowy: "11-111".into(),
        adres_miasto: "Klientowo".into(),
    })
}

pub async fn create_customer(Json(_payload): Json<Customer>) -> Json<&'static str> {
    Json("Customer created")
}

pub async fn update_customer(Path(_id): Path<i64>, Json(_payload): Json<Customer>) -> Json<&'static str> {
    Json("Customer updated")
}

pub async fn delete_customer(Path(_id): Path<i64>) -> Json<&'static str> {
    Json("Customer deleted")
}
