# Contacts

This small command line tool searches the ~/contacts.txt file for any match to the provided input, and shows the whole contact.

The contact file must be formatted as follows:

```
«contact-name»

    «detail-1»
    «detail-2»
    ...
    «detail-n»
```

Here's an example:

```
Local Laws

    Phone:  0407 508 448
    Org:    Central Goldfields Shire Council
    Role:   Local laws
    Role:   Ranger
```

If the search matches any text in that entire “Local Laws” contact's details, the whole contact will be listed as a match.

Note that the details you associate with the contact can be anything you like. I build mine using a key and value pair, but that's not mandatory at all. The only requirement is that you have the name of the contact at the start of the line, and the contact's details are indented. Spaces are better than tabs, as you'll get more consistent formatting when the contact is printed.

## Running the search

From the command line, a single argument is expected:

```
contacts «search»
```

The `«search»` argument specifies what to search for. All matching contacts are displayed. For example:

```
$ contacts ranger
Local Laws

    Phone:      0407 508 448
    Org:        Central Goldfields Shire Council
    Role:       Local laws
    Role:       Ranger

$
```