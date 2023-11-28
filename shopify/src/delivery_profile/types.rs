use crate::inventory::Location;

use super::country_code::CountryCode;



/// A shipping profile. In Shopify, a shipping profile is a set of shipping rates scoped to a set of products or variants that can be shipped from selected locations to zones.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryProfile {
  /// The number of active shipping rates for the profile.
  pub active_method_definitions_count: i32,
  /// Whether this is the default profile.
  pub default: bool,
  /// A globally-unique ID.
  pub id: String,
  /// Whether this shop has enabled legacy compatibility mode for delivery profiles.
  pub legacy_mode: bool,
  /// The number of locations without rates defined.
  pub location_without_rates_count: i32,
  /// The name of the delivery profile.
  pub name: String,
  /// The number of active origin locations for the profile.
  pub origin_location_count: i32,
  /// How many product variants are in this profile.
  pub product_variants_count_v2: DeliveryProductVariantsCount,
  /// The location groups and associated zones using this profile.
  pub profile_location_groups: Vec<DeliveryProfileLocationGroup>,
  /// List of locations that haven't been assigned to a location group for this profile.
  pub unassigned_locations: Vec<Location>,
  /// The number of countries with active rates to deliver to.
  pub zone_country_count: i32,
}

/// How many product variants are in a profile. This count is capped at 500.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryProductVariantsCount {
    /// Whether the count has reached the cap of 500.
    pub capped: bool,
    /// The product variant count.
    pub count: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryProfileLocationGroup {
    /// The countries already selected in any zone for the specified location group.
    pub countries_in_any_zone: Vec<DeliveryCountryAndZone>,
    /// The collection of locations that make up the specified location group.
    pub location_group: DeliveryLocationGroup,

}

/// The country details and the associated shipping zone.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryCountryAndZone {
    /// The country details.
    pub country: DeliveryCountry,
    /// The name of the shipping zone.
    pub zone: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryCountry {
    /// A two-letter country code in ISO 3166-1 alpha-2 standard. It also includes a flag indicating whether the country should be a part of the 'Rest Of World' shipping zone.
    pub code: DeliveryCountryCodeOrRestOfWorld,
    /// A globally-unique ID.
    pub id: String,
    /// The full name of the country.
    pub name: String,
    /// The list of regions associated with this country.
    pub provinces: Vec<DeliveryProvince>,
    /// The translated name of the country. The translation returned is based on the system's locale.
    pub tanslated_name: String,
}

/// The country code and whether the country is a part of the 'Rest Of World' shipping zone.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryCountryCodeOrRestOfWorld {
    /// The code designating a country/region, which generally follows ISO 3166-1 alpha-2 guidelines. If a territory doesn't have a country code value in the CountryCode enum, then it might be considered a subdivision of another country. For example, the territories associated with Spain are represented by the country code ES, and the territories associated with the United States of America are represented by the country code US.
    /// https://shopify.dev/docs/api/admin-graphql/2023-10/enums/CountryCode
    pub country_code: CountryCode,
    /// Whether the country is a part of the 'Rest of World' shipping zone.
    pub rest_of_world: bool,
}

/// A location group is a collection of locations. They share zones and delivery methods across delivery profiles.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryLocationGroup {
    /// A globally-unique ID.
    pub id: String,
    /// A count of all locations that are part of this location group.
    pub locations_count: i32,
}

/// A region that is used to define a shipping zone.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryProvince {
    /// The code of the region.
    pub code: String,
    /// A globally-unique ID.
    pub id: String,
    /// The full name of the region.
    pub name: String,
    /// The translated name of the region. The translation returned is based on the system's locale.
    pub translated_name: String,
}

/// Local pickup settings associated with a location.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryLocalPickupSettings {
    /// Additional instructions or information related to the local pickup.
    pub instructions: String,
    /// The estimated pickup time to show customers at checkout.
    pub pickup_time: DeliveryLocalPickupTime,
}


/// Possible pickup time values that a location enabled for local pickup can have.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryLocalPickupTime {
    /// Usually ready in 5+ days.
    FiveOrMoreDays,
    /// Usually ready in 4 hours.
    FourHours,
    /// Usually ready in 1 hour.
    OneHour,
    /// Usually ready in 24 hours.
    TwentyFourHours,
    /// Usually ready in 2 hours.
    TwoHours,
    /// Usually ready in 2-4 days.
    TwoToFourDays,
}