use sqlx::FromRow; 

#[derive(FromRow)]
pub struct ValidationSchemaRow {
    pub json_name: String,
    pub json_schema: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(FromRow)]
pub struct ValidationSchemaRowWhitId {
    pub id: i32,
    pub json_name: String,
    pub json_schema: String,
    pub created_at: String,
    pub updated_at: String,
}