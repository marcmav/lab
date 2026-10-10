use crate::*;

pub fn add(args: Vec<String>, expenses: &mut ExpenseList) {
    match args.get(1) {
        Some(op) => match op.as_str() {
            "--description" => match args.get(3) {
                Some(op) => match op.as_str() {
                    "--amount" => (),
                    _ => return,
                }
                None => return,
            },
            _ => return,
        }
        None => return,
    }
    let description: String = match args.get(2) {
        Some(descr) => descr.to_string(),
        None => return,
    };
    let amount: u32 = match args.get(4) {
        Some(amou) => match amou.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return,
        }
        None => return,
    };
    let id: u32 = (expenses.list.len() as u32) + 1;

    expenses.add(Expense::new(id, description, amount));
}

pub fn delete(args: Vec<String>, expenses: &mut ExpenseList) {
    match args.get(1) {
        Some(op) => match op.as_str() {
            "--id" => (),
            _ => return,
        }
        None => return,
    };

    if let Some(_) = args.get(3) {
        return;
    }

    let id: u32 = match args.get(2) {
        Some(op) => match op.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return,
        }
        None => return,
    };

    expenses.delete(id);
}
