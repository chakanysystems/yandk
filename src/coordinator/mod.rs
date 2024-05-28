use crate::relay::pool::RelayPool;
use crate::Result;
use nostrdb::{Ndb, ProfileRecord};

/// controls events, relays, you name it.
#[derive(Debug)]
pub struct Coordinator {
    pool: RelayPool,
    ndb: Ndb,
    transaction: nostrdb::Transaction,
}

#[allow(clippy::new_without_default)]
impl Coordinator {
    pub fn new() -> Self {
        let config = nostrdb::Config::new();
        let ndb = match Ndb::new("./test", &config) {
            Ok(db) => db,
            Err(e) => panic!("Could not add db {}", e),
        };

        let pool = RelayPool::new();
        let tx = match nostrdb::Transaction::new(&ndb) {
            Ok(t) => t,
            Err(e) => panic!("error creating nostrdb transaction {}", e),
        };

        Self {
            pool,
            ndb,
            transaction: tx,
        }
    }

    pub fn get_profile(&mut self, pubkey: &[u8; 32]) -> Result<ProfileRecord<'_>> {
        let profile = self.ndb.get_profile_by_pubkey(&self.transaction, pubkey)?;
        tokio::spawn(async move {});

        Ok(profile)
    }
}
