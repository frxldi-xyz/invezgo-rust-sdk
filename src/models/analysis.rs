//! Response types for analysis API

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetStockListResponse {
    pub code: String,
    pub name: String,
    pub sector: String,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetBrokerListResponse {
    pub name: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetIndexListResponse {
    pub code: String,
    pub name: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InformationResponseCommissionerItem {
    pub name: String,
    pub position: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InformationResponseDirectorItem {
    pub name: String,
    pub position: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InformationResponseSubsidiaryItem {
    pub name: String,
    pub percentage: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InformationResponse {
    pub code: String,
    pub address: String,
    pub industry: String,
    pub subsindustry: String,
    pub activity: String,
    pub name: String,
    pub npwp: String,
    pub board: String,
    pub sector: String,
    pub subsector: String,
    pub listing_date: String,
    pub website: String,
    pub logo: String,
    pub category: Vec<String>,
    pub additional_info: Option<String>,
    pub people: Option<serde_json::Value>,
    pub report_type: Option<String>,
    pub administration: Option<serde_json::Value>,
    pub description: Option<String>,
    pub ipo_pct: Option<f64>,
    pub ipo_price: Option<f64>,
    pub ipo_share: Option<i64>,
    pub ipo_underwriter: Option<String>,
    pub nominal_price: Option<f64>,
    pub commissioner: Vec<InformationResponseCommissionerItem>,
    pub director: Vec<InformationResponseDirectorItem>,
    pub subsidiary: Vec<InformationResponseSubsidiaryItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopChangeResponseGainItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopChangeResponseGainItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: f64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopChangeResponseGainItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopChangeResponseLossItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopChangeResponseLossItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: f64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopChangeResponseLossItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopChangeResponse {
    pub gain: Vec<TopChangeResponseGainItem>,
    pub loss: Vec<TopChangeResponseLossItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopForeignResponseAccumItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopForeignResponseAccumItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: f64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopForeignResponseAccumItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopForeignResponseDistItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopForeignResponseDistItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: f64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopForeignResponseDistItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopForeignResponse {
    pub accum: Vec<TopForeignResponseAccumItem>,
    pub dist: Vec<TopForeignResponseDistItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopAccumulationResponseAccumItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopAccumulationResponseAccumItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: f64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopAccumulationResponseAccumItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopAccumulationResponseDistItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopAccumulationResponseDistItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: i64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopAccumulationResponseDistItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopAccumulationResponse {
    pub accum: Vec<TopAccumulationResponseAccumItem>,
    pub dist: Vec<TopAccumulationResponseDistItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopRitelResponseAccumItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopRitelResponseAccumItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: f64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopRitelResponseAccumItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopRitelResponseDistItemGraphItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopRitelResponseDistItem {
    pub code: String,
    pub name: String,
    pub price: i64,
    pub change: i64,
    pub value: i64,
    pub volume: i64,
    pub logo: String,
    pub graph: Vec<TopRitelResponseDistItemGraphItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopRitelResponse {
    pub accum: Vec<TopRitelResponseAccumItem>,
    pub dist: Vec<TopRitelResponseDistItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetIntradayChartResponse {
    pub date: String,
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
    pub volume: i64,
    pub freq: i64,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetOrderBookResponseBidItem {
    pub bid1price: i64,
    pub bid1lot: i64,
    pub bid1freq: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetOrderBookResponseOfferItem {
    pub offer1price: i64,
    pub offer1lot: i64,
    pub offer1freq: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetOrderBookResponse {
    pub code: String,
    pub bid: Vec<GetOrderBookResponseBidItem>,
    pub offer: Vec<GetOrderBookResponseOfferItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetIntradayDataResponse {
    pub code: String,
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
    pub avg: f64,
    pub volume: i64,
    pub freq: i64,
    pub value: i64,
    pub prev: i64,
    pub bid_price: i64,
    pub bid_lot: i64,
    pub bid_freq: i64,
    pub offer_price: i64,
    pub offer_lot: i64,
    pub offer_freq: i64,
    pub iep: i64,
    pub iev: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetIntradayIndexResponse {
    pub code: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub freq: i64,
    pub value: i64,
    pub prev: f64,
    pub positive: i64,
    pub negative: i64,
    pub neutral: i64,
    pub suspend: i64,
    pub market_cap: i64,
    pub market_value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetAdvanceChartResponse {
    pub date: String,
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
    pub volume: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetIndexChartResponse {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetMultiTimeChartResponse {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetIndicatorChartResponse {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderDetailResponse {
    pub code: String,
    pub name: String,
    pub date: String,
    pub val: String,
    pub percent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderOneDetailResponse {
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub status: String,
    pub nationality: String,
    pub domicile: String,
    pub scripless: i64,
    pub scrip: i64,
    pub total: i64,
    pub percentage: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderHighResponse {
    pub code: String,
    pub date: String,
    pub percentage: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderNumberResponse {
    pub code: String,
    pub date: String,
    pub value: i64,
    pub price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderRelationResponseNodesItem {
    pub id: String,
    pub r#type: String,
    pub kind: String,
    pub label: String,
    pub root: bool,
    pub depth: i64,
    pub percentage: f64,
    pub source_type: String,
    pub code: String,
    pub company_name: String,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderRelationResponseEdgesItem {
    pub id: String,
    pub source: String,
    pub target: String,
    pub code: String,
    pub name: String,
    pub normalized_name: String,
    pub percentage: f64,
    pub weight: f64,
    pub source_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderRelationResponse {
    pub nodes: Vec<ShareholderRelationResponseNodesItem>,
    pub edges: Vec<ShareholderRelationResponseEdgesItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderResponse {
    pub name: String,
    pub percentage: f64,
    pub badge: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderKseiResponse {
    pub code: String,
    pub date: String,
    pub price: String,
    pub local_is: String,
    pub local_cp: String,
    pub local_pf: String,
    pub local_ib: String,
    pub local_id: String,
    pub local_mf: String,
    pub local_sc: String,
    pub local_fd: String,
    pub local_ot: String,
    pub foreign_is: String,
    pub foreign_cp: String,
    pub foreign_pf: String,
    pub foreign_ib: String,
    pub foreign_id: String,
    pub foreign_mf: String,
    pub foreign_sc: String,
    pub foreign_fd: String,
    pub foreign_ot: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderClassificationResponse {
    pub code: String,
    pub date: String,
    pub bk: String,
    pub gv: String,
    pub pe: String,
    pub tb: String,
    pub vc: String,
    pub pb: String,
    pub ef: String,
    pub im: String,
    pub ia: String,
    pub br: String,
    pub hf: String,
    pub sw: String,
    pub cm: String,
    pub cv: String,
    pub fm: String,
    pub sa: String,
    pub pp: String,
    pub ps: String,
    pub sp: String,
    pub cr: String,
    pub r#as: String,
    pub so: String,
    pub cb: String,
    pub oc: String,
    pub dc: String,
    pub cn: String,
    pub cg: String,
    pub cp: String,
    pub io: String,
    pub pl: String,
    pub pt: String,
    pub ed: String,
    pub mf: String,
    pub sc: String,
    pub pf: String,
    pub ib: String,
    pub is: String,
    pub fd: String,
    pub r#in: String,
    pub total: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderClassifyTableResponse {
    pub code: String,
    pub bk: String,
    pub gv: String,
    pub pe: String,
    pub tb: String,
    pub vc: String,
    pub pb: String,
    pub ef: String,
    pub im: String,
    pub ia: String,
    pub br: String,
    pub hf: String,
    pub sw: String,
    pub cm: String,
    pub cv: String,
    pub fm: String,
    pub sa: String,
    pub pp: String,
    pub ps: String,
    pub sp: String,
    pub cr: String,
    pub r#as: String,
    pub so: String,
    pub cb: String,
    pub oc: String,
    pub dc: String,
    pub cn: String,
    pub cg: String,
    pub cp: String,
    pub io: String,
    pub pl: String,
    pub pt: String,
    pub ed: String,
    pub mf: String,
    pub sc: String,
    pub pf: String,
    pub ib: String,
    pub is: String,
    pub fd: String,
    pub r#in: String,
    pub total: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SummaryStockResponse {
    pub code: String,
    pub buy_freq: String,
    pub buy_volume: String,
    pub buy_value: String,
    pub sell_freq: String,
    pub sell_volume: String,
    pub sell_value: String,
    pub buy_avg: f64,
    pub sell_avg: f64,
    pub net_value: String,
    pub net_volume: String,
    pub net_freq: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SummaryBrokerResponse {
    pub code: String,
    pub buy_freq: String,
    pub buy_volume: String,
    pub buy_value: String,
    pub sell_freq: String,
    pub sell_volume: String,
    pub sell_value: String,
    pub buy_avg: f64,
    pub sell_avg: f64,
    pub net_value: String,
    pub net_volume: String,
    pub net_freq: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectorStalkerResponseIndexItemDataItem {
    pub date: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectorStalkerResponseIndexItem {
    pub index: String,
    pub data: Vec<SectorStalkerResponseIndexItemDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectorStalkerResponse {
    pub index: Vec<SectorStalkerResponseIndexItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectorRotationResponseDataItemTrailItem {
    pub date: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectorRotationResponseDataItem {
    pub code: String,
    pub name: String,
    pub trail: Vec<SectorRotationResponseDataItemTrailItem>,
    pub quadrant: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectorRotationResponse {
    pub benchmark: String,
    #[serde(rename = "lastDate")]
    pub last_date: String,
    pub data: Vec<SectorRotationResponseDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStalkerResponseSummaryPeak {
    pub date: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStalkerResponseSummary {
    pub active: i64,
    pub total: i64,
    pub avg: f64,
    pub peak: BrokerStalkerResponseSummaryPeak,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStalkerResponseCalendarItem {
    pub date: String,
    pub value: i64,
    pub volume: i64,
    pub buy_value: i64,
    pub sell_value: i64,
    pub buy_volume: i64,
    pub sell_volume: i64,
    pub buy_dom: f64,
    pub avg_buy: f64,
    pub avg_sell: f64,
    pub avg_net: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStalkerResponse {
    pub broker: String,
    pub stock: String,
    pub summary: BrokerStalkerResponseSummary,
    pub calendar: Vec<BrokerStalkerResponseCalendarItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStalkerListResponseSummary {
    pub stocks: i64,
    pub total_net: i64,
    pub total_volume: i64,
    pub top: String,
    pub concentration: i64,
    pub buy_dom: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStalkerListResponseListItem {
    pub code: String,
    pub value: i64,
    pub volume: i64,
    pub transaction: i64,
    pub buy_dom: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStalkerListResponse {
    pub summary: BrokerStalkerListResponseSummary,
    pub list: Vec<BrokerStalkerListResponseListItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SummaryStockChartResponse {
    pub label: String,
    pub value: i64,
    pub fill: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SummaryBrokerChartResponse {
    pub label: String,
    pub value: i64,
    pub fill: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InventoryStockChartResponsePriceItem {
    pub code: String,
    pub date: String,
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
    pub volume: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InventoryStockChartResponseBrokerItemDataItem {
    pub date: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InventoryStockChartResponseBrokerItem {
    pub broker: String,
    pub data: Vec<InventoryStockChartResponseBrokerItemDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InventoryStockChartResponse {
    pub price: Vec<InventoryStockChartResponsePriceItem>,
    pub broker: Vec<InventoryStockChartResponseBrokerItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InventoryBrokerChartResponseDataItem {
    pub date: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InventoryBrokerChartResponse {
    pub code: String,
    pub data: Vec<InventoryBrokerChartResponseDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MomentumChartResponse {
    pub time: String,
    pub buy_lot: i64,
    pub sell_lot: i64,
    pub buy_percentage: i64,
    pub sell_percentage: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IntradayInventoryChartResponsePriceItem {
    pub x: String,
    pub o: i64,
    pub h: i64,
    pub l: i64,
    pub c: i64,
    pub v: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IntradayInventoryChartResponseBrokerItemDataItem {
    pub x: String,
    pub y: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IntradayInventoryChartResponseBrokerItem {
    pub code: String,
    pub name: String,
    pub data: Vec<IntradayInventoryChartResponseBrokerItemDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IntradayInventoryChartResponse {
    pub price: Vec<IntradayInventoryChartResponsePriceItem>,
    pub broker: Vec<IntradayInventoryChartResponseBrokerItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SankeyChartResponseNodesItem {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SankeyChartResponseLinksItem {
    pub source: String,
    pub target: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SankeyChartResponse {
    pub nodes: Vec<SankeyChartResponseNodesItem>,
    pub links: Vec<SankeyChartResponseLinksItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceTableResponse {
    pub price: i64,
    pub buy_volume: i64,
    pub sell_volume: i64,
    pub buy_freq: i64,
    pub sell_freq: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TimeTableResponse {
    pub time: String,
    pub buy_lot: f64,
    pub sell_lot: i64,
    pub buy_percentage: f64,
    pub sell_percentage: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceDiaryResponse {
    pub date: String,
    pub price: i64,
    pub value: String,
    pub volume: String,
    pub change: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceSeasonalityResponse {
    pub month: String,
    pub start_price: String,
    pub end_price: String,
    pub percentage_change: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderAboveResponseDataItem {
    pub date: String,
    pub code: String,
    pub name: String,
    pub format_securities: String,
    pub prev_val: String,
    pub prev_pct: i64,
    pub next_val: String,
    pub next_pct: f64,
    pub change: String,
    pub nationality: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderAboveResponse {
    #[serde(rename = "totalPage")]
    pub total_page: i64,
    pub page: i64,
    #[serde(rename = "nextPage")]
    pub next_page: i64,
    pub data: Vec<ShareholderAboveResponseDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderAboveChartResponse {
    pub date: String,
    pub share: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderOneResponseDataItem {
    pub date: String,
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub status: String,
    pub nationality: String,
    pub domicile: String,
    pub prev_scripless: String,
    pub next_scripless: String,
    pub prev_scrip: String,
    pub next_scrip: String,
    pub prev_total: String,
    pub next_total: String,
    pub prev_pct: f64,
    pub next_pct: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderOneResponse {
    #[serde(rename = "totalPage")]
    pub total_page: i64,
    pub page: i64,
    #[serde(rename = "nextPage")]
    pub next_page: Option<serde_json::Value>,
    pub data: Vec<ShareholderOneResponseDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShareholderOneChartResponse {
    pub date: String,
    pub scrip: String,
    pub scripless: String,
    pub total: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InsiderResponseDataItemSubrowItem {
    pub date: String,
    pub price: i64,
    pub status: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InsiderResponseDataItem {
    pub date: String,
    pub code: String,
    pub name: String,
    pub prev_percent: f64,
    pub prev_val: String,
    pub next_percent: f64,
    pub next_val: String,
    pub change: f64,
    pub badge: String,
    pub nationality: String,
    pub purpose: String,
    pub child: String,
    pub subrow: Vec<InsiderResponseDataItemSubrowItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InsiderResponse {
    #[serde(rename = "totalPage")]
    pub total_page: i64,
    pub page: i64,
    #[serde(rename = "nextPage")]
    pub next_page: i64,
    pub data: Vec<InsiderResponseDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InsiderChartResponse {
    pub date: String,
    pub share: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancialStatementResponseRowsItemValuesItem {
    pub col: String,
    pub year: i64,
    pub amount: i64,
    pub period: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancialStatementResponseRowsItem {
    pub id: String,
    pub name: String,
    pub level: i64,
    pub values: Vec<FinancialStatementResponseRowsItemValuesItem>,
    pub parent_id: Option<serde_json::Value>,
    pub is_abstract: bool,
    pub display_order: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancialStatementResponseColumnsItem {
    pub year: i64,
    pub label: String,
    pub period: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancialStatementResponse {
    pub rows: Vec<FinancialStatementResponseRowsItem>,
    pub columns: Vec<FinancialStatementResponseColumnsItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FinancialStatementChartResponse {
    pub date: String,
    pub year: i64,
    pub amount: i64,
    pub period: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeystatResponseRowsItemValuesItem {
    pub col: String,
    pub year: i64,
    pub amount: i64,
    pub period: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeystatResponseRowsItem {
    pub id: String,
    pub name: String,
    pub level: i64,
    pub values: Vec<KeystatResponseRowsItemValuesItem>,
    pub parent_id: Option<serde_json::Value>,
    pub is_abstract: bool,
    pub display_order: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeystatResponseColumnsItem {
    pub year: i64,
    pub label: String,
    pub period: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeystatResponse {
    pub rows: Vec<KeystatResponseRowsItem>,
    pub columns: Vec<KeystatResponseColumnsItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeystatChartResponse {
    pub date: String,
    pub year: i64,
    pub amount: i64,
    pub period: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetCalendarResponseDataItemPayload {
    #[serde(rename = "Total")]
    pub total: String,
    #[serde(rename = "Series")]
    pub series: String,
    #[serde(rename = "ExcPrice")]
    pub exc_price: String,
    #[serde(rename = "ExcPeriodEnd")]
    pub exc_period_end: String,
    #[serde(rename = "ExcPeriodStr")]
    pub exc_period_str: String,
    #[serde(rename = "ExcPeriodStart")]
    pub exc_period_start: String,
    #[serde(rename = "TradingPeriodEnd")]
    pub trading_period_end: String,
    #[serde(rename = "TradingPeriodStr")]
    pub trading_period_str: String,
    #[serde(rename = "TradingPeriodStart")]
    pub trading_period_start: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetCalendarResponseDataItem {
    pub code: String,
    pub r#type: String,
    pub payload: GetCalendarResponseDataItemPayload,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GetCalendarResponse {
    #[serde(rename = "totalPage")]
    pub total_page: i64,
    pub page: i64,
    #[serde(rename = "nextPage")]
    pub next_page: i64,
    pub data: Vec<GetCalendarResponseDataItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderQueueResponseItem {
    pub time: String,
    pub order_id: String,
    pub order_volume: i64,
    pub open_volume: i64,
    pub done_volume: i64,
    pub order_value: i64,
    pub open_value: i64,
    pub done_value: i64,
}

