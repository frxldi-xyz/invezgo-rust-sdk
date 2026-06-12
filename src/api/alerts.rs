//! Endpoints for alerts API

use crate::client::InvezgoClient;


pub struct AlertsApi {
    pub(crate) client: InvezgoClient,
}

impl AlertsApi {

    /// Membuat alert untuk mendapatkan notifikasi ketika saham mencapai formula tertentu.
    ///     
    ///     Contoh Penggunaan:
    ///     - Live Alert
    pub async fn create(&self, body: &super::super::models::shared::AlertDto) -> Result<(), crate::error::InvezgoError> {
        let path = "/alerts".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Daftar alert yang sudah dibuat.
    ///     
    ///     Contoh Penggunaan:
    ///     - Live Alert
    pub async fn list(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/alerts".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Test alert sebelum membuat alert.
    ///     
    ///     Contoh Penggunaan:
    ///     - Live Alert
    pub async fn test(&self, body: &super::super::models::shared::AlertTestDto) -> Result<(), crate::error::InvezgoError> {
        let path = "/alerts/test".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Perbarui alert yang sudah dibuat.
    ///     
    ///     Contoh Penggunaan:
    ///     - Live Alert
    pub async fn update(&self, id: &str, body: &super::super::models::shared::UpdateAlertDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/alerts/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PUT, &path, &query, Some(body)).await
    }


    /// Hapus alert yang sudah dibuat.
    ///     
    ///     Contoh Penggunaan:
    ///     - Live Alert
    pub async fn delete(&self, id: &str) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/alerts/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::DELETE, &path, &query, None::<&()>).await
    }

}
