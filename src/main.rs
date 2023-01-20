mod user;
use std::{collections::HashMap, process::Command};
use user::*;

#[cfg(test)]
mod tests {
    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}

fn pause() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

fn menu() {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut users = HashMap::new();
    let mut id = 1_usize;

    loop {
        println!("1. Cargar usuario");
        println!("2. Ver usuarios");
        println!("3. Buscar usuario por ID");
        println!("0. Salir");
        println!("\nElegí una opción:");

        stdin.read_line(&mut buffer).expect("Error en input");

        let respuesta = buffer.trim().parse::<i8>().unwrap_or(-1);

        match respuesta {
            1 => {
                users.insert(id, User::new());
                id += 1;
            }
            2 => {
                println!();
                for user in &users {
                    println!("ID: {}\n{}", user.0, user.1);
                }
                pause();
                println!();
            }
            3 => {
                println!("\nIngrese el ID que desea buscar:");
                let mut buffer = String::new();
                stdin.read_line(&mut buffer).expect("Error en input");
                let id = buffer.trim().parse::<i8>().unwrap_or(0);
                match get_user_by_id(id as usize, &users) {
                    Some(user) => println!("\n{}", user),
                    None => println!("\nNo se ha encontrado el usuario con ID {id}\n"),
                }
                pause();
                println!();
            }
            -1 => {
                println!("\nOpción inválida\n");
            }
            0 | _ => break,
        }

        buffer = String::new();
    }
}

fn main() {
    menu();
}
