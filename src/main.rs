mod database;
mod model;
mod cli;

use chrono::Local;

fn main() {
    let connection = database::connect().expect("Could not connect to database");
    println!("Connected to database at {:?}", database::get_database_file_path());

    database::create_tables(&connection).expect("Could not create tables");

    cli::parse();

    for i in 0..9 {
        let time = Local::now();
        database::create_task(&connection, format!("name {}", i), false, Some(time.with_timezone(time.offset()))).expect("Could not create task");
    }
    let tasks = database::get_all_tasks(&connection).expect("Could not get all tasks");
    for x in tasks {
        x.get_id();
    }
}
