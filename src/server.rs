use tokio::net::TcpListener;
use crate::db::StoreDB;
use crate::api;

/// HTTP server aplikácie.
#[derive(Clone)]
pub struct Server {
    db: StoreDB,
}

impl Server {
    /// Vytvorí nový server.
    ///
    /// # Arguments
    /// * `db` – databáza použitá serverom
    ///
    /// # Returns
    /// Nová inštancia `Server`
    pub fn new(db: StoreDB) -> Self {
        Self { db }
    }

    /// Spustí HTTP server.
    ///
    /// Server počúva na `0.0.0.0:8000`
    ///
    /// # Returns
    /// `Ok(())` ak sa server ukončí bez chyby
    ///
    /// # Errors
    /// Ak zlyhá vytvorenie socketu alebo spustenie servera
    pub async fn run(self) -> anyhow::Result<()> {
        let app = api::create_router(self.db);
        let listener = TcpListener::bind("0.0.0.0:8000").await?;

        println!("Databaza pripravena na: http://localhost:8000");
        axum::serve(listener, app).await?;
        Ok(())
    }
}
