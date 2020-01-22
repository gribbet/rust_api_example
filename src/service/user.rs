use crate::{model::user::User, schema::users};
use diesel::{prelude::*, result::Error, PgConnection};
use users::dsl::*;

pub fn create(connection: &PgConnection, user: User) -> Result<User, Error> {
    Ok(diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(connection)?)
}

pub fn list(connection: &PgConnection) -> Result<Vec<User>, Error> {
    users.load::<User>(connection)
}

pub fn get(
    connection: &PgConnection,
    user_id: i32,
) -> Result<Option<User>, Error> {
    Ok(users
        .filter(id.eq(user_id))
        .get_result::<User>(connection)
        .optional()?)
}
