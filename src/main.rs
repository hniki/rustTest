use std::error::Error;
use csv::Writer;
use std::process;
use postgres::{Client, NoTls};
//構造体を読み込む
mod map_data;

fn main() {

    if let Err(err) = write() {
        println!("error running read: {}", err);
        process::exit(1);
    }
    
    if let Err(err) = connect_db() {
        println!("error running read: {}", err);
        process::exit(1);
    }
}

fn write() -> Result<(), Box<dyn Error >>{    
    //cargo install csv
    let mut wtr = Writer::from_path("foo.csv")?;
    
    let mut map_data_list: Vec<map_data::MapData> = Vec::new();
    
    map_data_list.push(map_data::MapData {
        name: String::from("虎ノ門ヒルズ駅"),
        latitude: String::from("35.668087"),
        longitude: String::from("139.747885"),
    });

    map_data_list.push(map_data::MapData {
        name: String::from("肉のハナマサ 南麻布店"),
        latitude: String::from("35.647581"),
        longitude: String::from("139.728207"),
    });

    // let mut map_data_list = vec!
    // [
    //     map_data::MapData {
    //         name: String::from("虎ノ門ヒルズ駅"),
    //         latitude: String::from("35.668087"),
    //         longitude: String::from("139.747885"),
    //     },
    //     map_data::MapData {
    //         name: String::from("肉のハナマサ 南麻布店"),
    //         latitude: String::from("35.647581"),
    //         longitude: String::from("139.728207"),
    //     },
    // ];

    //ヘッダー行
    wtr.write_record(&["名前","緯度","経度"])?;
    
    //map_data_listイテレーション
    for map_data in map_data_list {
        wtr.write_record(&[map_data.name, map_data.latitude, map_data.longitude])?;
    }
    wtr.flush()?;

    Ok(())
}

fn connect_db() -> Result<(), postgres::Error> {
    //Cargo install postgres
    //Connection Stringについて
    //https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING

    let mut client = Client::connect("host=localhost user=postgres password=P@ssw0rd!1 dbname=20Maps", NoTls)?;
    //let mut client = Client::connect("postgresql://postgres:P@ssw0rd!1@localhost/20Maps", NoTls)?;
    
    //postgresql://postgres:P@ssw0rd!1@localhost/20Maps
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ")?;

    Ok(())
}