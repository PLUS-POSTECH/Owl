use db::models::{Team, TeamChangeset};
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::team::*;
use DaemonResource;

pub fn edit_team(resource: &DaemonResource, params: TeamEditParams) -> Result<(), Error> {
    use db::schema::teams::dsl::*;

    let con: &PgConnection = &*resource.db_pool.get()?;

    match params {
        TeamEditParams::Add {
            name: ref param_name,
            description: ref param_description,
        } => {
            diesel::insert_into(teams)
                .values((name.eq(param_name), description.eq(param_description)))
                .execute(con)?;

            Ok(())
        },
        TeamEditParams::Delete {
            name: ref param_name,
        } => {
            let rows = diesel::delete(teams.filter(name.eq(param_name))).execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Team {} not found", param_name)))
            } else {
                Ok(())
            }
        },
        TeamEditParams::Update {
            name: ref param_name,
            description: ref param_description,
        } => {
            let rows = diesel::update(teams.filter(name.eq(param_name)))
                .set(TeamChangeset {
                    name: None,
                    description: param_description.clone(),
                })
                .execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Team {} not found", param_name)))
            } else {
                Ok(())
            }
        },
    }
}

pub fn list_team(resource: &DaemonResource) -> Result<Vec<TeamData>, Error> {
    use db::schema::teams::dsl::*;

    let con: &PgConnection = &*resource.db_pool.get()?;

    let fetch = teams.load::<Team>(con)?;
    Ok(fetch
        .into_iter()
        .map(|team| TeamData {
            name: team.name,
            description: team.description,
        })
        .collect())
}
