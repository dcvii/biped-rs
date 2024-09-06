// mod s3_uploader;

// #[tokio::main]
// async fn main() {
//     let bucket = "darwin-cds-data-bucket";
//     let source_path = "data/fc.parquet";
//     let target_path = "biped-data/fc.parquet";

//     match s3_uploader::upload_file_to_s3(bucket, source_path, target_path).await {
//         Ok(_) => println!("File uploaded successfully!"),
//         Err(e) => eprintln!("Failed to upload file: {}", e),
//     }
// }

mod s3_upload; // Assuming you've saved the module in s3_upload.rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    s3_upload::upload_to_s3(
        "darwin-cds-data-bucket",
        "data/fc.parquet",
        "biped-data/fc.parquet",
    )
    .await?;
    Ok(())
}
