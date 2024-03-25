// ---------------------------------------------------------------------------
//
// Contacts
//
// This small command line tool searches the ~/contacts.txt file for any match
// to the provided input, and shows the whole contact.
//
// ---------------------------------------------------------------------------

mod contact;

use contact::Contact;
use std::env;

fn main() {
    // we only accept a single argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Single argument accepted");
        return;
    };
    let search_text = args.last().unwrap();

    // load the list of contacts
    let contact_list = Contact::load_contacts();

    // iterate over the list to see if we can find our search text; each
    // contact that matches is printed
    contact_list.iter().for_each(|contact| {
        if contact.is_match(search_text) {
            println!("{}", contact);
        }
    });
}
