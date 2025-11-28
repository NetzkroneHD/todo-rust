use chrono::Local;

mod database;
mod model;

fn main() {
    let connection = database::connect().expect("Could not connect to database");
    println!("Connected to database at {:?}", database::get_database_file_path());

    database::create_tables(&connection).expect("Could not create tables");
    for i in 0..1000 {
        database::create_task(&connection, format!("name {}", i), false, Some(Local::now().with_timezone(Local::now().offset()))).expect("Could not create task");
    }
    let tasks = database::get_all_tasks(&connection).expect("Could not get all tasks");
    tasks.iter().for_each(|task| println!("{}", task));
}