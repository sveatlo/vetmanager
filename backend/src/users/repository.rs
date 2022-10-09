use super::models::*;
use crate::repository::BaseRepository;
use crate::schema::users;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, QueryResult,
};

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct UserRepository {
    base: BaseRepository<Conn, users::table>,
}

#[allow(dead_code)]
impl UserRepository {
    pub fn new(conn: Conn) -> Self {
        UserRepository {
            base: BaseRepository::new(users::table, conn),
        }
    }

    pub fn create(&self, user: NewUser) -> QueryResult<User> {
        self.base.create(user)
    }

    pub fn find_by_id(&self, id: i64) -> QueryResult<User> {
        self.base.find_by_id(id)
    }

    pub fn find_by_username(&self, username: String) -> QueryResult<User> {
        self.base.find_one(users::username.eq(username))
    }

    pub fn delete_by_id(&self, id: i64) -> QueryResult<User> {
        diesel::delete(users::table.filter(users::id.eq(id))).get_result(&self.base.conn)
    }
}
