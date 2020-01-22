use crate::schema::users;

#[derive(Serialize, Deserialize, Insertable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}
