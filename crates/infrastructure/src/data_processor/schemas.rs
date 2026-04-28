use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::ast::Expr;

// El schema completo: un mapa de nombre_de_campo → definición
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineSchema {
    pub version: u32,
    pub fields: HashMap<String, FieldSchema>,
}

// La definición de un campo
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSchema {
    // "type" es palabra reservada en Rust, así que
    // le damos un nombre interno distinto y le decimos
    // a serde que en el JSON se llama "type"
    #[serde(rename = "type")]
    pub field_type: FieldType,

    pub required: bool,

    // Option<T> en serde = el campo puede ser null
    // o directamente ausente en el JSON
    pub default: Option<f64>,

    pub validation: Option<ValidationRule>,

    // None aquí significa "no transformar, pasar directo"
    pub operation: Option<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Float,
    Int,
    Bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationRule {
    pub min: Option<f64>,
    pub max: Option<f64>,
}