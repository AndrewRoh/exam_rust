use tiberius::Config;
use env_logger;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config = Config::from_ado_string("server=tcp:172.20.64.138,51433;user=sa;password=av7VzzAQzc5w;TrustServerCertificate=true;initial catalog=mir5_game;").unwrap();

    let mgr = bb8_tiberius::ConnectionManager::build(config)?;
    let pool = bb8::Pool::builder().max_size(5).build(mgr).await?;
    let mut conn = pool.get().await?;

    let res = conn.simple_query("select 'text' as version, 1 as one, convert([datetime2],getdate()) as n, newid() as uid")
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| {
            let ver: &str = row.get(0).unwrap();
            let one: i32 = row.get(1).unwrap();
            let n: chrono::naive::NaiveDateTime = row.get(2).unwrap();
            let uid: uuid::Uuid = row.get(3).unwrap();
            (String::from(ver),one, n, uid)
        })
        .collect::<Vec<_>>();
    println!("{:?}", res);

    //let stream = conn.query("select * from character", &[]).await?;
    //let rows = stream.into_first_result().await?;

    let results = conn.simple_query("select * from character")
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| {
            let mut n = 0_usize;
            let char_guid: uuid::Uuid = row.get(n).unwrap(); n += 1;
            let player_guid: uuid::Uuid = row.get(n).unwrap(); n += 1;
            let char_name: &str = row.get(n).unwrap(); n += 1;
            let gender: u8 = row.get(n).unwrap(); n += 1;
            let char_level: i16 = row.get(n).unwrap(); n+= 1;
            let gold: i32 = row.get(n).unwrap(); n+= 1;
            let silver: i32 = row.get(n).unwrap(); n+= 1;
            let diamond: i32 = row.get(n).unwrap(); n+= 1;
            let free_currency: f64 = row.get(n).unwrap(); n+= 1;
            let premium_currency: f64 = row.get(n).unwrap(); n+= 1;
            let max_health: f32 = row.get(n).unwrap(); n+= 1;
            let health: f32 = row.get(n).unwrap(); n+= 1;
            let health_regen_rate: f32 = row.get(n).unwrap(); n+= 1;
            let max_mana: f32 = row.get(n).unwrap(); n+= 1;
            let mana: f32 = row.get(n).unwrap(); n+= 1;
            let mana_regen_rate: f32 = row.get(n).unwrap(); n+= 1;
            let description_txt: &str = row.get(n).unwrap(); n+=1;
            let is_admin: bool = row.get(n).unwrap(); n+=1;
            let create_dt: chrono::naive::NaiveDateTime = row.get(n).unwrap();


            (char_guid,player_guid,String::from(char_name),gender,char_level,gold,silver,diamond,
             free_currency,premium_currency,max_health,health,health_regen_rate,max_mana,mana,
             mana_regen_rate,String::from(description_txt),is_admin,create_dt)
        })
        .collect::<Vec<_>>();

    for row in results.into_iter() {
        println!("\t row: {:?} {:?} {:?} {:?}", row.0,row.1,row.2,row.18 );
    }


    Ok(())
}
