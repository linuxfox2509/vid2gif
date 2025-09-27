use clap::{Arg, ArgAction, Command};
use std::path::Path;
use std::process::{Command as ProcessCommand, Stdio};

fn main() {
    let matches = Command::new("vid2gif")
        .version("1.2.0")
        .author("Your Name")
        .about("Simple video to GIF converter using ffmpeg")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .value_name("FILE")
                .help("Input video file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output GIF file (default: same as input with .gif extension)"),
        )
        .arg(
            Arg::new("optimize")
                .long("optimize")
                .action(ArgAction::SetTrue) // flag, no value
                .help("Use high-quality palette-based GIF conversion"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = match matches.get_one::<String>("output") {
        Some(o) => o.to_string(),
        None => {
            let path = Path::new(input_file);
            let stem = path.file_stem().unwrap().to_string_lossy();
            format!("{}.gif", stem)
        }
    };

    // Check ffmpeg
    let ffmpeg_check = ProcessCommand::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if ffmpeg_check.is_err() || !ffmpeg_check.unwrap().success() {
        eprintln!(
            "❌ Error: ffmpeg is not installed or not in PATH.\n\
            Please install it from https://ffmpeg.org/download.html"
        );
        std::process::exit(1);
    }

    println!("Converting '{}' -> '{}'", input_file, output_file);

    if matches.get_flag("optimize") {
        // High-quality palette method
        let palette_file = "palette.png";

        let status = ProcessCommand::new("ffmpeg")
            .args([
                "-i",
                input_file,
                "-vf",
                "fps=15,scale=800:-1:flags=lanczos,palettegen",
                palette_file,
            ])
            .status()
            .expect("Failed to generate palette");

        if !status.success() {
            eprintln!("❌ Failed to generate palette.");
            std::process::exit(1);
        }

        let status = ProcessCommand::new("ffmpeg")
            .args([
                "-i",
                input_file,
                "-i",
                palette_file,
                "-lavfi",
                "fps=15,scale=800:-1:flags=lanczos[x];[x][1:v]paletteuse",
                &output_file,
            ])
            .status()
            .expect("Failed to convert video to GIF with palette");

        if status.success() {
            println!("✅ Successfully created high-quality GIF '{}'", output_file);
        } else {
            eprintln!("❌ ffmpeg failed to convert video to GIF.");
        }
    } else {
        // Simple conversion
        let args = [
            "-i",
            input_file,
            "-vf",
            "fps=10,scale=480:-1:flags=lanczos",
            "-loop",
            "0",
            &output_file,
        ];

        let status = ProcessCommand::new("ffmpeg")
            .args(&args)
            .status()
            .expect("Failed to start ffmpeg process");

        if status.success() {
            println!("✅ Successfully created '{}'", output_file);
        } else {
            eprintln!("❌ ffmpeg failed to convert the video.");
        }
    }
}
