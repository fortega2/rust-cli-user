use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn new() -> Self {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        
        println!("\nUsername:");
        stdin.read_line(&mut buffer).unwrap();
        let username = buffer.parse::<String>().unwrap();

        buffer = String::new();

        println!("\nPassword:");
        stdin.read_line(&mut buffer).unwrap();
        let password = buffer.parse::<String>().unwrap();

        buffer = String::new();

        println!("\nEmail:");
        stdin.read_line(&mut buffer).unwrap();
        let email = buffer.parse::<String>().unwrap();

        println!();
        
        Self {
            username,
            password,
            email,
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}Password: {}Email: {}", self.username, self.password, self.email)
    }
}

pub fn get_user_by_id(id: usize, users: &HashMap<usize, User>) -> Option<&User> {
    if id == 0 { return  None; }
    for user in users.iter() {
        if user.0 == &id {
            return Some(user.1);
        }
    }
    None
}

pub fn delete_user_by_id(id: usize, users: &mut HashMap<usize, User>) -> Option<i8> {
    if id == 0 { return  None; }
    match users.remove(&id) {
        Some(_) => Some(id as i8),
        None => None,
    }
}