#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentOrder {
    pub assigned_location: Option<AssignedLocation>,
    pub assigned_location_id: Option<i64>,
    pub created_at: Option<String>,
    pub delivery_method: Option<DeliveryMethod>,
    pub destination: Option<serde_json::Value>,
    pub fulfill_at: Option<String>,
    pub fulfill_by: Option<serde_json::Value>,
    pub fulfillment_holds: Option<Vec<Option<serde_json::Value>>>,
    pub id: i64,
    pub international_duties: Option<serde_json::Value>,
    pub line_items: Vec<LineItem>,
    pub merchant_requests: Option<Vec<Option<serde_json::Value>>>,
    pub order_id: Option<i64>,
    pub request_status: Option<String>,
    pub shop_id: Option<i64>,
    pub status: FulfillmentOrderStatus,
    pub supported_actions: Option<Vec<String>>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentOrderStatus {
  Open,
  InProgress,
  Scheduled,
  Cancelled,
  OnHold,
  Incomplete,
  Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignedLocation {
    pub address1: Option<String>,
    pub address2: Option<serde_json::Value>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub location_id: Option<i64>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub zip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryMethod {
    pub id: Option<i64>,
    pub max_delivery_date_time: Option<serde_json::Value>,
    pub method_type: Option<String>,
    pub min_delivery_date_time: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    pub fulfillable_quantity: Option<i64>,
    pub fulfillment_order_id: Option<i64>,
    pub id: i64,
    pub inventory_item_id: Option<i64>,
    pub line_item_id: i64,
    pub quantity: Option<i64>,
    pub shop_id: Option<i64>,
    pub variant_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveFulfillmentOrderRequest {
    pub new_location_id: i64,
    pub fulfillment_order_line_items: Option<Vec<FulfillmentOrderLineItems>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentOrderLineItems {
    pub id: i64,
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveFulfillmentOrderResponse {
    pub original_fulfillment_order: FulfillmentOrder,
    pub moved_fulfillment_order: FulfillmentOrder,
}