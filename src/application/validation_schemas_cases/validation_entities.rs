use chrono::{DateTime, Utc};

pub struct ValidationSchema {
    pub name: String,
    pub schema: String,
}
impl ValidationSchema {
    pub fn new(name: impl Into<String>, schema: impl Into<String>) -> Self {
        ValidationSchema {
            name: name.into(),
            schema: schema.into(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn schema(&self) -> &str {
        &self.schema
    }
}

pub struct ValidationSchemaModel {
    pub id: u32,
    pub name: String,
    pub schema: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ValidationSchemaModel {
    pub fn new(
        id: u32,
        name: impl Into<String>,
        schema: impl Into<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ValidationSchemaModel {
            id, // El ID se asignará al guardar en la base de datos
            name: name.into(),
            schema: schema.into(),
            created_at,
            updated_at,
        }
    }
}

pub struct ValidationSchemeModeById {
    pub name: String,
    pub schema: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ValidationSchemeModeById {
    pub fn new(
        name: impl Into<String>,
        schema: impl Into<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ValidationSchemeModeById {
            name: name.into(),
            schema: schema.into(),
            created_at,
            updated_at,
        }
    }
}
