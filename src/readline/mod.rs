use rustyline::Editor;
use rustyline::error::ReadlineError;

use super::actions::Context;
use super::actions::parse_call_args;

//mod setup;

pub fn repl(history: Option<&str>) {
    let mut rl = Editor::<()>::new();

    if let Some(file) = history {
        if rl.load_history(file).is_err() {
            println!("No previous history.")
        }
    }

    let mut ctx = Context::new().expect("can't create context");

    loop {
        let readline = rl.readline(ctx.prompt().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let words: Vec<_> = line.splitn(2, char::is_whitespace).collect();
                let command = words.split_first();
                if command.is_none() {
                    continue
                }
                let command = command.unwrap();
                if command.0.is_empty() {
                    continue
                }
                match command {
                    (&"pb", _) => progress_bars(),
                    (&"host", args) if !args.is_empty() => {
                        let arg = *args.first().unwrap();
                        match ctx.set_host(arg) {
                            Ok(_) => println!("Ok."),
                            Err(e) => println!("Failed: {:?}", e),
                        }
                    },
                    (&"host", _) => println!("usage: host <3scale-system-host>"),
                    (&"token", args) if args.is_empty() => {
                        let token = ctx.straitjacket().token().unwrap_or("(not set)");
                        println!("{}", token);
                    },
                    (&"token", args) => {
                        let token = args.first().unwrap().to_string();
                        ctx.set_token(token)
                            .map_or_else(|e| println!("Failed: {:?}", e),
                                               |_| println!("Ok."));
                    },
                    (&"global", _) => ctx.set_host(None)
                        .map_or_else(|e| println!("Failed: {:?}", e),
                                           |_| println!("Ok.")),
                    (&"response", _) => println!("{:?}", ctx.straitjacket().response()),
                    (&"body", _) => {
                        // TODO FIXME this consumes the response or the body, so must find a way
                        // to reinstate the body for the next body command
                        let body = ctx.straitjacket_mut().fetch_body();
                        match body {
                            Ok(body) => {
                                use std::io::Write;
                                let mut out = std::io::stdout();
                                if let Err(e) = out.write_all(body) {
                                    println!("Failed to write_all: {:?}", e);
                                }
                                if let Err(e) = out.flush() {
                                    println!("Failed to flush: {:?}", e);
                                }
                            },
                            Err(e) => println!("Failed: {:?}", e),
                        }
                    },
                    (&"json", _) => {
                        let body = ctx.straitjacket_mut().fetch_body();
                        match body {
                            Ok(body) => {
                                match serde_json::from_slice::<serde_json::Value>(body) {
                                    Ok(v) => {
                                        serde_json::to_writer_pretty(std::io::stdout(), &v)
                                            .unwrap_or_else(|e| println!("Failed: {:?}", e));
                                    },
                                    Err(e) => println!("Failed: {:?}", e),
                                }
                            },
                            Err(e) => println!("Failed: {:?}", e),
                        }
                    },
                    (&"call", args) => {
                        if ctx.straitjacket().host_url().is_none() {
                            println!("Failed: set a host and token first");
                            continue
                        }
                        let parsed_call = parse_call_args(args);
                        if let Err(e) = parsed_call {
                            println!("Failed: {:?}", e);
                            continue
                        }
                        let (method, path, query_string, body) = parsed_call.unwrap();
                        let response = ctx.straitjacket().client()
                            .send(method, path, query_string, body);
                        match response {
                            Ok(response) => {
                                let status = response.status();
                                ctx.straitjacket_mut().set_response(response);
                                println!("Ok, response status {}.", status);
                            },
                            Err(e) => println!("Failed: {:?}", e),
                        }
                    },
                    (thing, _) => println!("Unknown command: {}", thing),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL+C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL+D");
                break
            },
            Err(err) => {
                println!("Error: {:#?}", err);
                break
            },
        }
    }

    if let Some(file) = history {
        rl.save_history(file).expect("failed to save history file")
    }
}

// example of progress bars below - integrate for long running requests?

use console::{style, Emoji};

fn progress_bars() {
    //use rand;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::thread;
    use std::time::{Duration, Instant};

    use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

    static PACKAGES: &'static [&'static str] = &[
        "fs-events",
        "my-awesome-module",
        "emoji-speaker",
        "wrap-ansi",
        "stream-browserify",
        "acorn-dynamic-import",
    ];

    static COMMANDS: &'static [&'static str] = &[
        "cmake .",
        "make",
        "make clean",
        "gcc foo.c -o foo",
        "gcc bar.c -o bar",
        "./helper.sh rebuild-cache",
        "make all-clean",
        "make test",
    ];

    static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
    static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "🚚  ");
    static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
    static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
    static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

    let mut rng = rand::thread_rng();
    let started = Instant::now();
    let spinner_style = ProgressStyle::default_spinner()
        //.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .tick_chars("🌍🌎🌏")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    println!(
        "{} {}Resolving packages...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );
    println!(
        "{} {}Fetching packages...",
        style("[2/4]").bold().dim(),
        TRUCK
    );

    println!(
        "{} {}Linking dependencies...",
        style("[3/4]").bold().dim(),
        CLIP
    );
    let deps = 1232;
    let pb = ProgressBar::new(deps);
    for _ in 0..deps {
        pb.inc(1);
        thread::sleep(Duration::from_millis(3));
    }
    pb.finish_and_clear();

    println!(
        "{} {}Building fresh packages...",
        style("[4/4]").bold().dim(),
        PAPER
    );
    let m = MultiProgress::new();
    for i in 0..4 {
        let count = rng.gen_range(30, 80);
        let pb = m.add(ProgressBar::new(count));
        pb.set_style(spinner_style.clone());
        pb.set_prefix(&format!("[{}/?]", i + 1));
        let _ = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let pkg = PACKAGES.choose(&mut rng).unwrap();
            for _ in 0..count {
                let cmd = COMMANDS.choose(&mut rng).unwrap();
                pb.set_message(&format!("{}: {}", pkg, cmd));
                pb.inc(1);
                thread::sleep(Duration::from_millis(rng.gen_range(25, 200)));
            }
            pb.finish_with_message("waiting...");
        });
    }
    m.join_and_clear().unwrap();

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}
