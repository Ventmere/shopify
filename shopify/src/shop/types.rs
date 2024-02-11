use crate::types::{DateTime, Utc, Value};

/// The Shopify API's shop object is a collection of the general settings and information about the shop.
#[derive(Debug, Deserialize)]
pub struct Shop {
  /// A unique numeric identifier for the shop.
  pub id: i64,
  /// The shop's street address.
  pub address1: String,
  /// The shop's additional street address (apt, suite, etc.).
  pub address2: String,
  /// The city in which the shop is located.
  pub city: String,
  /// The shop's country (by default equal to the two-letter country code).
  pub country: String,
  /// The two-letter country code corresponding to the shop's country.
  pub country_code: String,
  /// The shop's normalized country name.
  pub country_name: String,
  /// The date and time when the shop was created. The API returns this value in ISO 8601 format.
  pub created_at: DateTime<Utc>,
  /// The date and time when the shop was last updated. The API returns this value in ISO 8601 format.
  pub updated_at: DateTime<Utc>,
  /// The customer's email.
  pub customer_email: String,
  /// The three-letter code for the currency that the shop accepts.
  pub currency: String,
  /// The shop's domain.
  pub domain: String,
  /// The contact email address for the shop.
  pub email: String,
  /// Feature is present when a shop has a google app domain. It will be returned as a URL. If the shop does not have this feature enabled it will default to "null."
  pub google_apps_domain: Option<String>,
  /// Feature is present if a shop has google apps enabled. Those shops with this feature will be able to login to the google apps login. Shops without this feature enabled will default to "null."
  pub google_apps_login_enabled: Option<bool>,
  /// Geographic coordinate specifying the north/south location of a shop.
  pub latitude: f32,
  /// Geographic coordinate specifying the east/west location of a shop.
  pub longitude: f32,
  /// A string representing the way currency is formatted when the currency isn't specified.
  pub money_format: String,
  /// A string representing the way currency is formatted when the currency is specified.
  pub money_with_currency_format: String,
  /// A string representing the default unit of weight measurement for the shop.
  pub weight_unit: String,
  /// The shop's 'myshopify.com' domain.
  pub myshopify_domain: String,
  /// The name of the shop.
  pub name: String,
  /// The name of the Shopify plan the shop is on.
  pub plan_name: String,
  /// Indicates if any active discounts exist for the shop.
  pub has_discounts: bool,
  /// Indicates if any active gift cards exist for the shop.
  pub has_gift_cards: bool,
  /// The display name of the Shopify plan the shop is on.
  pub plan_display_name: String,
  /// Indicates whether the Storefront password protection is enabled.
  pub password_enabled: bool,
  /// The contact phone number for the shop.
  pub phone: Option<String>,
  /// The shop's primary locale.
  pub primary_locale: String,
  /// The shop's normalized province or state name.
  pub province: String,
  /// The two-letter code for the shop's province or state.
  pub province_code: Option<String>,
  /// The username of the shop owner.
  pub shop_owner: String,
  pub source: Option<String>,
  /// Indicates whether the shop forces requests made to its resources to be made over SSL, using the HTTPS protocol. If true, HTTP requests will be redirected to HTTPS.
  pub force_ssl: bool,
  /// Specifies whether or not taxes were charged for shipping. Valid values pub are: "true" or "false."
  pub tax_shipping: Option<bool>,
  /// The setting for whether applicable taxes are included in product prices. Valid values pub are: "true" or "null."
  pub taxes_included: Option<bool>,
  /// The setting for whether the shop is applying taxes on a per-county basis or not (US-only). Valid values pub are: "true" or "null."
  pub county_taxes: Option<bool>,
  /// The name of the timezone the shop is in.
  pub timezone: String,
  /// The named timezone assigned by the IANA.
  pub iana_timezone: String,
  /// The zip or postal code of the shop's address.
  pub zip: String,
  /// Indicates whether the shop has web-based storefront or not.
  pub has_storefront: bool,
  /// Indicates whether the shop has any outstanding setup steps or not.
  pub setup_required: bool,

  pub primary_location_id: Value,
  pub money_in_emails_format: Value,
  pub money_with_currency_in_emails_format: Value,
  pub eligible_for_payments: Value,
  pub requires_extra_payments_agreement: Value,
  pub eligible_for_card_reader_giveaway: Value,
  pub finances: Value,
}
