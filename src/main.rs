use std::env;
use std::env::Args;
use std::fs::{read, write};

mod embedment;
mod disembedment;

const EMBED_MODE_VALUE: &'static str = "embed";
const DISEMBED_MODE_VALUE: &'static str = "disembed";

fn main() {
    let mut args = env::args();
    args.next();

    let embedment = match args.next().as_deref() {
        Some(EMBED_MODE_VALUE) => true,
        Some(DISEMBED_MODE_VALUE) => false,
        None => panic!("No application mode provided"),
        _ => panic!("Invalid mode provided (use either 'embed' or 'disembed' mode)"),
    };

    if embedment { handle_embedment(&mut args); } else { handle_disembedment(&mut args); }
}

fn handle_embedment(mut args: &mut Args) {
    let img_path = get_img_path(&mut args);
    let img = image::open(img_path).unwrap();

    let msg_path = get_msg_path(&mut args);
    let msg = read(msg_path).unwrap();

    let output_path = get_output_path(&mut args);
    let output_buf = embedment::embed(&img, &msg);

    output_buf.save(output_path).unwrap();
}

fn handle_disembedment(mut args: &mut Args) {
    let img_path = get_img_path(&mut args);
    let img = image::open(img_path).unwrap();

    let msg_path = get_msg_path(&mut args);
    let msg = disembedment::disembed(&img);

    write(msg_path, msg).unwrap();
}

fn get_img_path(args: &mut Args) -> String {
    match args.next() {
        Some(s) => s,
        None => panic!("No image path provided")
    }
}

fn get_msg_path(args: &mut Args) -> String {
    match args.next() {
        Some(s) => s,
        None => panic!("No message path provided")
    }
}

fn get_output_path(args: &mut Args) -> String {
    match args.next() {
        Some(s) => s,
        None => panic!("No output path provided")
    }
}
