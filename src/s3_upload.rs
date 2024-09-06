use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use aws_sdk_s3::{Client, Error};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn upload_to_s3(bucket: &str, source_path: &str, target_path: &str) -> Result<(), Error> {
    // Load AWS configuration
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    // let config: SdkConfig = aws_config::from_env().region(region_provider).load().await;
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Read file contents
    let mut file = File::open(source_path).await.expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .await
        .expect("Unable to read file");

    // Upload to S3
    client
        .put_object()
        .bucket(bucket)
        .key(target_path)
        .body(contents.into())
        .send()
        .await?;

    println!("File uploaded successfully!");
    Ok(())
}
