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

use std::{
    collections::HashMap,
    io::{self, Write},
    str::Chars,
};

fn capitalize(s: &str) -> String {
    let mut c: Chars = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

enum Opt {
    Add { name: String, dpt: String },
    Department(String),
    All,
    Quit,
}

impl Opt {
    fn split_employee(s: &str) -> Option<Self> {
        let e: Vec<&str> = s.trim().split(' ').collect();

        match e.as_slice() {
            ["Add", name, "to", dpt] => Some(Opt::Add {
                name: capitalize(name),
                dpt: capitalize(dpt),
            }),
            ["Quit"] => Some(Opt::Quit),
            ["All"] => Some(Opt::All),
            [dpt] => Some(Opt::Department(dpt.to_string())),
            _ => None,
        }
    }
}

fn main() {
    let mut employees_list: HashMap<String, Vec<String>> = HashMap::new();

    println!("\nType 'Add <name> to <department>' to add an employee");
    println!("Type '<department>' to list all of employees of that department");
    println!("Type 'All' to see all employees and their respectives depertments");
    println!("Type 'Quit' to quit\n");

    loop {
        let mut input: String = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match Opt::split_employee(&input) {
            Some(Opt::Add { name, dpt }) => employees_list
                .entry(dpt)
                .or_insert_with(Vec::new)
                .push(name),

            Some(Opt::Department(dpt)) => match employees_list.get(&dpt) {
                Some(names) => {
                    println!("\n---- {} ----", dpt.to_uppercase());
                    for name in names {
                        println!(" {name}");
                    }
                    println!("\n");
                }
                None => println!("\nDepartment not found!\n"),
            },

            Some(Opt::All) => {
                for (dpt, names) in &employees_list {
                    println!("\n----- {} -----", dpt.to_uppercase());
                    let mut names: Vec<String> = names.clone();
                    names.sort();
                    for name in names {
                        println!("{name}");
                    }
                    println!("\n");
                }
            }

            Some(Opt::Quit) => break,
            None => println!("[ERROR]: Invalid Syntax!"),
        }
    }
}
