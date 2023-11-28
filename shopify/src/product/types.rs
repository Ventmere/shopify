use serde_json::Value;
use crate::{types::{DateTime, Utc}, delivery_profile::types::DeliveryProfile};
pub use crate::variant::Variant;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
  /// The number of publications a resource is published to without feedback errors.
  pub available_publication_count: i32,
  /// The compare-at price range of the product in the default shop currency.
  pub compare_at_price_range: Option<ProductCompareAtPriceRange>,
  /// The pricing that applies for a customer in a given context.
  pub contextual_pricing: ProductContextualPricing,
  /// The date and time (ISO 8601 format) when the product was created.
  pub created_at: DateTime<Utc>,
  /// A default cursor that returns the single next record, sorted ascending by ID.
  pub default_cursor: String,
  /// A stripped description of the product, single line with HTML tags removed.
  pub description: String,
  /// The description of the product, complete with HTML formatting.
  /// 
  /// `HTML` type in shopify: 
  /// A string containing HTML code. Refer to the HTML spec for a complete list of HTML elements.
  /// https://shopify.dev/docs/api/admin-graphql/2023-10/scalars/HTML
  pub description_html: String,
  /// The featured image for the product.
  pub featured_image: Option<Image>,
  /// The featured media for the product.
  pub featured_media: Option<Media>,
  /// Information about the product that's provided through resource feedback.
  pub feedback: Option<ResourceFeedback>,
  /// The theme template used when viewing the gift card in a store.
  pub gift_card_template_suffix: Option<String>,
  /// A unique human-friendly string of the product's title.
  pub handle: String,
  /// Whether the product has only a single variant with the default option and value.
  pub has_only_default_variant: bool,
  /// Whether the product has out of stock variants.
  pub has_out_of_stock_variants: bool,
  /// Determines if at least one of the product variant requires components. The default value is false.
  pub has_variants_that_requires_components: bool,
  /// Represents a unique identifier, often used to refetch an object. The ID type appears in a JSON response as a String, but it is not intended to be human-readable.
  ///
  /// Example value: "gid://shopify/Product/10079785100"
  pub id: String,
  /// Whether the product is in a given collection.
  pub in_collection: bool,
  /// Whether the product is a gift card.
  pub is_gift_card: bool,
  /// The ID of the corresponding resource in the REST Admin API.
  pub legacy_resource_id: u64,
  /// Total count of media belonging to a product.
  pub media_count: i32,
  /// Returns a metafield by namespace and key that belongs to the resource.
  pub metafield: Option<Metafield>,
  /// The online store preview URL.
  /// 
  /// Represents an RFC 3986 and RFC 3987-compliant URI string.
  ///
  /// For example, "https://johns-apparel.myshopify.com" is a valid URL. It includes a scheme (https) and a host (johns-apparel.myshopify.com).
  pub online_store_preview_url: Option<String>,
  /// The online store URL for the product. A value of null indicates that the product isn't published to the Online Store sales channel.
  pub online_store_url: Option<String>,
  /// A list of product options. The limit is specified by Shop.resourceLimits.maxProductOptions.
  pub options: Vec<ProductOption>,
  /// The price range of the product with prices formatted as decimals.
  pub price_range_v2: ProductPriceRangeV2,
  /// The product category specified by the merchant.
  pub product_category: ProductCategory,
  /// The product type specified by the merchant.
  pub product_type: String,
  /// The number of publications a resource is published on.
  pub publication_count: i32,
  /// The date and time (ISO 8601 format) when the product was published to the Online Store.
  pub published_at: Option<DateTime<Utc>>,
  /// Whether or not the product is published for a customer in the given context.
  pub published_in_context: bool,
  /// Check to see whether the resource is published to the calling app's publication.
  pub published_on_current_publication: bool,
  /// Check to see whether the resource is published to a given publication.
  pub published_on_publication: bool,
  /// Whether the product can only be purchased with a selling plan (subscription). Products that are sold on subscription (requiresSellingPlan: true) can be updated only for online stores. If you update a product to be subscription only, then the product is unpublished from all channels except the online store.
  pub requires_selling_plan: bool,
  /// The resource that's either published or staged to be published to the calling app's publication. Requires the read_product_listings scope.
  pub resource_publication_on_current_publication: Option<ResourcePublicationV2>,
  /// Count of selling plan groups associated with the product.
  pub selling_plan_group_count: i32,
  /// SEO information of the product.
  pub seo: SEO,
  /// The product status. This controls visibility across all channels.
  pub status: ProductStatus,
  /// A comma separated list of tags associated with the product. Updating tags overwrites any existing tags that were previously added to the product. To add new tags without overwriting existing tags, use the tagsAdd mutation.
  pub tags: Vec<String>,
  /// The theme template used when viewing the product in a store.
  pub template_suffix: Option<String>,
  /// The title of the product.
  pub title: String,
  /// The quantity of inventory in stock.
  pub total_inventory: i32,
  /// The number of variants that are associated with the product.
  pub total_variants: i32,
  /// Whether inventory tracking has been enabled for the product.
  pub tracks_inventory: bool,
  /// The translations associated with the resource.
  pub translations: Vec<Translation>,
  /// The date and time when the product was last modified. A product's updatedAt value can change for different reasons. For example, if an order is placed for a product that has inventory tracking set up, then the inventory adjustment is counted as an update.
  pub updated_at: DateTime<Utc>,
  /// The name of the product's vendor.
  pub vendor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
  pub id: i64,
  pub product_id: i64,
  pub position: i64,
  pub created_at: String,
  pub updated_at: Option<DateTime<Utc>>,
  pub alt: Option<String>,
  pub width: i64,
  pub height: i64,
  pub src: String,
  pub variant_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOption {
  pub id: i64,
  pub product_id: i64,
  pub name: String,
  pub position: i64,
  pub values: Vec<String>,
}

/// The compare-at price range of the product.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCompareAtPriceRange {
  pub max_variant_compare_at_price: MoneyV2,
  pub min_variant_compare_at_price: MoneyV2,
}

/// The price of a product in a specific country. Prices vary between countries.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductContextualPricing {
  /// The number of fixed quantity rules for the product's variants on the price list.
  pub fixed_quantity_rules_count: i32,
  /// The pricing of the variant with the highest price in the given context.
  pub max_variant_pricing: ProductVariantContextualPricing,
  /// The pricing of the variant with the lowest price in the given context.
  pub min_variant_pricing: ProductVariantContextualPricing,
  /// The price range of the product with prices formatted as decimals.
  pub price_range: ProductPriceRangeV2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVariantContextualPricing {
  pub compare_at_price: Option<MoneyV2>,
  pub price: MoneyV2,
  pub quantity_rule: QuantityRule,
}

/// The quantity rule for the product variant in a given context.
#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityRule {
  /// The value that specifies the quantity increment between minimum and maximum of the rule. Only quantities divisible by this value will be considered valid.
  ///
  /// The increment must be lower than or equal to the minimum and the maximum, and both minimum and maximum must be divisible by this value.
  pub increment: i32,
  /// Whether the quantity rule fields match one increment, one minimum and no maximum.
  pub is_default: bool,
  /// An optional value that defines the highest allowed quantity purchased by the customer. If defined, maximum must be lower than or equal to the minimum and must be a multiple of the increment.
  pub maximum: Option<i32>,
  /// The value that defines the lowest allowed quantity purchased by the customer. The minimum must be a multiple of the quantity rule's increment.
  pub minimum: i32,
  /// Whether the values of the quantity rule were explicitly set.
  pub origin_type: QuantityRuleOriginType,
  /// The product variant for which the quantity rule is applied.
  /// (INTERNAL NOTE) We are using a box here because this has indirection (E.G. Infinite size)
  pub product_variant: Box<ProductVariant>,
}

/// Represents a product variant.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVariant {
  /// Whether the product variant is available for sale.
  pub available_for_sale: bool,
  /// The value of the barcode associated with the product.
  pub barcode: Option<String>,
  /// The compare-at price of the variant in the default shop currency.
  /// 
  /// In shopify this is represented as "Money" which is 
  /// A monetary value string without a currency symbol or code. Example value: "100.57".
  pub compare_at_price: Option<String>,
  /// The pricing that applies for a customer in a given context.
  pub contextual_pricing: ProductVariantContextualPricing,
  /// The date and time when the variant was created.
  pub created_at: DateTime<Utc>,
  /// A default cursor that returns the single next record, sorted ascending by ID.
  pub default_cursor: String,
  pub delivery_profile: DeliveryProfile,
}

/// The price range of the product.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPriceRangeV2 {
  /// The highest variant's price.
  pub max_variant_price: MoneyV2,
  /// The lowest variant's price.
  pub min_variant_price: MoneyV2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SEO {
  /// SEO description.
  pub description: String,
  /// SEO Title.
  pub title: String,
}

/// The origin of quantity rule on a price list.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum QuantityRuleOriginType {
  Fixed,
  Relative,
}

/// A monetary value with currency.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyV2 {
  /// Decimal money amount.
  pub amount: f64,
  /// Currency of the money.
  /// TODO: make this an enum? https://shopify.dev/docs/api/admin-graphql/2023-10/enums/CurrencyCode
  pub currrency_code: String
}

/// Metafields enable you to attach additional information to a Shopify resource, such as a Product or a Collection. For more information about where you can attach metafields refer to HasMetafields. Some examples of the data that metafields enable you to store are specifications, size charts, downloadable documents, release dates, images, or part numbers. Metafields are identified by an owner resource, namespace, and key. and store a value along with type information for that value.
/// https://shopify.dev/docs/api/admin-graphql/2023-10/objects/Metafield
#[derive(Debug, Serialize, Deserialize)]
pub struct Metafield {
  /// The date and time when the metafield was created.
  pub created_at: DateTime<Utc>,
  /// The metafield definition that the metafield belongs to, if any.
  pub definition: Option<MetafieldDefinition>,
  /// A globally-unique ID.
  pub id: String,
  /// The unique identifier for the metafield within its namespace.
  pub key: String,
  /// The ID of the corresponding resource in the REST Admin API.
  pub legacy_resource_id: u64,
  /// The container for a group of metafields that the metafield is associated with.
  pub namespace: String,
  /// The resource that the metafield is attached to.
  pub owner: HasMetaFields,
  /// The type of resource that the metafield is attached to.
  pub owner_type: MetafieldOwnerType,
  /// Returns a reference object if the metafield definition's type is a resource reference.
  pub reference: MetafieldReference,
  /// The type of data that is stored in the metafield. Refer to the list of supported types.
  /// supported types: https://shopify.dev/apps/metafields/types
  pub r#type: String,
  /// The date and time when the metafield was updated.
  pub updated_at: DateTime<Utc>,
  /// The data stored in the metafield. Always stored as a string, regardless of the metafield's type.
  pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaFieldDefinition {

}
