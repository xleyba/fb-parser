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
    #[serde(default)]
    comments: Vec<Comment>,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoMetaData {
    photo_metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(default)]
    taken_timestamp: u64,
    #[serde(default)]
    modified_timestamp: u64,
    #[serde(default)]
    creation_timestamp: u64,
    #[serde(default)]
    f_stop: String,
    #[serde(default)]
    camera_make: String,
    #[serde(default)]
    camera_model: String,
    #[serde(default)]
    latitude: f64,
    #[serde(default)]
    longitude: f64,
    #[serde(default)]
    exposure: String,
    #[serde(default)]
    iso_speed: u64,
    #[serde(default)]
    focal_length: String,
    #[serde(default)]
    upload_ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    #[serde(default)]
    timestamp: u64,
    #[serde(default)]
    comment: String,
    #[serde(default)]
    author: String,
}

#[derive(StructOpt, Debug)]
struct Opt {

    #[structopt(value_name = "FILE", help = "The JSON file to be parsed")]
    jfile: String,
}


fn main() {

    let opt = Opt::from_args();

    let mut cmd_line = "".to_string();
    

    //let file = File::open("G:\\photos_and_videos\\album\\0.json").expect("File not found");
    let file = File::open(opt.jfile).expect("File not found");
    let  album: Album = serde_json::from_reader(file).expect("JSON was not well-formatted");

    for photo in album.photos {
        
        if photo.media_metadata.photo_metadata.taken_timestamp > 0 {
            let timestamp = photo.media_metadata.photo_metadata.taken_timestamp;            
            let d = UNIX_EPOCH + Duration::from_secs(timestamp);
            let datetime = DateTime::<Local>::from(d);
            
            let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();  
            
            cmd_line.push_str(&format!("\"-EXIF:DateTimeOriginal={}\" \"-EXIF:Create:Date={}\"", timestamp_str, timestamp_str));
        }

        if photo.media_metadata.photo_metadata.modified_timestamp > 0 {
            let timestamp = photo.media_metadata.photo_metadata.modified_timestamp;            
            let d = UNIX_EPOCH + Duration::from_secs(timestamp);
            let datetime = DateTime::<Local>::from(d);
            
            let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();  
            
            cmd_line.push_str(&format!("\"-EXIF:DateTimeOriginal={}\" \"-EXIF:Create:Date={}\"", timestamp_str, timestamp_str));
        }

        println!("Camera Make: {}", photo.media_metadata.photo_metadata.camera_make);

         
    }

    //println!("{}", cmd_line);   
    //println!("\n\nLista: \n {:?}", album);
    

}