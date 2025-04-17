use crate::authenticator::authentication;

pub fn controller() -> Result<(), String> {
    authentication()?;

    Ok(())
}
