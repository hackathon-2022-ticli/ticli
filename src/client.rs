use tikv_client::{Key, KvPair, RawClient, Result, TransactionClient as TxnClient, Value};

pub enum Client {
    Raw(RawClient),
    Txn(TxnClient),
}

impl Client {
    pub async fn raw(addr: impl Into<String>) -> Result<Self> {
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        RawClient::new(vec![addr], Some(logger)).await.map(Client::Raw)
    }

    pub async fn txn(addr: impl Into<String>) -> Result<Self> {
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        TxnClient::new(vec![addr], Some(logger)).await.map(Client::Txn)
    }

    pub async fn get(&self, key: impl Into<Key>) -> Result<Option<Value>> {
        match self {
            Client::Raw(c) => c.get(key).await,
            Client::Txn(c) => {
                let mut txn = c.begin_optimistic().await?;
                let value = txn.get(key).await?;
                txn.commit().await.map(|_| value)
            }
        }
    }

    pub async fn set(&self, key: impl Into<Key>, val: impl Into<Value>) -> Result<()> {
        match self {
            Client::Raw(c) => c.put(key, val).await,
            Client::Txn(c) => {
                let mut txn = c.begin_optimistic().await?;
                txn.put(key, val).await?;
                match txn.commit().await {
                    Ok(_) => Ok(()),
                    // if rollback is ok then return the original error,
                    // otherwise return the rollback error
                    Err(e) => match txn.rollback().await {
                        Ok(_) => Err(e),
                        Err(re) => Err(re),
                    },
                }
            }
        }
    }

    pub async fn scan(&self, prefix: impl Into<Key>, limit: usize) -> Result<Vec<KvPair>> {
        match self {
            Client::Raw(c) => c.scan(prefix.., limit as u32).await,
            Client::Txn(c) => {
                let mut txn = c.begin_optimistic().await?;
                let values = txn.scan(prefix.., limit as u32).await?;
                txn.commit().await.map(|_| values.collect())
            }
        }
    }

    pub async fn ping(&self) -> Result<()> {
        match self {
            Client::Raw(c) => c.get("".to_string()).await.map(|_| ()),
            Client::Txn(c) => c.current_timestamp().await.map(|_| ()),
        }
    }
}
