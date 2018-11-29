#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentServiceScope {
  CurrentClient,
  All,
}

impl AsRef<str> for FulfillmentServiceScope {
  fn as_ref(&self) -> &str {
    match *self {
      FulfillmentServiceScope::CurrentClient => "current_client",
      FulfillmentServiceScope::All => "all",
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct FulfillmentService {
  pub id: i64,
  pub name: String,
  pub handle: String,
  pub email: Option<String>,
  pub include_pending_stock: bool,
  pub service_name: String,
  pub inventory_management: bool,
  pub tracking_support: bool,
  pub provider_id: Option<i64>,
  pub location_id: i64,
}

#[derive(Debug, Serialize)]
pub struct NewFulfillmentService {
  pub name: String,
  pub callback_url: String,
  pub inventory_management: bool,
  pub tracking_support: bool,
  pub requires_shipping_method: bool,
  pub format: String,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdatetFulfillmentService {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub callback_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub inventory_management: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tracking_support: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub requires_shipping_method: Option<bool>,
}
