pub mod store_indicator;
pub mod store_model;
pub mod store_schema;

use store_model::StoreModel;
use store_schema::StoreSchema;

use crate::utils::get_input;

pub fn create_product(store: &mut StoreSchema) -> Result<(), String> {
    let name: String = get_input("Enter Name")?;

    let price: f64 = get_input("Enter Price")?;

    let quantity: i32 = get_input("Enter Quantity")?;

    let product = StoreModel {
        name,
        price,
        quantity,
    };

    store.add_to_store(product)?;

    Ok(())
}

pub fn view_products(store: &mut StoreSchema) {
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
}

pub fn delete_product(store: &mut StoreSchema) -> Result<(), String> {
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

    let name: String = get_input("Enter Product Name:")?;

    store.delete_product_from_shop(&name)?;

    Ok(())
}

pub fn update_product(store: &mut StoreSchema) -> Result<(), String> {
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

    let name: String = get_input("Enter Product Name:")?;

    let price: f64 = get_input("Enter Product Price:")?;

    let quantity: i32 = get_input("Enter Product Quantity:")?;

    store.update_product_in_shop(&name, price, quantity)?;

    Ok(())
}
