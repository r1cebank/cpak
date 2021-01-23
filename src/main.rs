use clap::{App, Arg};

fn main() {
    let matches = App::new("cpak")
        .author("Siyuan Gao <rbnk@elica.io>")
        .arg(
            Arg::with_name("auto")
                .short("a")
                .required(false)
                .takes_value(false),
        )
        .arg(Arg::with_name("dry-run").short("d").required(false))
        .get_matches();
    println!("{:?}", matches);
}
