use crate::command::{Command, CommandResult};
use serenity::prelude::Context;
use serenity::model::prelude::Message;
use crate::{DatabaseConnection, into_db};

use crate::schema::tags;
use crate::models::{TagInsert, TagId};
use diesel::{insert_into, RunQueryDsl, delete};
use diesel::prelude::*;
use std::ops::Deref;
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind;

pub struct Tag;
impl Command for Tag {
    fn execute(&self, ctx: Context, msg: Message) -> CommandResult {
        let guild_id = if let Some(guild_id) = msg.guild_id {
            guild_id
        } else {
            msg.channel_id.say(ctx.http, "Stop trying to slide into my DMs, this is only supported in a server!")?;
            return Ok(());
        };
        let mut args = msg.content.splitn(4, |c: char| c.is_ascii_whitespace()).skip(1);
        match args.next() {
            Some("add") => {
                let tag_name = if let Some(tag_name) = args.next() {
                    tag_name
                } else {
                    msg.channel_id.say(ctx.http, "Please provide a tag name")?;
                    return Ok(());
                };
                if tag_name == "add" || tag_name == "remove" {
                    msg.channel_id.say(ctx.http, format!("Cannot name a tag \"{}\"", tag_name))?;
                    return Ok(());
                }
                let tag_content = if let Some(tag_content) = args.next() {
                    tag_content
                } else {
                    msg.channel_id.say(ctx.http, format!("Please provide content for tag \"{}\"", tag_name))?;
                    return Ok(());
                };
                let data = ctx.data.read();
                let mtx = data.get::<DatabaseConnection>().unwrap();
                let conn = mtx.lock().unwrap();
                let tag_to_insert = TagInsert {
                    guild_id: into_db(guild_id.0),
                    tag_name: tag_name.to_string(),
                    tag_content: tag_content.to_string()
                };

                insert_into(tags::table)
                    .values(&tag_to_insert)
                    .execute(conn.deref())?;

                msg.channel_id.say(ctx.http, format!("Successfully created tag \"{}\"", tag_name))?;
                Ok(())
            }
            Some("remove") => {
                let tag_name = if let Some(tag_name) = args.next() {
                    tag_name
                } else {
                    msg.channel_id.say(ctx.http, "Please provide a tag name")?;
                    return Ok(());
                };
                let data = ctx.data.read();
                let mtx = data.get::<DatabaseConnection>().unwrap();
                let conn = mtx.lock().unwrap();
                let to_delete = tags::table
                    .filter(tags::guild_id.eq(into_db(guild_id.0)).and(tags::tag_name.eq(tag_name)));
                let affected_rows = delete(to_delete)
                    .execute(conn.deref())?;
                if affected_rows > 0 {
                    msg.channel_id.say(ctx.http, format!("Successfully removed tag \"{}\"", tag_name))?;
                } else {
                    msg.channel_id.say(ctx.http, format!("Tag \"{}\" does not exist", tag_name))?;
                }
                Ok(())
            }
            Some(tag_name) => {
                let data = ctx.data.read();
                let mtx = data.get::<DatabaseConnection>().unwrap();
                let conn = mtx.lock().unwrap();
                let tag: Option<TagId> = tags::table
                    .filter(tags::guild_id.eq(into_db(guild_id.0)).and(tags::tag_name.eq(tag_name)))
                    .first::<TagId>(conn.deref()).optional()?;
                if let Some(tag) = tag {
                    msg.channel_id.say(ctx.http, tag.tag_content)?;
                    Ok(())
                } else {
                    msg.channel_id.say(ctx.http, format!("No tag called \"{}\" found", tag_name))?;
                    Ok(())
                }
            }
            _ => {
                let data = ctx.data.read();
                let mtx = data.get::<DatabaseConnection>().unwrap();
                let conn = mtx.lock().unwrap();
                let tags: Vec<String> = tags::table
                    .select(tags::tag_name)
                    .filter(tags::guild_id.eq(into_db(guild_id.0)))
                    .load(conn.deref())?;

                let message = if tags.is_empty() {
                    String::from("No Tags")
                } else {
                    format!("Tags:\n{}", tags.join(", "))
                };

                msg.channel_id.say(ctx.http, message)?;
                Ok(())
            }
        }
    }

    fn name(&self) -> &'static str {
        "tag"
    }
}