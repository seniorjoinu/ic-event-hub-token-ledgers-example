use ic_cdk::caller;

use crate::get_state;

pub fn controller_guard() -> Result<(), String> {
    if get_state().controllers.contains(&caller()) {
        Ok(())
    } else {
        Err(String::from("The caller is not a controller"))
    }
}
