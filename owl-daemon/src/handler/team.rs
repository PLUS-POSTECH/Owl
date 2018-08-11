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
            name: param_name,
            description: param_description,
        } => {
            diesel::insert_into(teams)
                .values((name.eq(&param_name), description.eq(&param_description)))
                .execute(con)?;

            info!("[Team] Insert record: {}", &param_name);
            Ok(())
        },
        TeamEditParams::Delete { name: param_name } => {
            let rows = diesel::delete(teams.filter(name.eq(&param_name))).execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Team {} not found", &param_name)))
            } else {
                info!("[Team] Delete record: {}", &param_name);
                Ok(())
            }
        },
        TeamEditParams::Update {
            name: param_name,
            description: param_description,
        } => {
            let rows = diesel::update(teams.filter(name.eq(&param_name)))
                .set(TeamChangeset {
                    name: None,
                    description: param_description,
                })
                .execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Team {} not found", &param_name)))
            } else {
                info!("[Team] Update record: {}", &param_name);
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
