use clap::{Arg, Command};

/// Represents the command-line arguments passed to the application.
/// The input_file and output_file fields are required, while the sample_size field is optional.
/// If the sample_size is not provided, the user will be prompted to enter it.
pub struct CliArgs {
    pub input_file: String,
    pub output_dir: String,
    pub sample_size: Option<usize>, // Optional argument for sample size
}

impl CliArgs {
    /// Parses command-line arguments using clap and returns a CliArgs instance.
    pub fn new() -> CliArgs {
        let matches = Command::new("RRRS")
            .version("0.1.0")
            .author("Ethan Wickstrom")
            .about("Rust(ic) Rapid Random Sampler")
            .arg(Arg::new("INPUT")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input CSV file to use")
                .required(true)
            )
            .arg(Arg::new("OUTPUT")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to use for the sampled data")
                .required(true)
            )
            .arg(Arg::new("SAMPLE_SIZE")
                .short('n')
                .long("sample-size")
                .value_name("NUMBER")
                .help("Specifies the number of rows to sample from the input file")
                .required(false)
            )
            .get_matches();

        CliArgs {
            input_file: matches.get_one::<String>("INPUT").unwrap().to_string(),
            output_dir: matches.get_one::<String>("OUTPUT").unwrap().to_string(),
            sample_size: matches.get_one::<String>("SAMPLE_SIZE").map(|s| s.parse::<usize>().unwrap()),
        }
    }

    /// Prompts the user to enter the sample size if it was not provided as a command-line argument.
    pub fn prompt_for_sample_size(&mut self) {
        if self.sample_size.is_none() {
            let sample_size = loop {
                println!("\nPlease enter the number of rows to sample from the input file:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read input");
                match input.trim().parse::<usize>() {
                    Ok(n) => break n,
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            };
            self.sample_size = Some(sample_size);
        }
    }
}