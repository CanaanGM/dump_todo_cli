pub mod structs;
use structs::done::Done;
use structs::pending::Pending;
use structs::on_hold::OnHold;
pub enum ItemType {
    Pending(Pending),
    Done(Done),
    OnHold(OnHold)
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemType, &'static str> {
    if item_type == "pending"{
        let pending_item = Pending::new(item_title);
        Ok(ItemType::Pending(pending_item))
    }
    else if item_type == "done" {
        let done = Done::new(item_title);
        Ok(ItemType::Done(done))
    }
    else if item_type == "on-hold" {
        let on_hold = OnHold::new(item_title);
        Ok(ItemType::OnHold(on_hold))
    }
    else {
        Err("the item type isn't accepted")
    }
}