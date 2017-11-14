use types::{DateTime, Utc, Value};

/// The Shopify API's shop object is a collection of the general settings and information about the shop.
#[derive(Debug, Deserialize)]
pub struct Shop {
  /// A unique numeric identifier for the shop.
  id: i64,
  /// The shop's street address.
  address1: String,
  /// The shop's additional street address (apt, suite, etc.).
  address2: String,
  /// The city in which the shop is located.
  city: String,
  /// The shop's country (by default equal to the two-letter country code).
  country: String,
  /// The two-letter country code corresponding to the shop's country.
  country_code: String,
  /// The shop's normalized country name.
  country_name: String,
  /// The date and time when the shop was created. The API returns this value in ISO 8601 format.
  created_at: DateTime<Utc>,
  /// The date and time when the shop was last updated. The API returns this value in ISO 8601 format.
  updated_at: DateTime<Utc>,
  /// The customer's email.
  customer_email: String,
  /// The three-letter code for the currency that the shop accepts.
  currency: String,
  /// The shop's domain.
  domain: String,
  /// The contact email address for the shop.
  email: String,
  /// Feature is present when a shop has a google app domain. It will be returned as a URL. If the shop does not have this feature enabled it will default to "null."
  google_apps_domain: Option<String>,
  /// Feature is present if a shop has google apps enabled. Those shops with this feature will be able to login to the google apps login. Shops without this feature enabled will default to "null."
  google_apps_login_enabled: Option<bool>,
  /// Geographic coordinate specifying the north/south location of a shop.
  latitude: f32,
  /// Geographic coordinate specifying the east/west location of a shop.
  longitude: f32,
  /// A string representing the way currency is formatted when the currency isn't specified.
  money_format: String,
  /// A string representing the way currency is formatted when the currency is specified.
  money_with_currency_format: String,
  /// A string representing the default unit of weight measurement for the shop.
  weight_unit: String,
  /// The shop's 'myshopify.com' domain.
  myshopify_domain: String,
  /// The name of the shop.
  name: String,
  /// The name of the Shopify plan the shop is on.
  plan_name: String,
  /// Indicates if any active discounts exist for the shop.
  has_discounts: bool,
  /// Indicates if any active gift cards exist for the shop.
  has_gift_cards: bool,
  /// The display name of the Shopify plan the shop is on.
  plan_display_name: String,
  /// Indicates whether the Storefront password protection is enabled.
  password_enabled: bool,
  /// The contact phone number for the shop.
  phone: Option<String>,
  /// The shop's primary locale.
  primary_locale: String,
  /// The shop's normalized province or state name.
  province: String,
  /// The two-letter code for the shop's province or state.
  province_code: String,
  /// The username of the shop owner.
  shop_owner: String,
  source: Option<String>,
  /// Indicates whether the shop forces requests made to its resources to be made over SSL, using the HTTPS protocol. If true, HTTP requests will be redirected to HTTPS.
  force_ssl: bool,
  /// Specifies whether or not taxes were charged for shipping. Valid values are: "true" or "false."
  tax_shipping: Option<bool>,
  /// The setting for whether applicable taxes are included in product prices. Valid values are: "true" or "null."
  taxes_included: Option<bool>,
  /// The setting for whether the shop is applying taxes on a per-county basis or not (US-only). Valid values are: "true" or "null."
  county_taxes: Option<bool>,
  /// The name of the timezone the shop is in.
  timezone: String,
  /// The named timezone assigned by the IANA.
  iana_timezone: String,
  /// The zip or postal code of the shop's address.
  zip: String,
  /// Indicates whether the shop has web-based storefront or not.
  has_storefront: bool,
  /// Indicates whether the shop has any outstanding setup steps or not.
  setup_required: bool,

  primary_location_id: Value,
  money_in_emails_format: Value,
  money_with_currency_in_emails_format: Value,
  eligible_for_payments: Value,
  requires_extra_payments_agreement: Value,
  eligible_for_card_reader_giveaway: Value,
  finances: Value,
}