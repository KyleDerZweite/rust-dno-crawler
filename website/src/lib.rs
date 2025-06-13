pub mod components;
pub mod layout;
pub mod routes;

// handlers module only needed for server features
#[cfg(feature = "server")]
pub mod handlers;