use crate::args::{
    EntryCommand,
   EntrySubcommands,
   CreateEntry,
    // UpdateEntry,
};
// db connection
use crate::db::establish_connection;
// db tables
use crate::models::{Entry,NewEntry};
// diesel
use diesel::prelude::*;

pub fn handle_show_command(){
    use crate::schema::entries::dsl::*;

    let mut db_conn = establish_connection();
    let results = entries
    .load::<Entry>(& mut db_conn)
        .unwrap();

    println!("\n");
    println!("Displaying {} entries",results.len());
    for entry in results {
        println!("✅{:?}",entry);
    }
}

// handle cli commands
pub fn handle_entry_command(entry: EntryCommand){
    let command = entry.command;
    match command {
        EntrySubcommands::Create(entry) => {
            create_entry(entry);
        }
        // EntrySubcommands::Update(entry) => {
        //     update_entry(entry);
        // }
    }
}
// handle new entry function
fn create_entry(entry: CreateEntry) {
    println!("Creating entry ✅{:?}",entry);
    use crate::schema::entries::dsl::*;

    let mut db_connect = establish_connection();
    let new_entry = NewEntry {
        name: &entry.name,
        invoice: &entry.invoice,
        paid:&entry.paid.unwrap(),
    };

    diesel::insert_into(entries)
    .values(&new_entry)
        .execute(& mut db_connect)
        .expect("❌ Error Saving entry");
}

// fn update_entry(entry: UpdateEntry){
//     println!("Updating entry {:?}",entry);
//     use crate::schema::entries::dsl::*;
//
//     let db_conn = establish_connection();
//     let updated_entry = Entry {
//         id:entry.id,
//         invoice: entry.invoice,
//         name: entry.name,
//         paid:entry.paid,    
//     };
//
//     diesel::update(entries.find(entry.id))
//         .set(&updated_entry)
//         .execute(& mut db_conn)
//         .expect("Error updating entry");
// }
