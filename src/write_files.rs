use std::io::Write;
use crate::PasswordEntry;
use std::fs::File;

/*So for this function we are passing in our vector with our password entries in it. We are doing the borrowing because we want to keep the ownership within main
We create the file to write to, Passwords.csv, and then use writeln marco to write to the file. We are making a new file each time we save just so we don't have to append each time we do something
We use a for loop to write to the file, using csv logic. Once again using the writeln macro to write to our file.
*/
pub fn write_file(password_vec: &Vec<PasswordEntry>) -> std::io::Result <()> {
    let mut file = File::create("Passwords.csv")?;

    writeln!(file, "website,username,password")?;

    for entry in password_vec {
        writeln!(file, "{},{},{}", entry.website, entry.username, entry.password)?;
        //I had tried to write similar to this before but had ChatGPT help me with figuring out how to input the file into the writeln syntax.
    }

    Ok(())
}