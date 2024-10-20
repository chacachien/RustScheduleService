use sqlx::FromRow;
#[derive(Debug, FromRow)]
pub struct Label {
    pub id: i64,
    pub course: i64,
    pub name: String,
    pub added: i64,
    pub timemodified: i64
}

#[derive(Debug, FromRow)]
pub struct LabelDocument{
    pub id: i64,
    pub course: i64,
    pub name: String,
    pub intro: String,
    pub introformat: i16,
    pub timemodified: i64
}