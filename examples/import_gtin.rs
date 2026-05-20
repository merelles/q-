use anyhow::Result;
use bytes::Bytes;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = "host=localhost user=postgres password=password dbname=poc_q";

    let csv_path = "/home/merelles/Downloads/open4goods-gtin-dataset.csv";

    let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    client
        .batch_execute(
            r#"
            CREATE SCHEMA IF NOT EXISTS external_catalog;

            DROP TABLE IF EXISTS external_catalog.open4goods_gtin_raw;

            CREATE TABLE external_catalog.open4goods_gtin_raw
            (
                oid                     BIGSERIAL PRIMARY KEY,
                gtin                    TEXT NULL,
                brand                   TEXT NULL,
                model                   TEXT NULL,
                name                    TEXT NULL,
                last_updated             TEXT NULL,
                gs1_country              TEXT NULL,
                gtintype                 TEXT NULL,
                offers_count             TEXT NULL,
                min_price                TEXT NULL,
                min_price_compensation   TEXT NULL,
                currency                 TEXT NULL,
                categories               TEXT NULL,
                url                      TEXT NULL,
                source                  TEXT NOT NULL DEFAULT 'open4goods',
                imported_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                processed               BOOLEAN NOT NULL DEFAULT FALSE
            );
            "#,
        )
        .await?;

    let copy_sql = r#"
        COPY external_catalog.open4goods_gtin_raw
        (
            gtin,
            brand,
            model,
            name,
            last_updated,
            gs1_country,
            gtintype,
            offers_count,
            min_price,
            min_price_compensation,
            currency,
            categories,
            url
        )
        FROM STDIN
        WITH (
            FORMAT csv,
            HEADER true,
            QUOTE '"',
            ESCAPE '"',
            NULL ''
        )
    "#;

    let sink = client.copy_in(copy_sql).await?;
    tokio::pin!(sink);

    let mut file = File::open(csv_path).await?;
    let mut buffer = vec![0u8; 1024 * 1024 * 8]; // 8 MB

    let mut total_bytes: u64 = 0;

    loop {
        let read = file.read(&mut buffer).await?;

        if read == 0 {
            break;
        }

        total_bytes += read as u64;

        futures::SinkExt::send(&mut sink, Bytes::copy_from_slice(&buffer[..read])).await?;

        if total_bytes % (1024 * 1024 * 512) < read as u64 {
            println!("Enviado: {:.2} GB", total_bytes as f64 / 1024.0 / 1024.0 / 1024.0);
        }
    }

    let inserted = futures::SinkExt::close(&mut sink).await?;

    println!("Importação finalizada.");
    println!("Linhas inseridas: {:?}", inserted);

    Ok(())
}