// © 2024 Stefan Kennedy <stefankennedy5@gmail.com> All Rights Reserved

use crate::broker_service::BrokerService;
use crate::broker_service::BrokerClient;
use crate::trade::TickData;
use crate::trade::TickStream;
use crate::trade::TradeOperator;
use crate::trade::TradeOperation;
use std::sync::Arc;

pub struct IbkrBrokerClient {
  pub client: *mut CppClient,
}

impl BrokerClient for IbkrBrokerClient {
  type ThirdPartyInterface = *mut CppClient;
  fn raw_interface(&self) -> Self::ThirdPartyInterface {
    return self.client;
  }
}

pub struct IbkrService { }

impl IbkrService {
  pub fn process_messages(client: &Box<dyn BrokerClient<ThirdPartyInterface
                                             = *mut TestCppClient>>) {
  }

  pub fn is_connected(client: &Box<dyn BrokerClient<ThirdPartyInterface
                             = *mut TestCppClient>>) -> bool {
    return false;
  }
}

impl BrokerService for IbkrService {
  type ThirdPartyInterface = *mut TestCppClient;

  fn connect(&mut self, address: &str, port: i32)
      -> Box<dyn BrokerClient<ThirdPartyInterface = *mut CppClient>> {
    return Box::new();
  }

  fn next_valid_id(
      &mut self,
      client: &Box<dyn BrokerClient<ThirdPartyInterface = Self::ThirdPartyInterface>>)
          -> i64 {
    return -1;
  }

  fn read_historical_ticks(
      &mut self,
      request_id: i64,
      stream: TickStream,
      timezone: String,
      historical_end_datetime: String,
      callback: &Arc<fn(i32, i64, f64, f64, f64, f64, bool)>,
      finish_historical_ticks_callback: &Arc<fn(i32)>,
      client: &Box<dyn BrokerClient<ThirdPartyInterface
                                        = *mut CppClient>>) {
  }

  fn start_tick_stream(
      &mut self, request_id: i64, stream: TickStream,
      callback: &Arc<fn(i32, i64, f64, f64, f64, f64, bool)>,
      client: &Box<dyn BrokerClient<ThirdPartyInterface
                                        = *mut CppClient>>) {
  }

  fn trade(&mut self, trade_details: TradeOperation,
           client: &Box<dyn BrokerClient<ThirdPartyInterface
                                             = *mut CppClient>>) {
  }

  fn get_executions(
      &mut self,
      callback: &Arc<fn(String, String, String, f64)>,
      finish_callback: &Arc<fn()>,
      client: &Box<dyn BrokerClient<ThirdPartyInterface
                                        = *mut CppClient>>) {
  }

}
