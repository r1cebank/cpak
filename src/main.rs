mod backup;
mod location;
mod resolver;

use crate::resolver::vscode::VsCode;
use crate::resolver::Resolver;

use clap::{App, Arg};
use std::fs;
use std::io::Read;

struct Test {
    test: Box<dyn Resolver>,
}

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
    let test = Test {
        test: Box::new(VsCode::new()),
    };
    let mut path = test.test.resolve_config().unwrap();
    let mut buf = String::new();
    path.files[2].1.read_to_string(&mut buf).unwrap();
    println!("{:?}", buf);
    println!("{:?}", matches);
}
