
use structopt::StructOpt;
use csv_challenge::{
    Opt,
    {load_csv, write_csv},
    replace_column,
};
use std::path::PathBuf;
use std::process;

fn main() {
    let opt = Opt::from_args();
    let file_name = PathBuf::from(opt.input);
    let csv_date = match load_csv(file_name) {
        Ok(fname) => { fname },
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    };
    let modifed_date = match replace_column(csv_date, &opt.column_name, &opt.replacement) {
        Ok(data) => { data },
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    };
    let output_file = &opt.output;
    match write_csv(&modifed_date, output_file) {
        Ok(_) => {
            println!("write success!");
        },
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    }
}
