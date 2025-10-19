use std::fs::File;
use crate::PasswordEntry;
use csv::ReaderBuilder;
//So for read_file, we are opening up our Passwords.csv to get information about out Vector PasswordEntry
pub fn read_file() -> Result<Vec<PasswordEntry>, csv::Error>  {
    /*Used ChatGPT to help me figure out how to handle the second parameter for the Error so that it would return a result.
    Basically when you use result you have to return an error because Result can either pass with the value you want or return an error if it doesn't.
    So just in case we will pass the csv::Error if we fail to do a csv read. Rust is very particular about catching errors before they happen, so we have to provide it beforehand.*/
    let file_name = "Passwords.csv";
    let file = File::open(file_name)?;
    //This line was provided by chatGPT as I was figuring out how I wanted to read the file and what avenue to use and I slowly had mixed them all together by accident.
    //So for line we are making a new immutable file variable where we assign the actual file to the file variable.
    let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(file);
    /*Provided by https://docs.rs/csv/latest/csv/struct.Reader.html
    This is from the csv library using the ReaderBuilder struct to read the file that we opened. So we creat a new one
    Then we use the delimiter to tell the reader what we are going to be reading, so for a csv we are seperated by commas, so we're using comma's to tell us
    What the variables that are stored inside of it are. */
    let mut password_vec = Vec::new();

    for record in reader.records(){
        //Here we are using the reader we created to get the individual entries that are inside. ReaderBuilder is 
        let website_info = record?;
        let entry = PasswordEntry{
            website: website_info.get(0).unwrap_or_default().to_string(),
            username: website_info.get(1).unwrap_or_default().to_string(),
            password: website_info.get(2).unwrap_or_default().to_string()
            //ChatGPT changed website_info.get(1).unwrap().parse().unwrap() to website_info.get(0).unwrap_or_default().to_string() to help me get the data type from the csv file and be able to make it into a string.
        };

        password_vec.push(entry)
    }
    Ok(password_vec)
}