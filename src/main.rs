use clap::Parser;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(
    name = "hot_reload",
    about = "Run and hot reload a Flutter app on file save"
)]
struct Args {
    /// Path to the Flutter project
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Debounce delay in milliseconds
    #[arg(short = 'b', long, default_value_t = 500)]
    debounce_ms: u64,

    /// Extra arguments to pass to flutter run (after --)
    #[arg(last = true)]
    extra: Vec<String>,
}

fn spawn_flutter_run(path: &str, extra: &[String]) -> std::io::Result<Child> {
    println!("🚀 Launching flutter in: {}", path);
    println!("🧩 With args: {:?}", extra);

    let mut cmd = Command::new("flutter");
    cmd.arg("run")
        .current_dir(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    cmd.args(extra);
    cmd.spawn()
}

fn main() -> notify::Result<()> {
    let args = Args::parse();
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;
    watcher.watch(Path::new(&args.path), RecursiveMode::Recursive)?;

    println!("🚀 Starting Flutter app and watching for changes...");
    let mut child =
        spawn_flutter_run(&args.path, &args.extra).expect("Failed to start flutter run");
    let stdin = child
        .stdin
        .as_mut()
        .expect("Failed to access flutter stdin");

    let mut last_reload = Instant::now();

    while let Ok(res) = rx.recv() {
        match res {
            Ok(event) => {
                let is_write = matches!(event.kind, EventKind::Modify(_))
                    && event
                        .paths
                        .iter()
                        .any(|p| p.extension().map(|ext| ext == "dart").unwrap_or(false));

                if is_write && last_reload.elapsed() > Duration::from_millis(args.debounce_ms) {
                    println!("♻️ Hot reloading...");
                    if let Err(e) = writeln!(stdin, "r") {
                        eprintln!("Failed to send reload: {}", e);
                    }
                    last_reload = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
            }
        }
    }

    Ok(())
}
