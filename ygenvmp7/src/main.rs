use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{CreateCollection, SearchPoints, VectorParams, VectorsConfig};
use qdrant_client::qdrant::PointStruct;
use serde_json::json;
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(&CreateCollection {
            collection_name: collection_name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 10,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;

    let _collection_info = client.collection_info(collection_name).await?;
    // dbg!(collection_info);


    println!("Creating collection: {}", collection_name);
    let points = vec![
        PointStruct::new(
            1,
            vec![0.05, 0.61, 0.76, 0.74, 0.19, 0.81, 0.75, 0.11, 0.19, 0.81],
            json!(
                {"card": "DarkMagician", "series": "Yu-Gi-Oh!"}
            )
            .try_into()
            .unwrap(),
        ),
        PointStruct::new(
            2,
            vec![0.19, 0.81, 0.75, 0.11, 0.19, 0.52, 0.38, 0.27, 0.78, 0.99],
            json!(
                {"card": "BlitzGreymon", "series": "Digimon"}
            )
            .try_into()
            .unwrap(),
        ),
        PointStruct::new(
            3,
            vec![0.29, 0.31, 0.52, 0.11, 0.19, 0.25, 0.38, 0.27, 0.78, 0.99],
            json!(
                {"card": "Bolshack Yamato Dragon", "series": "Duel Masters"}
            )
            .try_into()
            .unwrap(),
        ),
    ];
    let _operation_info = client
        .upsert_points_blocking("test".to_string(), None, points, None)
        .await?;

    // dbg!(operation_info);

    println!("Upserted points");

    
    let mut search_vector = Vec::new();

    println!("Enter values for the search vector (10 values expected):");

    // Read 10 values from the user
    for _ in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let value: f32 = input.trim().parse().expect("Invalid input");
        search_vector.push(value);
    }

    // Perform the search with the user-provided search vector
    let search_result = client
        .search_points(&SearchPoints {
            collection_name: "test".to_string(),
            vector: search_vector,
            limit: 3,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;

    dbg!(search_result);

    Ok(())
}