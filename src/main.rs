use clap::load_yaml;
use clap::App;
use pmrs::objects::ocel::validator::{validate_ocel, validate_ocel_verbose};


fn main() {
    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    if let Some(validate_matches) = matches.subcommand_matches("validate") {
        let input_file = validate_matches.value_of("INPUT").unwrap();

        if input_file.ends_with(".jsonocel") {
            if validate_matches.is_present("verbose") {
                match validate_ocel_verbose(input_file) {
                    Ok(v) => {
                        println!("{}: {}", input_file, v);
                    }
                    Err(e) => println!("There was an Error: {}", e),
                }
            } else {
                match validate_ocel(input_file) {
                    Ok(v) => {
                        println!("{}: {}", input_file, v);
                    }
                    Err(e) => println!("There was an Error: {}", e),
                }
            }
        } else {
            println!("Error: {} file format is not supported.", input_file);
        }
    }
}
