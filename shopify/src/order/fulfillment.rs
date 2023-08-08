use super::FulfillmentOrderLineItems;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fulfillment {
  pub id: i64,
  pub order_id: Option<i64>,
  pub status: Option<String>,
  pub created_at: Option<String>,
  pub service: Option<String>,
  pub updated_at: Option<String>,
  pub tracking_company: Option<serde_json::Value>,
  pub shipment_status: Option<serde_json::Value>,
  pub location_id: Option<i64>,
  pub line_items: Vec<LineItem>,
  pub tracking_number: Option<String>,
  pub tracking_numbers: Option<Vec<String>>,
  pub tracking_url: Option<String>,
  pub tracking_urls: Option<Vec<String>>,
  pub receipt: Option<Receipt>,
  pub name: Option<String>,
  pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
  pub id: Option<i64>,
  pub variant_id: Option<i64>,
  pub title: Option<String>,
  pub quantity: Option<i64>,
  pub sku: Option<String>,
  pub variant_title: Option<String>,
  pub vendor: Option<serde_json::Value>,
  pub fulfillment_service: Option<String>,
  pub product_id: Option<i64>,
  pub requires_shipping: Option<bool>,
  pub taxable: Option<bool>,
  pub gift_card: Option<bool>,
  pub name: Option<String>,
  pub variant_inventory_management: Option<serde_json::Value>,
  pub properties: Option<Vec<Option<serde_json::Value>>>,
  pub product_exists: Option<bool>,
  pub fulfillable_quantity: Option<i64>,
  pub grams: Option<i64>,
  pub price: Option<String>,
  pub total_discount: Option<String>,
  pub fulfillment_status: Option<String>,
  pub price_set: Option<Set>,
  pub total_discount_set: Option<Set>,
  pub discount_allocations: Option<Vec<Option<serde_json::Value>>>,
  pub duties: Option<Vec<Option<serde_json::Value>>>,
  pub admin_graphql_api_id: Option<String>,
  pub tax_lines: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
  pub shop_money: Option<Money>,
  pub presentment_money: Option<Money>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
  pub amount: Option<String>,
  pub currency_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFulfillmentRequest {
  pub line_items_by_fulfillment_order: Vec<LineItemsByFulfillmentOrder>,
  pub notify_customer: Option<bool>,
  pub tracking_info: Option<TrackingInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItemsByFulfillmentOrder {
  pub fulfillment_order_id: i64,
  pub fulfillment_order_line_items: Vec<FulfillmentOrderLineItems>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingInfo {
  pub company: String,
  pub number: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
}
