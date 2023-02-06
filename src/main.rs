use png_to_svg::{Config, convert_image_to_svg};

fn main() {
    let config = Config::from_args();
    let result = convert_image_to_svg(config);
    println!("Sanity Check");
    match result {
        Ok(()) => {
            println!("Conversion successful. :0");
        },
        Err(msg) => {
            panic!("Conversion failed with error message: {}", msg);
        }
    }
}