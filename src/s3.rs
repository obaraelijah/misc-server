use actix_web::web::Json;
use actix_web::{
    get,
    web::{scope, Data, Query, ServiceConfig},
};
use aws_sdk_s3::Client;
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};

use crate::common::Config;
use crate::errors::{Result, ServerError};
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

pub fn s3_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/s3").service(list_objects));
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

    let objects: Vec<ObjectEntry> = res
        .contents
        .unwrap_or_default()
        .into_iter()
        .map(|o| ObjectEntry {
            name: o.key.unwrap(),
            kind: ObjectType::File,
        })
        .collect();

    let mut dirs: Vec<ObjectEntry> = res
        .common_prefixes
        .unwrap_or_default()
        .into_iter()
        .map(|o| ObjectEntry {
            name: o.prefix.unwrap(),
            kind: ObjectType::Dir,
        })
        .collect();

    dirs.extend(objects);
    Ok(Json(dirs))
}

#[get("/get_object")]
async fn get_object(
    file_path: Option<Query<S3Query>>,
    s3: Data<Client>,
    config: Data<Config>,
) -> Result<Json<S3Object>> {
    // file path
    let file_path = file_path.ok_or_else(|| ServerError::GetObject {
        message: "No file path".to_string(),
    });


    let key = &file_path.unwrap().path;

    let obj = s3
        .get_object()
        .bucket(&config.bucket_name)
        .key(key)
        .send()
        .await?;

    let bytes = obj
        .body
        .collect()
        .await
        .map_err(|e| ServerError::GetObject {
            message: e.to_string(),
        })?;

    // encode the bytes to a base64 string
    let blob = STANDARD_NO_PAD.encode(bytes.to_vec());

    // Retrieve the MIME type from the object's metadata
    let mime_type = obj.content_type.ok_or_else(|| ServerError::GetObject {
        message: "No content type".to_string(),
    })?;

    let name = key.split('/').last().unwrap().to_string();

    Ok(Json(S3Object {
        blob,
        name,
        mime_type,
    }))
}