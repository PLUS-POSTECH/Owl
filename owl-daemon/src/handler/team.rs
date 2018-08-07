use db::models::Team;
use db::DbPool;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::team::*;

pub fn edit_team(db_pool: DbPool, params: TeamEditParams) -> Result<(), Error> {
    use db::schema::teams::dsl::*;

    let con: &PgConnection = &*db_pool.get()?;

    match params {
        TeamEditParams::Add {
            name: param_name,
            description: param_description,
        } => {
            diesel::insert_into(teams)
                .values((name.eq(param_name), description.eq(param_description)))
                .execute(con)?;
        },
        TeamEditParams::Delete { name: param_name } => {
            diesel::delete(teams.filter(name.eq(param_name))).execute(con)?;
        },
        TeamEditParams::Update {
            name: param_name,
            description: param_description,
        } => {
            if let Some(param_description) = param_description {
                diesel::update(teams.filter(name.eq(param_name)))
                    .set(description.eq(param_description))
                    .execute(con)?;
            }
        },
    };

    Ok(())
}

pub fn list_team(db_pool: DbPool) -> Result<Vec<TeamData>, Error> {
    use db::schema::teams::dsl::*;

    let con: &PgConnection = &*db_pool.get()?;

    let fetch = teams.load::<Team>(con)?;
    Ok(fetch
        .into_iter()
        .map(|team| TeamData {
            name: team.name,
            description: team.description,
        })
        .collect())
}
