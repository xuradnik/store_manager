use crate::db::StoreDB;
use crate::structs::{Employee, Product};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Pomocná štruktúra pre uloženie celého stavu databázy do súboru.
#[derive(Serialize, Deserialize)]
struct StoreData {
    employees: Vec<Employee>,
    products: Vec<Product>,
}

/// Nástroj na import a export databázových dát.
pub struct DBFiller;

impl DBFiller {
    /// Načíta dáta z JSON súboru do databázy.
    /// Ak súbor neexistuje, nič sa nenačíta a funkcia skončí bez chyby.
    ///
    /// # Arguments
    /// * `db` – databáza, do ktorej sa majú dáta vložiť
    /// * `file_path` – cesta k JSON súboru
    ///
    /// # Returns
    /// `Ok(())` ak sa načítanie podarilo alebo súbor neexistuje
    ///
    /// # Errors
    /// Ak sa nepodarí otvoriť súbor, načítať JSON alebo zapísať do databázy
    pub async fn load_from_json(db: &StoreDB, file_path: &str) -> Result<()> {
        if !Path::new(file_path).exists() {
            println!("JSON  súbor {} neexistuje", file_path);
            return Ok(());
        }

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let data: StoreData = serde_json::from_reader(reader)?;

        println!("Načitávam dáta z JSON-u");

        for employee in data.employees {
            db.add_employee_to_store_db(&employee).await?;
        }

        for product in data.products {
            db.add_product_to_store_db(&product).await?;
        }

        println!("Databáza načitana úspešne z {}", file_path);
        Ok(())
    }

    /// Uloží obsah databázy do JSON súboru.
    ///
    /// # Arguments
    /// * `db` – databáza, z ktorej sa čítajú dáta
    /// * `file_path` – cieľový súbor
    ///
    /// # Returns
    /// `Ok(())` ak sa súbor úspešne uloží
    ///
    /// # Errors
    /// Ak zlyhá čítanie z databázy, zápis do súboru alebo serializácia
    pub async fn save_to_json(db: &StoreDB, file_path: &str) -> Result<()> {
        let employees = db.get_employees(Employee::new_empty()).await?;
        let products = db.get_products(Product::new_empty()).await?;

        let data = StoreData {
            employees,
            products,
        };

        let file = File::create(file_path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &data)?;

        println!("Databáza uložená do JSON-u {}", file_path);
        Ok(())
    }
}
