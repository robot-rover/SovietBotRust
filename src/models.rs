use crate::schema::{guilds, tags};

#[derive(Queryable, Insertable)]
#[table_name = "guilds"]
pub struct Guild {
    pub id: i64,
    pub trigger: Option<String>,
}

#[derive(Queryable, Insertable)]
#[table_name = "tags"]
pub struct TagInsert {
    pub guild_id: i64,
    pub tag_name: String,
    pub tag_content: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "tags"]
pub struct TagId {
    pub id: i32,
    pub guild_id: i64,
    pub tag_name: String,
    pub tag_content: String,
}