//! Endpoints for membership API

use crate::client::InvezgoClient;


pub struct MembershipApi {
    pub(crate) client: InvezgoClient,
}

impl MembershipApi {

    /// Daftar Membership
    pub async fn get_memberships(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/membership".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Tambah Membership Baru
    pub async fn add_membership(&self, body: &super::super::models::shared::AddMembershipDto) -> Result<(), crate::error::InvezgoError> {
        let path = "/membership".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::POST, &path, &query, Some(body)).await
    }


    /// Daftar Scope Membership
    pub async fn get_scope(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/membership/scope".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Transaksi Membership
    pub async fn get_transaction_membership(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/membership/list".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Update Membership
    pub async fn change_membership(&self, id: &str, body: &super::super::models::shared::AddMembershipDto) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/membership/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::PUT, &path, &query, Some(body)).await
    }


    /// Hapus Membership
    pub async fn delete_membership(&self, id: &str) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/membership/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::DELETE, &path, &query, None::<&()>).await
    }

}
