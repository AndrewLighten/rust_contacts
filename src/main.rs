// ---------------------------------------------------------------------------
//
// Contacts
//
// This small command line tool searches the ~/contacts.txt file for any match
// to the provided input, and shows the whole contact.
//
// The contact file must be formatted as follows:
//
// «contact-name»
//
//      «key»:      «value»
//      «key»:      «value»
//      ...
//
// Here's an example:
//
// Local Laws
//
//      Phone:  0407 508 448
//      Org:    Central Goldfields Shire Council
//      Role:   Local laws
//      Role:   Ranger
//
// If the search matches any text in that entire “Local Laws” contact's details,
// the whole contact will be listed as a match.
//
// ---------------------------------------------------------------------------

mod contact;

use std::env;
use contact::Contact;

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
