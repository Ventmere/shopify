use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::{fulfillment_service::FulfillmentService, delivery_profile::types::DeliveryLocalPickupSettings, product::Metafield};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
  /// Whether this location can be reactivated.
  pub activatable: bool,
  /// The address of this location.
  pub address: LocationAddress,
  /// Whether the location address has been verified.
  pub address_verified: bool,
  /// Whether this location can be deactivated.
  pub deactivatable: bool,
  /// The date and time (ISO 8601 format) that the location was deactivated at. For example, 3:30 pm on September 7, 2019 in the time zone of UTC (Universal Time Coordinated) is represented as "2019-09-07T15:50:00Z".
  /// 
  /// (INTERNAL NOTE:) Warning, shopify says this is a String field...
  /// https://shopify.dev/docs/api/admin-graphql/2023-10/objects/Location
  pub deactivated_at: Option<DateTime<Utc>>,
  /// Whether this location can be deleted.
  pub deletable: bool,
  /// Name of the service provider that fulfills from this location.
  pub fulfillment_service: Option<FulfillmentService>,
  /// Whether this location can fulfill online orders.
  pub fulfills_online_orders: bool,
  pub has_active_inventory: bool,
  pub has_unfulfilled_orders: bool,
  pub id: String,
  pub inventory_level: Option<InventoryLevel>,
  pub is_active: bool,
  pub legacy_resource_id: u64,
  pub local_pickup_settings_v2: Option<DeliveryLocalPickupSettings>,
  pub metafield: Option<Metafield>,
  pub name: String,
  pub ships_inventory: bool,
  pub suggested_addresses: Vec<LocationSuggestedAddress>,
}

/// Represents the address of a location.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationAddress {
  /// The first line of the address for the location.
  pub address1: Option<String>,
  /// The second line of the address for the location.
  pub address2: Option<String>,
  /// The city of the location.
  pub city: Option<String>,
  /// The ZIP code of the location.
  pub zip: Option<String>,
  /// The province of the location.
  pub province: Option<String>,
  /// The code for the province, state, or district of the address of the location.
  pub province_code: Option<String>,
  /// The country of the location.
  pub country: Option<String>,
  /// The country code of the location.
  pub country_code: Option<String>,
  /// A formatted version of the address for the location.
  pub formatted: Vec<String>,
  /// The approximate latitude coordinates of the location.
  pub latitude: Option<f64>,
  /// The approximate longitude coordinates of the location.
  pub longitude: Option<f64>,
  /// The phone number of the location.
  pub phone: Option<String>,
}

/// Represents the address of a location.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationSuggestedAddress {
  /// The first line of the address for the location.
  pub address1: Option<String>,
  /// The second line of the address for the location.
  pub address2: Option<String>,
  /// The city of the location.
  pub city: Option<String>,
  /// The ZIP code of the location.
  pub zip: Option<String>,
  /// The province of the location.
  pub province: Option<String>,
  /// The code for the province, state, or district of the address of the location.
  pub province_code: Option<String>,
  /// The country of the location.
  pub country: Option<String>,
  /// The country code of the location.
  pub country_code: Option<String>,
  /// A formatted version of the address for the location.
  pub formatted: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryLevel {
  pub inventory_item_id: i64,
  pub location_id: i64,
  pub available: Value,
  pub admin_graphql_api_id: String,
  pub updated_at: DateTime<Utc>,
}
