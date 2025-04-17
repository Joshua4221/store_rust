use std::collections::HashMap;

use super::buyer_model::BuyerModel;

pub struct BuyerSchema {
    buyers: HashMap<String, BuyerModel>,
}

impl BuyerSchema {
    pub fn new() -> Self {
        Self {
            buyers: HashMap::new(),
        }
    }

    pub fn select_product(&mut self, product: BuyerModel) -> Result<(), String> {
        if self.buyers.contains_key(&product.name) {
            return Err("Product already exists".into());
        }

        self.buyers.insert(product.name.clone(), product);

        Ok(())
    }

    pub fn view_selected_product(&mut self) -> Vec<&BuyerModel> {
        self.buyers.values().collect()
    }

    // pub fn get_single_buyer_product(&mut self, name: &String) -> Option<&BuyerModel> {
    //     let data = self.buyers.get(name);

    //     data
    // }

    pub fn delete_bought_product(&mut self, name: &String) -> Result<(), String> {
        self.buyers
            .remove(name)
            .map(|_| ())
            .ok_or("Product not found".into())
    }

    pub fn update_bought_product(
        &mut self,
        name: &String,
        price: f64,
        quantity: i32,
    ) -> Result<(), String> {
        match self.buyers.get_mut(name) {
            Some(item) => {
                item.price = price;
                item.quantity = quantity;

                Ok(())
            }
            None => Err("Product not found".into()),
        }
    }
}
