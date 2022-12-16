use mpi::environment::Universe;
use mpi::topology::{Process, SystemCommunicator};
use mpi::traits::*;
use mpi::Rank;

const MASTER_RANK: Rank = 0;

pub struct MpiState {
    pub universe: Universe,
    pub world: SystemCommunicator,
}

impl Default for MpiState {
    fn default() -> Self {
        let universe = mpi::initialize().expect("MPI was already initialized");
        let world = universe.world();

        if world.size() < 2 {
            panic!("at least 2 processses are required");
        }

        Self { universe, world }
    }
}

impl MpiState {
    #[inline]
    pub fn is_master(&self) -> bool {
        self.world.rank() == MASTER_RANK
    }

    #[inline]
    pub fn master(&self) -> Process<SystemCommunicator> {
        self.world.process_at_rank(0)
    }

    #[inline]
    pub fn get_slave_processs(&self, index: usize) -> Process<SystemCommunicator> {
        self.world.process_at_rank((index + 1) as i32)
    }

    #[inline]
    pub fn slave_processs_count(&self) -> usize {
        self.world.size() as usize - 1
    }
}
