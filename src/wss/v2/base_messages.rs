use crate::wss::v2::admin_messages::StatusUpdate;
use crate::wss::v2::market_data_messages::{Instruments, Ohlc, Ticker, Trade, L2, L3};
use crate::wss::v2::trading_messages::{
    AddOrderResult, BatchCancelResponse, CancelAllOrdersResult, CancelOnDisconnectResult,
    CancelOrderResult, EditOrderResult,
};
use crate::wss::v2::user_data_messages::{
    BalanceResponse, ExecutionResponse, SubscriptionResult, UnsubscriptionResult,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value::Null;
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WssMessage {
    Channel(ChannelMessage),
    Method(MethodMessage),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "method")]
pub enum MethodMessage {
    #[serde(rename = "add_order")]
    AddOrder(ResultResponse<AddOrderResult>),
    #[serde(rename = "edit_order")]
    EditOrder(ResultResponse<EditOrderResult>),
    #[serde(rename = "cancel_order")]
    CancelOrder(ResultResponse<CancelOrderResult>),
    #[serde(rename = "cancel_all")]
    CancelAllOrders(ResultResponse<CancelAllOrdersResult>),
    #[serde(rename = "cancel_all_orders_after")]
    CancelOnDisconnect(ResultResponse<CancelOnDisconnectResult>),
    #[serde(rename = "batch_add")]
    BatchOrder(ResultResponse<Vec<AddOrderResult>>),
    #[serde(rename = "batch_cancel")]
    BatchCancel(BatchCancelResponse),
    #[serde(rename = "subscribe")]
    Subscription(ResultResponse<SubscriptionResult>),
    #[serde(rename = "unsubscribe")]
    Unsubscription(ResultResponse<UnsubscriptionResult>),
    #[serde(alias = "ping")]
    Ping(ResultResponse<Option<()>>),
    #[serde(rename = "pong")]
    Pong(PongResponse),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "channel")]
pub enum ChannelMessage {
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "status")]
    Status(SingleResponse<StatusUpdate>),
    #[serde(rename = "executions")]
    Execution(ExecutionResponse),
    #[serde(rename = "balances")]
    Balance(BalanceResponse),
    #[serde(rename = "trade")]
    Trade(MarketDataResponse<Vec<Trade>>),
    #[serde(rename = "ticker")]
    Ticker(SingleResponse<Ticker>),
    #[serde(rename = "ohlc")]
    Ohlc(MarketDataResponse<Vec<Ohlc>>),
    #[serde(rename = "instrument")]
    Instrument(MarketDataResponse<Instruments>),
    #[serde(rename = "book")]
    Orderbook(SingleResponse<L2>),
    #[serde(rename = "level3")]
    L3(SingleResponse<L3>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message<T>
where
    T: Debug,
{
    pub method: String,
    #[serde(skip_serializing_if = "is_none")]
    pub params: T,
    pub req_id: i64,
}

impl<T> Message<T>
where
    T: Debug,
{
    pub fn new_subscription(params: T, req_id: i64) -> Self {
        Message {
            method: "subscribe".to_string(),
            params,
            req_id,
        }
    }

    pub fn new_unsubscription(params: T, req_id: i64) -> Self {
        Message {
            method: "unsubscribe".to_string(),
            params,
            req_id,
        }
    }
}

// this is required to not serialize None for generic type parameters
//  (skip_serializing_none fails there)
fn is_none<T: Serialize>(t: T) -> bool {
    serde_json::to_value(t).unwrap_or(Null).is_null()
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Pong {
    pub warning: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MarketDataResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SingleResponse<T>
where
    T: for<'a> Deserialize<'a>,
{
    #[serde(deserialize_with = "flatten_vec")]
    pub data: T,
}

fn flatten_vec<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> Deserialize<'a>,
{
    let mut vec: VecDeque<T> = de::Deserialize::deserialize(deserializer)?;
    vec.pop_front()
        .ok_or(de::Error::custom("Expected Vec with at least one element"))
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResultResponse<T> {
    pub result: Option<T>,
    pub error: Option<String>,
    pub success: bool,
    pub req_id: i64,
    pub time_in: String,
    pub time_out: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ErrorResponse {
    pub error: Option<String>,
    pub method: String,
    pub status: String,
    pub success: bool,
    pub req_id: i64,
    pub time_in: String,
    pub time_out: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PongResponse {
    pub error: Option<String>,
    pub req_id: i64,
    pub time_in: String,
    pub time_out: String,
}

#[cfg(test)]
mod tests {
    use crate::request_types::{TimeInForce, TriggerType};
    use crate::response_types::{BuySell, OrderStatusV2, OrderType, SystemStatus};
    use crate::wss::v2::admin_messages::StatusUpdate;
    use crate::wss::v2::base_messages::{
        ChannelMessage, ErrorResponse, SingleResponse, WssMessage,
    };
    use crate::wss::v2::trading_messages::{FeePreference, PriceType};
    use crate::wss::v2::user_data_messages::{
        ExecutionResponse, ExecutionResult, ExecutionType, TriggerDescription, TriggerStatus,
        UserDataResponse,
    };
    use rust_decimal_macros::dec;
    use serde_json::Number;
    use std::str::FromStr;

    #[test]
    fn test_deserializing_status_update() {
        let message = r#"{"channel":"status","data":[{"api_version":"v2","connection_id":18266300427528990701,"system":"online","version":"2.0.4"}],"type":"update"}"#;
        let expected = WssMessage::Channel(ChannelMessage::Status(SingleResponse {
            data: StatusUpdate {
                api_version: "v2".to_string(),
                connection_id: Number::from_str("18266300427528990701").unwrap(),
                system: SystemStatus::Online,
                version: "2.0.4".to_string(),
            },
        }));

        let parsed = serde_json::from_str::<WssMessage>(message).unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_deserializing_l2_update() {
        let raw = r#"{"channel":"book","type":"update","data":[{"symbol":"BTC/USD","bids":[],"asks":[{"price":66732.5,"qty":5.48256063}],"checksum":2855135483,"timestamp":"2024-05-19T16:32:26.777454Z"}]}"#;
        let _parsed = serde_json::from_str::<ChannelMessage>(raw).unwrap();
    }

    #[test]
    fn test_deserializing_execution_snapshot() {
        let message = r#"{"channel":"executions","type":"snapshot","data":[{"order_id":"AHOJQ8-1E72C-8M2VQH","symbol":"ADX/USD","order_qty":81.36256082,"cum_cost":0,"time_in_force":"GTC","exec_type":"pending_new","side":"buy","order_type":"stop-loss-limit","order_userref":0,"limit_price_type":"static","triggers":{"price":0.2,"price_type":"static","reference":"index","status":"untriggered"},"stop_price":0.2,"limit_price":0.2,"trigger":"index","order_status":"pending_new","fee_usd_equiv":0,"fee_ccy_pref":"fciq","timestamp":"2024-05-18T12:01:56.165888Z"}],"sequence":120}"#;
        let expected = WssMessage::Channel(ChannelMessage::Execution(ExecutionResponse::Snapshot(
            UserDataResponse {
                sequence: 120,
                data: vec![ExecutionResult {
                    execution_type: ExecutionType::PendingNew,
                    cash_order_quantity: None,
                    contingent: None,
                    cost: None,
                    execution_id: None,
                    fees: None,
                    liquidity_indicator: None,
                    last_price: None,
                    last_quantity: None,
                    average_price: None,
                    reason: None,
                    cumulative_cost: Some(dec!(0.0)),
                    cumulative_quantity: None,
                    display_quantity: None,
                    effective_time: None,
                    expire_time: None,
                    fee_preference: Some(FeePreference::Quote),
                    fee_usd_equivalent: Some(dec!(0.0)),
                    limit_price: Some(dec!(0.2)),
                    limit_price_type: Some(PriceType::Static),
                    margin: None,
                    no_market_price_protection: None,
                    order_ref_id: None,
                    order_id: "AHOJQ8-1E72C-8M2VQH".to_string(),
                    order_quantity: Some(dec!(81.36256082)),
                    order_type: Some(OrderType::StopLossLimit),
                    order_status: OrderStatusV2::PendingNew,
                    order_user_ref: Some(0),
                    post_only: None,
                    position_status: None,
                    reduce_only: None,
                    side: Some(BuySell::Buy),
                    symbol: Some("ADX/USD".to_string()),
                    time_in_force: Some(TimeInForce::GTC),
                    timestamp: "2024-05-18T12:01:56.165888Z".to_string(),
                    trade_id: None,
                    triggers: Some(TriggerDescription {
                        reference: TriggerType::Index,
                        price: dec!(0.2),
                        price_type: PriceType::Static,
                        actual_price: None,
                        peak_price: None,
                        last_price: None,
                        status: TriggerStatus::Untriggered,
                        timestamp: None,
                    }),
                    client_order_id: None,
                }],
            },
        )));

        let parsed = serde_json::from_str::<WssMessage>(message).unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_deserializing_error_message() {
        let raw = r#"{"error":"ESession:Invalid session","method":"subscribe","req_id":42,"status":"error","success":false,"time_in":"2023-04-19T12:04:41.320119Z","time_out":"2023-04-19T12:04:41.980119Z"}"#;

        let expected = WssMessage::Error(ErrorResponse {
            error: Some("ESession:Invalid session".to_string()),
            method: "subscribe".to_string(),
            status: "error".to_string(),
            success: false,
            req_id: 42,
            time_in: "2023-04-19T12:04:41.320119Z".to_string(),
            time_out: "2023-04-19T12:04:41.980119Z".to_string(),
        });

        let parsed = serde_json::from_str::<WssMessage>(raw).unwrap();
        assert_eq!(expected, parsed);
    }
}
