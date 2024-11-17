use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Processor {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub base_url: String,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Processor {
    pub fn new(name: &str, base_url: &str, desc: &str) -> Self {
        let now = Utc::now();
        Self {
            uuid: uuid::Uuid::now_v7(),
            name: name.to_string(),
            base_url: base_url.to_string(),
            description: desc.to_string(),
            status: "active".to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorQuery {
    name: Option<String>,
    status: Option<String>,
}

impl Default for ProcessorQuery {
    fn default() -> Self {
        ProcessorQuery{
            name: None,
            status: None,
        }
    }
}

impl ProcessorQuery {
    pub fn new(name: Option<String>, status: Option<String>) -> ProcessorQuery {
        ProcessorQuery { name, status }
    }
    pub fn get_query(&self) -> (String, Vec<String>) {
        let mut query_arr: Vec<String> = Vec::new();
        let mut value_arr: Vec<String> = vec![];

        if let Some(name) = &self.name {
            query_arr.push("name = ?".to_string());
            value_arr.push(name.to_owned());
        }

        if let Some(status) = &self.status {
            query_arr.push("status = ?".to_string());
            value_arr.push(status.to_owned());
        }
        let where_clause = if !query_arr.is_empty() {
            format!("where {}", query_arr.join(" and "))
        } else {
            "".to_string()
        };

        (where_clause, value_arr)
    }
}

mod test_processor_query {
    use super::*;

    #[test]
    fn get_empty_query() {
        let query = ProcessorQuery::default();
        let (where_clause, value_arr) = query.get_query();
        assert_eq!(where_clause, "");
        assert_eq!(value_arr.len(), 0);
    }

    #[test]
    fn get_name_query() {
        let query = ProcessorQuery::new(Some("test".to_string()), None);
        let (where_clause, value_arr) = query.get_query();
        assert_eq!(where_clause, "where name like ?");
        assert_eq!(value_arr.len(), 1);
        assert_eq!(value_arr[0], "test");
    }

    #[test]
    fn get_status_query() {
        let query = ProcessorQuery::new(None, Some("active".to_string()));
        let (where_clause, value_arr) = query.get_query();
        assert_eq!(where_clause, "where status = ?");
        assert_eq!(value_arr.len(), 1);
        assert_eq!(value_arr[0], "active");
    }

    #[test]
    fn get_name_and_status_query() {
        let query = ProcessorQuery::new(Some("test".to_string()), Some("active".to_string()));
        let (where_clause, value_arr) = query.get_query();
        assert_eq!(where_clause, "where name like ? and status = ?");
        assert_eq!(value_arr.len(), 2);
        assert_eq!(value_arr[0], "test");
        assert_eq!(value_arr[1], "active");
    }
}