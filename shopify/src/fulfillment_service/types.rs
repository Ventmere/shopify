use crate::inventory::Location;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
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

/// A Fulfillment Service is a third party warehouse that prepares and ships orders on behalf of the store owner. Fulfillment services charge a fee to package and ship items and update product inventory levels. Some well known fulfillment services with Shopify integrations include: Amazon, Shipwire, and Rakuten. When an app registers a new FulfillmentService on a store, Shopify automatically creates a Location that's associated to the fulfillment service. To learn more about fulfillment services, refer to Manage fulfillments as a fulfillment service app guide.
/// https://shopify.dev/docs/api/admin-graphql/2023-10/objects/FulfillmentService
#[derive(Debug, Deserialize, Serialize)]
pub struct FulfillmentService {
  /// The callback URL that the fulfillment service has registered for requests. The following considerations apply:
  ///
  /// Shopify queries the <callbackUrl>/fetch_tracking_numbers endpoint to retrieve tracking numbers for orders, if trackingSupport is set to true.
  /// Shopify queries the <callbackUrl>/fetch_stock endpoint to retrieve inventory levels, if inventoryManagement is set to true.
  /// Shopify uses the <callbackUrl>/fulfillment_order_notification endpoint to send fulfillment and cancellation requests, if the fulfillment service has opted in to the fulfillment order based workflow for managing fulfillments (fulfillmentOrdersOptIn is set to true).
  pub callback_url: String,
  /// The ID of the fulfillment service.
  pub id: String,
  /// The name of the fulfillment service as seen by merchants.
  pub service_name: String,
  /// Human-readable unique identifier for this fulfillment service.
  pub handle: String,
  /// Whether the fulfillment service tracks product inventory and provides updates to Shopify.
  pub inventory_management: bool,
  /// Location associated with the fulfillment service.
  pub location: Box<Location>,
  /// Whether the fulfillment service can stock inventory alongside other locations.
  pub permits_sku_sharing: bool,
  /// Whether the fulfillment service supports local deliveries.
  pub product_based: bool,
  /// Type associated with the fulfillment service.
  pub r#type: FulfillmentServiceType,
  /// Whether the fulfillment service uses the fulfillment order based workflow for managing fulfillments.
  pub fulfillment_orders_opt_in: bool,
}

#[derive(Debug, Serialize)]
pub struct NewFulfillmentService {
  pub name: String,
  pub callback_url: String,
  pub inventory_management: bool,
  pub tracking_support: bool,
  pub requires_shipping_method: bool,
  pub format: String,
  pub permits_sku_sharing: bool,
  pub fulfillment_orders_opt_in: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateFulfillmentService {
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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub permits_sku_sharing: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fulfillment_orders_opt_in: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FulfillmentServiceType {
  GiftCard,
  Manual,
  ThirdParty,
}

