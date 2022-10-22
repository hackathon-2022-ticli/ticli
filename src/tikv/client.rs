use std::io::Read;

use anyhow::anyhow;
use tikv_client::{BoundRange, Error::KvError, Key, KvPair, RawClient, Result, TransactionClient as TxnClient, Value};

const MAX_RAW_KV_SCAN_LIMIT: u32 = 10240;

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

    pub async fn scan(&self, range: impl Into<BoundRange>, limit: usize) -> Result<Vec<KvPair>> {
        match self {
            Client::Raw(c) => c.scan(range, limit as u32).await,
            Client::Txn(c) => {
                let mut txn = c.begin_optimistic().await?;
                let values = txn.scan(range, limit as u32).await?;
                txn.commit().await.map(|_| values.collect())
            }
        }
    }

    pub async fn load_csv(
        &self,
        reader: Box<dyn Read>,
        has_headers: bool,
        delimiter: char,
        batch_size: usize,
    ) -> Result<()> {
        let rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .delimiter(delimiter as u8)
            .from_reader(reader);
        let mut kvs = rdr.into_records().enumerate().map(|(i, r)| {
            r.map_err(|e| anyhow!(e))
                .and_then(|r| match (r.get(0), r.get(1), r.get(2)) {
                    (Some(k), Some(v), None) => Ok((k.to_owned(), v.to_owned())),
                    _ => Err(anyhow!("invalid kv pair at record #{}", i + 1)),
                })
        });
        match self {
            Client::Raw(c) => loop {
                let mut batch = Vec::with_capacity(batch_size);
                for _ in 0..batch_size {
                    match kvs.next() {
                        Some(Ok(kv)) => batch.push(kv),
                        Some(Err(e)) => return Err(KvError { message: e.to_string() }),
                        None => break,
                    }
                }
                match batch.len() {
                    0 => break,
                    _ => c.batch_put(batch).await?,
                }
            },
            Client::Txn(c) => loop {
                let mut batch = Vec::with_capacity(batch_size);
                for _ in 0..batch_size {
                    match kvs.next() {
                        Some(Ok(kv)) => batch.push(kv),
                        Some(Err(e)) => return Err(KvError { message: e.to_string() }),
                        None => break,
                    }
                }
                match batch.len() {
                    0 => break,
                    _ => {
                        let mut txn = c.begin_optimistic().await?;
                        for (k, v) in batch {
                            txn.put(k, v).await?;
                        }
                        match txn.commit().await {
                            Ok(_) => (),
                            // if rollback is ok then return the original error,
                            // otherwise return the rollback error
                            Err(e) => match txn.rollback().await {
                                Ok(_) => return Err(e),
                                Err(re) => return Err(re),
                            },
                        }
                    }
                }
            },
        }
        Ok(())
    }

    pub async fn ping(&self) -> Result<()> {
        match self {
            Client::Raw(c) => c.get("".to_string()).await.map(|_| ()),
            Client::Txn(c) => c.current_timestamp().await.map(|_| ()),
        }
    }

    pub async fn delete(&self, key: impl Into<Key>) -> Result<()> {
        match self {
            Client::Raw(c) => c.delete(key).await,
            Client::Txn(c) => {
                let mut txn = c.begin_optimistic().await?;
                txn.delete(key).await?;
                txn.commit().await?;
                Ok(())
            }
        }
    }

    pub async fn flush_all(&self) -> Result<()> {
        match self {
            Client::Raw(c) => c.delete_range(..).await,
            Client::Txn(c) => {
                let mut range = Key::EMPTY..;
                loop {
                    let mut txn = c.begin_optimistic().await?;
                    let keys: Vec<Key> = txn.scan_keys(range.clone(), 1000).await?.collect();
                    range = match keys.last() {
                        Some(key) => key.clone()..,
                        None => {
                            txn.commit().await?;
                            break;
                        }
                    };

                    for key in keys {
                        txn.delete(key).await?;
                    }
                    txn.commit().await?;
                }
                Ok(())
            }
        }
    }

    pub async fn count(&self, range: impl Into<BoundRange>) -> Result<usize> {
        match self {
            Client::Raw(c) => {
                // FIXME: scroll the cursor to get the total count
                c.scan_keys(range, MAX_RAW_KV_SCAN_LIMIT).await.map(|keys| keys.len())
            }
            Client::Txn(c) => {
                let mut txn = c.begin_optimistic().await?;
                let keys = txn.scan_keys(range, u32::MAX).await?;
                txn.commit().await.map(|_| keys.count())
            }
        }
    }

    pub async fn strlen(&self, key: impl Into<Key>) -> Result<Option<usize>> {
        self.get(key).await.map(|value| value.map(|val| val.len()))
    }

    pub async fn exists(&self, key: impl Into<Key>) -> Result<bool> {
        match self {
            Client::Raw(_) => self.get(key).await.map(|val| val.is_some()),
            Client::Txn(c) => {
                let mut txn = c.begin_optimistic().await?;
                let exists = txn.key_exists(key).await?;
                txn.commit().await.map(|_| exists)
            }
        }
    }

    pub async fn incr_by(&self, key: impl Into<Key>, delta: i128) -> anyhow::Result<Option<Value>> {
        let try_add = |prev: Vec<u8>, delta: i128| {
            let prev: i128 = String::from_utf8(prev)?.parse()?;
            prev.checked_add(delta)
                .map(|x| x.to_string().into_bytes())
                .ok_or_else(|| anyhow!("integer overflow: {} + {} is out of range", prev, delta))
        };

        let key: Key = key.into();
        match self {
            Client::Raw(c) => loop {
                let key = key.clone();
                let (prev, next) = match self.get(key.clone()).await? {
                    None => (None, delta.to_string().into_bytes()),
                    Some(prev) => (Some(prev.clone()), try_add(prev, delta)?),
                };
                if let (_, true) = c
                    .with_atomic_for_cas()
                    .compare_and_swap(key, prev, next.clone())
                    .await?
                {
                    return Ok(Some(next));
                }
            },
            Client::Txn(c) => {
                let mut txn = c.begin_pessimistic().await?;
                let next = match self.get(key.clone()).await? {
                    None => delta.to_string().into_bytes(),
                    Some(prev) => try_add(prev, delta)?,
                };
                txn.put(key, next.clone()).await?;
                txn.commit().await?;
                Ok(Some(next))
            }
        }
    }
}
