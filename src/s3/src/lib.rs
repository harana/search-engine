use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use std::path::Path;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_smithy_http::byte_stream::{ByteStream, Length};
use aws_sdk_s3::operation::complete_multipart_upload::{CompleteMultipartUploadError, CompleteMultipartUploadOutput};
use futures::StreamExt;
use hyper::{Body, Request};

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

pub async fn download_as_file(client: &Client, bucket_name: &str, key: &str, file_name: &str) {
    let obj = client.get_object().bucket(bucket_name.to_string()).key( key.to_string()).send().await.unwrap();
    if (obj.metadata().unwrap().len() as i64) == obj.content_length() {
        println!("Data lengths match.");
    } else {
        println!("The data was not the same size!");
    }
}

// pub async fn downloadAsResponse(client: &Client, bucket_name: &str, key: &str, file_name: &str) -> HttpResponse {
//
// }
//
// pub async fn downloadAsStream(client: &Client, bucket_name: &str, key: &str, file_name: &str) -> HttpResponse {
//
// }


pub async fn upload_request_body(client: &Client, bucket_name: &str, key: &str, req: Request<Body>) -> Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>> {
    let obj = client
        .create_multipart_upload()
        .bucket(bucket_name.to_string())
        .key(key.to_string())
        .send()
        .await
        .unwrap();

    let upload_id = obj.upload_id().unwrap();

    let mut upload_parts: Vec<CompletedPart> = Vec::new();
    let mut body = req.into_body();
    let mut part_number = 0;

    loop {
        match body.next().await {
            Some(next) => {
                let byte_stream = ByteStream::from(next.unwrap());

                let upload_part_res = client
                    .upload_part()
                    .key(key.to_string())
                    .bucket(bucket_name.to_string())
                    .upload_id(upload_id)
                    .body(byte_stream)
                    .part_number(part_number)
                    .send()
                    .await
                    .unwrap();

                upload_parts.push(
                    CompletedPart::builder()
                        .e_tag(upload_part_res.e_tag.unwrap_or_default())
                        .part_number(part_number)
                        .build()
                );

                part_number += 1;
            }
            None => break
        }
    }

    client
        .complete_multipart_upload()
        .bucket(bucket_name.to_string())
        .key(key.to_string())
        .multipart_upload(CompletedMultipartUpload::builder().set_parts(Some(upload_parts)).build())
        .upload_id(upload_id)
        .send()
        .await
}

pub async fn upload_stream(client: &Client, bucket_name: &str, key: &str, stream: ByteStream) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    client
        .put_object()
        .bucket(bucket_name.to_string())
        .key(key.to_string())
        .body(stream)
        .send()
        .await
}

pub async fn upload_file(client: &Client, bucket_name: &str, key: &str, file_name: &str) -> Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>> {
    let path = Path::new(file_name);
    let file_size = tokio::fs::metadata(path).await.unwrap().len();

    let obj = client
        .create_multipart_upload()
        .bucket(bucket_name.to_string())
        .key(key.to_string())
        .send()
        .await
        .unwrap();

    let upload_id = obj.upload_id().unwrap();

    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = CHUNK_SIZE;
        chunk_count -= 1;
    }

    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            CHUNK_SIZE
        };
        let stream = ByteStream::read_from()
            .path(path)
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();

        let part_number = (chunk_index as i32) + 1;
        let upload_part_res = client
            .upload_part()
            .key(key.to_string())
            .bucket(bucket_name.to_string())
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await
            .unwrap();

        upload_parts.push(
        CompletedPart::builder()
            .e_tag(upload_part_res.e_tag.unwrap_or_default())
            .part_number(part_number)
            .build()
        );
    }

    client
        .complete_multipart_upload()
        .bucket(bucket_name.to_string())
        .key(key.to_string())
        .multipart_upload(CompletedMultipartUpload::builder().set_parts(Some(upload_parts)).build())
        .upload_id(upload_id)
        .send()
        .await
}

