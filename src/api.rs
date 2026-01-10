use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    db::StoreDB,
    structs::{Employee, Product},
};

/// Vytvorí a nakonfiguruje HTTP router aplikácie.
pub fn create_router(db: StoreDB) -> Router {
    Router::new()
        .route("/", get(index_page))
        .route("/employees", get(list_employees).post(add_employee))
        .route("/employees/search", post(search_employees))
        .route("/employees/{id}", delete(delete_employee).put(update_employee))
        .route("/products", get(list_products).post(add_product))
        .route("/products/search", post(search_products))
        .route("/products/{id}", delete(delete_product).put(update_product))
        .with_state(db)
}



/// Vráti hlavnú HTML stránku.
///
/// # Returns
/// HTML obsah stránky
async fn index_page() -> Html<&'static str> {
    Html(include_str!("web/index.html"))
}



/// Vráti všetkých zamestnancov.
///
/// # Arguments
/// * `db` – databázový stav aplikácie
///
/// # Returns
/// Zoznam zamestnancov v JSONe
///
/// # Errors
/// Ak zlyhá čítanie z databázy
async fn list_employees(
    State(db): State<StoreDB>,
) -> Result<Json<Vec<Employee>>, StatusCode> {
    db.get_employees(Employee::new_empty())
        .await
        .map(Json)
        .map_err(|e| {
            eprintln!("Chyba pri načítaní zamestnancov: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Vyhľadá zamestnancov podľa filtra.
///
/// # Arguments
/// * `db` – databáza
/// * `filter` – vyhľadávacie kritériá
///
/// # Returns
/// Zoznam nájdených zamestnancov
///
/// # Errors
/// Ak zlyhá vyhľadávanie
async fn search_employees(
    State(db): State<StoreDB>,
    Json(filter): Json<Employee>,
) -> Result<Json<Vec<Employee>>, StatusCode> {
    db.get_employees(filter)
        .await
        .map(Json)
        .map_err(|e| {
            eprintln!("Chyba pri vyhľadávaní zamestnancov: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Pridá nového zamestnanca.
///
/// # Arguments
/// * `db` – databáza
/// * `emp` – dáta zamestnanca
///
/// # Returns
/// HTTP status kód výsledku
async fn add_employee(
    State(db): State<StoreDB>,
    Json(emp): Json<Employee>,
) -> StatusCode {
    match db.add_employee_to_store_db(&emp).await {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            eprintln!("Chyba pri pridávaní zamestnanca: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Vymaže zamestnanca podľa ID.
///
/// # Arguments
/// * `db` – databáza
/// * `id` – ID zamestnanca
///
/// # Returns
/// HTTP status kód výsledku
async fn delete_employee(
    State(db): State<StoreDB>,
    Path(id): Path<u32>,
) -> StatusCode {
    match db.delete_employee(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(e) => {
            eprintln!("Chyba pri mazaní zamestnanca: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Aktualizuje zamestnanca.
///
/// # Arguments
/// * `db` – databáza
/// * `id` – ID zamestnanca
/// * `emp` – nové dáta
///
/// # Returns
/// HTTP status kód výsledku
async fn update_employee(
    State(db): State<StoreDB>,
    Path(id): Path<u32>,
    Json(mut emp): Json<Employee>,
) -> StatusCode {
    emp.id = Some(id);
    match db.update_employee(&emp).await {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(e) => {
            eprintln!("Chyba pri updatovaní zamestnanca: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}



/// Vráti všetky produkty.
///
/// # Arguments
/// * `db` – databáza
///
/// # Returns
/// Zoznam produktov
///
/// # Errors
/// Ak zlyhá čítanie z databázy
async fn list_products(
    State(db): State<StoreDB>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    db.get_products(Product::new_empty())
        .await
        .map(Json)
        .map_err(|e| {
            eprintln!("Chyba pri načítaní produktov: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Vyhľadá produkty podľa filtra.
///
/// # Arguments
/// * `db` – databáza
/// * `filter` – vyhľadávacie kritériá
///
/// # Returns
/// Zoznam nájdených produktov
///
/// # Errors
/// Ak zlyhá vyhľadávanie
async fn search_products(
    State(db): State<StoreDB>,
    Json(filter): Json<Product>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    db.get_products(filter)
        .await
        .map(Json)
        .map_err(|e| {
            eprintln!("Chyba pri vyhľadávaní produktov: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Pridá nový produkt.
///
/// # Arguments
/// * `db` – databáza
/// * `prod` – dáta produktu
///
/// # Returns
/// HTTP status kód výsledku
async fn add_product(
    State(db): State<StoreDB>,
    Json(prod): Json<Product>,
) -> StatusCode {
    match db.add_product_to_store_db(&prod).await {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            eprintln!("Chyba pri pridávaní produktu: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Vymaže produkt podľa ID.
///
/// # Arguments
/// * `db` – databáza
/// * `id` – ID produktu
///
/// # Returns
/// HTTP status kód výsledku
async fn delete_product(
    State(db): State<StoreDB>,
    Path(id): Path<u32>,
) -> StatusCode {
    match db.delete_product(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(e) => {
            eprintln!("Chyba pri mazaní produktu: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Aktualizuje produkt.
///
/// # Arguments
/// * `db` – databáza
/// * `id` – ID produktu
/// * `prod` – nové dáta
///
/// # Returns
/// HTTP status kód výsledku
async fn update_product(
    State(db): State<StoreDB>,
    Path(id): Path<u32>,
    Json(mut prod): Json<Product>,
) -> StatusCode {
    prod.id = Some(id);
    match db.update_product(&prod).await {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(e) => {
            eprintln!("Chyba pri updatovaní produktu: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
