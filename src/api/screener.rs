//! Endpoints for screener API

use crate::client::InvezgoClient;


pub struct ScreenerApi {
    pub(crate) client: InvezgoClient,
}

impl ScreenerApi {

    /// Screener New
    pub async fn screen(&self, body: &super::super::models::shared::ScreenDto) -> Result<serde_json::Value, crate::error::InvezgoError> {
        let path = "/screener/screen".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Daftar Preset Screener
    pub async fn list(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/screener".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Simpan Preset Screener
    pub async fn save(&self, body: &super::super::models::shared::ScreenSaveDto) -> Result<(), crate::error::InvezgoError> {
        let path = "/screener".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Update Preset Screener
    pub async fn update(&self, id: &str, body: &super::super::models::shared::ScreenUpdateDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/screener/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PUT, &path, &query, Some(body)).await
    }


    /// Delete Preset Screener
    pub async fn delete(&self, id: &str) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/screener/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::DELETE, &path, &query, None::<&()>).await
    }

}
