/*
Usando um hash map e vetores, crie uma interface de texto para permitir que um usuário adicione
nomes de funcionários para um departamento da empresa. Por exemplo, “Add Sally to Engineering” ou
“Add Amir to Sales”. Em seguida, deixe o usuário recuperar uma lista de todas
as pessoas de um departamento ou todas as pessoas na empresa por departamento,
ordenadas alfabeticamente.
 */

/*
Add -> Adicionar funcionário
Sales, Engineering, ... -> Listar todos os funcionários do departamento
All -> Listar todos os funcionários com respectivos departamentos em ordem alfabética
 */

extern crate colored;

use std::{
    collections::HashMap,
    io::{self, Write},
    str::Chars,
};

use colored::Colorize;
use terminal_emoji::Emoji;

fn capitalize_word(s: &str) -> String {
    let mut c: Chars = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn help_prompt() {
    println!(
        "\nType {} to add an employee",
        "'Add <name> to <department>'".blue().bold()
    );
    println!(
        "Type {} to list all of employees of that department",
        "'<department>'".blue().bold()
    );
    println!(
        "Type {} to see all employees and their respectives departments",
        "'All'".blue().bold()
    );
    println!(
        "Type {} or {} to show this prompt",
        "'Help'".green().bold(),
        "'?'".yellow().bold()
    );
    println!("Type {} to quit\n", "'Quit'".red().bold());
}

enum Opt {
    Add { name: String, dpt: String },
    Department(String),
    All,
    Help,
    Quit,
    // TODO: Alter,
    // TODO: Delete,
}

impl Opt {
    fn split_employee(s: &str) -> Option<Self> {
        let e: Vec<&str> = s.trim().split(' ').collect();

        match e.as_slice() {
            ["Add", name, "to", dpt] => Some(Opt::Add {
                name: capitalize_word(name),
                dpt: capitalize_word(dpt),
            }),
            ["Quit"] => Some(Opt::Quit),
            ["All"] => Some(Opt::All),
            ["Help" | "?"] => Some(Opt::Help),
            [dpt] => Some(Opt::Department(dpt.to_string())),
            _ => None,
        }
    }
}

fn main() {
    let mut employees_list: HashMap<String, Vec<String>> = HashMap::new();

    help_prompt();

    loop {
        let mut input: String = String::new();

        let arrow_emoji: Emoji = Emoji::new("▶️", ">");
        print!("{} ", arrow_emoji.0);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match Opt::split_employee(&capitalize_word(&input)) {
            Some(Opt::Add { name, dpt }) => {
                for (_dpt, names) in &employees_list {
                    for n in names {
                        if n.to_owned() == name {
                            let alert_emoji = Emoji::new("🛑", "a");
                            println!(
                                "{} {}",
                                alert_emoji.0,
                                "An employee with this name already exists!".red().bold()
                            );
                            // TODO: Check if name already exists.
                        }
                    }
                }

                employees_list
                    .entry(dpt)
                    .or_insert_with(Vec::new)
                    .push(name);

                let check_emoji: Emoji = Emoji::new("✅", "v");
                println!(
                    "{} {}\n",
                    check_emoji.0,
                    "Successfully added!".green().bold()
                );
            }

            Some(Opt::Department(dpt)) => match employees_list.get(&dpt) {
                Some(names) => {
                    println!("\n---- {} ----", dpt.to_uppercase().blue().bold());
                    for name in names {
                        println!(" {name}");
                    }
                    println!("\n");
                }
                None => {
                    let warning_emoji: Emoji = Emoji::new("⚠️", "w");
                    println!(
                        "\n{}  {}\n",
                        warning_emoji.0,
                        "Department not found!".yellow().bold()
                    );
                }
            },

            Some(Opt::All) => {
                for (dpt, names) in &employees_list {
                    println!("\n----- {} -----", dpt.to_uppercase().blue().bold());
                    let mut names: Vec<String> = names.clone();
                    names.sort();
                    for name in names {
                        println!("{name}");
                    }
                    println!("\n");
                }
            }

            Some(Opt::Help) => help_prompt(),

            Some(Opt::Quit) => break,

            None => {
                let error_emoji: Emoji = Emoji::new("🚫", "e");
                println!(
                    "\n{} {}\n",
                    error_emoji.0,
                    "[ERROR]: Invalid Syntax!".red().bold()
                );
            }
        }
    }
}
