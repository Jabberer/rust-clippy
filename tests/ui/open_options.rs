#![feature(plugin)]
#![plugin(clippy)]
use std::fs::OpenOptions;

#[allow(unused_must_use)]
#[deny(nonsensical_open_options)]
fn main() {
    OpenOptions::new().read(true).truncate(true).open("foo.txt");
    OpenOptions::new().append(true).truncate(true).open("foo.txt");

    OpenOptions::new().read(true).read(false).open("foo.txt");
    OpenOptions::new().create(true).create(false).open("foo.txt");
    OpenOptions::new().write(true).write(false).open("foo.txt");
    OpenOptions::new().append(true).append(false).open("foo.txt");
    OpenOptions::new().truncate(true).truncate(false).open("foo.txt");
}
