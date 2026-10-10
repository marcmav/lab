use crate::*;

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
