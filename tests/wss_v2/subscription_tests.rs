use crate::wss_v2::shared::CallResponseTest;
use kraken_async_rs::wss::v2::base_messages::{MethodResponse, ResponseMessage, ResultResponse};
use kraken_async_rs::wss::v2::market_data_messages::{
    EventTrigger, TickerSubscription, TickerSubscriptionResponse,
};
use kraken_async_rs::wss::v2::user_data_messages::SubscriptionResult;
use serde_json::{json, Value};

mod execution_subscription {
    use super::*;
    use kraken_async_rs::crypto::secrets::Token;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::user_data_messages::{
        ExecutionSubscription, ExecutionsSubscriptionResult,
    };

    fn get_expected_execution_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"executions","token":"someToken","snapshot_trades":true,"snapshot":true},"req_id":0})
    }

    fn get_execution_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":0,"result":{"channel":"executions","maxratecount":180,"snapshot":true,"warnings":["cancel_reason is deprecated, use reason","stop_price is deprecated, use triggers.price","trigger is deprecated use triggers.reference","triggered_price is deprecated use triggers.last_price"]},"success":true,"time_in":"2024-05-19T19:30:36.343170Z","time_out":"2024-05-19T19:30:36.350083Z"}"#.to_string()
    }

    fn get_expected_execution_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Execution(
                ExecutionsSubscriptionResult {
                    max_rate_count: Some(180),
                    snapshot: Some(true),
                    warnings: Some(vec![
                        "cancel_reason is deprecated, use reason".into(),
                        "stop_price is deprecated, use triggers.price".into(),
                        "trigger is deprecated use triggers.reference".into(),
                        "triggered_price is deprecated use triggers.last_price".into(),
                    ]),
                },
            )),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T19:30:36.343170Z".to_string(),
            time_out: "2024-05-19T19:30:36.350083Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_execution_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::Execution(ExecutionSubscription {
                token: Token::new("someToken".to_string()),
                snapshot: Some(true),
                snapshot_trades: Some(true),
                rate_counter: None,
            })),
            req_id: 0,
        });

        CallResponseTest::builder()
            .match_on(get_expected_execution_subscription())
            .respond_with(get_execution_subscription_response())
            .send(subscription)
            .expect(get_expected_execution_message())
            .build()
            .test()
            .await;
    }
}

mod execution_unsubscription {
    use super::*;
    use kraken_async_rs::crypto::secrets::Token;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelUnsubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::user_data_messages::{
        ExecutionUnsubscription, ExecutionsUnsubscriptionResult, UnsubscriptionResult,
    };

    fn get_expected_request() -> Value {
        json!({"method":"unsubscribe","params":{"channel":"executions","token":"someToken"},"req_id":121})
    }

    fn get_expected_response() -> String {
        r#"{"method":"unsubscribe","req_id":121,"result":{"channel":"executions"},"success":true,"time_in":"2024-05-19T19:06:57.002983Z","time_out":"2024-05-19T19:06:57.003037Z"}"#.to_string()
    }

    fn get_expected_parsed_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Unsubscription(ResultResponse {
            result: Some(UnsubscriptionResult::Execution(
                ExecutionsUnsubscriptionResult {},
            )),
            error: None,
            success: true,
            req_id: 121,
            time_in: "2024-05-19T19:06:57.002983Z".to_string(),
            time_out: "2024-05-19T19:06:57.003037Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_execution_unsubscription() {
        let message = RequestMessage::Unsubscribe(RequestMessageBody {
            params: Some(ChannelUnsubscription::Execution(ExecutionUnsubscription {
                token: Token::new("someToken".to_string()),
            })),
            req_id: 121,
        });

        CallResponseTest::builder()
            .match_on(get_expected_request())
            .respond_with(get_expected_response())
            .send(message)
            .expect(get_expected_parsed_message())
            .build()
            .test()
            .await;
    }
}

mod balances_subscription {
    use super::*;
    use kraken_async_rs::crypto::secrets::Token;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::user_data_messages::{
        BalanceSubscriptionResult, BalancesSubscription,
    };

    fn get_expected_balances_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"balances","token":"anotherToken","snapshot":true},"req_id":10312008})
    }

    fn get_balances_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":10312008,"result":{"channel":"balances","snapshot":true},"success":true,"time_in":"2024-05-19T16:25:28.289124Z","time_out":"2024-05-19T16:25:28.293750Z"}"#.to_string()
    }

    fn get_expected_balances_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Balance(BalanceSubscriptionResult {
                snapshot: Some(true),
                warnings: None,
            })),
            error: None,
            success: true,
            req_id: 10312008,
            time_in: "2024-05-19T16:25:28.289124Z".to_string(),
            time_out: "2024-05-19T16:25:28.293750Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_balances_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::Balance(BalancesSubscription {
                token: Token::new("anotherToken".to_string()),
                snapshot: Some(true),
            })),
            req_id: 10312008,
        });

        CallResponseTest::builder()
            .match_on(get_expected_balances_subscription())
            .respond_with(get_balances_subscription_response())
            .send(subscription)
            .expect(get_expected_balances_message())
            .build()
            .test()
            .await;
    }
}

mod ticker_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };

    fn get_expected_ticker_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"ticker","symbol":["BTC/USD"]},"req_id":42})
    }

    fn get_ticker_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":42,"result":{"channel":"ticker","event_trigger":"trades","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-15T11:20:43.013486Z","time_out":"2024-05-15T11:20:43.013545Z"}"#.to_string()
    }

    fn get_expected_ticker_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Ticker(TickerSubscriptionResponse {
                symbol: "BTC/USD".to_string(),
                event_trigger: Some(EventTrigger::Trades),
                snapshot: Some(true),
            })),
            error: None,
            success: true,
            req_id: 42,
            time_in: "2024-05-15T11:20:43.013486Z".to_string(),
            time_out: "2024-05-15T11:20:43.013545Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_ticker_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::Ticker(TickerSubscription {
                symbol: vec!["BTC/USD".into()],
                event_trigger: None,
                snapshot: None,
            })),
            req_id: 42,
        });

        CallResponseTest::builder()
            .match_on(get_expected_ticker_subscription())
            .respond_with(get_ticker_subscription_response())
            .send(subscription)
            .expect(get_expected_ticker_message())
            .build()
            .test()
            .await;
    }
}

mod book_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::market_data_messages::{
        BookSubscription, BookSubscriptionResponse,
    };

    fn get_expected_book_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"book","symbol":["BTC/USD"],"depth":10,"snapshot":true},"req_id":11})
    }

    fn get_book_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":11,"result":{"channel":"book","depth":10,"snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T16:27:13.694962Z","time_out":"2024-05-19T16:27:13.695006Z"}"#.to_string()
    }

    fn get_expected_book_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Book(BookSubscriptionResponse {
                symbol: "BTC/USD".to_string(),
                snapshot: Some(true),
                depth: Some(10),
                warnings: None,
            })),
            error: None,
            success: true,
            req_id: 11,
            time_in: "2024-05-19T16:27:13.694962Z".to_string(),
            time_out: "2024-05-19T16:27:13.695006Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_book_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::Orderbook(BookSubscription {
                symbol: vec!["BTC/USD".into()],
                depth: Some(10),
                snapshot: Some(true),
                token: None,
            })),
            req_id: 11,
        });

        CallResponseTest::builder()
            .match_on(get_expected_book_subscription())
            .respond_with(get_book_subscription_response())
            .send(subscription)
            .expect(get_expected_book_message())
            .build()
            .test()
            .await;
    }
}

mod l3_subscription {
    use super::*;
    use kraken_async_rs::crypto::secrets::Token;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::market_data_messages::{
        BookSubscription, BookSubscriptionResponse,
    };

    fn get_expected_l3_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"level3","symbol":["BTC/USD"],"snapshot":true,"token":"someToken"},"req_id":99})
    }

    fn get_l3_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":99,"result":{"channel":"level3","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T18:51:30.701627Z","time_out":"2024-05-19T18:51:30.708403Z"}"#.to_string()
    }

    fn get_expected_l3_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::L3(BookSubscriptionResponse {
                symbol: "BTC/USD".to_string(),
                snapshot: Some(true),
                depth: None,
                warnings: None,
            })),
            error: None,
            success: true,
            req_id: 99,
            time_in: "2024-05-19T18:51:30.701627Z".to_string(),
            time_out: "2024-05-19T18:51:30.708403Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_l3_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::L3(BookSubscription {
                symbol: vec!["BTC/USD".into()],
                token: Some(Token::new("someToken".to_string())),
                snapshot: Some(true),
                depth: None,
            })),
            req_id: 99,
        });

        CallResponseTest::builder()
            .match_on(get_expected_l3_subscription())
            .respond_with(get_l3_subscription_response())
            .send(subscription)
            .expect(get_expected_l3_message())
            .build()
            .test()
            .await;
    }
}

mod ohlc_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::market_data_messages::{
        OhlcSubscription, OhlcSubscriptionResponse,
    };

    fn get_expected_ohlc_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"ohlc","symbol":["ETH/USD"],"interval":60},"req_id":121})
    }

    fn get_ohlc_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":121,"result":{"channel":"ohlc","interval":60,"snapshot":true,"symbol":"ETH/USD","warnings":["timestamp is deprecated, use interval_begin"]},"success":true,"time_in":"2024-05-19T19:06:57.002983Z","time_out":"2024-05-19T19:06:57.003037Z"}"#.to_string()
    }

    fn get_expected_ohlc_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Ohlc(OhlcSubscriptionResponse {
                symbol: Some("ETH/USD".to_string()),
                snapshot: Some(true),
                warnings: Some(vec!["timestamp is deprecated, use interval_begin".into()]),
                interval: 60,
            })),
            error: None,
            success: true,
            req_id: 121,
            time_in: "2024-05-19T19:06:57.002983Z".to_string(),
            time_out: "2024-05-19T19:06:57.003037Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_ohlc_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::Ohlc(OhlcSubscription {
                symbol: vec!["ETH/USD".into()],
                interval: 60,
                snapshot: None,
            })),
            req_id: 121,
        });

        CallResponseTest::builder()
            .match_on(get_expected_ohlc_subscription())
            .respond_with(get_ohlc_subscription_response())
            .send(subscription)
            .expect(get_expected_ohlc_message())
            .build()
            .test()
            .await;
    }
}

mod ohlc_unsubscription {
    use super::*;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelUnsubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::market_data_messages::{
        OhlcUnsubscription, OhlcUnsubscriptionResponse,
    };
    use kraken_async_rs::wss::v2::user_data_messages::UnsubscriptionResult;

    fn get_expected_request() -> Value {
        json!({"method":"unsubscribe","params":{"channel":"ohlc","symbol":["ETH/USD"],"interval":60},"req_id":121})
    }

    fn get_expected_response() -> String {
        r#"{"method":"unsubscribe","req_id":121,"result":{"channel":"ohlc","interval":60,"symbol":"ETH/USD"},"success":true,"time_in":"2024-05-19T19:06:57.002983Z","time_out":"2024-05-19T19:06:57.003037Z"}"#.to_string()
    }

    fn get_expected_parsed_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Unsubscription(ResultResponse {
            result: Some(UnsubscriptionResult::Ohlc(OhlcUnsubscriptionResponse {
                symbol: "ETH/USD".to_string(),
                interval: 60,
            })),
            error: None,
            success: true,
            req_id: 121,
            time_in: "2024-05-19T19:06:57.002983Z".to_string(),
            time_out: "2024-05-19T19:06:57.003037Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_ohlc_unsubscription() {
        let message = RequestMessage::Unsubscribe(RequestMessageBody {
            params: Some(ChannelUnsubscription::Ohlc(OhlcUnsubscription {
                symbol: vec!["ETH/USD".into()],
                interval: 60,
            })),
            req_id: 121,
        });

        CallResponseTest::builder()
            .match_on(get_expected_request())
            .respond_with(get_expected_response())
            .send(message)
            .expect(get_expected_parsed_message())
            .build()
            .test()
            .await;
    }
}

mod trade_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::market_data_messages::{
        TradeSubscriptionResponse, TradesSubscription,
    };

    fn get_expected_trade_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"trade","symbol":["BTC/USD"]},"req_id":0})
    }

    fn get_trade_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":0,"result":{"channel":"trade","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T19:11:23.034030Z","time_out":"2024-05-19T19:11:23.034073Z"}"#.to_string()
    }

    fn get_expected_trade_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Trade(TradeSubscriptionResponse {
                symbol: Some("BTC/USD".to_string()),
                snapshot: Some(true),
                warnings: None,
            })),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T19:11:23.034030Z".to_string(),
            time_out: "2024-05-19T19:11:23.034073Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_trade_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::Trade(TradesSubscription {
                symbol: vec!["BTC/USD".into()],
                snapshot: None,
            })),
            req_id: 0,
        });

        CallResponseTest::builder()
            .match_on(get_expected_trade_subscription())
            .respond_with(get_trade_subscription_response())
            .send(subscription)
            .expect(get_expected_trade_message())
            .build()
            .test()
            .await;
    }
}

mod instruments_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::base_messages::{
        ChannelSubscription, RequestMessage, RequestMessageBody,
    };
    use kraken_async_rs::wss::v2::market_data_messages::InstrumentsSubscription;
    use kraken_async_rs::wss::v2::user_data_messages::InstrumentSubscriptionResult;

    fn get_expected_instruments_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"instrument","snapshot":true},"req_id":0})
    }

    fn get_instruments_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":0,"result":{"channel":"instrument","snapshot":true,"warnings":["tick_size is deprecated, use price_increment"]},"success":true,"time_in":"2024-05-19T19:44:43.264430Z","time_out":"2024-05-19T19:44:43.264464Z"}"#.to_string()
    }

    fn get_expected_instruments_message() -> ResponseMessage {
        ResponseMessage::Method(MethodResponse::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Instrument(
                InstrumentSubscriptionResult {
                    snapshot: Some(true),
                    warnings: Some(vec!["tick_size is deprecated, use price_increment".into()]),
                },
            )),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T19:44:43.264430Z".to_string(),
            time_out: "2024-05-19T19:44:43.264464Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_instruments_subscription() {
        let subscription = RequestMessage::Subscribe(RequestMessageBody {
            params: Some(ChannelSubscription::Instrument(InstrumentsSubscription {
                snapshot: Some(true),
            })),
            req_id: 0,
        });

        CallResponseTest::builder()
            .match_on(get_expected_instruments_subscription())
            .respond_with(get_instruments_subscription_response())
            .send(subscription)
            .expect(get_expected_instruments_message())
            .build()
            .test()
            .await;
    }
}
