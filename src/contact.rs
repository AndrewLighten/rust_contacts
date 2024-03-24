/// This structure represents a contact that's been fetched from the contacts
/// file.

use dirs;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Contact {

    /// The contact's name
    pub name: String,

    /// Additional strings that the contact has (phone number, nickname, role,
    /// etc.)
    pub additional: Vec<String>,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the fields of MyStruct as a string
        write!(f, "{}\n\n{}\n", self.name, self.additional.join("\n"))
    }
}

impl Contact {

    /// Creates a new contact.
    /// 
    /// # Arguments
    /// 
    /// `name` - The name of the contact.
    /// 
    /// # Returns
    /// 
    /// A newly created contact with just a name. The contact's additional
    /// information is an empty vector.
    
    pub fn new(name: String) -> Contact {
        Contact{name, additional: vec![]}
    }

    /// Loads a vector of contacts from the well-known contact file.
    ///
    /// The file is assumed to be named “contacts.txt”, and is expected
    /// to be located in the user's home directory.
    /// 
    /// See the README.md file for the expected contact file format.
    /// 
    /// # Returns
    /// 
    /// A vector of contacts. If any errors occur, this vector will
    /// be empty.

    pub fn load_contacts() -> Vec<Contact> {

        // contact vector
        let mut contact_list: Vec<Contact> = vec![];
        
        // find the user's home directory
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => {
                println!("Cannot identify home directory");
                return contact_list;
            }
        };

        // construct the full path to the contacts file
        let contacts_path = home_dir.join("contacts.txt");

        // open the contacts.txt file
        let file = File::open(contacts_path).expect("File open");
        let reader = BufReader::new(file);

        // process each file line
        for line in reader.lines() {
            
            // fetch the line
            let content = line.expect("Wanted a file line");

            // ignore blank lines
            if content.is_empty() {
                continue;
            }

            // if the string doesn't have any leading whitespace, it's the
            // start of a new contact
            if !content.starts_with(" ") {
                contact_list.push(Contact::new(content));
                continue;
            }

            // the line doesn't start with whitespace, so it's an additional
            // line for the contact we're currently building up
            match contact_list.last_mut() {
                Some(last_element) => {
                    (*last_element).additional.push(content);
                }
                None => {
                    println!("Found additional data before the first contact");
                }
            }
        }

        // return our vector of contacts
        contact_list
    }

    /// Checks whether the given search string is contained in the contact.
    /// 
    /// The search string is converted to lowercase before being compared to
    /// the contact's details.
    /// 
    /// # Arguments
    /// 
    /// * `search` - A string slice to search for in this contact.
    /// 
    /// # Returns
    /// 
    /// True if the needle is within the contact; otherwise, false.

    pub fn is_match(&self, search: &str) -> bool {

        // get a lowercase representation of the needle
        let needle_lower = search.to_lowercase();

        // check the name
        if self.name.to_lowercase().contains(&needle_lower) {
            return true;
        }

        // check the additional details
        for x in &self.additional {
            if x.to_lowercase().contains(&needle_lower) {
                return true;
            }
        }
        // no match
        return false;
    }

}