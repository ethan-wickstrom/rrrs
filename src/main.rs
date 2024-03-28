#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;
mod library;

use std::time::Instant;

#[tokio::main]
async fn main() {
    let mut cli_args = args::CliArgs::new();

    // Optionally, prompt the user for a sample size if it wasn't provided.
    if cli_args.sample_size.is_none() {
        cli_args.prompt_for_sample_size();
    }
    
    let sample_size = cli_args.sample_size.unwrap();

    // Start the timer
    let start = Instant::now();

    if let Err(e) = library::perform_random_sampling(&cli_args.input_file, &cli_args.output_file, sample_size).await {
        eprintln!("Error occurred: {}", e);
    } else {
        println!("Sampled data has been successfully written.");
    }
    
    // Print the elapsed time
    println!("Elapsed time: {:.2?}", start.elapsed());
}