use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind,
};
use std::{error::Error, io, time::Duration};

const AMQP_ADDR: &str = "amqp://guest:guest@127.0.0.1:5672/%2f";
const EVENT_NAME: &str = "user_created";
const EXCHANGE_NAME: &str = "user_created_exchange";

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| AMQP_ADDR.to_owned());
    let conn = tokio::time::timeout(
        Duration::from_secs(10),
        Connection::connect(&addr, ConnectionProperties::default()),
    )
    .await
    .map_err(|_| {
        io::Error::new(
            io::ErrorKind::TimedOut,
            format!("timed out connecting to RabbitMQ at {addr}"),
        )
    })??;
    let channel = conn.create_channel().await?;

    channel
        .exchange_declare(
            EXCHANGE_NAME.into(),
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let messages = [
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406395291-Amir".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406395291-Budi".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406395291-Cica".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406395291-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406395291-Emir".to_owned(),
        },
    ];

    for message in messages {
        let mut buffer = Vec::new();
        message.serialize(&mut buffer)?;
        channel
            .basic_publish(
                EXCHANGE_NAME.into(),
                EVENT_NAME.into(),
                BasicPublishOptions::default(),
                &buffer,
                BasicProperties::default(),
            )
            .await?
            .await?;
    }

    conn.close(0, "publisher finished".into()).await?;
    Ok(())
}
