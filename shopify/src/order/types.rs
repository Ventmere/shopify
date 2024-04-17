use crate::types::{DateTime, Utc, Value};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentStatus {
  Fulfilled,
  Partial,
  Restocked,
  NotEligible,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialStatus {
  Pending,
  Authorized,
  PartiallyPaid,
  Paid,
  PartiallyRefunded,
  Refunded,
  Voided,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShipmentStatus {
  Confirmed,
  InTransit,
  OutForDelivery,
  AttemptedDelivery,
  Delivered,
  Failure,
  LabelPrinted,
  Delayed,
  ReadyForPickup
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  pub first_name: Option<String>,
  pub address1: Option<String>,
  pub phone: Option<String>,
  pub city: Option<String>,
  pub zip: Option<String>,
  pub province: Option<String>,
  pub country: Option<String>,
  pub last_name: Option<String>,
  pub address2: Option<String>,
  pub company: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub name: Option<String>,
  pub country_code: Option<String>,
  pub province_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDetails {
  browser_ip: Option<String>,
  accept_language: Option<String>,
  user_agent: Option<String>,
  session_hash: Option<Value>,
  browser_width: Option<i64>,
  browser_height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
  pub id: i64,
  pub email: Option<String>,
  pub accepts_marketing: bool,
  pub created_at: String,
  pub updated_at: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub orders_count: Option<i64>,
  pub state: String,
  pub total_spent: Option<String>,
  pub last_order_id: Option<i64>,
  pub note: Value,
  pub verified_email: bool,
  pub multipass_identifier: Value,
  pub tax_exempt: bool,
  pub phone: Option<String>,
  pub tags: String,
  pub last_order_name: Option<String>,
  pub default_address: Option<DefaultAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultAddress {
  pub id: i64,
  pub customer_id: i64,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub company: Option<String>,
  pub address1: Option<String>,
  pub address2: Option<String>,
  pub city: Option<String>,
  pub province: Option<String>,
  pub country: Option<String>,
  pub zip: Option<String>,
  pub phone: Option<String>,
  pub name: Option<String>,
  pub province_code: Option<String>,
  pub country_code: Option<String>,
  pub country_name: Option<String>,
  pub default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
  pub name: String,
  pub value: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItems {
  pub id: i64,
  pub variant_id: Option<i64>,
  pub title: String,
  pub quantity: i64,
  pub price: String,
  pub grams: i64,
  pub sku: Option<String>,
  pub variant_title: Option<String>,
  pub vendor: Option<String>,
  pub fulfillment_service: String,
  pub product_id: Option<i64>,
  pub requires_shipping: bool,
  pub taxable: bool,
  pub gift_card: bool,
  pub name: String,
  pub variant_inventory_management: Option<String>,
  pub properties: Vec<Property>,
  pub product_exists: bool,
  pub fulfillable_quantity: i64,
  pub total_discount: String,
  pub fulfillment_status: Option<FulfillmentStatus>,
  pub tax_lines: Vec<TaxLines>,
  pub origin_location: Option<Location>,
  pub destination_location: Option<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
  id: i64,
  country_code: String,
  province_code: String,
  name: String,
  address1: String,
  address2: Option<String>,
  city: String,
  zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountCode {
  pub amount: String,
  pub code: String,
  #[serde(rename = "type")]
  pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  pub id: i64,
  pub email: Option<String>,
  pub closed_at: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub number: i64,
  pub note: Option<String>,
  pub token: String,
  pub gateway: Option<String>,
  pub test: bool,
  pub total_price: String,
  pub subtotal_price: String,
  pub total_weight: Option<i64>,
  pub total_tax: String,
  pub taxes_included: bool,
  pub currency: String,
  pub financial_status: FinancialStatus,
  pub confirmed: bool,
  pub total_discounts: String,
  pub total_line_items_price: String,
  pub cart_token: Option<String>,
  pub buyer_accepts_marketing: bool,
  pub name: String,
  pub referring_site: Value,
  pub landing_site: Value,
  pub cancelled_at: Option<DateTime<Utc>>,
  pub cancel_reason: Option<String>,
  pub total_price_usd: Option<String>,
  pub checkout_token: Value,
  pub reference: Value,
  pub user_id: Option<i64>,
  pub location_id: Option<i64>,
  pub source_identifier: Value,
  pub source_url: Option<String>,
  pub processed_at: String,
  pub device_id: Value,
  pub phone: Option<String>,
  pub customer_locale: Option<String>,
  pub app_id: i64,
  pub browser_ip: Option<String>,
  pub landing_site_ref: Value,
  pub order_number: i64,
  pub discount_codes: Vec<DiscountCode>,
  pub note_attributes: Vec<Property>,
  pub payment_gateway_names: Vec<String>,
  pub processing_method: Option<String>,
  pub checkout_id: Value,
  pub source_name: String,
  pub fulfillment_status: Option<FulfillmentStatus>,
  pub tax_lines: Vec<TaxLines>,
  pub tags: String,
  pub contact_email: Option<String>,
  pub order_status_url: String,
  pub line_items: Vec<LineItems>,
  pub shipping_lines: Vec<ShippingLines>,
  pub billing_address: Option<Address>,
  pub shipping_address: Option<Address>,
  pub fulfillments: Vec<OrderFulfillment>,
  pub client_details: Option<ClientDetails>,
  pub refunds: Vec<Value>,
  pub customer: Option<Customer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderFulfillment {
  pub id: i64,
  pub order_id: i64,
  pub status: String,
  pub created_at: String,
  pub service: String,
  pub updated_at: String,
  pub tracking_company: Option<String>,
  pub shipment_status: Option<ShipmentStatus>,
  pub tracking_number: Option<String>,
  pub tracking_numbers: Vec<String>,
  pub tracking_url: Option<String>,
  pub tracking_urls: Vec<String>,
  pub receipt: Value,
  pub line_items: Vec<LineItems>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingLines {
  pub id: i64,
  pub title: String,
  pub price: String,
  pub code: String,
  pub source: Option<String>,
  pub phone: Option<String>,
  pub requested_fulfillment_service_id: Value,
  pub delivery_category: Option<Value>,
  pub carrier_identifier: Value,
  pub discounted_price: String,
  pub tax_lines: Vec<TaxLines>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxLines {
  pub title: String,
  pub price: String,
  pub rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRisk {
  pub cause_cancel: Option<bool>,
  pub checkout_id: Option<i64>,
  pub display: Option<bool>,
  pub id: Option<i64>,
  pub message: String,
  pub order_id: Option<i64>,
  pub recommendation: String,
  pub score: Option<String>,
  pub source: String,
}

#[derive(Debug, Serialize)]
pub struct OrderUpdateParams {
  pub buyer_accepts_marketing: Option<bool>,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub note: Option<String>,
  pub tags: Option<String>,
  pub shipping_address: Option<Address>,
}

#[derive(Debug, Serialize)]
pub struct OrderUpdateRequestParams {
  pub order_id: i64,
  pub buyer_accepts_marketing: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub note: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tags: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shipping_address: Option<Address>,
}
