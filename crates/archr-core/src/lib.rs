pub mod model;
pub mod io {
    pub mod xml;
    pub mod yaml;
}
pub mod diff;
pub mod layout;
#[cfg(test)]
mod test_ecore;
pub mod validate;
