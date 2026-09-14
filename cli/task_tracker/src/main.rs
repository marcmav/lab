#![allow(unused_variables)] //cant go in production;

use task_tracker::*;
use std::env;

fn main() {
    let args: Vec<String> =env::args().skip(1).collect();
    //TODO accept input as the numbers assigned
    //TODO accept multiple parameters later
    match args.get(0) {
        Some(s) => match s.as_str() {
            "--init" => { init_task_tracker(); },
            /*
            "--add" => //call add function,
            "--update" => //call update function,
            "--delete" => //call delete function,
            */
            _ => {
                println!("Invalid Input!");
                println!("Program expects either: '<--init>, <--add>, <--update>, <--delete>'");
                return;
            },
        },
        None => init_task_tracker(),
    }
}
