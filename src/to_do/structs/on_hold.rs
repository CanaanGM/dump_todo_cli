use super::base::Base;

use super::traits::get::Get;
use super::traits::edit::Edit;

pub struct OnHold{
    pub super_struct: Base
}

impl OnHold{
    pub fn new (
        input_title: &str
    ) -> OnHold {
        let base: Base = Base::new(input_title, "on-hold");
        return OnHold{super_struct:base}
    }
}

impl Get for OnHold {}
impl Edit for OnHold {}