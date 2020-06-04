use std::borrow::Cow;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::actions::parse_call_args;
use super::actions::Context;

pub enum CommandAction<'s, T> {
    SetContext(Cow<'s, str>, T),
    Failed(Cow<'s, str>),
    SideEffect(Cow<'s, str>),
    NoProgress(Cow<'s, str>),
    Usage(Cow<'s, str>),
    NotFound,
}

pub trait ReadLineContext {
    fn prompt(&self) -> &str;
    fn command(&mut self, cmd: &str, args: &[&str]) -> CommandAction<'_, Box<dyn ReadLineContext>>;
    fn set_parent(&mut self, parent: &dyn ReadLineContext);
    fn parent_mut(&self) -> &mut dyn ReadLineContext;
}

fn handle_line(mut ctx: Box<dyn ReadLineContext>, line: String) -> Box<dyn ReadLineContext> {
    let words: Vec<_> = line.splitn(2, char::is_whitespace).collect();
    let command = words.split_first();
    if command.is_none() {
        return ctx;
    }
    let (command, args)= command.unwrap();
    if command.is_empty() {
        return ctx;
    }

    let mut result = None;

    match ctx.command(command, args) {
        CommandAction::SetContext(msg, new_ctx) => {
            println!("New context: {}", msg);
            result = Some(new_ctx);
        }
        CommandAction::Failed(msg) => println!("Failed: {}", msg),
        CommandAction::SideEffect(msg) => println!("Side effect: {}", msg),
        CommandAction::NoProgress(msg) => println!("No progress: {}", msg),
        CommandAction::Usage(msg) => println!("Usage: {}", msg),
        CommandAction::NotFound => match (*command, args) {
            //("cd", ["..", ..]) => ctx = ctx.parent_mut(),
            _ => println!("Not found."),
        }
    };

    return result.unwrap_or(ctx);
}

pub fn repl(history: Option<&str>) {
    let mut rl = Editor::<()>::new();

    if let Some(file) = history {
        if rl.load_history(file).is_err() {
            println!("No previous history.")
        }
    }

    //let mut ctx = Context::new().expect("can't create context");
    let mut rootctx = Box::new(super::actions::root::RootContext::new());
    let mut ctx: Box<dyn ReadLineContext> = rootctx;

    loop {
        let mut prompt = ctx.prompt().to_string();
        prompt.push_str(">>");
        let readline = rl.readline(prompt.as_str());
        ctx = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                handle_line(ctx, line)
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted. Please close stdin (typically C-d) to exit.");
                ctx
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Unhandled error: {:#?}", err);
                break;
            }
        }
    }

    if let Some(file) = history {
        rl.save_history(file).expect("failed to save history file")
    }
}