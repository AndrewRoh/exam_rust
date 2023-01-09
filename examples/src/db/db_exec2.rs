include!("mir5_game.rs");

use crate::tiberius::ToSql;
use tiberius::Config;
use env_logger;
use tonic::async_trait;
//
// struct User {
//     age: i32,
//     is_admin: bool
// }
// pub trait FromRow {
//     fn from_row(row: &tiberius::Row) -> Option<Self> where Self: Sized;
// }
// fn get<'a, R, I>(&'a self, idx: I) -> Option<R>
//     where
//         R: FromSql<'a>
//         I: QueryIdx
//
// fn from_row(row: &tiberius::Row) -> User {
//     User
//     { age: row.get("age").unwrap()
//         , is_admin: row.get("is_admin").unwrap()
//     }
// }
// impl QueryIdx for &str {
//     fn idx(&self, row: &tiberius::Row) -> Option<usize> {
//         row.columns.iter().position(|c| c.name() == *self)
//     }
// }

// #[async_trait]
// pub trait DBConnection {
//     async fn new(connection_string: &str, options: Option<&str>) -> Option<Self> where Self: Sized;
//     async fn query<T>(&mut self, query: &str, params: &[&dyn ToSql]) -> Vec<T> where T: FromRow;
//     async fn execute(&mut self, command: &str) -> bool;
// }

// macro_rules! sql_struct {
// ( $struct_name:ident { $( $db_name:ident : $type:ty ),*} ) => {
//     use std::panic::{ catch_unwind };
//
//     #[allow(dead_code)]
//     #[derive(Debug)]
//     #[allow(non_snake_case)]
//     pub struct $struct_name { $( pub $db_name: $type ),* }
//
//     impl FromRow for $struct_name {
//         fn from_row(row: &tiberius::Row) -> Option<Self> {
//                 let output = catch_unwind(|| {
//                     $struct_name {
//                         $( $db_name:row.get(stringify!($db_name)).unwrap() ),*
//                     }
//                 });
//
//                 output.ok()
//             }
//         }
//     }
// }
// sql_struct! {
//     User
//     { age: i32
//     , is_admin: bool
//     }
// }
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config = Config::from_ado_string("server=tcp:172.20.64.138,51433;user=sa;password=av7VzzAQzc5w;TrustServerCertificate=true;initial catalog=mir5_game;").unwrap();

    let mgr = bb8_tiberius::ConnectionManager::build(config)?;
    let pool = bb8::Pool::builder().max_size(5).build(mgr).await?;
    let mut conn = pool.get().await?;



    // let users: Vec<User> = conn.query::<User>("SELECT * FROM Users").await;
    // users.iter().for_each(|user| {
    //     println!("{}", user.age);
    // });

    // let characters: Vec<CharacterTable> = conn.query::<CharacterTable>("select * from character",&[]).await;
    //
    // characters.iter().for_each(|row| {
    //     println!("\t row: {:?} {:?} {:?} {:?}", row.char_guid,row.char_name,row.char_level,row.create_dt );
    // });

    Ok(())
}
