use std::collections::HashMap;

use super::store_model::StoreModel;

pub struct StoreSchema {
    store: HashMap<String, StoreModel>,
}

impl StoreSchema {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn add_to_store(&mut self, product: StoreModel) -> Result<(), String> {
        if self.store.contains_key(&product.name) {
            return Err("Product already exists".into());
        }

        self.store.insert(product.name.clone(), product);

        Ok(())
    }

    pub fn view_store(&mut self) -> Vec<&StoreModel> {
        self.store.values().collect()
    }

    pub fn get_single_product(&mut self, name: &String) -> Option<&StoreModel> {
        self.store.get(name)
    }

    pub fn delete_product_from_shop(&mut self, name: &String) -> Result<(), String> {
        self.store
            .remove(name)
            .map(|_| ())
            .ok_or("Product not found".into())
    }

    pub fn update_product_in_shop(
        &mut self,
        name: &String,
        price: f64,
        quantity: i32,
    ) -> Result<(), String> {
        match self.store.get_mut(name) {
            Some(item) => {
                item.price = price;
                item.quantity = quantity;

                Ok(())
            }
            None => Err("Product not found".into()),
        }
    }
}
