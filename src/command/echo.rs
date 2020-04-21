use crate::command::{Command, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub struct Echo;
impl Command for Echo {
    fn execute(&self, ctx: Context, msg: Message) -> CommandResult {
        if let Some(message) = msg.content.splitn(2, |c: char| c.is_ascii_whitespace()).skip(1).next() {
            msg.channel_id.say(&ctx.http, message)?;
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "echo"
    }
}