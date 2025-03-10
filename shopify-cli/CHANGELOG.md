# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/Ventmere/shopify/releases/tag/shopify-cli-v0.1.0) - 2024-02-11

### Added
- new pagination support; upgrade Product & Variant api to 2020-07

### Fixed
- fix order Property::value type

### Other
- rustfmt and fix warnings ([#8](https://github.com/Ventmere/shopify/pull/8))
- add update_fulfillment_tracking, mark field optional.
- Fix fulfillment order creation field name
- Support new FulfillmentOrder API
- Shopify schema changes
- Order risk api
- upgrade clap
- shipping line source is optional
- add OrderApi::get
