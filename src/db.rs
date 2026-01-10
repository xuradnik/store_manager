use anyhow::Result;
use chrono::NaiveDate;
use sqlx::{sqlite::SqlitePoolOptions, Arguments, Row, SqlitePool};
use crate::structs::{Employee, Product};

/// Wrapper nad SQLite databázou obchodu.
#[derive(Clone)]
pub struct StoreDB {
    m_pool: SqlitePool,
}

impl StoreDB {
    /// Vytvorí alebo otvorí databázu a pripraví tabuľky.
    ///
    /// # Returns
    /// Inicializovaná inštancia `StoreDB`
    ///
    /// # Errors
    /// Ak zlyhá vytvorenie súboru alebo pripojenie k databáze
    pub async fn new() -> Result<Self> {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("store.db")?;

        let m_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:store.db")
            .await?;

        // employees
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS employees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                surname TEXT NOT NULL,
                position TEXT NOT NULL,
                department TEXT,
                shift TEXT,
                salary REAL,
                phone_number TEXT,
                email TEXT,
                status INTEGER,
                note TEXT,
                hire_date TEXT
            );
            "#,
        )
            .execute(&m_pool)
            .await?;

        // products
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                status INTEGER,
                bar_code INTEGER NOT NULL,
                cost_price REAL NOT NULL,
                sell_price REAL NOT NULL,
                description TEXT,
                brand TEXT,
                supplier TEXT,
                employee_id INTEGER,
                date_added TEXT,
                date_remove TEXT,
                FOREIGN KEY (employee_id) REFERENCES employees(id)
            );
            "#,
        )
            .execute(&m_pool)
            .await?;

        println!("Databáza pripravená.");
        Ok(Self { m_pool })
    }

    // ==========================
    // Employees
    // ==========================

    /// Pridá zamestnanca do databázy.
    ///
    /// # Arguments
    /// * `employee` – dáta zamestnanca
    ///
    /// # Errors
    /// Ak zlyhá zápis do databázy
    pub async fn add_employee_to_store_db(&self, employee: &Employee) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO employees (
                name, surname, position, department, shift, salary,
                phone_number, email, status, note, hire_date
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
            .bind(employee.name.clone())
            .bind(employee.surname.clone())
            .bind(employee.position.clone())
            .bind(employee.department.clone())
            .bind(employee.shift.clone())
            .bind(employee.salary.clone())
            .bind(employee.phone_number.clone())
            .bind(employee.email.clone())
            .bind(employee.status.clone())
            .bind(employee.note.clone())
            .bind(employee.hire_date.clone())
            .execute(&self.m_pool)
            .await?;

        Ok(())
    }

    /// Vymaže zamestnanca podľa ID.
    ///
    /// # Arguments
    /// * `id` – ID zamestnanca
    ///
    /// # Returns
    /// `true` ak bol záznam vymazaný
    pub async fn delete_employee(&self, id: u32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM employees WHERE id = ?")
            .bind(id)
            .execute(&self.m_pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Aktualizuje zamestnanca podľa ID.
    ///
    /// Aktualizujú sa len polia, ktoré nie sú `None`.
    ///
    /// # Arguments
    /// * `employee` – nové dáta (musí obsahovať `id`)
    ///
    /// # Returns
    /// `true` ak bol záznam zmenený
    pub async fn update_employee(&self, employee: &Employee) -> Result<bool> {
        let id = match employee.id {
            Some(id) => id,
            None => return Ok(false),
        };

        let mut query = String::from("UPDATE employees SET ");
        let mut args = sqlx::sqlite::SqliteArguments::default();
        let mut updates = Vec::new();

        if let Some(v) = &employee.name { updates.push("name = ?"); args.add(v); }
        if let Some(v) = &employee.surname { updates.push("surname = ?"); args.add(v); }
        if let Some(v) = &employee.position { updates.push("position = ?"); args.add(v); }
        if let Some(v) = &employee.department { updates.push("department = ?"); args.add(v); }
        if let Some(v) = &employee.shift { updates.push("shift = ?"); args.add(v); }
        if let Some(v) = &employee.salary { updates.push("salary = ?"); args.add(v); }
        if let Some(v) = &employee.phone_number { updates.push("phone_number = ?"); args.add(v); }
        if let Some(v) = &employee.email { updates.push("email = ?"); args.add(v); }
        if let Some(v) = &employee.status { updates.push("status = ?"); args.add(v); }
        if let Some(v) = &employee.note { updates.push("note = ?"); args.add(v); }
        if let Some(v) = &employee.hire_date { updates.push("hire_date = ?"); args.add(v); }

        if updates.is_empty() {
            return Ok(false);
        }

        query.push_str(&updates.join(", "));
        query.push_str(" WHERE id = ?");
        args.add(id);

        let result = sqlx::query_with(&query, args).execute(&self.m_pool).await?;
        Ok(result.rows_affected() > 0)
    }

    /// Vráti zoznam zamestnancov podľa filtra.
    ///
    /// # Arguments
    /// * `employee` – filter (polia `None` sa ignorujú)
    ///
    /// # Returns
    /// Zoznam zamestnancov
    pub async fn get_employees(&self, employee: Employee) -> Result<Vec<Employee>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM Employees WHERE 1=1");
        let mut args = sqlx::sqlite::SqliteArguments::default();

        if let Some(id) = employee.id { query.push_str(" AND id = ?"); args.add(id); }
        if let Some(name) = employee.name {
            if !name.is_empty() { query.push_str(" AND name LIKE ?"); args.add(format!("%{}%", name)); }
        }
        if let Some(surname) = employee.surname {
            if !surname.is_empty() { query.push_str(" AND surname LIKE ?"); args.add(format!("%{}%", surname)); }
        }
        if let Some(position) = employee.position { query.push_str(" AND position = ?"); args.add(position); }
        if let Some(department) = employee.department { query.push_str(" AND department = ?"); args.add(department); }
        if let Some(shift) = employee.shift { query.push_str(" AND shift = ?"); args.add(shift); }
        if let Some(salary) = employee.salary { query.push_str(" AND salary = ?"); args.add(salary); }
        if let Some(phone) = employee.phone_number { query.push_str(" AND phone_number = ?"); args.add(phone); }
        if let Some(email) = employee.email { query.push_str(" AND email = ?"); args.add(email); }
        if let Some(status) = employee.status { query.push_str(" AND status = ?"); args.add(status); }
        if let Some(note) = employee.note {
            if !note.is_empty() { query.push_str(" AND note LIKE ?"); args.add(format!("%{}%", note)); }
        }
        if let Some(date) = employee.hire_date { query.push_str(" AND hire_date = ?"); args.add(date); }

        let rows = sqlx::query_with(&query, args).fetch_all(&self.m_pool).await?;

        Ok(rows.into_iter().map(|row| Employee {
            id: row.get::<Option<i64>, _>("id").map(|v| v as u32),
            name: row.get("name"),
            surname: row.get("surname"),
            position: row.get("position"),
            department: row.get("department"),
            shift: row.get("shift"),
            salary: row.get("salary"),
            phone_number: row.get("phone_number"),
            email: row.get("email"),
            status: row.get::<Option<i64>, _>("status").map(|v| v == 1),
            note: row.get("note"),
            hire_date: row.get("hire_date"),
        }).collect())
    }

    // ==========================
    // Products
    // ==========================

    /// Pridá produkt do databázy.
    pub async fn add_product_to_store_db(&self, product: &Product) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO products (
                name, category, quantity, status, bar_code, cost_price, sell_price,
                description, brand, supplier, employee_id, date_added, date_remove
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
            .bind(product.name.clone())
            .bind(product.category.clone())
            .bind(product.quantity.clone())
            .bind(product.status.clone())
            .bind(product.bar_code.clone())
            .bind(product.cost_price.clone())
            .bind(product.sell_price.clone())
            .bind(product.description.clone())
            .bind(product.brand.clone())
            .bind(product.supplier.clone())
            .bind(product.employee_id.clone())
            .bind(product.date_added.clone())
            .bind(product.date_remove.clone())
            .execute(&self.m_pool)
            .await?;

        Ok(())
    }

    /// Vymaže produkt podľa ID.
    pub async fn delete_product(&self, id: u32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&self.m_pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Aktualizuje produkt podľa ID.
    pub async fn update_product(&self, product: &Product) -> Result<bool> {
        let id = match product.id {
            Some(id) => id,
            None => return Ok(false),
        };

        let mut query = String::from("UPDATE products SET ");
        let mut args = sqlx::sqlite::SqliteArguments::default();
        let mut updates = Vec::new();

        if let Some(v) = &product.name { updates.push("name = ?"); args.add(v); }
        if let Some(v) = &product.category { updates.push("category = ?"); args.add(v); }
        if let Some(v) = &product.quantity { updates.push("quantity = ?"); args.add(v); }
        if let Some(v) = &product.status { updates.push("status = ?"); args.add(v); }
        if let Some(v) = &product.bar_code { updates.push("bar_code = ?"); args.add(v); }
        if let Some(v) = &product.cost_price { updates.push("cost_price = ?"); args.add(v); }
        if let Some(v) = &product.sell_price { updates.push("sell_price = ?"); args.add(v); }
        if let Some(v) = &product.description { updates.push("description = ?"); args.add(v); }
        if let Some(v) = &product.brand { updates.push("brand = ?"); args.add(v); }
        if let Some(v) = &product.supplier { updates.push("supplier = ?"); args.add(v); }
        if let Some(v) = &product.employee_id { updates.push("employee_id = ?"); args.add(v); }
        if let Some(v) = &product.date_added { updates.push("date_added = ?"); args.add(v); }
        if let Some(v) = &product.date_remove { updates.push("date_remove = ?"); args.add(v); }

        if updates.is_empty() {
            return Ok(false);
        }

        query.push_str(&updates.join(", "));
        query.push_str(" WHERE id = ?");
        args.add(id);

        let result = sqlx::query_with(&query, args).execute(&self.m_pool).await?;
        Ok(result.rows_affected() > 0)
    }

    /// Vráti zoznam produktov podľa filtra.
    pub async fn get_products(&self, product: Product) -> Result<Vec<Product>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM Products WHERE 1=1");
        let mut args = sqlx::sqlite::SqliteArguments::default();

        if let Some(id) = product.id { query.push_str(" AND id = ?"); args.add(id); }
        if let Some(name) = product.name {
            if !name.is_empty() { query.push_str(" AND name LIKE ?"); args.add(format!("%{}%", name)); }
        }
        if let Some(category) = product.category { query.push_str(" AND category = ?"); args.add(category); }
        if let Some(quantity) = product.quantity { query.push_str(" AND quantity = ?"); args.add(quantity as i64); }
        if let Some(status) = product.status { query.push_str(" AND status = ?"); args.add(status); }
        if let Some(barcode) = product.bar_code { query.push_str(" AND bar_code = ?"); args.add(barcode as i64); }
        if let Some(cost) = product.cost_price { query.push_str(" AND cost_price = ?"); args.add(cost); }
        if let Some(price) = product.sell_price { query.push_str(" AND sell_price = ?"); args.add(price); }
        if let Some(desc) = product.description {
            if !desc.is_empty() { query.push_str(" AND description LIKE ?"); args.add(format!("%{}%", desc)); }
        }
        if let Some(brand) = product.brand { query.push_str(" AND brand = ?"); args.add(brand); }
        if let Some(supplier) = product.supplier { query.push_str(" AND supplier = ?"); args.add(supplier); }
        if let Some(emp_id) = product.employee_id { query.push_str(" AND employee_id = ?"); args.add(emp_id as i64); }
        if let Some(date) = product.date_added { query.push_str(" AND date_added = ?"); args.add(date); }
        if let Some(date) = product.date_remove { query.push_str(" AND date_remove = ?"); args.add(date); }

        let rows = sqlx::query_with(&query, args).fetch_all(&self.m_pool).await?;

        Ok(rows.into_iter().map(|row| Product {
            id: row.get::<Option<i64>, _>("id").map(|v| v as u32),
            name: row.get("name"),
            category: row.get("category"),
            quantity: row.get::<Option<i64>, _>("quantity").map(|v| v as u32),
            status: row.get::<Option<i64>, _>("status").map(|v| v == 1),
            bar_code: row.get::<Option<i64>, _>("bar_code").map(|v| v as i64),
            cost_price: row.get("cost_price"),
            sell_price: row.get("sell_price"),
            description: row.get("description"),
            brand: row.get("brand"),
            supplier: row.get("supplier"),
            employee_id: row.get::<Option<i64>, _>("employee_id").map(|v| v as u32),
            date_added: row.get("date_added"),
            date_remove: row.get("date_remove"),
        }).collect())
    }
}
