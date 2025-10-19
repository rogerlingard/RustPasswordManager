use std::io;
mod add_password;
mod read_file;
mod write_files;
mod remove_file;

pub struct PasswordEntry {
    pub website: String,
    pub username: String,
    pub password: String,
}

//https://www.tutorialspoint.com/rust/rust_structure.htm and also used some Chatgpt to help get an understanding of what I should use if I wanted to incorporate a data structure.
//This main function is about making a menu and then going to the other rs files that have functions within them. We use an input from the user to chose what function we are going to do.
fn main() {
    let mut input = String::new();
    // We are converting the string from input into 16 bits type. We change from string to int with parse
    //we also have a thing called a 'turbo fish' to help parse know what type we are converting to. We use
    //unwrap so we can deal with the program panicking from us not having an error case with result that the parse uses
    let mut password_vec: Vec<PasswordEntry> = read_file::read_file().unwrap();

    let mut running = true;
    while running {
        input.clear();
        //ChatGPT provided this clear code. I was trying to figure out how to clear the standard input.
        println!("1.Add a new Password Entry\n2.Show Entry List\n3.Save Entry\n4.Delete an entry\n0.Exit Program"); //println! is a marco we use
        println!("\n Choose a number to do that action: ");


        //Because this is a while loop we need to clear the output so that we can put in a new entry, so that we don't append strings we already have.
        io::stdin().read_line(&mut input).expect("Unable to read Standard input");

        //https://doc.rust-lang.org/std/io/struct.Stdin.html

        let user_choice =   input.trim().parse::<String>().unwrap();

        if user_choice == "1" {

            add_password::add_password(&mut password_vec);

        } else if user_choice == "2" {
            println!();
            for entry in &password_vec {
                //We add the & so we are just referencing the password list so the for loop doesn't take ownership and destroy it.
                println!("Website Name: {} - User Name: {} - Password: {}", entry.website, entry.username, entry.password);
            }
            println!();

        } else if user_choice == "3" {
            //chose entry to look into
            write_files::write_file(&password_vec).expect("Could not write the files");

            println!("Saved Passwords");

        } else if user_choice == "4" {
            // delete an entry
            remove_file::remove_file(&mut password_vec);
        }else if user_choice == "0" {
            println!("Exiting Program");
            running = false;
        } else {
            println!("That was a invalid option, try again")
        }
    }
}