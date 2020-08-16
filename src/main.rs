use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "stringtool")]
enum StringTool {
    /// Split source string into sub strings by given delimiter
    #[structopt(name = "split")]
    Split {
        /// The source string which to be splitted
        source: String,
        /// The delimiter which to be used in splitting
        #[structopt(short = "b", long = "by", default_value = ",")]
        by: String,
    },
    /// Trim source string by given direction
    #[structopt(name = "trim")]
    Trim {
        /// The source string which to be trimed
        source: String,
        #[structopt(subcommand)]
        direction: Direction,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "direction")]
enum Direction {
    /// Trim source string from left
    #[structopt(name = "left")]
    Left,
    /// Trim source string from right
    #[structopt(name = "right")]
    Right,
    /// Trim source string from both side
    #[structopt(name = "both")]
    Both,
}

impl fmt::Display for StringTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringTool::Split { source, by } => write!(f, "Split \"{}\" by \"{}\" ...", source, by),
            StringTool::Trim { source, direction } => {
                write!(f, "Trim \"{}\" on \"{}\" ...", direction, source)
            }
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Both => write!(f, "both"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

fn main() {
    let options = StringTool::from_args();
    println!("{}", options);
    match options {
        StringTool::Split { source, by } => split(source, by),
        StringTool::Trim { source, direction } => trim(source, direction),
    }
}

fn split(source: String, by: String) {
    let v: Vec<String> = source
        .split(by.as_str())
        .map(|str| String::from(str))
        .collect();
    let v_iter = v.iter();
    for elem in v_iter {
        println!("{:?}", elem);
    }
}

fn trim(source: String, direction: Direction) {
    match direction {
        Direction::Both => println!("{:?}", source.trim()),
        Direction::Left => println!("{:?}", source.trim_start()),
        Direction::Right => println!("{:?}", source.trim_end()),
    }
}
