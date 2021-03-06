use iron::{Handler, Request, Response, IronResult, status};
use router::Router;

use diesel::sqlite::SqliteConnection;
use diesel::insert;

use r2d2_diesel::ConnectionManager;
use r2d2;

use models::test::Test;
use schema::tests;
use diesel::prelude::*;
use std::ops::DerefMut;

pub struct Api {
    router: Router,
    database_connection_pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

impl Api {
    pub fn new(database_url: String) -> Api {
        // Set up a pool of sqlite connections.
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");

        // TODO: Make this routes
        use schema::tests::dsl::*;
        let mut the_test_connection = pool.get().unwrap();
        let the_test = Test { id: 1 };
        insert(&the_test)
            .into(tests)
            .execute(the_test_connection.deref_mut());

        Api {
            router: Router::new(),
            database_connection_pool: pool,
        }
    }

    fn get_database_connection(&self) -> Result<r2d2::PooledConnection<ConnectionManager<SqliteConnection>>, r2d2::GetTimeout> {
        self.database_connection_pool.get()
    }
}

impl Handler for Api {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.get_database_connection().unwrap();
        Ok(Response::with((status::Ok, "Hello from API")))
    }
}
