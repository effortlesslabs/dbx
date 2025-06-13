use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

/// Represents database metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    /// Database name
    pub name: String,

    /// Database version
    pub version: String,

    /// List of tables in the database
    pub tables: Vec<TableMetadata>,

    /// Additional database-specific metadata
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Represents table metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    /// Table name
    pub name: String,

    /// List of columns in the table
    pub columns: Vec<ColumnMetadata>,

    /// Primary key columns
    #[serde(default)]
    pub primary_key: Vec<String>,

    /// Foreign key constraints
    #[serde(default)]
    pub foreign_keys: Vec<ForeignKeyMetadata>,

    /// Table indexes
    #[serde(default)]
    pub indexes: Vec<IndexMetadata>,

    /// Additional table-specific metadata
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Represents column metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMetadata {
    /// Column name
    pub name: String,

    /// Column data type
    pub data_type: String,

    /// Whether the column can be null
    pub is_nullable: bool,

    /// Whether the column is part of the primary key
    pub is_primary_key: bool,

    /// Default value for the column
    #[serde(default)]
    pub default_value: Option<serde_json::Value>,

    /// Column constraints
    #[serde(default)]
    pub constraints: Vec<String>,

    /// Additional column-specific metadata
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Represents foreign key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyMetadata {
    /// Name of the foreign key constraint
    pub name: String,

    /// Columns in this table that form the foreign key
    pub columns: Vec<String>,

    /// Referenced table name
    pub referenced_table: String,

    /// Referenced columns in the foreign table
    pub referenced_columns: Vec<String>,

    /// On delete action
    #[serde(default)]
    pub on_delete: Option<String>,

    /// On update action
    #[serde(default)]
    pub on_update: Option<String>,
}

/// Represents index metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadata {
    /// Index name
    pub name: String,

    /// Columns in the index
    pub columns: Vec<String>,

    /// Whether the index is unique
    pub is_unique: bool,

    /// Index type (e.g., "btree", "hash")
    #[serde(default)]
    pub index_type: Option<String>,
}
