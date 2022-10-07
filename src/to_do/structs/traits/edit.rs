use serde_json::{Value, Map, json};

use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>){
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to done.\n\n", title);

    }

    fn set_to_pending(&self, title:&String, state: &mut Map<String, Value>){
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to pending\n\n", title);

    }

    fn set_to_on_hold(&self, title:&String, state: &mut Map<String, Value>){
        state.insert(title.to_string(), json!(String::from("on-hold")));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to on-hold\n\n", title);

    }
}