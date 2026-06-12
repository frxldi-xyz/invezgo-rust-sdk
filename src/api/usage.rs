//! Endpoints for usage API

use crate::client::InvezgoClient;


pub struct UsageApi {
    pub(crate) client: InvezgoClient,
}

impl UsageApi {

    /// Informasi jumlah penggunaan API.
    pub async fn usage_api(&self) -> Result<serde_json::Value, crate::error::InvezgoError> {
        let path = "/usage/api".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }

}
