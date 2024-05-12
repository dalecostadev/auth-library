# auth-library
Rust library to implement auth using JWT bearer token.

# Install WASM

rustup target add wasm32-unknown-unknown

cargo install wasm-pack

# Project structure

src/
│
├── lib.rs                 # Configuring module exports
├── models/                # Directory for data models
│   └── auth_model.rs      # Contains Auth model class
└── services/              # Directory for service logic
    └── auth_service.rs    # Contains AuthService class