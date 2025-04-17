mod admin_system;
mod authenticator;
mod buyer_system;
mod company;
mod controller;
mod store;
mod users;
mod utils;

use controller::controller;

fn main() {
    if let Err(err) = controller() {
        eprintln!("Application error: {}", err);
    }
}
