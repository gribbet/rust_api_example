use crate::database::Database;
use crate::error::ApiError;
use crate::model::user::User;
use crate::service::user;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/")]
pub async fn list(
    database: State<'_, Database>,
) -> Result<Json<Vec<User>>, ApiError> {
    Ok(Json(user::list(&*&database.get()?)?))
}

#[get("/<id>")]
pub async fn get(
    database: State<'_, Database>,
    id: i32,
) -> Result<Option<Json<User>>, ApiError> {
    Ok(user::get(&*&database.get()?, id)?.map(|user| Json(user)))
}

#[post("/", format = "json", data = "<user>")]
pub async fn create(
    database: State<'_, Database>,
    user: Json<User>,
) -> Result<Json<User>, ApiError> {
    Ok(Json(user::create(&*&database.get()?, user.into_inner())?))
}
