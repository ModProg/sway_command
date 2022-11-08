use commands::SubCommand;
use criteria::{Criteria, CriteriaList};
use derive_more::{AsRef, Display};

pub mod commands;
pub mod criteria;

// TODO make AsRef a feature (maybe)
// Without it you'd just call `.to_string()`
// AsRef necessitates updating the `rep` string on every change... might be not
// ideal performance wise, but maybe also doesn't matter as long as you don't
// add criteria after the fact
#[derive(AsRef)]
pub struct CommandList {
    // To be able to implement `AsRef<str>`
    #[as_ref(forward)]
    rep: String,
    commands: Vec<Command>,
}

impl CommandList {
    pub fn get_commands(&self) -> &[Command] {
        &self.commands
    }
    pub fn command(&mut self, command: Command) -> &mut Self {
        self.rep.push(';');
        self.rep.push_str(command.as_ref());
        self.commands.push(command);
        self
    }
}

// TODO https://github.com/JelteF/derive_more/issues/219
// #[derive(AsRef)]
#[derive(Display)]
pub enum Command {
    // #[as_ref(forward)]
    CriteriaCommand(CriteriaCommand),
    // CriterialessCommand,
    Raw(String),
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match self {
            Command::CriteriaCommand(cmd) => cmd.as_ref(),
            Command::Raw(cmd) => cmd.as_ref(),
        }
    }
}

#[derive(AsRef, Display)]
#[display(fmt = "{rep}")]
pub struct CriteriaCommand {
    // To be able to implement `AsRef<str>`
    #[as_ref(forward)]
    rep: String,
    criteria: Option<CriteriaList>,
    commands: Vec<SubCommand>,
}

impl CriteriaCommand {
    pub fn get_commands(&self) -> &[SubCommand] {
        &self.commands
    }
    pub fn command(&mut self, command: SubCommand) -> &mut Self {
        if !self.commands.is_empty() {
            self.rep.push(',');
        }
        self.rep.push_str(&command.to_string());
        self.commands.push(command);
        self
    }
    /// Preformance note:
    ///
    /// When adding criteria after adding the first commands, the string
    /// representation needs to be rebuild
    pub fn criteria(&mut self, criteria: Criteria) -> &mut Self {
        if self.commands.is_empty() && self.criteria.is_some() {
            let Some(criterias) = &mut self.criteria else { unreachable!() };
            criterias.criteria(criteria);
            // TODO investigate if this could be replaced with `self.rep =
            // criterias.to_string()`
            assert_eq!(self.rep.pop(), Some(']'));
            self.rep.push_str(" {criteria}]")
        } else {
            if let Some(criterias) = &mut self.criteria {
                criterias.criteria(criteria);
                self.rep = String::with_capacity(self.rep.len());
                self.rep.push_str(criterias.as_ref());
            } else {
                self.criteria = Some(CriteriaList::new(criteria));
                self.rep = self.criteria.as_ref().unwrap().to_string();
            }
            // TODO no need to rebuild, just copy the original string here, just need to
            // remember where the commands start.
            if !self.commands.is_empty() {
                self.rep.push_str(&self.commands[0].to_string());
                for command in &self.commands[1..] {
                    self.rep.push(',');
                    self.rep.push_str(&command.to_string());
                }
            }
        }
        self
    }
}
