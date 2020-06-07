use std::borrow::Cow;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::actions::parse_call_args;
use super::actions::Context;

//#[derive(Clone)]
pub struct CommandAction<'s> {
    action: Action<'s>,
}

impl<'s> CommandAction<'s> {
    //pub fn new(action: Action<'s>, context: &'s dyn ReadLineContext) -> Self {
    pub fn new(action: Action<'s>) -> Self {
        Self {
            action,
            //context,
        }
    }

    //pub fn context(self) -> &'s dyn ReadLineContext {
    //    self.context
    //}

    pub fn action(&self) -> &Action<'s> {
        &self.action
    }
}

pub enum Action<'s> {
    SetContext(Cow<'s, str>, &'s dyn ReadLineContext<'s>),
    Failed(Cow<'s, str>),
    SideEffect(Cow<'s, str>),
    NoProgress(Cow<'s, str>),
    Usage(Cow<'s, str>),
    NotFound,
}

impl<'s> Action<'s> {
    pub fn message(&self) -> &str {
        match self {
            Self::SetContext(msg, _) => msg,
            Self::Failed(msg) => msg,
            Self::SideEffect(msg) => msg,
            Self::NoProgress(msg) => msg,
            Self::Usage(msg) => msg,
            Self::NotFound => "not found",
        }
    }

    pub fn context(&self) -> Option<&'s dyn ReadLineContext> {
        if let Self::SetContext(_, ctx) = self {
            Some(*ctx)
        } else {
            None
        }
    }
}

pub trait ReadLineContext<'s> {
    fn prompt(&self) -> &str;
    fn command(&'s mut self, cmd: &str, args: &[&str]) -> CommandAction<'s>;
}

fn parse_line(line: &str) -> Option<Vec<&str>> {
    let words: Vec<_> = line.split(char::is_whitespace).collect();
    if words[0].is_empty() {
        return None;
    }

    Some(words)
}

fn handle_line<'s>(ctx: &'s mut dyn ReadLineContext<'s>, command: &str, args: &[&str]) -> &'s dyn ReadLineContext<'s> {
    let ca = ctx.command(command, args);
    let ca_action = ca.action();
    match ca_action {
        Action::SetContext(msg, newctx) => {
            println!("New context: {}", msg);
        }
        Action::Failed(msg) => println!("Failed: {}", msg),
        Action::SideEffect(msg) => println!("Side effect: {}", msg),
        Action::NoProgress(msg) => println!("No progress: {}", msg),
        Action::Usage(msg) => println!("Usage: {}", msg),
        Action::NotFound => match (*command, args) {
            //("cd", ["..", ..]) => ctx = ctx.parent_mut(),
            _ => println!("Not found."),
        },
    };

    let ss = ca_action.context().unwrap_or(ctx);
    ss
}

pub fn repl(history: Option<&str>) {
    let mut rl = Editor::<()>::new();

    if let Some(file) = history {
        if rl.load_history(file).is_err() {
            println!("No previous history.")
        }
    }

    let mut root = super::actions::root::Root::new();
    //let mut ctx = Context::new().expect("can't create context");
    let rootctx = super::actions::root::RootCtx::new(&mut root);
    let mut ctx: &dyn ReadLineContext = &rootctx;
    //let mut ctx = &mut rootctx;

    ctx.command("dx", &[]);

    loop {
        let mut prompt = ctx.prompt().to_string();
        prompt.push_str(">>");
        let readline = rl.readline(prompt.as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Some(words) = parse_line(line.as_str()) {
                    let cc = handle_line(ctx, words[0], &words[1..]);
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
        }
    }

    if let Some(file) = history {
        rl.save_history(file).expect("failed to save history file")
    }
}
