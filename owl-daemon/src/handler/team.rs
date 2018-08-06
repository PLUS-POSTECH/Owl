use db::models::*;
use db::schema::*;
use db::DbPool;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::team::*;

pub fn edit_team(db_pool: DbPool, params: TeamEditParams) -> Result<(), Error> {
    let con: &PgConnection = &*db_pool.get()?;

    match params {
        TeamEditParams::Add(params) => {
            diesel::insert_into(teams::table)
                .values((
                    teams::name.eq(params.name),
                    teams::description.eq(params.description),
                ))
                .execute(con)?;
        },
        TeamEditParams::Delete(params) => {
            diesel::delete(teams::table)
                .filter(teams::name.eq(params.name))
                .execute(con)?;
        },
        TeamEditParams::Update(params) => {
            unimplemented!();
        },
    };

    Ok(())
}

pub fn list_team(db_pool: DbPool) -> Result<Vec<TeamData>, Error> {
    let con: &PgConnection = &*db_pool.get()?;

    let fetch = teams::table.load::<Team>(con)?;
    Ok(fetch
        .into_iter()
        .map(|team| TeamData {
            name: team.name,
            description: team.description,
        })
        .collect())
}
