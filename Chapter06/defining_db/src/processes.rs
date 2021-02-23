use serde_json::Map;
use serde_json::value::Value;

use super::to_do::ItemTypes;
use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::get::Get;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;


/// This function processes the command on a pending to do item.
///
/// # item
/// * item (Pending): the to do item to be processed
/// * command (String): command to be acted on the to do item
/// * state (&Map<String, Value>): the state of the to do items for the program
///
/// # Returns
/// None
pub fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status, &mut state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command)
    }
}


/// This function processes the command on a done to do item.
///
/// # item
/// * item (Pending): the to do item to be processed
/// * command (String): command to be acted on the to do item
/// * state (&Map<String, Value>): the state of the to do items for the program
///
/// # Returns
/// None
pub fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command)
    }
}


/// This function processes the user input to decide which function to operate.
///
/// # Arguments
/// * item (ItemTypes): one of the to do item types to be processed
/// * command (String): the command to be acted on the to do item
/// * state (&Map<String, Value>): the state of the to do item for the program
///
/// # Returns
/// None
pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state)
    }
}
