use actix_web::{
    get,
    web::{Data, Query},
};
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};
use crate::errors::Result;
use crate::common::Config;
use actix_web::web::Json;

// deserializing query params from incoming requests
#[derive(Deserialize, Debug)]
struct S3Query {
    path: String,
}

// enum to represent whether an s3 object is a file or a Dir
#[derive(Deserialize, Serialize, Debug)]
enum ObjectType {
    File,
    Dir,
}

// struct to represent an entry s3 object containing the name and type 
#[derive(Deserialize, Serialize, Debug)]
struct ObjectEntry {
    name: String,
    kind: ObjectType,
}

// struct to represent the data of an s3 object, 
#[derive(Deserialize, Serialize, Debug)]
struct S3Object {
    blob: String, // base64-encoded content
    name: String,
    mime_type: String,
}

#[get("/list_objects")]
async fn list_objects(
    path: Option<Query<S3Query>>,
    s3: Data<Client>,
    config: Data<Config>,
) -> Result<Json<Vec<ObjectEntry>>> {
    let prefix = path.as_ref().map_or("".to_string(), |p| p.path.clone());

    let res = s3
        .list_objects_v2()
        .bucket(&config.bucket_name)
        .delimiter("/")
        .prefix(&prefix)
        .send()
        .await?;

    let objects: Vec<ObjectEntry> = res.contents.unwrap_or_default().into_iter()
        .map(|o| ObjectEntry {
            name: o.key.unwrap(),
            kind: ObjectType::File,
        })
        .collect();

    let mut dirs: Vec<ObjectEntry> = res.common_prefixes.unwrap_or_default().into_iter()
        .map(|o| ObjectEntry {
            name: o.prefix.unwrap(),
            kind: ObjectType::Dir,
        })
        .collect();

    dirs.extend(objects);
    Ok(Json(dirs))
}
