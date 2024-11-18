use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub beneficiary_account: String,
    pub beneficiary_bank_code: String,
    pub amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ETransferStatus {
    Pending{code: String},
    Success,
    Failed,
}

impl ETransferStatus {
    pub fn to_string(&self) -> String {
        match self {
            ETransferStatus::Pending { code: _ } => "Pending".to_string(),
            ETransferStatus::Success => "Success".to_string(),
            ETransferStatus::Failed => "Failed".to_string(),
        }
    }
    pub fn get_code(&self) -> String {
        match self {
            ETransferStatus::Pending { code } => code.to_string(),
            ETransferStatus::Success => "".to_string(),
            ETransferStatus::Failed => "".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResponse{
    pub message: String,
    pub status: ETransferStatus,
}

impl TransferResponse {
    pub fn new(message: &str, status: ETransferStatus) -> Self {
        Self {
            message: message.to_string(),
            status,
        }
    }
}

impl Default for TransferResponse {
    fn default() -> Self {
        Self {
            message: "success".to_string(),
            status: ETransferStatus::Success,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ETransferMethod {
    Default,
    Intrabank,
    Interbank,
    BIFast
}

impl From<String> for ETransferMethod {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "INTRABANK" => ETransferMethod::Intrabank,
            "INTERBANK" => ETransferMethod::Interbank,
            "BIFAST" => ETransferMethod::BIFast,
            _ => ETransferMethod::Default,
        }
    }
}

impl Into<String> for ETransferMethod {
    fn into(self) -> String{
        match self {
            ETransferMethod::Intrabank => "INTRABANK".to_string(),
            ETransferMethod::Interbank => "INTERBANK".to_string(),
            ETransferMethod::BIFast => "BIFAST".to_string(),
            ETransferMethod::Default => "DEFAULT".to_string(),
        }
    }
}

impl ToString for ETransferMethod {
    fn to_string(&self) -> String {
        match self {
            ETransferMethod::Intrabank => "INTRABANK".to_string(),
            ETransferMethod::Interbank => "INTERBANK".to_string(),
            ETransferMethod::BIFast => "BIFAST".to_string(),
            ETransferMethod::Default => "DEFAULT".to_string(),
        }
    }
}