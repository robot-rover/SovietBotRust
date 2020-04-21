use serenity::prelude::Context;
use serenity::model::prelude::Message;
use serenity;
use std::{sync, io, fmt};
use std::sync::MutexGuard;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Debug};
use diesel::result::Error;

mod echo;
mod ping;
mod tag;

pub const COMMAND_LIST: &[&dyn Command] = &[&echo::Echo, &ping::Ping, &tag::Tag];

pub type CommandResult = Result<(), CommandError>;

pub struct CommandError {
    err: Box<dyn StdError>
}

impl From<serenity::Error> for CommandError {
    fn from(err: serenity::Error) -> Self {
        CommandError {
            err: Box::new(err)
        }
    }
}

impl From<diesel::result::Error> for CommandError {
    fn from(err: diesel::result::Error) -> Self {
        CommandError {
            err: Box::new(err)
        }
    }
}

impl Debug for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.err, f)
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.err, f)
    }
}

impl StdError for CommandError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.err.source()
    }
}

pub trait Command {
    fn execute(&self, ctx: Context, msg: Message) -> CommandResult;
    fn name(&self) -> &'static str;
}