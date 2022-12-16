use crate::mpi_state::MpiState;
use crate::utils;
use mpi::traits::*;

pub fn run() {
    let state = MpiState::default();

    if state.is_master() {
        let (p1, p2) = utils::generate_polynomials(12);

        let mut result = vec![0_i64; p1.len() + p2.len() - 1];
        let chunk_size = result.len() / state.slave_process_count();
        let last_chunk_size = chunk_size + result.len() % state.slave_process_count();

        // Send standard chunks to all slave process but one
        for i in 0..(state.slave_process_count() - 1) {
            let slave = state.get_slave_process(i);

            slave.send(&((i * chunk_size) as u64));
            slave.send(&(chunk_size as u64));
            slave.send(&p1);
            slave.send(&p2);
        }

        // Send extended chunk to last slave process
        {
            let slave = state.get_slave_process(state.slave_process_count() - 1);

            slave.send(&((result.len() - last_chunk_size) as u64));
            slave.send(&(last_chunk_size as u64));
            slave.send(&p1);
            slave.send(&p2);
        }

        // Receive standard chunks from all but one slave process
        for i in 0..(state.slave_process_count() - 1) {
            state
                .get_slave_process(i)
                .receive_into(&mut result[(i * chunk_size)..((i + 1) * chunk_size)]);
        }

        // Receive extended chunk from last slave process
        {
            let chunk_start = result.len() - last_chunk_size;

            state
                .get_slave_process(state.slave_process_count() - 1)
                .receive_into(&mut result[chunk_start..]);
        }

        println!("{:?}", result);
    } else {
        let (chunk_start, _) = state.master().receive::<u64>();
        let (chunk_len, _) = state.master().receive::<u64>();
        let (p1, _) = state.master().receive_vec::<i64>();
        let (p2, _) = state.master().receive_vec::<i64>();

        let chunk_start = chunk_start as usize;
        let chunk_len = chunk_len as usize;
        let chunk_end = chunk_start + chunk_len;

        let chunk = (chunk_start..chunk_end)
            .map(|r_index| {
                let p1_index_range = if r_index < p1.len() {
                    0..(r_index + 1)
                } else {
                    (r_index - p2.len() + 1)..p1.len()
                };

                p1_index_range
                    .map(|p1_index| {
                        let c1 = p1[p1_index];
                        let c2 = p2[r_index - p1_index];
                        c1 * c2
                    })
                    .sum::<i64>()
            })
            .collect::<Vec<_>>();

        state.master().send(&chunk);
    }
}
