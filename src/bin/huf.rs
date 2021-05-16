use clap::{crate_authors, crate_version, App, Arg, ArgMatches, SubCommand};
use huffman::{Decoder, Encoder};
use std::{
    path::{Path, PathBuf},
    process,
};

fn main() {
    let encode_command = SubCommand::with_name("encode")
        .about("Encodes UTF-8 encoded text into a compressed huffman format.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to encode")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Optionally set the output path. Uses the input file name with extension '.huf' by default.")
                .index(2)
        );
    let decode_command = SubCommand::with_name("decode")
        .about("Decodes a huffman encoded binary into a UTF-8 encoded text.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to decode")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Optionally set the output path. Uses the input file name with extension '.txt' by default.")
                .index(2)
        );
    let matches = App::new("Huffman compression")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(encode_command)
        .subcommand(decode_command)
        .get_matches();
    let verbose_level = matches.occurrences_of("verbose") as u8;
    match matches.subcommand() {
        ("encode", Some(args)) => encode(args, verbose_level),
        ("decode", Some(args)) => decode(args, verbose_level),
        _ => {}
    }
}

fn get_input_and_output(args: &ArgMatches, default_output_extension: &str) -> (PathBuf, PathBuf) {
    let input = Path::new(args.value_of("INPUT").unwrap_or_else(|| {
        eprintln!(
            "Input path was not provided. This should have been caught by args parser 'clap'"
        );
        process::exit(1);
    }))
    .to_path_buf();
    let output = if let Some(output_path) = args.value_of("OUTPUT") {
        Path::new(output_path).to_path_buf()
    } else {
        input.with_extension(default_output_extension)
    };
    (input, output)
}

fn encode(args: &ArgMatches, verbose_level: u8) {
    let (input, output) = get_input_and_output(args, "huf");
    let mut encoder = Encoder::from_file(&input, &output, verbose_level).unwrap_or_else(|err| {
        eprintln!("Error with files: {}", err);
        process::exit(1);
    });
    encoder.encode().unwrap_or_else(|err| {
        eprintln!("Error compressing the file: {}", err);
        process::exit(1);
    });
}

fn decode(args: &ArgMatches, verbose_level: u8) {
    let (input, output) = get_input_and_output(args, "txt");
    let decodeder = Decoder::from_file(input, output, verbose_level);
    decodeder.decode().unwrap_or_else(|err| {
        eprintln!("Error de-compressing the file: {}", err);
        process::exit(1);
    })
}
