
mod database;
mod model;

fn main() {
    let connection = database::connect().expect("Could not connect to database");

    println!("Connected to database at {:?}", database::get_database_file_path());
}
