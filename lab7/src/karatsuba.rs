use crate::mpi_state::MpiState;
use crate::utils;
use mpi::traits::{Communicator, Destination, Source};

pub fn multiply_distributed() {
    let state = MpiState::default();

    let (p1, p2, parent, mut slaves) = if state.is_master() {
        let (p1, p2) = utils::generate_polynomials(1024);
        (p1, p2, None, (1..state.world.size()).collect::<Vec<_>>())
    } else {
        let (p1, status) = state.world.any_process().receive_vec::<i64>();

        if p1.is_empty() {
            return;
        }

        let (p2, _) = state.world.any_process().receive_vec::<i64>();
        let (slaves, _) = state.world.any_process().receive_vec::<i32>();

        (p1, p2, Some(status.source_rank()), slaves)
    };

    if p1.len() <= 32 {
        let result = simple_multiply(&p1, &p2);

        match parent {
            Some(parent) => state.world.process_at_rank(parent).send(&result),
            None => println!("Result: {:?}", result),
        }

        for slave in slaves {
            state.world.process_at_rank(slave).send::<[i64]>(&[]);
        }

        return;
    }

    let half_len = p1.len() / 2;
    let (p1_low, p1_high) = p1.split_at(half_len);
    let (p2_low, p2_high) = p2.split_at(half_len);

    let p1_sum = p1_low.iter().zip(p1_high.iter()).map(|(c1, c2)| c1 + c2).collect::<Vec<_>>();
    let p2_sum = p2_low.iter().zip(p2_high.iter()).map(|(c1, c2)| c1 + c2).collect::<Vec<_>>();

    let (slave1, slave2, slave3) = (slaves.pop(), slaves.pop(), slaves.pop());
    let (slaves1, slaves2, slaves3) = split_slaves(&slaves);

    let mut low_product = match slave1 {
        Some(slave1) => {
            let slave = state.world.process_at_rank(slave1);
            slave.send(p1_low);
            slave.send(p2_low);
            slave.send(slaves1);
            vec![]
        }
        None => multiply(p1_low, p2_low),
    };

    let mut high_product = match slave2 {
        Some(slave2) => {
            let slave = state.world.process_at_rank(slave2);
            slave.send(p1_high);
            slave.send(p2_high);
            slave.send(slaves2);
            vec![]
        }
        None => multiply(p1_high, p2_high),
    };

    let mut sum_product = match slave3 {
        Some(slave3) => {
            let slave = state.world.process_at_rank(slave3);
            slave.send(&p1_sum);
            slave.send(&p2_sum);
            slave.send(slaves3);
            vec![]
        }
        None => multiply(&p1_sum, &p2_sum),
    };

    if let Some(slave1) = slave1 {
        (low_product, _) = state.world.process_at_rank(slave1).receive_vec::<i64>();
    }

    if let Some(slave2) = slave2 {
        (high_product, _) = state.world.process_at_rank(slave2).receive_vec::<i64>();
    }

    if let Some(slave3) = slave3 {
        (sum_product, _) = state.world.process_at_rank(slave3).receive_vec::<i64>();
    }

    let middle_product: Vec<i64> = sum_product
        .iter()
        .zip(low_product.iter())
        .zip(high_product.iter())
        .map(|((s, l), h)| s - l - h)
        .collect();

    let mut result = vec![0; p1.len() + p2.len() - 1];

    for i in 0..(p1.len() - 1) {
        result[i] += low_product[i];
        result[i + half_len] += middle_product[i];
        result[i + p1.len()] += high_product[i];
    }

    match parent {
        Some(parent) => state.world.process_at_rank(parent).send(&result),
        None => println!("Result: {:?}", result),
    }

    for slave in slaves {
        state.world.process_at_rank(slave).send::<[i64]>(&[]);
    }
}

fn multiply(p1: &[i64], p2: &[i64]) -> Vec<i64> {
    if p1.len() <= 32 {
        return simple_multiply(p1, p2);
    }

    let half_len = p1.len() / 2;
    let (p1_low, p1_high) = p1.split_at(half_len);
    let (p2_low, p2_high) = p2.split_at(half_len);

    let p1_sum = p1_low.iter().zip(p1_high.iter()).map(|(c1, c2)| c1 + c2).collect::<Vec<_>>();
    let p2_sum = p2_low.iter().zip(p2_high.iter()).map(|(c1, c2)| c1 + c2).collect::<Vec<_>>();

    let low_product = multiply(p1_low, p2_low);
    let high_product = multiply(p1_high, p2_high);
    let sum_product = multiply(&p1_sum, &p2_sum);

    let middle_product: Vec<i64> = sum_product
        .iter()
        .zip(low_product.iter())
        .zip(high_product.iter())
        .map(|((s, l), h)| s - l - h)
        .collect();

    let mut result = vec![0; p1.len() + p2.len() - 1];

    for i in 0..(p1.len() - 1) {
        result[i] += low_product[i];
        result[i + half_len] += middle_product[i];
        result[i + p1.len()] += high_product[i];
    }

    result
}

fn simple_multiply(p1: &[i64], p2: &[i64]) -> Vec<i64> {
    let mut result = vec![0_i64; p1.len() + p2.len() - 1];

    for (i, c1) in p1.iter().enumerate() {
        for (j, c2) in p2.iter().enumerate() {
            result[i + j] += c1 * c2;
        }
    }

    result
}

fn split_slaves(slaves: &[i32]) -> (&[i32], &[i32], &[i32]) {
    let chunk_size = slaves.len() / 3;

    (
        &slaves[..chunk_size],
        &slaves[chunk_size..(chunk_size + chunk_size)],
        &slaves[(chunk_size + chunk_size)..],
    )
}
