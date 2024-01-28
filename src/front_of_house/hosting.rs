pub fn add_to_waitlist() {}

fn server_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::server_order(); // super 访问父级目录
    }
    fn cook_order() {}
}
