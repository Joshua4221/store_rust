use crate::{
    store::{
        create_product, delete_product, store_indicator::StoreIndicator, store_schema::StoreSchema,
        update_product, view_products,
    },
    users::{buyer_schema::BuyerSchema, view_buyer_product},
    utils::get_input,
};

pub fn admin_system(store: &mut StoreSchema, buyer: &mut BuyerSchema) -> Result<(), String> {
    loop {
        StoreIndicator::admin_show();

        let admin_input: i32 = get_input("Enter admin selection")?;

        match StoreIndicator::indicator(admin_input) {
            Some(StoreIndicator::CreateProduct) => create_product(store)?,
            Some(StoreIndicator::ViewProduct) => view_products(store),
            Some(StoreIndicator::DeleteProduct) => delete_product(store)?,
            Some(StoreIndicator::UpdateProduct) => update_product(store)?,
            Some(StoreIndicator::ViewBuyerOrder) => view_buyer_product(buyer),
            Some(StoreIndicator::GoBack) => return Ok(()),
            None => println!("Invalid input. Please try again."),
        }
    }
}
