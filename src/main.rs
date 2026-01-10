mod db;
mod structs;
mod db_filler;
mod api;
mod server;

use db_filler::DBFiller;
use anyhow::Result;
use db::StoreDB;
use std::path::Path;
use tokio::signal;

/// Vstupný bod aplikácie.
#[tokio::main]
async fn main() -> Result<()> {
    let db_path = "store.db";
    let json_path = "store_data.json";
    let db_exists = Path::new(db_path).exists();

    // Inicializácia databázy
    let store_db = StoreDB::new().await?;

    // Ak DB neexistuje, pokúsi sa ju naplniť z JSONu
    if !db_exists {
        println!("Databaza nenajdena, pokus o načitávanie z JSONu...");
        match DBFiller::load_from_json(&store_db, json_path).await {
            Ok(_) => {
                println!("Načtenie úspešné.");
                if let Err(e) = DBFiller::save_to_json(&store_db, json_path).await {
                    eprintln!("Nepodarilo sa uložiť databázu do JSONu: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Nepodarilo sa načítať dáta z JSONu: {}. Začina sa z prázdnou databázou", e);
            }
        }
    } else {
        println!("Databáza už existuje.");
    }

    // Spustenie servera
    let server = server::Server::new(store_db.clone());
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.run().await {
            eprintln!("Server error: {}", e);
        }
    });

    // Čakanie na Ctrl+C
    signal::ctrl_c().await.ok();
    println!("\nVypína sa server");

    // Uloženie databázy do JSONu
    println!("Databáza sa uloží do JSONu...");
    if let Err(e) = DBFiller::save_to_json(&store_db, json_path).await {
        eprintln!("Nepodarilo sa uložiť databázu do JSONu: {}", e);
    } else {
        println!("Databáza úspešne uložená.");
    }

    server_handle.abort();
    Ok(())
}
