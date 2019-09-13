#![feature(async_await)]

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use s3::{bucket::Bucket, region::Region};

fn gimme_a_bucket() -> s3::bucket::Bucket {
    let region = Region::Custom {
        region: "custom".to_owned(),
        endpoint: "http://localhost:9001".to_owned(),
    };

    let bucket = Bucket::new("minio-test", region, Default::default()).unwrap();

    bucket
}

fn index_single() -> impl Future<Item = HttpResponse, Error = Error> {
    let bucket = gimme_a_bucket();
    let fut_single = bucket
        .get_object_async("/data.txt")
        .map_err(|_e| {
            // Usability comment: it seems that _e does not impl any traits that
            // would be useful for formatting an error message such as
            // Debug, Display, or Error
            Error::from(())
        })
        .map(|(data, _code)| {
            println!("Got data");
            data
        });

    fut_single.map(|td| HttpResponse::Ok().body(td))
}

/*
fn index_list() -> impl Future<Item = HttpResponse, Error = Error> {
    let bucket = gimme_a_bucket();

    bucket
        .list_all_async("/".to_owned(), Some("/".to_owned()))
        .map_err(|_e| {
            // Usability comment: it seems that _e does not impl any traits that
            // would be useful for formatting an error message such as
            // Debug, Display, or Error
            Error::from(())
        })
        .map(|results| {
            // collect multi-page results into a flat vec of object keys
            results
                .into_iter()
                .flat_map(|a| a.contents.into_iter().map(|obj| obj.key.clone()))
                .collect()
        })
        .map(|obj_keys: Vec<String>| {
            let concatenated: String = obj_keys.join(", ");
            HttpResponse::Ok().body(concatenated)
        })
} // */
fn main() {
    HttpServer::new(|| {
        App::new().route("/single", web::get().to_async(index_single))
        //.route("/list", web::get().to_async(index_list)) // */
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
