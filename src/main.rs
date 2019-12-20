#[macro_use]
extern crate serde_derive;
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Local;
use std::time::{UNIX_EPOCH, Duration};
use structopt::StructOpt;

use std::fs::File;

use std::vec::Vec;


#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    name: String,
    photos: Vec<Photo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Photo {
    uri: String,
    creation_timestamp: u64,
    media_metadata: PhotoMetaData,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoMetaData {
    photo_metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(default)]
    taken_timestamp: String,
    #[serde(default)]
    modified_timestamp: String,
    #[serde(default)]
    creation_timestamp: String,
    #[serde(default)]
    f_stop: String,
    #[serde(default)]
    camera_make: String,
    #[serde(default)]
    camera_model: String,
    #[serde(default)]
    latitude: String,
    #[serde(default)]
    longitude: String,
    #[serde(default)]
    exposure: String,
    #[serde(default)]
    iso_speed: String,
    #[serde(default)]
    focal_length: String,
    #[serde(default)]
    upload_ip: String,
}

#[derive(StructOpt, Debug)]
struct Opt {
    
    /// This option is positional, meaning it is the first unadorned string
    /// you provide (multiple others could follow).
    jfile: String,
}


fn main() {

    let opt = Opt::from_args();

    println!("{:?}", opt);
    

    let file = File::open("G:\\photos_and_videos\\album\\0.json").expect("File not found");
    let  album: Album = serde_json::from_reader(file).expect("JSON was not well-formatted");

    for photo in &album.photos {
        let timestamp = photo.creation_timestamp;

        // Creates a new SystemTime from the specified number of whole seconds
        let d = UNIX_EPOCH + Duration::from_secs(timestamp);
        // Create DateTime from SystemTime
        let datetime = DateTime::<Local>::from(d);
        // Formats the combined date and time with the specified format string.
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("-CreateDate=\"{}\" {}", timestamp_str, photo.uri);    
    }

    println!("\n\nLista: \n {:?}", album);
    

}