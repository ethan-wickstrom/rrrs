mod args;
mod library;

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let mut cli_args = args::CliArgs::new();

    // Optionally, prompt the user for a sample size if it wasn't provided.
    if cli_args.sample_size.is_none() {
        cli_args.prompt_for_sample_size();
    }
    
    let sample_size = cli_args.sample_size.unwrap();
    
    // Create a progress bar
    let pb = ProgressBar::new(cli_args.sample_size.unwrap() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Start the timer
    let start = Instant::now();

    if let Err(e) = library::perform_random_sampling(&cli_args.input_file, &cli_args.output_dir, sample_size).await {
        eprintln!("Error occurred: {}", e);
    } else {
        println!("Sampled data has been successfully written.");
    }

    // Finish the progress bar
    pb.finish_with_message("Sampling completed");

    // Print the elapsed time
    println!("Elapsed time: {:.2?}", start.elapsed());
}