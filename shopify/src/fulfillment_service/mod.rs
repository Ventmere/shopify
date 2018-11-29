use client::{Client, Method};
use result::*;

mod types;
pub use self::types::*;

pub trait FulfillmentServiceApi {
  fn get_list(
    &self,
    scope: Option<FulfillmentServiceScope>,
  ) -> ShopifyResult<Vec<FulfillmentService>>;

  fn create(
    &self,
    fulfillment_service: &NewFulfillmentService,
  ) -> ShopifyResult<FulfillmentService>;

  fn get(&self, id: i64) -> ShopifyResult<FulfillmentService>;

  fn update(
    &self,
    id: i64,
    fulfillment_service: &UpdateFulfillmentService,
  ) -> ShopifyResult<FulfillmentService>;

  fn delete(&self, id: i64) -> ShopifyResult<()>;
}

impl FulfillmentServiceApi for Client {
  fn get_list(
    &self,
    scope: Option<FulfillmentServiceScope>,
  ) -> ShopifyResult<Vec<FulfillmentService>> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_services: Vec<FulfillmentService>,
      }
    }

    let res: Res = self.request_with_params(
      Method::Get,
      "/admin/fulfillment_services.json",
      &scope.map(|scope| ("scope", scope)),
      |_| {},
    )?;
    Ok(res.into_inner())
  }

  fn create(
    &self,
    fulfillment_service: &NewFulfillmentService,
  ) -> ShopifyResult<FulfillmentService> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = "/admin/fulfillment_services.json";
    let res: Res = self.request(Method::Post, &path, move |b| {
      b.json(&json!({ "fulfillment_service": fulfillment_service }));
    })?;
    Ok(res.into_inner())
  }

  fn get(&self, id: i64) -> ShopifyResult<FulfillmentService> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = format!("/admin/fulfillment_services/{}.json", id);
    let res: Res = self.request(Method::Get, &path, move |_| {})?;
    Ok(res.into_inner())
  }

  fn update(
    &self,
    id: i64,
    fulfillment_service: &UpdateFulfillmentService,
  ) -> ShopifyResult<FulfillmentService> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = format!("/admin/fulfillment_services/{}.json", id);
    let res: Res = self.request(Method::Put, &path, move |b| {
      b.json(&json!({ "fulfillment_service": fulfillment_service }));
    })?;
    Ok(res.into_inner())
  }

  fn delete(&self, id: i64) -> ShopifyResult<()> {
    use serde_json::Value;
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = format!("/admin/fulfillment_services/{}.json", id);
    self.request::<Value, _>(Method::Delete, &path, |_| {})?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn test_fulfillment_service_get_list() {
    let client = ::client::get_test_client();
    let service = client
      .get_list(Some(FulfillmentServiceScope::CurrentClient))
      .unwrap();
    println!("{:#?}", service);
  }

  #[test]
  #[ignore]
  fn test_fulfillment_service_get_one() {
    let client = ::client::get_test_client();
    let service = client.get(191681).unwrap();
    println!("{:#?}", service);
  }

  #[test]
  #[ignore]
  fn test_fulfillment_service_create() {
    let client = ::client::get_test_client();
    let service = client
      .create(&NewFulfillmentService {
        name: "S2".to_owned(),
        callback_url: "https://requestb.in/1gnver61".to_owned(),
        inventory_management: true,
        tracking_support: true,
        requires_shipping_method: true,
        format: "json".to_owned(),
      })
      .unwrap();
    println!("{:#?}", service);
  }

  #[test]
  #[ignore]
  fn test_fulfillment_service_update() {
    let client = ::client::get_test_client();
    let mut update = UpdateFulfillmentService::default();
    update.name = Some("Ventmere S2".to_owned());
    let service = client.update(13008909, &update).unwrap();
    println!("{:#?}", service);
  }

  #[test]
  #[ignore]
  fn test_fulfillment_service_delete() {
    let client = ::client::get_test_client();
    client.delete(12976141).optional().unwrap();
  }
}
