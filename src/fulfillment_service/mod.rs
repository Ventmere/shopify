use error::*;
use client::{Client, Method};

mod types;
pub use self::types::*;

pub struct FulfillmentServiceApi<'a> {
  client: &'a Client,
}

impl Client {
  pub fn fulfillment_service(&self) -> FulfillmentServiceApi {
    FulfillmentServiceApi { client: self }
  }
}

impl<'a> FulfillmentServiceApi<'a> {
  pub fn get_list(
    &self,
    scope: Option<FulfillmentServiceScope>,
  ) -> Result<Vec<FulfillmentService>> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_services: Vec<FulfillmentService>,
      }
    }

    let res: Res = self.client.request_with_params(
      Method::Get,
      "/admin/fulfillment_services.json",
      &scope.map(|scope| ("scope", scope)),
      |_| {},
    )?;
    Ok(res.into_inner())
  }

  pub fn create(&self, fulfillment_service: &NewFulfillmentService) -> Result<FulfillmentService> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = "/admin/fulfillment_services.json";
    let res: Res = self.client.request(Method::Post, &path, move |b| {
      b.json(&json!({
        "fulfillment_service": fulfillment_service
      }));
    })?;
    Ok(res.into_inner())
  }

  pub fn get(&self, id: i64) -> Result<FulfillmentService> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = format!("/admin/fulfillment_services/{}.json", id);
    let res: Res = self.client.request(Method::Get, &path, move |_| {})?;
    Ok(res.into_inner())
  }

  pub fn update(
    &self,
    id: i64,
    fulfillment_service: &UpdatetFulfillmentService,
  ) -> Result<FulfillmentService> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = format!("/admin/fulfillment_services/{}.json", id);
    let res: Res = self.client.request(Method::Put, &path, move |b| {
      b.json(&json!({
        "fulfillment_service": fulfillment_service
      }));
    })?;
    Ok(res.into_inner())
  }

  pub fn delete(&self, id: i64) -> Result<()> {
    use serde_json::Value;
    shopify_wrap! {
      pub struct Res {
        fulfillment_service: FulfillmentService,
      }
    }
    let path = format!("/admin/fulfillment_services/{}.json", id);
    self.client.request::<Value, _>(
      Method::Delete,
      &path,
      |_| {},
    )?;
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
      .fulfillment_service()
      .get_list(Some(FulfillmentServiceScope::All))
      .unwrap();
    println!("{:#?}", service);
  }

  #[test]
  #[ignore]
  fn test_fulfillment_service_get_one() {
    let client = ::client::get_test_client();
    let service = client.fulfillment_service().get(191681).unwrap();
    println!("{:#?}", service);
  }

  #[test]
  #[ignore]
  fn test_fulfillment_service_create() {
    let client = ::client::get_test_client();
    let service = client
      .fulfillment_service()
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
    let mut update = UpdatetFulfillmentService::default();
    update.name = Some("Ventmere S2".to_owned());
    let service = client
      .fulfillment_service()
      .update(13008909, &update)
      .unwrap();
    println!("{:#?}", service);
  }

  #[test]
  #[ignore]
  fn test_fulfillment_service_delete() {
    let client = ::client::get_test_client();
    client
      .fulfillment_service()
      .delete(12976141)
      .optional()
      .unwrap();
  }
}