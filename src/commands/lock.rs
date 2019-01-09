use crate::config::is_admin;
use crate::config::Config;
use crate::database::models::FactoidEnum;
use crate::database::Db;
use crate::handler::Command;
use crate::handler::Response;

use failure::Error;

pub fn lock(command: Command, config: &Config, db: &Db) -> Result<Response, Error> {
    if !is_admin(command.source_nick, config) {
        return Ok(Response::Say("Only an admin can lock a factoid".into()));
    }

    if command.arguments.len() < 1 {
        return Ok(Response::Notice(
            "Invalid command format, please use ~lock <factoid>".into(),
        ));
    }

    let actual_factoid = command.arguments.join(" ");
    Ok(match db.get_factoid(&actual_factoid)? {
        Some(factoid) => {
            db.create_factoid(
                command.source_nick,
                factoid.intent,
                &factoid.label,
                &factoid.description,
                true,
            )?;
            Response::Notice(format!("locked factoid '{}'", factoid.label))
        }
        None => Response::Notice(format!(
            "cannot lock factoid '{}' because it doesn't exist",
            actual_factoid
        )),
    })
}

pub fn unlock(command: Command, config: &Config, db: &Db) -> Result<Response, Error> {
    if !is_admin(command.source_nick, config) {
        return Ok(Response::Say("Only an admin canun lock a factoid".into()));
    }

    if command.arguments.len() < 1 {
        return Ok(Response::Notice(
            "Invalid command format, please use ~unlock <factoid>".into(),
        ));
    }

    let actual_factoid = command.arguments.join(" ");
    Ok(match db.get_factoid(&actual_factoid)? {
        Some(factoid) => {
            db.create_factoid(
                command.source_nick,
                factoid.intent,
                &factoid.label,
                &factoid.description,
                false,
            )?;
            Response::Notice(format!("unlocked factoid '{}'", factoid.label))
        }
        None => Response::Notice(format!(
            "cannot unlock factoid '{}' because it doesn't exist",
            actual_factoid
        )),
    })
}
