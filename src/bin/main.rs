use bit_vec::BitVec;
use huffman::{huffman_decode, huffman_encode};
use std::{env, error::Error, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = Args::parse(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing the arguments: {}", err);
        process::exit(1);
    });
    match args.command {
        Command::Encode => encode_file(args).unwrap_or_else(|err| {
            eprintln!("Error encoding the file {}", err);
            process::exit(1);
        }),
        Command::Decode => decode_file(args).unwrap_or_else(|err| {
            eprintln!("Error decoding the file {}", err);
            process::exit(1);
        }),
    };
}

fn encode_file(args: Args) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(args.input.as_ref())?;
    let bits = huffman_encode(&text[..]);
    if let Some(path) = args.output {
        fs::write(path.as_ref(), bits.to_bytes())?;
    } else {
        fs::write(args.input.with_extension("huf"), bits.to_bytes())?;
    }
    Ok(())
}

fn decode_file(args: Args) -> Result<(), Box<dyn Error>> {
    let bytes = fs::read(args.input.as_ref())?;
    let bits = BitVec::from_bytes(&bytes);
    let decoded_text = huffman_decode(bits);
    if let Some(output) = args.output {
        fs::write(output.as_ref(), decoded_text)?;
    } else {
        fs::write(args.input.with_extension("txt"), decoded_text)?;
    }
    Ok(())
}

enum Command {
    Encode,
    Decode,
}

struct Args<'a> {
    command: Command,
    input: Box<&'a Path>,
    output: Option<Box<&'a Path>>,
}

impl<'a> Args<'a> {
    fn parse(args: &'a [String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Please give at least 2 arguments.");
        }

        let command = if args[1] == "encode" {
            Command::Encode
        } else if args[1] == "decode" {
            Command::Decode
        } else {
            return Err("Command needs to be either 'encode' or 'decode'.");
        };

        let input = Box::new(Path::new(&args[2]));

        let output = args
            .get(3)
            .map_or(None, |arg| Some(Box::new(Path::new(arg))));

        Ok(Self {
            command,
            input,
            output,
        })
    }
}
