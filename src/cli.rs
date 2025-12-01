use clap::{Arg, Command};

pub fn parse() {
    let mut command = Command::new("todo")
        .version("1.0.0")
        .about("A simple todo-cli")
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(
                    Arg::new("name")
                        .help("Name of the task")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .required(true),
                )
                .arg(
                    Arg::new("deadline")
                        .help("Deadline in RFC3339 format")
                        .short('d')
                        .long("deadline")
                        .value_name("TIMESTAMP")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("done").about("Mark a task as done").arg(
                Arg::new("id")
                    .help("ID of the task")
                    .short('i')
                    .long("id")
                    .value_name("ID")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("undone").about("Mark a task as not done").arg(
                Arg::new("name")
                    .help("Name of the task")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .required(true),
            ),
        );
    let matches = command.clone().get_matches();

    if matches.subcommand_name().is_none() {
        command.print_help().unwrap();
    }


}
