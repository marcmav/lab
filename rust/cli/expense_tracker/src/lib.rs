use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Expense {
    id: u32,
    description: String,
    amount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ExpenseList {
    pub list: Vec<Expense>,
}

impl Expense {
    pub fn new(id: u32, description: String, amount: u32) -> Self {
        Self {
            id,
            description,
            amount,
        }
    }
}

impl ExpenseList {
    pub fn add(&mut self, expense: Expense) {
        let id = expense.id;
        self.list.push(expense);
        println!("Success: Added a new expense - ID: {id}");
    }

    pub fn delete(&mut self, id: u32) {
        let i: usize = self.list
            .iter()
            .position(|e| e.id ==  id)
            .expect("Error: ID does not exist");
        self.list.remove(i);
        println!("Success: Expense Deleted - ID: {i}");
    }

    pub fn list(&self) {
        println!("{:<7}{:^20}{:>10}", "ID", "Description", "Amount");
        self.list
            .iter()
            .for_each(|e| {
                println!("{:<7}{:^20}{:>10}", e.id, e.description, e.amount); });
    }

    pub fn summary(&self) {
        let sum: u32 = self.list
            .iter()
            .map(|e| e.amount)
            .sum();
        println!("Total expenses: {}kz", sum);
    }
}

pub mod utils;
