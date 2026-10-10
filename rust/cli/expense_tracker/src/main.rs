use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use expense_tracker::{*, utils::*};

fn main() -> io::Result<()> {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("Expenses.json")?;

    let mut input: String = String::new();
    f.read_to_string(&mut input)?;

    let mut expenses: ExpenseList = ExpenseList { list: Vec::new(), };
    match input.is_empty() {
        true => (),
        false => {
            expenses = serde_json::from_str(input.as_str())?;
        },
    };

    match args.get(0) {
        Some(op) => match op.as_str() {
            "add" => add(args, &mut expenses),
            "delete" => delete(args, &mut expenses),
            "list" => expenses.list(),
            "summary" => expenses.summary(),
            _ => return Ok(()),
        },
        None => return Ok(()),
    };

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("Expenses.json")?;

    f.write(serde_json::to_string::<ExpenseList>(&expenses)?.as_bytes())?;
    Ok(())
}
