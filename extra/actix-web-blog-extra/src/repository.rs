use crate::error::ApiError;
use crate::schema::*;
use crate::types::post::{Body, Id, Title};
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize}; // (d1)

type DbPool =
    r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost {
    title: Title,
    body: Body,
}

// (d3)
#[derive(Serialize, Queryable, Debug)]
pub struct Post {
    id: Id,
    title: Title,
    body: Body,
    published: bool,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostChangeset {
    title: Option<Title>,
    body: Option<Body>,
}

#[derive(Deserialize)]
pub struct PostPublishRequest {
    pub published: bool,
}

pub struct Repository {
    pool: DbPool,
}

impl Repository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<
            SqliteConnection,
        >::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create a pool.");
        Self { pool }
    }

    pub async fn create_post(
        &self,
        new_post: NewPost,
    ) -> Result<Post, ApiError> {
        let mut conn = self.pool.get()?;
        let post = web::block(move || {
            diesel::insert_into(posts::table)
                .values(new_post)
                .get_result(&mut conn)
        })
        .await??;

        Ok(post)
    }

    pub async fn list_posts(
        &self,
    ) -> Result<Vec<Post>, ApiError> {
        let mut conn = self.pool.get()?;
        let res = web::block(move || {
            posts::table.load(&mut conn)
        })
        .await??;

        Ok(res)
    }

    pub async fn get_post(
        &self,
        id: i32,
    ) -> Result<Post, ApiError> {
        let mut conn = self.pool.get()?;
        let res = web::block(move || {
            posts::table
                .find(id)
                .first(&mut conn)
                .optional()
        })
        .await??
        .ok_or(ApiError::NotFound)?;

        Ok(res)
    }

    pub async fn update_post(
        &self,
        id: i32,
        changeset: PostChangeset,
    ) -> Result<Post, ApiError> {
        let mut conn = self.pool.get()?;
        let post = web::block(move || {
            diesel::update(posts::table.find(id))
                .set(changeset)
                .get_result(&mut conn)
        })
        .await??;

        Ok(post)
    }

    pub async fn publish_post(
        &self,
        id: i32,
        published: bool,
    ) -> Result<(), ApiError> {
        let mut conn = self.pool.get()?;
        web::block(move || {
            diesel::update(posts::table.find(id))
                .set(posts::published.eq(published))
                .execute(&mut conn)
        })
        .await??;

        Ok(())
    }

    pub async fn delete_post(
        &self,
        id: i32,
    ) -> Result<(), ApiError> {
        let mut conn = self.pool.get()?;
        web::block(move || {
            diesel::delete(posts::table.find(id))
                .execute(&mut conn)
        })
        .await??;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conn() {
        let database_url =
            std::env::var("DATABASE_URL").unwrap();
        let repo = Repository::new(&database_url);
        assert!(repo.pool.get().is_ok());
    }
}
