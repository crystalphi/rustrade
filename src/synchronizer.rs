use crate::{exchange::Exchange, repository::Repository};

pub struct Synchronizer {
    repo: Repository,
    exchange: Exchange,
}

impl Synchronizer {
    pub fn new(repository: Repository, exchange: Exchange) -> Self {
        Synchronizer {
            repo: repository,
            exchange,
        }
    }

    pub fn synchronize(&self, symbol: &str) {
        let last_id = self.repo.last_id();
        let last_close_time = self.repo.last_close_time(symbol);

        
    }

}
