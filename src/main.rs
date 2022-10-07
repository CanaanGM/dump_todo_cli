mod to_do;

use to_do::ItemType;
use to_do::to_do_factory;

fn main() {
    let to_do_item: Result<ItemType, &'static str> = to_do_factory("done", "make");
    match to_do_item.unwrap() {
        ItemType::Pending(item) => println!("Its a {} item with the title {}.", item.super_struct.status, item.super_struct.title),
        ItemType::Done(item) => println!("Its a {} item with the title {}.", item.super_struct.status, item.super_struct.title),
    }

    println!("Hello, world!");
}
