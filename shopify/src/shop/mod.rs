use crate::client::{Client, Method};
use crate::result::*;

mod types;
pub use self::types::*;

shopify_wrap! {
  pub struct GetShop {
    shop: Shop,
  }
}

pub trait ShopApi {
  fn get(&self) -> ShopifyResult<Shop>;
}

impl ShopApi for Client {
  fn get(&self) -> ShopifyResult<Shop> {
    let res: GetShop = self.request(Method::GET, "/admin/shop.json", std::convert::identity)?;
    Ok(res.into_inner())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deserialize() {
    use serde_json;
    serde_json::from_str::<GetShop>(
      r##"
    {
      "shop": {
        "id": 111,
        "name": "111 Canada",
        "email": "info@111.com",
        "domain": "111.myshopify.com",
        "created_at": "2015-02-20T09:22:32-05:00",
        "province": "Ontario",
        "country": "CA",
        "address1": "11 Random Ave",
        "zip": "111 111",
        "city": "Toronto",
        "source": "source",
        "phone": "4168430398",
        "updated_at": "2017-11-09T15:22:21-05:00",
        "customer_email": "webstore@111.com",
        "latitude": 41.8054023,
        "longitude": -71.5281833,
        "primary_location_id": 11111111,
        "primary_locale": "en",
        "address2": "Suite 11",
        "country_code": "CA",
        "country_name": "Canada",
        "currency": "CAD",
        "timezone": "(GMT-05:00) Eastern Time (US & Canada)",
        "iana_timezone": "America/New_York",
        "shop_owner": "111 Ltd",
        "money_format": "${{amount}}",
        "money_with_currency_format": "${{amount}} CAD",
        "weight_unit": "kg",
        "province_code": "ON",
        "taxes_included": false,
        "tax_shipping": true,
        "county_taxes": true,
        "plan_display_name": "Basic Shopify",
        "plan_name": "basic",
        "has_discounts": true,
        "has_gift_cards": false,
        "myshopify_domain": "111.myshopify.com",
        "google_apps_domain": null,
        "google_apps_login_enabled": null,
        "money_in_emails_format": "${{amount}}",
        "money_with_currency_in_emails_format": "${{amount}} CAD",
        "eligible_for_payments": true,
        "requires_extra_payments_agreement": false,
        "password_enabled": false,
        "has_storefront": true,
        "eligible_for_card_reader_giveaway": true,
        "finances": true,
        "setup_required": false,
        "force_ssl": true
      }
    }
    "##,
    )
    .unwrap();
  }

  #[test]
  #[ignore]
  fn test_get_shop() {
    let client = crate::client::get_test_client();
    let shop = client.get().unwrap();
    println!("{:#?}", shop);
  }
}
