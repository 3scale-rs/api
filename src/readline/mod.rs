use std::borrow::Cow;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::actions::parse_call_args;
use super::actions::Context;

pub enum NextContext {
    Unchanged,
    Parent,
    New(Box<dyn ReadLineContext>),
}

pub trait ReadLineContext {
    fn prompt(&self) -> &str;
    //fn command(&self, cmd: &str, args: &[&str]) -> Option<Box<dyn ReadLineContext>>;
    fn command(&mut self, cmd: &str, args: &[&str]) -> NextContext;
}

fn parse_line(line: &str) -> Option<Vec<&str>> {
    let words: Vec<_> = line.split(char::is_whitespace).collect();
    if words[0].is_empty() {
        return None;
    }

    Some(words)
}

fn handle_line<'a, 'b>(
    ctx: &'a mut dyn ReadLineContext,
    command: &str,
    args: &[&str],
) -> Option<&'b mut dyn ReadLineContext> {
    let ca = ctx.command(command, args);
    match ca {
        NextContext::New(b) => Some(Box::leak(b)),
        _ => None,
    }
}

pub fn repl(history: Option<&str>) {
    let mut rl = Editor::<()>::new();

    if let Some(file) = history {
        if rl.load_history(file).is_err() {
            println!("No previous history.")
        }
    }

    let mut root = super::actions::root::Root::new();
    let rootctx = super::actions::root::RootCtx::new(&mut root);
    let mut ctx: Box<dyn ReadLineContext> = Box::new(rootctx);

    loop {
        let prompt = ctx.prompt().to_string();
        //prompt.push_str(">>");
        let readline = rl.readline(prompt.as_str());
        let next = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Some(words) = parse_line(line.as_str()) {
                    let (command, args) = words.split_first().unwrap();
                    match ctx.command(command, args) {
                        NextContext::New(rlc) => Some(rlc),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted. Please close stdin (typically C-d) to exit.");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Unhandled error: {:#?}", err);
                break;
            }
        };
        ctx = next.unwrap_or(ctx);
    }

    if let Some(file) = history {
        rl.save_history(file).expect("failed to save history file")
    }
}
