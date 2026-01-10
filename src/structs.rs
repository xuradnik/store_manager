use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

/// Reprezentuje produkt v obchode.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id:           Option<u32>,
    pub name:         Option<String>,
    pub category:     Option<String>,
    pub quantity:     Option<u32>,
    pub status:       Option<bool>,
    pub bar_code:     Option<i64>,
    pub cost_price:   Option<f64>,
    pub sell_price:   Option<f64>,
    pub description:  Option<String>,
    pub brand:        Option<String>,
    pub supplier:     Option<String>,
    pub employee_id:  Option<u32>,
    pub date_added:   Option<NaiveDate>,
    pub date_remove:  Option<NaiveDate>,
}

/// Reprezentuje zamestnanca obchodu.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id:           Option<u32>,
    pub name:         Option<String>,
    pub surname:      Option<String>,
    pub position:     Option<String>,
    pub department:   Option<String>,
    pub shift:        Option<String>,
    pub salary:       Option<f64>,
    pub phone_number: Option<String>,
    pub email:        Option<String>,
    pub status:       Option<bool>,
    pub note:         Option<String>,
    pub hire_date:    Option<NaiveDate>,
}

impl Employee {
    /// Vytvorí novú inštanciu zamestnanca.
    ///
    /// # Arguments
    /// * `id_p` – ID zamestnanca
    /// * `name_p` – meno
    /// * `surename_p` – priezvisko
    /// * `position_p` – pozícia
    /// * `department_p` – oddelenie
    /// * `shift_p` – pracovná zmena
    /// * `salart_p` – mzda
    /// * `phoen_number_p` – telefón
    /// * `email_p` – email
    /// * `status_p` – aktívny / neaktívny
    /// * `note_p` – poznámka
    /// * `hire_date_p` – dátum nástupu
    ///
    /// # Returns
    /// Nový `Employee`
    pub fn new(
        id_p:           Option<u32>,
        name_p:         Option<String>,
        surename_p:     Option<String>,
        position_p:     Option<String>,
        department_p:   Option<String>,
        shift_p:        Option<String>,
        salart_p:       Option<f64>,
        phoen_number_p: Option<String>,
        email_p:        Option<String>,
        status_p:       Option<bool>,
        note_p:         Option<String>,
        hire_date_p:    Option<NaiveDate>,
    ) -> Self {
        Self {
            id:             id_p,
            name:           name_p,
            surname:        surename_p,
            position:       position_p,
            department:     department_p,
            shift:          shift_p,
            salary:         salart_p,
            phone_number:   phoen_number_p,
            email:          email_p,
            status:         status_p,
            note:           note_p,
            hire_date:      hire_date_p,
        }
    }

    /// Vytvorí prázdneho zamestnanca (všetky polia `None`).
    ///
    /// Používa sa napríklad ako filter pre vyhľadávanie.
    ///
    /// # Returns
    /// Prázdny `Employee`
    pub fn new_empty() -> Self {
        Self {
            id:             None,
            name:           None,
            surname:        None,
            position:       None,
            department:     None,
            shift:          None,
            salary:         None,
            phone_number:   None,
            email:          None,
            status:         None,
            note:           None,
            hire_date:      None,
        }
    }

    /// Vypíše informácie o zamestnancovi na konzolu.
    fn print_employee(&self) {
        println!("ID: {}", self.id.map(|v| v.to_string()).unwrap_or("None".into()));
        println!("Name: {}", self.name.as_deref().unwrap_or("None"));
        println!("Surename: {}", self.surname.as_deref().unwrap_or("None"));
        println!("Position: {}", self.position.as_deref().unwrap_or("None"));
        println!("Department: {}", self.department.as_deref().unwrap_or("None"));
        println!("Shift: {}", self.shift.as_deref().unwrap_or("None"));
        println!("Salary: {}", self.salary.map(|v| v.to_string()).unwrap_or("None".into()));
        println!("Phone Number: {}", self.phone_number.as_deref().unwrap_or("None"));
        println!("Email: {}", self.email.as_deref().unwrap_or("None"));
        println!(
            "Status: {}",
            match self.status {
                Some(true) => "active",
                Some(false) => "inactive",
                None => "None",
            }
        );
        println!("Note: {}", self.note.as_deref().unwrap_or("None"));
        println!(
            "Hire date: {}",
            self.hire_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or("None".into())
        );
    }
}

impl Product {
    /// Vytvorí nový produkt.
    ///
    /// # Arguments
    /// Všetky polia produktu (ID, názov, cena, dátumy, atď.)
    ///
    /// # Returns
    /// Nový `Product`
    pub fn new(
        id_p:           Option<u32>,
        name_p:         Option<String>,
        category_p:     Option<String>,
        quantity_p:     Option<u32>,
        status_p:       Option<bool>,
        bar_code_p:     Option<i64>,
        cost_price_p:   Option<f64>,
        sell_price_p:   Option<f64>,
        description_p:  Option<String>,
        brand_p:        Option<String>,
        supplier_p:     Option<String>,
        employee_id_p:  Option<u32>,
        date_added_p:   Option<NaiveDate>,
        date_remove_p:  Option<NaiveDate>,
    ) -> Self {
        Self {
            id:             id_p,
            name:           name_p,
            category:       category_p,
            quantity:       quantity_p,
            status:         status_p,
            bar_code:       bar_code_p,
            cost_price:     cost_price_p,
            sell_price:     sell_price_p,
            description:    description_p,
            brand:          brand_p,
            supplier:       supplier_p,
            employee_id:    employee_id_p,
            date_added:     date_added_p,
            date_remove:    date_remove_p,
        }
    }

    /// Vytvorí prázdny produkt (všetky polia `None`).
    ///
    /// # Returns
    /// Prázdny `Product`
    pub fn new_empty() -> Self {
        Self {
            id:             None,
            name:           None,
            category:       None,
            quantity:       None,
            status:         None,
            bar_code:       None,
            cost_price:     None,
            sell_price:     None,
            description:    None,
            brand:          None,
            supplier:       None,
            employee_id:    None,
            date_added:     None,
            date_remove:    None,
        }
    }

    /// Vypíše informácie o produkte na konzolu.
    fn print_product(&self) {
        println!("ID: {}", self.id.map(|v| v.to_string()).unwrap_or("None".into()));
        println!("Name: {}", self.name.as_deref().unwrap_or("None"));
        println!("Category: {}", self.category.as_deref().unwrap_or("None"));
        println!("Quantity: {}", self.quantity.map(|v| v.to_string()).unwrap_or("None".into()));
        println!(
            "Status: {}",
            match self.status {
                Some(true) => "active",
                Some(false) => "inactive",
                None => "None",
            }
        );
        println!("Bar Code: {}", self.bar_code.map(|v| v.to_string()).unwrap_or("None".into()));
        println!("Cost Price: {}", self.cost_price.map(|v| format!("{:.2}", v)).unwrap_or("None".into()));
        println!("Sell Price: {}", self.sell_price.map(|v| format!("{:.2}", v)).unwrap_or("None".into()));
        println!("Description: {}", self.description.as_deref().unwrap_or("None"));
        println!("Brand: {}", self.brand.as_deref().unwrap_or("None"));
        println!("Supplier: {}", self.supplier.as_deref().unwrap_or("None"));
        println!("Date Added: {}", self.date_added.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or("None".into()));
        println!("Date Removed: {}", self.date_remove.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or("None".into()));
        println!();
    }
}
