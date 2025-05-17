mod utils;

use clap::Parser;
use orx_fixed_vec::*;
use orx_parallel::*;
use rayon::iter::*;
use utils::timed_collect_all;

#[derive(Parser, Debug)]
struct Args {
    /// Number of items in the input iterator.
    #[arg(long, default_value_t = 100000)]
    len: usize,
    /// Number of repetitions to measure time; total time will be reported.
    #[arg(long, default_value_t = 100)]
    num_repetitions: usize,
}

fn main() {
    let args = Args::parse();

    let expected_output = {
        let fixed_vec: FixedVec<_> = (0..args.len as usize).collect();

        fixed_vec
            .iter()
            .map(|x| x.to_string())
            .filter_map(|x| (!x.starts_with('1')).then_some(x))
            .flat_map(|x| [format!("{}!", &x), x])
            .filter(|x| !x.starts_with('2'))
            .filter_map(|x| x.parse::<u64>().ok())
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    };

    let computations: Vec<(&str, Box<dyn Fn() -> Vec<String>>)> = vec![
        (
            "Sequential over Vec",
            Box::new(move || {
                let vec: Vec<_> = (0..args.len as usize).collect();

                vec.iter()
                    .map(|x| x.to_string())
                    .filter_map(|x| (!x.starts_with('1')).then_some(x))
                    .flat_map(|x| [format!("{}!", &x), x])
                    .filter(|x| !x.starts_with('2'))
                    .filter_map(|x| x.parse::<u64>().ok())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            }),
        ),
        (
            "Parallelized over Vec using rayon",
            Box::new(move || {
                let vec: Vec<_> = (0..args.len as usize).collect();
                vec.par_iter()
                    .map(|x| x.to_string())
                    .filter_map(|x| (!x.starts_with('1')).then_some(x))
                    .flat_map(|x| [format!("{}!", &x), x])
                    .filter(|x| !x.starts_with('2'))
                    .filter_map(|x| x.parse::<u64>().ok())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            }),
        ),
        (
            "Parallelized over Vec using orx_parallel",
            Box::new(move || {
                let vec: Vec<_> = (0..args.len as usize).collect();

                vec.par()
                    .map(|x| x.to_string())
                    .filter_map(|x| (!x.starts_with('1')).then_some(x))
                    .flat_map(|x| [format!("{}!", &x), x])
                    .filter(|x| !x.starts_with('2'))
                    .filter_map(|x| x.parse::<u64>().ok())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            }),
        ),
        (
            "Parallelized over FixedVec using orx_parallel",
            Box::new(move || {
                let fixed_vec: FixedVec<_> = (0..args.len as usize).collect();

                fixed_vec
                    .par() // replace iter (into_iter) with par (into_par) to parallelize !
                    .map(|x| x.to_string())
                    .filter_map(|x| (!x.starts_with('1')).then_some(x))
                    .flat_map(|x| [format!("{}!", &x), x])
                    .filter(|x| !x.starts_with('2'))
                    .filter_map(|x| x.parse::<u64>().ok())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            }),
        ),
    ];

    timed_collect_all(
        "benchmark_parallelization",
        args.num_repetitions,
        &expected_output,
        &computations,
    );
}
