//! Shared DTO schemas from components

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScreenDto {
    pub formula: String,
    pub category: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScreenSaveDto {
    pub name: String,
    pub description: Option<String>,
    pub scope: Option<Vec<String>>,
    pub formula: String,
    pub category: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScreenUpdateDto {
    pub name: String,
    pub description: Option<String>,
    pub scope: Option<Vec<String>>,
    pub formula: String,
    pub category: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CreateWatchlistDto {
    pub group: String,
    pub code: String,
    pub price: f64,
    pub note: Option<String>,
    pub scope: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CreateGroupDto {
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateWatchlistDto {
    pub group: String,
    pub price: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateNoteWatchlistDto {
    pub note: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AddJournalTransactionDto {
    pub code: String,
    pub broker: String,
    pub date: String,
    pub lot: f64,
    pub fee: f64,
    pub price: f64,
    pub status: Option<String>,
    pub scope: Vec<String>,
    pub note: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NoteJournalTransactionDto {
    pub note: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertDtoEvery {
    #[serde(rename = "FOURTY_FIVE_SECONDS")]
    #[default]
    FourtyFiveSeconds,
    #[serde(rename = "ONE_MINUTE")]
    OneMinute,
    #[serde(rename = "FIVE_MINUTES")]
    FiveMinutes,
    #[serde(rename = "TEN_MINUTES")]
    TenMinutes,
    #[serde(rename = "THIRTY_MINUTES")]
    ThirtyMinutes,
    #[serde(rename = "ONE_HOUR")]
    OneHour,
    #[serde(rename = "END_SESSION")]
    EndSession,
    #[serde(rename = "END_OF_DAY")]
    EndOfDay,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertDtoSend {
    #[serde(rename = "IN_OUT")]
    #[default]
    InOut,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertDto {
    pub name: String,
    pub description: Option<String>,
    pub category: Vec<String>,
    pub every: AlertDtoEvery,
    pub send: AlertDtoSend,
    pub scope: Option<Vec<String>>,
    pub formula: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertTestDto {
    pub formula: String,
    pub category: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UpdateAlertDtoEvery {
    #[serde(rename = "FOURTY_FIVE_SECONDS")]
    #[default]
    FourtyFiveSeconds,
    #[serde(rename = "ONE_MINUTE")]
    OneMinute,
    #[serde(rename = "FIVE_MINUTES")]
    FiveMinutes,
    #[serde(rename = "TEN_MINUTES")]
    TenMinutes,
    #[serde(rename = "THIRTY_MINUTES")]
    ThirtyMinutes,
    #[serde(rename = "ONE_HOUR")]
    OneHour,
    #[serde(rename = "END_SESSION")]
    EndSession,
    #[serde(rename = "END_OF_DAY")]
    EndOfDay,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UpdateAlertDtoSend {
    #[serde(rename = "IN_OUT")]
    #[default]
    InOut,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateAlertDto {
    pub name: String,
    pub description: Option<String>,
    pub category: Vec<String>,
    pub every: UpdateAlertDtoEvery,
    pub send: UpdateAlertDtoSend,
    pub formula: String,
    pub scope: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AddMembershipDto {
    pub name: String,
    pub price: f64,
    pub benefit: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NoteTradeDto {
    pub note: Option<String>,
}

