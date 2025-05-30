use fred::{
    clients::RedisClient,
    interfaces::*,
    types::{RedisValue, Expiration},
};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use ahash::AHasher;
use std::hash::BuildHasherDefault;

// L1 cache: In-memory cache for hot data
static L1_CACHE: Lazy<DashMap<String, (RedisValue, Instant), BuildHasherDefault<AHasher>>> = 
    Lazy::new(|| DashMap::with_hasher(BuildHasherDefault::default()));

use std::time::Instant;

pub struct OptimizedRedisClient {
    client: RedisClient,
    l1_cache_ttl: Duration,
}

impl OptimizedRedisClient {
    pub fn new(client: RedisClient) -> Self {
        Self {
            client,
            l1_cache_ttl: Duration::from_secs(60), // 1 minute L1 cache
        }
    }

    pub async fn get_cached<T>(&self, key: &str) -> Result<Option<T>, fred::error::RedisError>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Check L1 cache first
        if let Some((value, timestamp)) = L1_CACHE.get(key) {
            if timestamp.elapsed() < self.l1_cache_ttl {
                if let Ok(deserialized) = serde_json::from_str::<T>(&value.as_string().unwrap_or_default()) {
                    return Ok(Some(deserialized));
                }
            } else {
                L1_CACHE.remove(key);
            }
        }

        // Check Redis (L2 cache)
        let result: Option<String> = self.client.get(key).await?;
        
        if let Some(value) = result {
            // Update L1 cache
            L1_CACHE.insert(
                key.to_string(), 
                (RedisValue::String(value.clone()), Instant::now())
            );
            
            let deserialized: T = serde_json::from_str(&value)
                .map_err(|e| fred::error::RedisError::new(
                    fred::error::RedisErrorKind::Parse, 
                    Some(e.to_string())
                ))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    pub async fn set_cached<T>(
        &self, 
        key: &str, 
        value: &T, 
        ttl: Option<Duration>
    ) -> Result<(), fred::error::RedisError>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)
            .map_err(|e| fred::error::RedisError::new(
                fred::error::RedisErrorKind::Parse, 
                Some(e.to_string())
            ))?;

        // Update L1 cache
        L1_CACHE.insert(
            key.to_string(), 
            (RedisValue::String(serialized.clone()), Instant::now())
        );

        // Update Redis
        if let Some(ttl) = ttl {
            self.client.set(key, &serialized, Some(Expiration::EX(ttl.as_secs() as i64)), None, false).await?;
        } else {
            self.client.set(key, &serialized, None, None, false).await?;
        }

        Ok(())
    }

    pub async fn batch_get(&self, keys: &[&str]) -> Result<Vec<Option<String>>, fred::error::RedisError> {
        self.client.mget(keys).await
    }

    pub async fn batch_set(&self, pairs: &[(&str, &str)]) -> Result<(), fred::error::RedisError> {
        let args: Vec<(String, String)> = pairs.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.client.mset(args, false).await
    }
}
