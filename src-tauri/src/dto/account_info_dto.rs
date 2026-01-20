use serde::Serialize;

#[derive(Serialize)]
pub struct AccountInfoDto {
    pub name: String,
    pub balance: f64,
    pub account_type: Option<String>,
    pub currency: Option<String>,
    pub initial_balance : Option<f64>,
    pub credit_limit:Option<f64>,    
}