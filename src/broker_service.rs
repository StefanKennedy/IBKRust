// © 2024 Stefan Kennedy <stefankennedy5@gmail.com> All Rights Reserved

use crate::trade::TickStream;
use crate::trade::TradeOperation;
use std::sync::Arc;

pub trait BrokerClient {
  type ThirdPartyInterface;
  fn raw_interface(&self) -> Self::ThirdPartyInterface;
}

pub trait BrokerService {
  type ThirdPartyInterface;

  fn connect(&mut self, address: &str, port: i32)
      -> Box<dyn BrokerClient<ThirdPartyInterface = Self::ThirdPartyInterface>>;

  fn next_valid_id(
      &mut self,
      client: &Box<dyn BrokerClient<ThirdPartyInterface = Self::ThirdPartyInterface>>)
          -> i64;

  fn read_historical_ticks(
      &mut self,
      request_id: i64, stream: TickStream, timezone: String,
      historical_end_datetime: String,
      callback: &Arc<fn(i32, i64, f64, f64, f64, f64, bool)>,
      finish_historical_ticks_callback: &Arc<fn(i32)>,
      client: &Box<dyn BrokerClient<ThirdPartyInterface
                                        = Self::ThirdPartyInterface>>);

  fn start_tick_stream(
      &mut self, request_id: i64, stream: TickStream,
      callback: &Arc<fn(i32, i64, f64, f64, f64, f64, bool)>,
      client: &Box<dyn BrokerClient<ThirdPartyInterface = Self::ThirdPartyInterface>>);

  fn trade(&mut self, trade_details: TradeOperation,
           client: &Box<dyn BrokerClient<ThirdPartyInterface
                                             = Self::ThirdPartyInterface>>);

  fn get_executions(&mut self, callback: &Arc<fn(String, String, String, f64)>,
                    finish_callback: &Arc<fn()>,
                    client: &Box<dyn BrokerClient<ThirdPartyInterface
                                             = Self::ThirdPartyInterface>>);
}
