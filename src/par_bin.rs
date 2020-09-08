use ayatori;
use std::io;

mod arg_matcher;

fn main() -> Result<(), io::Error> {
    let arg = arg_matcher::parse_arg();

    let graph = ayatori::par_run(
        arg.environment,
        arg.base_file_path,
        arg.topic_file_name,
        arg.subscription_file_name,
    )?;

    if &arg.output_format == &"json" {
        let json = serde_json::to_string(&graph)?;
        println!("{}", json);
    } else {
        if cfg!(debug_assertions) {
            dbg!(graph);
        } else {
            println!("{:#?}", graph);
        }
    }

    Ok(())
}
