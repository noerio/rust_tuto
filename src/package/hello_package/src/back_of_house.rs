use crate::front_of_house;

#[allow(dead_code)]
pub fn fix_incorrect_order() {
    cook_order();
    front_of_house::serving::serve_order();
}

pub fn cook_order() {}
