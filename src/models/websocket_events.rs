use crate::prelude::*;

/// Enum representing various WebSocket event types.
///
/// Encapsulates different types of WebSocket events, such as order book updates, trades, and position changes, received from Bybit’s WebSocket API. Bots use this to handle real-time market and account data for trading strategies.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketEvents {
    /// An order book update event.
    ///
    /// Contains real-time updates to the order book for a trading pair. Bots use this for market depth analysis and liquidity monitoring.
    OrderBookEvent(OrderBookUpdate),
    /// A trade event.
    ///
    /// Contains details of executed trades in the market. Bots use this for price discovery and trade signal generation.
    TradeEvent(TradeUpdate),
    /// A ticker event.
    ///
    /// Contains real-time ticker data, such as last price and volume. Bots use this for monitoring market conditions and technical analysis.
    TickerEvent(WsTicker),
    /// A liquidation event.
    ///
    /// Contains details of liquidation events in the market. Bots use this to assess market risk and volatility.
    LiquidationEvent(Liquidation),
    /// A kline (candlestick) event.
    ///
    /// Contains real-time candlestick data for a trading pair. Bots use this for technical analysis and trend detection.
    KlineEvent(WsKline),
    /// A position update event.
    ///
    /// Contains updates to the account’s positions. Bots use this to track position changes and manage risk in real time.
    PositionEvent(PositionEvent),
    /// An execution event.
    ///
    /// Contains details of order executions for the account. Bots use this to confirm trade executions and update order status.
    ExecutionEvent(Execution),
    /// An order update event.
    ///
    /// Contains updates to the account’s orders. Bots use this to monitor order status and implement dynamic order management.
    OrderEvent(OrderEvent),
    /// A wallet update event.
    ///
    /// Contains updates to the account’s wallet balance and margin. Bots use this to monitor account health and manage capital.
    Wallet(WalletEvent),
    /// A trade stream event.
    ///
    /// Contains real-time trade stream data, typically for high-frequency trading. Bots use this for low-latency trade monitoring.
    TradeStream(TradeStreamEvent),
    /// A fast execution event.
    ///
    /// Contains minimal execution data for low-latency processing. Bots use this for high-frequency trading and rapid execution confirmation.
    FastExecEvent(FastExecution),
}
