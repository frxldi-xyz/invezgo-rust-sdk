//! Endpoints for trades API

use crate::client::InvezgoClient;


pub struct TradesApi {
    pub(crate) client: InvezgoClient,
}

impl TradesApi {

    /// Daftar Transaksi Terealisasi
    pub async fn list_transactions(&self, from: &str, to: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/trades".to_string();
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Hapus Transaksi Terealisasi
    pub async fn delete_watchlists(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/trades".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::DELETE, &path, &query, None::<&()>).await
    }


    /// Ringkasan Transaksi Terealisasi
    pub async fn get_transactions_summary(&self, from: &str, to: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/trades/summary".to_string();
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Ringkasan Transaksi Terealisasi Grafik
    pub async fn get_summary_chart(&self, from: &str, to: &str, code: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/trades/summary-chart".to_string();
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
            ("code", Some(code.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Update Catatan Transaksi Terealisasi
    pub async fn update_note_watchlist(&self, id: &str, body: &super::super::models::shared::NoteTradeDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/trades/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PATCH, &path, &query, Some(body)).await
    }

}
