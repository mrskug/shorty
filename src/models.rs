use diesel::Queryable;

#[derive(Queryable)]
pub struct ShUrl {
    pub id: i32,
    pub hash: String,
    pub url: String,
}
