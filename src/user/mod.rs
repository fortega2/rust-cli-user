use std::fmt;

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

    // pub fn view(&self) {
    //     println!("{}", self);
    // }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}Password: {}Email: {}", self.username, self.password, self.email)
    }
}