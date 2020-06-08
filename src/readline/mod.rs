use std::borrow::Cow;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::actions::parse_call_args;
use super::actions::Context;

pub enum NextContext {
    Unchanged,
    Pop(Option<Box<dyn ReadLineContext>>),
    Push(Box<dyn ReadLineContext>),
}

pub trait ReadLineContext {
    fn prompt(&self) -> &str;
    fn command(&mut self, cmd: &str, args: &[&str]) -> NextContext;
}

fn parse_line(line: &str) -> Option<Vec<&str>> {
    let words: Vec<_> = line.split(char::is_whitespace).collect();
    if words[0].is_empty() {
        return None;
    }

    Some(words)
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

    let mut stack: Vec<Box<dyn ReadLineContext>> = vec![];

    loop {
        let prompt = ctx.prompt().to_string();
        //prompt.push_str(">>");
        let readline = rl.readline(prompt.as_str());
        let next_context = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Some(words) = parse_line(line.as_str()) {
                    let (command, args) = words.split_first().unwrap();
                    ctx.command(command, args)
                } else {
                    NextContext::Unchanged
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
        ctx = match next_context {
            NextContext::Push(rlc) => {
                stack.push(ctx);
                rlc
            }
            NextContext::Pop(Some(rlc)) => {
                let _ = stack.pop();
                rlc
            }
            NextContext::Pop(None) => stack.pop().unwrap_or(ctx),
            _ => ctx,
        };
    }

    if let Some(file) = history {
        rl.save_history(file).expect("failed to save history file")
    }
}
