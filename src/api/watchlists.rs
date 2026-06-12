//! Endpoints for watchlists API

use crate::client::InvezgoClient;


pub struct WatchlistsApi {
    pub(crate) client: InvezgoClient,
}

impl WatchlistsApi {

    /// Daftar Watchlist
    pub async fn list_watchlist(&self, group: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/watchlists".to_string();
        let query = [
            ("group", Some(group.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Tambah Saham Baru Ke Watchlist
    pub async fn add_watchlist(&self, body: &super::super::models::shared::CreateWatchlistDto) -> Result<(), crate::error::InvezgoError> {
        let path = "/watchlists".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Hapus Watchlist
    pub async fn delete_watchlists(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/watchlists".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::DELETE, &path, &query, None::<&()>).await
    }


    /// Daftar Grup Watchlist
    pub async fn list_group_watchlist(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/watchlists/group".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Tambahkan Grup Baru Ke Watchlist
    pub async fn add_group_watchlist(&self, body: &super::super::models::shared::CreateGroupDto) -> Result<(), crate::error::InvezgoError> {
        let path = "/watchlists/group".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Update Grup Watchlist
    pub async fn update_group_watchlist(&self, id: &str, body: &super::super::models::shared::CreateGroupDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/watchlists/group/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PUT, &path, &query, Some(body)).await
    }


    /// Delete Grup Watchlist
    pub async fn delete_group_watchlist(&self, id: &str) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/watchlists/group/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::DELETE, &path, &query, None::<&()>).await
    }


    /// Update Watchlist
    pub async fn update_watchlist(&self, id: &str, body: &super::super::models::shared::UpdateWatchlistDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/watchlists/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PUT, &path, &query, Some(body)).await
    }


    /// Update Catatan Watchlist
    pub async fn update_note_watchlist(&self, id: &str, body: &super::super::models::shared::UpdateNoteWatchlistDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/watchlists/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PATCH, &path, &query, Some(body)).await
    }

}
