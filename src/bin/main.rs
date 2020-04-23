//#[macro_use]
extern crate serde_derive;
extern crate csv;

mod koelner_phonetik;

use koelner_phonetik::get_phonetic_code;
use std::error::Error;
use std::io;
use csv::{Reader,Writer};
use std::fs::OpenOptions;
use std::thread;

struct PhoneBook {

}

impl PhoneBook {
    fn list_commands() {
        println!("list - lists all saved contacts");
        println!("find name - finds a contact by name");
        println!("find number - searches for a contact by number");
        println!("add - saves a new contact entry into the phone book");
        println!("edit - modifies an existing contact");
        println!("delete - removes a contact from the phone book");
        println!("help - lists all valid commands");
        println!("---------------------------");
    }

    fn list_contacts() -> Result<(), Box<Error>> {
        let mut rdr = Reader::from_path("src/bin/contacts.csv")?;
        for result in rdr.records() {
            let record = result?;
            println!("name:               {}", &record[0]);
            println!("phone number 1:     {}", &record[1]);
            println!("phone number 2:     {}", &record[2]);
            println!("phone number 3:     {}", &record[3]);
            println!("street:             {}", &record[4]);
            println!("zip:                {}", &record[5]);
            println!("town:               {}", &record[6]);
            println!("=====================================");
        }
        Ok(())
    }

    fn list_contacts_fix() {
        PhoneBook::list_contacts();
    }

    fn find_contact_by_name(name: &str) -> Result<(), Box<Error>> {
        let mut rdr = Reader::from_path("src/bin/contacts.csv")?;
        let mut contact_found = false;
        for result in rdr.records() {
            let record = result?;
            let record_name = &record[0];
            let phonetik_name = &get_phonetic_code(record_name)[..];
                 if phonetik_name == name {
                    contact_found = true;
                    println!("name:               {}", &record[0]);
                    println!("phone number 1:     {}", &record[1]);
                    println!("phone number 2:     {}", &record[2]);
                    println!("phone number 3:     {}", &record[3]);
                    println!("street:             {}", &record[4]);
                    println!("zip:                {}", &record[5]);
                    println!("town:               {}", &record[6]);
                    println!("=====================================");
                }
        }
        if !contact_found { println!("No contact found.") }
        Ok(())
    }

    fn find_contact_by_name_fix() {
        println!("Enter the name you are looking for or 'quit' to abort.");
        loop {
            let input = read_user_input().to_owned();
            let static_input: &str = &input[..];
            let phonetik_code: &str = &get_phonetic_code(static_input)[..];
            if input == "quit".to_string() { main_menu() } else { PhoneBook::find_contact_by_name(phonetik_code); }
        }
    }

    fn find_contact_by_number(number: &str) -> Result<(), Box<Error>> {
        let mut rdr = Reader::from_path("src/bin/contacts.csv")?;
        let mut contact_found = false;

        for result in rdr.records() {
            let record = result?;
            let record_number_1 = &record[1];
            let record_number_2 = &record[2];
            let record_number_3 = &record[3];
            match number {
                _ if (number == record_number_1) || (number == record_number_2) || (number == record_number_3) => {
                    contact_found = true;
                    println!("name:               {}", &record[0]);
                    println!("phone number 1:     {}", &record[1]);
                    println!("phone number 2:     {}", &record[2]);
                    println!("phone number 3:     {}", &record[3]);
                    println!("street:             {}", &record[4]);
                    println!("zip:                {}", &record[5]);
                    println!("town:               {}", &record[6]);
                    println!("=====================================");
                },

                _ => ()
            }
        }
        if !contact_found { println!("No contact found.") }
        Ok(())
    }

    fn find_contact_by_number_fix() {
        println!("Enter the number you are looking for or 'quit' to abort.");
        loop {
            let input = read_user_input().to_owned();
            let static_input: &str = &input[..];
            if input == "quit".to_string() { main_menu() } else { PhoneBook::find_contact_by_number(static_input); }
        }
    }

    fn add_contact(name: &str, phone_number_1: &str, phone_number_2: &str, phone_number_3: &str, street: &str, zip: &str, town: &str) -> Result<(), Box<Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("src/bin/contacts.csv")
            .unwrap();
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_record(&[name, phone_number_1, phone_number_2, phone_number_3, street, zip, town])?;
        wtr.flush()?;
        Ok(())
    }

    fn add_contact_fix() {
        println!("You are about to add a new contact to the phone book. Type 'quit' to abort.");
        println!("Enter contact name:");
        let input = read_user_input().to_owned();
        if input == "quit".to_string() { main_menu() }
        let name: &str = &input[..];
        println!("Enter contact number 1:");
        let input = read_user_input().to_owned();
        if input == "quit".to_string() { main_menu() }
        let number_1: &str = &input[..];
        println!("Enter contact number 2:");
        let input = read_user_input().to_owned();
        if input == "quit".to_string() { main_menu() }
        let number_2: &str = &input[..];
        println!("Enter contact number 3:");
        let input = read_user_input().to_owned();
        if input == "quit".to_string() { main_menu() }
        let number_3: &str = &input[..];
        println!("Enter contact street:");
        let input = read_user_input().to_owned();
        if input == "quit".to_string() { main_menu() }
        let street: &str = &input[..];
        println!("Enter contact zip:");
        let input = read_user_input().to_owned();
        if input == "quit".to_string() { main_menu() }
        let zip: &str = &input[..];
        println!("Enter contact town:");
        let input = read_user_input().to_owned();
        if input == "quit".to_string() { main_menu() }
        let town: &str = &input[..];

        PhoneBook::add_contact(name, number_1, number_2, number_3, street, zip, town);
        println!("Contact saved successfully.")
    }

    fn delete_contact_by_name(name:&str) -> Result<(), Box<Error>> {
        let path = "src/bin/contacts.csv";
        let mut contact_deleted = false;
        let mut record_vec = Vec::new();
        let mut rdr = Reader::from_path(path)?;
        for result in rdr.records() {
            let record = result?;
            let record_name = &record[0];
            if !(name == record_name) {record_vec.push(record);}
            else {contact_deleted = true;}
        }
        let mut wtr =  Writer::from_path(path)?;
        //header erneut schreiben
        wtr.write_record(&["name","phone_number_1","phone_number_2","phone_number_3","street","zip","town"])?;
        for x in record_vec {
            wtr.write_record(&x)?;
        }
        if contact_deleted {println!("Contact '{}' deleted successfully.",name);}
        else {println!("No contact found.");}
        wtr.flush()?;
        Ok(())
    }

    fn delete_contact_by_name_fix() {
        loop {
            println!("Enter name of the contact to be deleted or 'quit' to abort.");
            let input = read_user_input().to_owned();
            let static_input: &str = &input[..];
            if input == "quit".to_string() {
                main_menu()
            }
            else {
                PhoneBook::delete_contact_by_name(static_input); }
        }

    }




}


fn main()  {
  main_menu()
}


pub fn main_menu() {
    println!("RUST PHONE BOOK");
    println!("=======================");
    println!("Type a command or 'quit' to exit:");
    PhoneBook::list_commands();
    println!("> ");

    loop {
        let input = read_user_input();
        match input.as_ref() {
            "list" =>        PhoneBook::list_contacts_fix(),
            "find name" =>   PhoneBook::find_contact_by_name_fix(),
            "find number" => PhoneBook::find_contact_by_number_fix(),
            "add" =>         PhoneBook::add_contact_fix(),
            "delete" =>      PhoneBook::delete_contact_by_name_fix(),
            "help" =>        PhoneBook::list_commands(),
            "quit" => std::process::exit(1),
            _ => println!("Please enter a valid command.")
        }
    }
}


fn read_user_input() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            return input.trim().to_string();
        }
        Err(_error) =>{
            return "(-_-)".to_string();
        }
    }




}


