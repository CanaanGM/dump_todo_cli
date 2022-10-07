use serde_json::Map;
use serde_json::value::Value;
use crate::to_do::ItemType;

use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::on_hold::OnHold;

use super::to_do::structs::traits::get::Get;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;

fn process_pending(item: Pending, command: String, state: &Map<String, Value>){
    let mut state = state.clone();

    match command.as_str(){
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status, &mut state) ,
        "done" => item.set_to_done(&item.super_struct.title, &mut state),
        "on-hold" => item.set_to_on_hold(&item.super_struct.title,&mut state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command )
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>){
    let mut state = state.clone();

    match command.as_str(){
        "get" => item.get(&item.super_struct.title, &state),
        "resume" => item.set_to_pending(&item.super_struct.title, &mut state),
        "on-hold" => item.set_to_on_hold(&item.super_struct.title,&mut state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command )
    }
}
fn process_on_hold(item: OnHold, command: String, state: &Map<String, Value>){
    let mut state = state.clone();

    match command.as_str(){
        "get" => item.get(&item.super_struct.title, &state),
        "done" => item.set_to_done(&item.super_struct.title, &mut state),
        "resume" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command )
    }
}

pub fn process_input(item: ItemType, command:String, state: &Map<String, Value>){
    match item {
        ItemType::Pending(item) => process_pending(item, command, state),
        ItemType::Done(item) => process_done(item, command, state),
        ItemType::OnHold(item) => process_on_hold(item, command, state)
    }
}