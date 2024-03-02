mod args;
mod library;

#[tokio::main]
async fn main() {
    let mut cli_args = args::CliArgs::new();

    // Optionally, prompt the user for a sample size if it wasn't provided.
    cli_args.prompt_for_sample_size();

    match library::perform_random_sampling(&cli_args.input_file, &cli_args.output_file, cli_args.sample_size).await {
        Ok(()) => println!("Sampled data has been written successfully."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
