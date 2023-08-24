![Crates.io](https://img.shields.io/crates/l/shopify/0.1.0)
![Crates.io](https://img.shields.io/crates/v/shopify)
![Crates.io](https://img.shields.io/crates/d/shopify)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Ventmere/shopify/ci.yml)

# shopify


Shopify is an unofficial sdk for the Shopify e-commerce platform written in rust.

## Installation
```rust
cargo install shopify
```

## Usage 
```rust
use shopify::client::Client;
use shopify::shop;

// Create a new client
let client = Client::new(
    "SHOPIFY_BASE_URL",
    "SHOPIFY_API_KEY",
    "SHOPIFY_PASSWORD"
);

// Retrieve shop details
let shop = shop::ShopApi::get(&client);

```

## License
This project is license under an MIT license