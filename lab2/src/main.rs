use std::sync::{Condvar, Mutex};

fn main() {
    let v1 = (1..=100).map(f64::from).collect::<Vec<_>>();
    let v2 = (1..=100).map(f64::from).map(|n| n * n).collect::<Vec<_>>();

    assert_eq!(v1.len(), v2.len());
    let total_elements = v1.len();

    let state = Mutex::new(Option::<f64>::None);
    let some_condvar = Condvar::new();
    let none_condvar = Condvar::new();

    let mut sum = 0.0;

    std::thread::scope(|scope| {
        // Producer
        scope.spawn(|| {
            for (e1, e2) in v1.iter().zip(v2.iter()) {
                let mut state_guard = state.lock().unwrap();

                state_guard = none_condvar
                    .wait_while(state_guard, |state| state.is_some())
                    .unwrap();

                *state_guard = Some(e1 * e2);
                some_condvar.notify_one();
            }
        });

        // Consumer
        let mut processed_elements = 0;

        while processed_elements != total_elements {
            let mut state_guard = state.lock().unwrap();

            state_guard = some_condvar
                .wait_while(state_guard, |state| state.is_none())
                .unwrap();

            sum += state_guard.take().unwrap();
            processed_elements += 1;
            none_condvar.notify_one();
        }
    });

    println!("Sum is {}", sum);
}
