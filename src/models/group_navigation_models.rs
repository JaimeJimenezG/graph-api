use diesel::Queryable;

#[derive(Queryable, Debug)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Navigation))]
pub struct GroupNavigation {
    pub id: i32,
    pub navigation_id: i32,
    pub group_id: i32,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}
