use crate::{
    store::{store_schema::StoreSchema, view_products},
    users::{
        buy_product, buyer_schema::BuyerSchema, delete_buyer_product, update_bought_product,
        user_indicator::UserIndicator, view_buyer_product,
    },
    utils::get_input,
};

pub fn buyer_system(buyer: &mut BuyerSchema, store: &mut StoreSchema) -> Result<(), String> {
    loop {
        UserIndicator::user_show();

        let buyer_input: i32 = get_input("Enter Selection:")?;

        match UserIndicator::user_indicator(buyer_input) {
            Some(UserIndicator::SelectProduct) => buy_product(buyer, store)?,
            Some(UserIndicator::ViewSelectedProduct) => view_buyer_product(buyer),
            Some(UserIndicator::DeleteFromSelectedProduct) => delete_buyer_product(buyer)?,
            Some(UserIndicator::EditFromSelectedProduct) => update_bought_product(buyer, store)?,
            Some(UserIndicator::ViewListedProduct) => view_products(store),
            Some(UserIndicator::GoBack) => return Ok(()),
            None => println!("Invalid input. Please try again."),
        }
    }
}
