//! Endpoints for journals API

use crate::client::InvezgoClient;


pub struct JournalsApi {
    pub(crate) client: InvezgoClient,
}

impl JournalsApi {

    /// Ekstrak Jurnal Dari File
    pub async fn extract_information(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/journals/file".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, None::<&()>).await
    }


    /// Tambah Transaksi Jurnal Baru
    pub async fn add_transaction(&self, body: &super::super::models::shared::AddJournalTransactionDto) -> Result<(), crate::error::InvezgoError> {
        let path = "/journals".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Daftar Transaksi Jurnal
    pub async fn list_transactions(&self, from: &str, to: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/journals".to_string();
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Hapus Jurnal
    pub async fn delete_watchlists(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/journals".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::DELETE, &path, &query, None::<&()>).await
    }


    /// Ringkasan Transaksi Jurnal
    pub async fn get_transactions_summary(&self, from: &str, to: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/journals/summary".to_string();
        let query = [
            ("from", Some(from.to_string())),
            ("to", Some(to.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Update Catatan Transaksi Jurnal
    pub async fn update_note_watchlist(&self, id: &str, body: &super::super::models::shared::NoteJournalTransactionDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/journals/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PATCH, &path, &query, Some(body)).await
    }

}
