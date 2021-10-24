use aws_sdk_s3::output::GetObjectOutput;
use aws_sdk_s3::error::GetObjectError;
use aws_sdk_s3::{Client, Error, SdkError};

use structopt::StructOpt;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The name of the bucket.
    #[structopt(short, long)]
    bucket: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    file: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Opt {
        bucket,
        file,
    } = Opt::from_args();

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(file_key) = line {
                let resp = client.get_object().bucket(&bucket).key(&file_key).send().await;
                print_result(file_key, resp)
            }
        }
    }


    Ok(())
}

fn print_result(file_key: String, resp: Result<GetObjectOutput, SdkError<GetObjectError>>){
    match resp {
        Ok(_b) => {
            println!("{},yes", file_key);
        }
        Err(e) => {
            match e {
                SdkError::ServiceError{raw: _, err: _} => {
                    println!("{},no", file_key);
                },
                _ => {
                    println!("{},no", e);
                },
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

