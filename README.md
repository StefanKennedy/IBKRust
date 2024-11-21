# IBKRust

Non-blocking, comprehensive and easy-to-use IBKR API in Rust (Interactive Brokers API in Rust). This
works by invoking the C++ API using a Foreign Function Interface (FFI).

### Executing Trades

```rust
let gme_symbol = Symbol {
  symbol_id: "GME".to_string(),
  currency: "USD".to_string(),
  security_type: "STK".to_string(),
  exchange: "SMART".to_string(),
  timezone: "US/Eastern".to_string(),
};
let operation = TradeOperation {
  symbol: symbol,
  operator: TradeOperator::BUY,
};

let broker_client = Box::new(IbkrBrokerClient {
  client: ibkr_service::CompatibleClient {
    client: stat_ibkr_service().lock().unwrap().connect("127.0.0.1", 7497).raw_interface()
  }
});
let broker_service = IbkrService{};

broker_service.trade(operation, &broker_client);
```

### Subscribing to Tick-by-tick Data

```rust
pub fn on_tick(req_id: i32, timestamp: i64, bid: f64, ask: f64, bid_size: f64,
               ask_size: f64, historical: bool) {
   // Callback code here
}

let broker_client = Box::new(IbkrBrokerClient {
  client: ibkr_service::CompatibleClient {
    client: stat_ibkr_service().lock().unwrap().connect("127.0.0.1", 7497).raw_interface()
  }
});
let broker_service = IbkrService{};
let request_id: i64 =
  broker_service.next_valid_id(&broker_client);

let gme_symbol = Symbol {
  symbol_id: "GME".to_string(),
  currency: "USD".to_string(),
  security_type: "STK".to_string(),
  exchange: "SMART".to_string(),
  timezone: "US/Eastern".to_string(),
};
let stream = TickStream {
  symbol: gme_symbol,
  primed: false  // Useful for initialising the strategy using the tick stream.
}
let tick_callback: Arc<fn(i32, i64, f64, f64, f64, f64, bool)> = Arc::new(on_tick);
broker_service.lock().unwrap().start_tick_stream(
    request_id, stream, tick_callback, &broker_client);
```

### Requesting Historical Tick-by-tick Data

```rust
pub fn on_tick(req_id: i32, timestamp: i64, bid: f64, ask: f64, bid_size: f64,
               ask_size: f64, historical: bool) {
   // Callback code here
}
pub fn historical_ticks_done(req_id: i32) {
  // Finish callback code here
}

let broker_client = Box::new(IbkrBrokerClient {
  client: ibkr_service::CompatibleClient {
    client: stat_ibkr_service().lock().unwrap().connect("127.0.0.1", 7497).raw_interface()
  }
});
let broker_service = IbkrService{};
let request_id: i64 =
  broker_service.lock().unwrap().next_valid_id(&broker_client);

let gme_symbol = Symbol {
  symbol_id: "GME".to_string(),
  currency: "USD".to_string(),
  security_type: "STK".to_string(),
  exchange: "SMART".to_string(),
  timezone: "US/Eastern".to_string(),
};
let stream = TickStream {
  symbol: gme_symbol,
  primed: false  // Useful for initialising the strategy using the tick stream.
}
let end_time = Local::now().format("%Y%m%d 00:00:00").to_string()
let historical_ticks_finished_callback: Arc<fn(i32)> =
  Arc::new(historical_config.finish_data_callback);
let tick_stream_callback: Arc<fn(i32, i64, f64, f64, f64, f64, bool)> =
  Arc::new(on_tick);
broker_service.lock().unwrap().read_historical_ticks(
  request_id, stream, end_time, tick_stream_callback,
  &historical_ticks_finished_callback, &broker_client);
```

### Requesting Executed Trades

```rust
pub fn execution(symbol: String, execution_type: String, time: String, price: f64) {
  // Callback code here
}
pub fn finish_executions() {
  // Finish callback code here
}

let broker_client = Box::new(IbkrBrokerClient {
  client: ibkr_service::CompatibleClient {
    client: stat_ibkr_service().lock().unwrap().connect("127.0.0.1", 7497).raw_interface()
  }
});
let broker_service = IbkrService{};

let execution_callback: Arc<fn(String, String, String, f64)> = Arc::new(execution);
let finish_executions_callback: Arc<fn()> = Arc::new(finish_executions);
stat_ibkr_service().lock().unwrap().get_executions(
    &execution_callback, &finish_executions_callback, &broker_client);
```
