mod server;
mod config;
mod from_doc;
mod directive;
mod blueprint;

pub fn is_default<T: Default + Eq>(val: &T) -> bool {
    *val == T::default()
}
