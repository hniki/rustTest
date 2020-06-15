// use std::error::Error;
// use csv::Writer;
use std::process;
use std::fs::File;
use std::io::{Cursor, Seek, SeekFrom, Read};
use byteorder::{BigEndian, LittleEndian,ReadBytesExt, ByteOrder};
//use postgres::{Client, NoTls};
//構造体を読み込む
mod map_data;
//mod binary;

fn main() {

    // if let Err(err) = write() {
    //     println!("error running read: {}", err);
    //     process::exit(1);
    // }
    
    // if let Err(err) = connect_db() {
    //     println!("error running read: {}", err);
    //     process::exit(1);
    // }
    
    //Vec<u8>でFileを読み込む
    let file = match std::fs::read("C:/Work/rust_practice/rustTest/test.bmp") {
        Ok(bytes) =>{
            bytes
        }
        Err(err) => {
            println!("error running read: {}", err);
            process::exit(1);
        }
    };

    //Cursorを作成する
    let mut cursor = Cursor::new(file);
    
    let first_bite = cursor.read_u8().unwrap();
    let second_bite = cursor.read_u8().unwrap();
    
    println!("data: {:?}, {:?}", first_bite, second_bite);
    //BMPファイルなので、10進数66,77と表示されればOK
    //66 = B
    //77 = M
    //https://algorithm.joho.info/image-processing/binary-editor-stirling-bmp-file/

    //byteOrader
    //https://docs.rs/byteorder/1.1.0/byteorder/trait.ReadBytesExt.html

    // let result = read_u8(&mut cursor);
    // println!("data: {:?}, position {}", result, cursor.position());

    // let result2 = read_u8(&mut cursor);
    // println!("data: {:?}, position {}", result2, cursor.position());

    // match std::fs::read("C:/Work/rust_practice/rustTest/test.bmp") {
    //     Ok(bytes) =>{
    //         println!("{:?}", bytes);
    //     }
    //     Err(err) => {
    //         println!("error running read: {}", err);
    //         process::exit(1);
    //     }
    // }

}

pub type ByteCursor = Cursor<Vec<u8>>;

pub fn read_u8(cursor: &mut ByteCursor) -> std::io::Result<u8>{
    cursor.read_u8()
}

pub fn read_u16(cursor: &mut ByteCursor) -> std::io::Result<u16>{
    cursor.read_u16::<BigEndian>()
}

// pub fn read_u8(cursor: &mut ByteCursor) -> std::io::Result<u8>{
//     cursor.seek(SeekFrom::End())
// }

// pub fn read_u16(cursor: &mut ByteCursor) -> std::io::Result<u16>{
//     cursor.read_u16::<BigEndian>()
// }

// fn write() -> Result<(), Box<dyn Error >>{    
//     //cargo install csv
//     let mut wtr = Writer::from_path("foo.csv")?;
    
//     let mut map_data_list: Vec<map_data::MapData> = Vec::new();
    
//     map_data_list.push(map_data::MapData {
//         name: String::from("虎ノ門ヒルズ駅"),
//         latitude: String::from("35.668087"),
//         longitude: String::from("139.747885"),
//     });

//     map_data_list.push(map_data::MapData {
//         name: String::from("肉のハナマサ 南麻布店"),
//         latitude: String::from("35.647581"),
//         longitude: String::from("139.728207"),
//     });

//     // let mut map_data_list = vec!
//     // [
//     //     map_data::MapData {
//     //         name: String::from("虎ノ門ヒルズ駅"),
//     //         latitude: String::from("35.668087"),
//     //         longitude: String::from("139.747885"),
//     //     },
//     //     map_data::MapData {
//     //         name: String::from("肉のハナマサ 南麻布店"),
//     //         latitude: String::from("35.647581"),
//     //         longitude: String::from("139.728207"),
//     //     },
//     // ];

//     //ヘッダー行
//     wtr.write_record(&["名前","緯度","経度"])?;
    
//     //map_data_listイテレーション
//     for map_data in map_data_list {
//         wtr.write_record(&[map_data.name, map_data.latitude, map_data.longitude])?;
//     }
//     wtr.flush()?;

//     Ok(())
// }

// fn connect_db() -> Result<(), postgres::Error> {
//     //Cargo install postgres
//     //Connection Stringについて
//     //https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING

//     let mut client = Client::connect("host=localhost user=postgres password=P@ssw0rd!1 dbname=20Maps", NoTls)?;
//     //let mut client = Client::connect("postgresql://postgres:P@ssw0rd!1@localhost/20Maps", NoTls)?;
    
//     //postgresql://postgres:P@ssw0rd!1@localhost/20Maps
//     client.batch_execute("
//         CREATE TABLE IF NOT EXISTS author (
//             id              SERIAL PRIMARY KEY,
//             name            VARCHAR NOT NULL,
//             country         VARCHAR NOT NULL
//             )
//     ")?;

//     client.batch_execute("
//         CREATE TABLE IF NOT EXISTS book  (
//             id              SERIAL PRIMARY KEY,
//             title           VARCHAR NOT NULL,
//             author_id       INTEGER NOT NULL REFERENCES author
//             )
//     ")?;

//     Ok(())
// }