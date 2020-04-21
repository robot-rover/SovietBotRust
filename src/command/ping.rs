use crate::command::{Command, CommandResult};
use serenity::prelude::Context;
use serenity::model::prelude::Message;

pub struct Ping;
impl Command for Ping {
    fn execute(&self, ctx: Context, msg: Message) -> CommandResult {
        msg.reply(ctx, "Pong!")?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "ping"
    }
}