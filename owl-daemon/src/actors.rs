use std::vec::Vec;

use actix::{Actor, Message, SyncContext};
use actix::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use ::models::*;
use ::schema::*;

pub struct CreateTeam {
    pub name: String,
    pub description: String,
}

impl Message for CreateTeam {
    type Result = Result<usize, Error>;
}

pub struct GetTeam;

impl Message for GetTeam {
    type Result = Result<Vec<Team>, Error>;
}

pub struct DeleteTeam {
    pub name: String,
}

impl Message for DeleteTeam {
    type Result = Result<usize, Error>;
}

pub struct DbExecutor {
    pub db_connection: PgConnection,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateTeam> for DbExecutor {
    type Result = Result<usize, Error>;

    fn handle(&mut self, msg: CreateTeam, _ctx: &mut Self::Context) -> Self::Result {
        diesel::insert_into(teams::table)
            .values((teams::name.eq(msg.name), teams::description.eq(msg.description)))
            .execute(&self.db_connection)
    }
}

impl Handler<GetTeam> for DbExecutor {
    type Result = Result<Vec<Team>, Error>;

    fn handle(&mut self, msg: GetTeam, _ctx: &mut Self::Context) -> Self::Result {
        teams::table
            .load::<Team>(&self.db_connection)
    }
}

impl Handler<DeleteTeam> for DbExecutor {
    type Result = Result<usize, Error>;

    fn handle(&mut self, msg: DeleteTeam, _ctx: &mut Self::Context) -> Self::Result {
        diesel::delete(teams::table)
            .filter(teams::name.eq(msg.name))
            .execute(&self.db_connection)
    }
}
