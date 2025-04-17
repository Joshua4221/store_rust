use buyer_model::BuyerModel;
use buyer_schema::BuyerSchema;

use crate::{store::store_schema::StoreSchema, utils::get_input};

pub mod buyer_model;
pub mod buyer_schema;
pub mod user_indicator;

pub fn buy_product(buyer: &mut BuyerSchema, store: &mut StoreSchema) -> Result<(), String> {
    if store.view_store().is_empty() {
        println!("store is empty");
    }

    let store_items = store.view_store();

    for item in store_items {
        println!(
            "{} of {} is of {} quantity",
            item.name, item.price, item.quantity
        );
    }

    let name: String = get_input("Enter Product Name")?;

    let product = store
        .get_single_product(&name)
        .ok_or_else(|| "Product not found.".to_string())?;

    let quantity: i32 = get_input("Enter Quantity")?;

    let main_price = product.price;
    let store_quantity = product.quantity;

    let price = &main_price * quantity as f64;

    if quantity > product.quantity {
        return Err("Quantity selected is more than what's available in the store.".to_string());
    }

    let product = BuyerModel {
        name: name.clone(),
        price,
        quantity,
    };

    buyer.select_product(product)?;

    store.update_product_in_shop(&name, main_price, store_quantity - quantity)?;

    println!("Product bought successfully.");

    Ok(())
}

pub fn view_buyer_product(buyer: &mut BuyerSchema) {
    if buyer.view_selected_product().is_empty() {
        println!("no product bought currently")
    }

    let buyer_items = buyer.view_selected_product();

    for item in buyer_items {
        println!(
            "{} of {} is of {} quantity",
            item.name, item.price, item.quantity
        );
    }
}

pub fn delete_buyer_product(buyer: &mut BuyerSchema) -> Result<(), String> {
    if buyer.view_selected_product().is_empty() {
        println!("no product bought currently")
    }

    let buyer_items = buyer.view_selected_product();

    for item in buyer_items {
        println!(
            "{} of {} is of {} quantity",
            item.name, item.price, item.quantity
        );
    }

    let name: String = get_input("Enter Bought Product Name:")?;

    buyer.delete_bought_product(&name)?;

    Ok(())
}

pub fn update_bought_product(
    buyer: &mut BuyerSchema,
    store: &mut StoreSchema,
) -> Result<(), String> {
    if buyer.view_selected_product().is_empty() {
        println!("no product bought currently")
    }

    let buyer_items = buyer.view_selected_product();

    for item in buyer_items {
        println!(
            "{} of {} is of {} quantity",
            item.name, item.price, item.quantity
        );
    }

    let name: String = get_input("Enter Product Name")?;

    let product = store
        .get_single_product(&name)
        .ok_or_else(|| "Product not found.".to_string())?;

    let quantity: i32 = get_input("Enter Quantity")?;

    let price = product.price * quantity as f64;

    buyer.update_bought_product(&name, price, quantity)?;

    Ok(())
}
