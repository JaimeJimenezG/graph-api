use diesel::Queryable;

#[derive(Queryable, Debug)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(User))]
pub struct GroupUser {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}
