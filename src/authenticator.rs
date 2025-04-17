use crate::{
    admin_system::admin_system, buyer_system::buyer_system,
    company::company_indicator::CompanyIndicator, store::store_schema::StoreSchema,
    users::buyer_schema::BuyerSchema, utils::get_input,
};

pub fn authentication() -> Result<(), String> {
    let mut store = StoreSchema::new();
    let mut shoped_items = BuyerSchema::new();

    loop {
        CompanyIndicator::company_show();

        let auth_input: i32 = get_input("Enter User Type")?;

        match CompanyIndicator::company_indicator(auth_input) {
            Some(CompanyIndicator::Admin) => admin_system(&mut store, &mut shoped_items)?,
            Some(CompanyIndicator::User) => buyer_system(&mut shoped_items, &mut store)?,
            None => return Ok(()),
        }
    }
}
