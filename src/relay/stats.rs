use std::time::Instant;

#[derive(Debug)]
pub struct RelayStats {
    pub attempts: u32,
    pub success: i32,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connected_at: Option<Instant>,
    pub first_connect_time: Option<Instant>,
}

impl RelayStats {
    pub fn new() -> Self {
        Self {
            attempts: 0,
            success: 0,
            bytes_sent: 0,
            bytes_received: 0,
            connected_at: None,
            first_connect_time: None,
        }
    }

    pub fn add_attempt(&mut self) {
        self.attempts += 1;
    }

    pub fn add_success(&mut self) {
        if self.success == 0 {
            self.first_connect_time = Some(Instant::now());
        }
        self.connected_at = Some(Instant::now());
        self.success += 1;
    }
}
