use crate::relay::pool::RelayPool;
use crate::relay::Subscription;
use crate::Result;
use nostrdb::{Ndb, ProfileRecord};
use tracing::error;

/// controls events, relays, you name it.
/// meant for gui apps
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

    pub fn add_relay(
        &mut self,
        url: String,
        wake_up: impl Fn() + Send + Sync + 'static,
    ) -> Result<()> {
        self.pool.add_relay_with_wakeup(url, wake_up)?;

        Ok(())
    }

    /// recieve events and deposit them into ndb
    /// should be called whenever you need it.
    pub fn try_recv(&mut self) {
        if let Ok(event) = self.pool.try_recv() {
            if let Some(event) = event {
                if let Err(_e) = self.ndb.process_event(event.as_str()) {
                    error!("could not process event")
                }
            }
        }
    }

    pub fn get_profile(&mut self, pubkey: &[u8; 32]) -> Result<Option<ProfileRecord<'_>>> {
        match self.ndb.get_profile_by_pubkey(&self.transaction, pubkey) {
            Ok(profile) => {
                return Ok(Some(profile));
            }
            Err(e) => match e {
                nostrdb::Error::NotFound => {
                    let sub = Subscription::default();
                    match self.pool.add_subscription(sub) {
                        Ok(s) => s,
                        Err(e) => error!("could not add subscription to pool: {}", e),
                    };
                    return Ok(None);
                }
                _ => return Err(e.into()),
            },
        };
    }
}
