use serde_json::Value;
use crate::{types::{DateTime, Utc}, delivery_profile::types::DeliveryProfile};
pub use crate::variant::Variant;

use super::currency_code::CurrencyCode;

/// The Product resource lets you manage products in a merchant’s store. You can use ProductVariants to create or update different versions of the same product. You can also add or update product Media. Products can be organized by grouping them into a Collection.
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
  pub resource_publication_on_current_publication: Option<Value>,
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
pub struct Translation {
  /// On the resource that this translation belongs to, the reference to the value being translated.
  pub key: String,
  /// ISO code of the translation locale.
  pub locale: String,
  /// The market that the translation is specific to. Null value means the translation is available in all markets.
  pub market: Option<Value>,
  /// Whether the original content has changed since this translation was updated.
  pub outdated: bool,
  /// The date and time when the translation was updated.
  pub updated_at: Option<DateTime<Utc>>,
  /// Translation value.
  pub value: Option<String>,
}

/// The details of a specific product category within the Shopify product taxonomy.
/// https://help.shopify.com/txt/product_taxonomy/en.txt?shpxid=17a8c640-B29B-49DC-6D21-E5D2ABA3D757
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCategory {
  pub product_taxonomy_node: ProductTaxonomyNode,
}

/// Represents a Shopify product taxonomy node.
/// https://help.shopify.com/txt/product_taxonomy/en.txt?shpxid=17a928d8-51FA-4E10-CCAD-101BDBA62509
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductTaxonomyNode {
  /// The full name of the product taxonomy node. For example, Animals & Pet Supplies > Pet Supplies > Dog Supplies > Dog Beds.
  pub full_name: String,
  /// The ID of the product taxonomy node.
  pub id: String,
  /// Whether the node is a leaf node.
  pub is_leaf: bool,
  /// Whether the node is a root node.
  pub is_root: bool,
  /// The name of the product taxonomy node. For example, Dog Beds.
  pub name: String,
}

/// The possible product statuses.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductStatus {
  /// The product is ready to sell and can be published to sales channels and apps. Products with an active status aren't automatically published to sales channels, such as the online store, or apps. By default, existing products are set to active.
  Active,
  /// The product is no longer being sold and isn't available to customers on sales channels and apps.
  Archived,
  /// The product isn't ready to sell and is unavailable to customers on sales channels and apps. By default, duplicated and unarchived products are set to draft.
  Draft,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
  /// A unique ID for the image.
  pub id: Option<String>,
  /// A word or phrase to share the nature or contents of an image.
  pub alt_text: Option<String>,
  /// The original width of the image in pixels. Returns null if the image isn't hosted by Shopify.
  pub width: Option<i64>,
  /// The original height of the image in pixels. Returns null if the image isn't hosted by Shopify.
  pub height: Option<i64>,
  /// The location of the image as a URL.
  /// If no transform options are specified, then the original image will be preserved including any pre-applied transforms.
  /// All transformation options are considered "best-effort". Any transformation that the original image type doesn't support will be ignored.
  /// If you need multiple variations of the same image, then you can use GraphQL aliases
  pub url: String,
  /// Returns a metafield by namespace and key that belongs to the resource.
  pub metafield: Metafield,
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
  pub currrency_code: CurrencyCode
}

/// https://shopify.dev/docs/api/admin-graphql/2023-10/objects/ResourceFeedback
/// Represents feedback from apps about a resource, and the steps required to set up the apps on the shop.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceFeedback {
  /// List of AppFeedback detailing issues regarding a resource.
  pub details: Vec<Value>,
  /// Summary of resource feedback pertaining to the resource.
  pub summary: String,
}

/// Metafields enable you to attach additional information to a Shopify resource, such as a Product or a Collection. For more information about where you can attach metafields refer to HasMetafields. Some examples of the data that metafields enable you to store are specifications, size charts, downloadable documents, release dates, images, or part numbers. Metafields are identified by an owner resource, namespace, and key. and store a value along with type information for that value.
/// https://shopify.dev/docs/api/admin-graphql/2023-10/objects/Metafield
#[derive(Debug, Serialize, Deserialize)]
pub struct Metafield {
  /// The date and time when the metafield was created.
  pub created_at: DateTime<Utc>,
  /// The metafield definition that the metafield belongs to, if any.
  pub definition: Option<MetafieldDefinition>,
  /// The description of the metafield.
  pub description: Option<String>,
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
  /// TODO: WTF pub reference: MetafieldReference,
  pub reference: Value,
  /// The type of data that is stored in the metafield. Refer to the list of supported types.
  /// supported types: https://shopify.dev/apps/metafields/types
  pub r#type: String,
  /// The date and time when the metafield was updated.
  pub updated_at: DateTime<Utc>,
  /// The data stored in the metafield. Always stored as a string, regardless of the metafield's type.
  pub value: String,
}

/// Represents information about the metafields associated to the specified resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct HasMetaFields {
  pub metafield: Box<Metafield>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetafieldDefinition {
  /// The access settings associated with the metafield definition.
  pub access: MetaFieldAccess,
  /// The description of the metafield definition.
  pub description: Option<String>,
  /// A globally-unique ID.
  pub id: String,
  /// The unique identifier for the metafield definition within its namespace.
  pub key: String,
  /// The count of the metafields that belong to the metafield definition.
  pub metafields_count: i32,
  /// The human-readable name of the metafield definition.
  pub name: String,
  /// The container for a group of metafields that the metafield definition is associated with.
  pub namespace: String,
  /// The resource type that the metafield definition is attached to.
  pub owner_type: MetafieldOwnerType,
  /// The position of the metafield definition in the pinned list.
  pub pinned_position: Option<i32>,
  /// The standard metafield definition template associated with the metafield definition.
  pub standard_template: Option<StandardMetafieldDefinitionTemplate>,
  /// The type of data that each of the metafields that belong to the metafield definition will store. Refer to the list of supported types.
  /// https://shopify.dev/docs/apps/custom-data/metafields/types
  pub r#type: MetafieldDefinitionType,
  /// Whether the metafield definition can be used as a collection condition.
  pub use_as_collection_condition: bool,
  /// The validation status for the metafields that belong to the metafield definition.
  pub validation_status: MetafieldDefinitionValidationStatus,
  /// A list of validation options for the metafields that belong to the metafield definition. For example, for a metafield definition with the type date, you can set a minimum date validation so that each of the metafields that belong to it can only store dates after the specified minimum.
  /// https://shopify.dev/docs/apps/custom-data/metafields/definitions/validation
  pub validations: MetafieldDefinitionValidation,
  /// Whether each of the metafields that belong to the metafield definition are visible from the Storefront API.
  pub visible_to_storefront_api: bool,
}


/// The access settings for this metafield definition.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaFieldAccess {
  /// The default admin access setting used for the metafields under this definition.
  pub admin: Option<MetaFieldAdminAccess>,
  /// The explicit grants for this metafield definition, superseding the default admin access for the specified grantees.
  pub grants: Vec<MetaFieldAccessGrant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MetaFieldAdminAccess {
  /// Owner gets full access. The merchant has read-only access. No one else has access rights.
  MerchantRead,
  /// Owner gets full access. The merchant has read and write access. No one else has access rights.
  MerchantReadWrite,
  /// Owner gets full access. No one else has access rights.
  Private,
  /// Owner gets full access. All applications and the merchant have read-only access.
  PublicRead
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaFieldAccessGrant {
  /// The level of access the grantee has.
  pub access: MetafieldGrantAccessLevel,
  /// The grantee being granted access.
  pub grantee: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MetafieldGrantAccessLevel {
  /// Read metafield access.
  Read,
  /// Read and write metafield access.
  ReadWrite
}

/// Possible types of a metafield's owner resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetafieldOwnerType {
    /// The Api Permission metafield owner type.
    ApiPermission,
    /// The Article metafield owner type.
    Article,
    /// The Blog metafield owner type.
    Blog,
    /// The Collection metafield owner type.
    Collection,
    /// The Company metafield owner type.
    Company,
    /// The Company Location metafield owner type.
    CompanyLocation,
    /// The Customer metafield owner type.
    Customer,
    /// The Delivery Customization metafield owner type.
    DeliveryCustomization,
    /// The Discount metafield owner type.
    Discount,
    /// The Draft Order metafield owner type.
    #[serde(rename = "DRAFTORDER")]
    DraftOrder,
    /// The Fulfillment Constraint Rule metafield owner type.
    FulfillmentConstraintRule,
    /// The Location metafield owner type.
    Location,
    /// The Market metafield owner type.
    Market,
    /// The Media Image metafield owner type.
    MediaImage,
    /// The Order metafield owner type.
    Order,
    /// The Page metafield owner type.
    Page,
    /// The Payment Customization metafield owner type.
    PaymentCustomization,
    /// The Product metafield owner type.
    Product,
    /// The Product Variant metafield owner type.
    #[serde(rename = "PRODUCTVARIANT")]
    ProductVariant,
    /// The Shop metafield owner type.
    Shop,
}

/// Standard metafield definition templates provide preset configurations to create metafield definitions. Each template has a specific namespace and key that we've reserved to have specific meanings for common use cases.
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardMetafieldDefinitionTemplate {
  /// The description of the standard metafield definition.
  pub description: Option<String>,
  /// A globally-unique ID.
  pub id: String,
  /// The key owned by the definition after the definition has been activated.
  pub key: String,
  /// The human-readable name for the standard metafield definition.
  pub name: String,
  /// The namespace owned by the definition after the definition has been activated.
  pub namespace: String,
  /// The list of resource types that the standard metafield definition can be applied to.
  pub owner_types: Vec<MetafieldOwnerType>,
  /// The associated metafield definition type that the metafield stores.
  /// https://shopify.dev/docs/apps/custom-data/metafields/types
  pub r#type: MetafieldDefinitionType,
  /// The configured validations for the standard metafield definition.
  pub validations: Vec<MetafieldDefinitionValidation>,
  /// Whether metafields for the definition are by default visible using the Storefront API.
  pub visible_to_storefront_api: bool,
}

/// A metafield definition type provides basic foundation and validation for a metafield.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetafieldDefinitionType {
  /// The category associated with the metafield definition type.
  pub category: String,
  /// The name of the type for the metafield definition. See the list of supported types.
  /// https://shopify.dev/docs/apps/custom-data/metafields/types
  pub name: String,
  /// The supported validations for a metafield definition type.
  pub supported_validations: Vec<MetafieldDefinitionSupportedValidation>,
  /// Whether metafields without a definition can be migrated to a definition of this type.
  pub supports_definition_migrations: bool,
}

/// The type and name for the optional validation configuration of a metafield.
/// 
/// For example, a supported validation might consist of a max name and a number_integer type. This validation can then be used to enforce a maximum character length for a single_line_text_field metafield.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetafieldDefinitionSupportedValidation {
  /// The name of the metafield definition validation.
  pub name: String,
  /// The type of input for the validation.
  pub r#type: String,
}

/// A configured metafield definition validation.
/// 
/// For example, for a metafield definition of number_integer type, you can set a validation with the name max and a value of 15. This validation will ensure that the value of the metafield is a number less than or equal to 15.
///
/// Refer to the list of supported validations.
/// https://shopify.dev/docs/api/admin-graphql/unstable/queries/metafieldDefinitionTypes#examples-Fetch_all_metafield_definition_types
#[derive(Debug, Serialize, Deserialize)]
pub struct MetafieldDefinitionValidation {
  /// The validation name.
  pub name: String,
  /// The name for the metafield type of this validation.
  pub r#type: String,
  /// The validation value.
  pub value: Option<String>,
}

/// Possible metafield definition validation statuses.
#[derive(Debug, Serialize, Deserialize)]
pub enum MetafieldDefinitionValidationStatus {
  /// All of this definition's metafields are valid.
  AllValid,
  /// Asynchronous validation of this definition's metafields is in progress.
  InProgress,
  /// Some of this definition's metafields are invalid.
  SomeInvalid,
}

///The possible content types for a media object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaContentType {
  /// An externally hosted video.
  ExternalVideo,
  /// A Shopify-hosted image.
  Image,
  /// A 3d model.
  #[serde(rename = "MODEL_3D")]
  Model3D,
  /// A Shopify-hosted video.
  Video,
}

/// Represents a media interface.
#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
  /// A word or phrase to share the nature or contents of a media.
  pub alt: Option<String>,
  /// A globally-unique ID.
  pub id: String,
  /// The media content type.
  pub media_content_type: MediaContentType,
  /// Any errors which have occurred on the media.
  pub media_errors: Vec<MediaError>,
  /// The warnings attached to the media.
  pub media_warnings: Vec<MediaWarning>,
  /// The preview image for the media.
  pub preview: Option<MediaPreviewImage>,
  /// Current status of the media.
  pub status: MediaStatus,
}

/// Represents a media error. This typically occurs when there is an issue with the media itself causing it to fail validation. Check the media before attempting to upload again.
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaError {
  /// Code representing the type of error.
  pub code: MediaErrorCode,
  /// Additional details regarding the error.
  pub details: Option<String>,
  /// Translated error message.
  pub message: String,
}

/// Error types for media.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaErrorCode {
  /// Media could not be created because a file with the same name already exists.
  DuplicateFilenameError,
  /// Media could not be created because embed permissions are disabled for this video.
  ExternalVideoEmbedDisabled,
  /// Media could not be created because video is either not found or still transcoding.
  ExternalVideoEmbedNotFoundOrTranscoding,
  /// Media could not be created because the external video has an invalid aspect ratio.
  ExternalVideoInvalidAspectRatio,
  /// Media could not be created because the external video could not be found.
  ExternalVideoNotFound,
  /// Media could not be created because the external video is not listed or is private.
  ExternalVideoUnlisted,
  /// Media could not be created because the cumulative file storage limit would be exceeded.
  FileStorageLimitExceeded,
  /// File could not be processed because the source could not be downloaded.
  GenericFileDownloadFailure,
  /// File could not be created because the size is too large.
  GenericFileInvalidSize,
  /// Media could not be processed because the image could not be downloaded.
  ImageDownloadFailure,
  /// Media could not be processed because the image could not be processed.
  ImageProcessingFailure,
  /// Media could not be created because the image has an invalid aspect ratio.
  InvalidImageAspectRatio,
  /// Media could not be created because the image size is too large.
  InvalidImageFileSize,
  /// Media could not be created because the image's resolution exceeds the max limit.
  InvalidImageResolution,
  /// Media could not be processed because the signed URL was invalid.
  InvalidSignedUrl,
  /// Media timed out because it is currently being modified by another operation.
  MediaTimeoutError,
  /// Media could not be created because the model file failed processing.
  Model3dGlbOutputCreationError,
  /// Media could not be created because the model can't be converted to USDZ format.
  Model3dGlbToUsdzConversionError,
  /// Media could not be created because the model file failed processing.
  Model3dProcessingFailure,
  /// Media could not be created because the model's thumbnail generation failed.
  Model3dThumbnailGenerationError,
  /// There was an issue while trying to generate a new thumbnail.
  Model3dThumbnailRegenerationError,
  /// Model failed validation.
  Model3dValidationError,
  /// Media error has occurred for an unknown reason.
  Unknown,
  /// Media could not be created because the image is an unsupported file type.
  UnsupportedImageFileType,
  /// Media could not be created because it has an invalid file type.
  VideoInvalidFileTypeError,
  /// Media could not be created because it does not meet the maximum duration requirement.
  VideoMaxDurationError,
  /// Media could not be created because it does not meet the maximum height requirement.
  VideoMaxHeightError,
  /// Media could not be created because it does not meet the maximum width requirement.
  VideoMaxWidthError,
  /// Media could not be created because the metadata could not be read.
  VideoMetadataReadError,
  /// Media could not be created because it does not meet the minimum duration requirement.
  VideoMinDurationError,
  /// Media could not be created because it does not meet the minimum height requirement.
  VideoMinHeightError,
  /// Media could not be created because it does not meet the minimum width requirement.
  VideoMinWidthError,
  /// Video failed validation.
  VideoValidationError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaWarning {
  /// The code representing the type of warning.
  pub code: MediaWarningCode,
  /// Translated warning message.
  pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaWarningCode {
  /// 3D model physical size might be invalid. The dimensions of your model are very large. Consider reviewing your model to ensure they are correct.
  ModelLargePhysicalSize,
  /// 3D model physical size might be invalid. The dimensions of your model are very small. Consider reviewing your model to ensure they are correct.
  ModelSmallPhysicalSize,
}

/// Represents the preview image for a media.
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaPreviewImage {
  /// The preview image for the media. Returns null until status is READY.
  pub image: Image,
  /// Current status of the preview image.
  pub status: MediaStatus,
}

/// The possible statuses for a media object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaStatus {
  /// Media processing has failed.
  Failed,
  /// Media is being processed.
  Processing,
  /// Media is ready to be displayed.
  Ready,
  /// Media has been uploaded but not yet processed.
  Uploaded,
}