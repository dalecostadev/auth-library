// src/lib.rs

pub mod services;
pub use services::auth_service::AuthService; // Re-export AuthService so users of the library can access it directly

// Optionally, you can also export Auth and other models if they need to be accessed directly
// pub mod models;
// pub use models::auth_model::Auth;