use crate::model::user::User;
use crate::schema::users;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
use users::dsl::*;

pub fn create(connection: &PgConnection, user: User) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(&user)
        .execute(connection)?;
    Ok(user)
}

pub fn list(connection: &PgConnection) -> Result<Vec<User>, Error> {
    users.load::<User>(connection)
}

pub fn get(
    connection: &PgConnection,
    user_id: i32,
) -> Result<Option<User>, Error> {
    let mut items = users.filter(id.eq(user_id)).load::<User>(connection)?;
    Ok(items.pop())
}
