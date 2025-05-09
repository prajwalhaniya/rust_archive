use restaurant::front_of_house::{hosting, serving};

fn main() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}