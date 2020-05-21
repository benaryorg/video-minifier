use std::path::Path;
use std::process::Command;

fn main() -> std::io::Result<()>
{
	let matches =
		clap::App::new("video-minifier")
			.version("0.1.0")
			.arg
				( clap::Arg::with_name("delete")
				.short("d")
				.long("delete")
				.help("delete input")
				.takes_value(false)
				)
			.arg
				( clap::Arg::with_name("INPUT")
				.required(true)
				.index(1)
				.help("input directory")
				)
			.arg
				( clap::Arg::with_name("OUTPUT")
				.required(false)
				.index(2)
				.help("output directory")
				)
			.get_matches()
		;

	let delete = matches.is_present("delete");
	let input_dir = matches.value_of("INPUT").expect("no input");
	let output_dir = matches.value_of("OUTPUT").unwrap_or(".");
	let output_dir = Path::new(output_dir);
	assert!(output_dir.is_dir());
	for file in std::fs::read_dir(input_dir)?
	{
		let source = file?.path();
		if !source.is_file() { continue; }
		match source.extension()
		{
			Some(ext) if ext == "mp4" => {},
			_ => continue,
		};
		let target = match source.with_extension("x265.mkv").file_name()
		{
			Some(filename) => output_dir.join(filename),
			_ => continue,
		};
		if target.exists() { continue; }
		println!("{} => {}",source.display(),target.display());
		let status = Command::new("ffmpeg.exe")
			.args(&["-hide_banner","-loglevel","error","-nostats","-i",])
			.arg(&source)
			.args(&["-c:v","libx265","-x265-params","log-level=error:lossless=1","-c:a","copy",])
			.arg(&target)
			.status()?;
		if !status.success()
		{
			break;
		}
		if delete
		{
			std::fs::remove_file(&source)?;
		}
	}

	Ok(())
}

