use clap::{arg, Args};
use rand::Rng;
use owo_colors::OwoColorize;

use crate::ColorGenerationError;

#[derive(Args)]
pub struct RandomOptions {
    #[arg(short, long)]
    maxed: Option<String>,
    #[arg(short, long)]
    gray: bool,
}

pub fn generate(options: &RandomOptions) -> Result<(), ColorGenerationError>{

    let RandomOptions{
        maxed,
        gray
    } = options;
    
    let mut rng = rand::thread_rng();

    let mut color = if *gray {
        let scale = rng.gen_range(0..255);
        owo_colors::Rgb(
            scale,
            scale,
            scale,
        )
    } else {
        owo_colors::Rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    };

    if let Some(maxed) = maxed{
        if maxed.contains("r") { color.0 = 255 }
        if maxed.contains("g") { color.1 = 255 }
        if maxed.contains("b") { color.2 = 255 }
    }

    let debug_str = "    ";
    print!(
        "{} #{:x}{:x}{:x}",
        debug_str.on_color(color),
        color.0,
        color.1,
        color.2
    );
    Ok(())
}