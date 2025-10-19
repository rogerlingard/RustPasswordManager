use crate::PasswordEntry;
use std::io;
//This function is where we add the password to our Vector that holds Password Entries. First, we get the user input for the website and password.
//Then we make an entry from the user provided information to make a password entry data struct, and then push that entry into the vector of password entry.
pub fn add_password(password_vec: &mut Vec<PasswordEntry>) {
    //ChatGPT provided the &mut. This &mut lets us append to the passed in Vector that has our password entries. Since if we passed in the list then it would also transfer ownership, and we couldn't use it within main.
     println!("First add the website name: ");
     let mut website_name = String::new();
     io::stdin().read_line(&mut website_name).expect("Unable to read Standard input");
     let user_website_name = website_name.trim().to_string();
     //Have to add trim here because when you hit enter with the command prompt it adds a new line character to the end of the

     println!("Then add the website username: ");
     let mut username = String::new();
     io::stdin().read_line(&mut username).expect("Unable to read Standard input");
     let user_username = username.trim().to_string();

     println!("Then add the website password: ");
     let mut password = String::new();
     io::stdin().read_line(&mut password).expect("Unable to read Standard input");
     let user_password = password.trim().to_string();


     let entry = PasswordEntry {website: user_website_name, username: user_username,password: user_password};
    password_vec.push(entry);
     
 }