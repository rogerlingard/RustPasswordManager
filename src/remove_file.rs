use std::io;

use crate::PasswordEntry;
/* This is the remove function, where we remove an entry from the vectored passwordEntry. We do this by the website name
Which supposedly is going to be unique. We do this by getting the user input, then finding it within the password Entry using  the retain function
We are keeping track of the original Vec length to just double-check that removed the item and let the user know what was removed.
 */
pub fn remove_file(password_vec: &mut Vec<PasswordEntry>) {
    println!("First add the website name: ");
    let mut website_name = String::new();
    io::stdin().read_line(&mut website_name).expect("Unable to read Standard input");
    let user_website_name = website_name.trim().to_lowercase().to_string();
    //Have to add trim here because when you hit enter with the command prompt it adds a new line character to the end of the
    

    let original_length = password_vec.len();
    password_vec.retain(|entry| entry.website.trim().to_lowercase() != user_website_name);
    /*
    ChatGPT helped me with this line trying to find how to remove an entry from a vector without transferring the ownership
    Retain is an interesting function as it removes everything that is false, so we return true for everything that isn't the website name, and so the false is removed and the entries after it are moved left
    https://doc.rust-lang.org/std/vec/struct.Vec.html read more about it here
    */

    if password_vec.len() < original_length {
        println!("{} was removed from the list\n", user_website_name);
    } else {
        println!("Could not find {}. Try again, make sure that it matches the name.\n", user_website_name);
    }
    //ChatGPT gave me the idea to use the if statement to give the user feedback and use the list length to do so. I also think it is useful for debugging too

}