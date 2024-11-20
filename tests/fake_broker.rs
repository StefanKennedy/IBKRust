// © 2024 Stefan Kennedy <stefankennedy5@gmail.com> All Rights Reserved

use crate::broker_service::BrokerClient;

pub struct FakeBrokerClient {}

impl BrokerClient for FakeBrokerClient {
  type ThirdPartyInterface = bool;
  fn raw_interface(&self) -> Self::ThirdPartyInterface {
    return true;
  }
}
