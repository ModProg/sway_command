#![warn(missing_docs)]
//! Implements a builder for swaymsg.
use std::vec;

use commands::{CriterialessCommand, SubCommand};
use criteria::{Criteria, CriteriaList};
use derive_more::{AsRef, Display, From};

/// Contains the types for command creation
pub mod commands;
/// Contains the types for criteria creation
pub mod criteria;

// TODO make AsRef a feature (maybe)
// Without it you'd just call `.to_string()`
// AsRef necessitates updating the `rep` string on every change... might be not
// ideal performance wise, but maybe also doesn't matter as long as you don't
// add criteria after the fact
/// Create a command list able to be run via sway ipc
#[derive(AsRef, Default)]
pub struct CommandList {
    // To be able to implement `AsRef<str>`
    #[as_ref(forward)]
    rep: String,
    commands: Vec<Command>,
}

#[doc(hidden)]
pub fn normalize_whitespace(value: impl AsRef<str>) -> String {
    value
        .as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
impl CommandList {
    /// Get the commands
    pub fn get_commands(&self) -> &[Command] {
        &self.commands
    }
    /// ```
    /// # use sway_command::*;
    /// # use sway_command::normalize_whitespace;
    /// # use sway_command::commands::*;
    /// # use sway_command::criteria::*;
    /// let cmd = CommandList::default()
    ///     .command("workspace 5")
    ///     .command(SubCommand::Border(Border::None))
    ///     .command(
    ///         CriteriaCommand::default()
    ///             .criteria(Criteria::Floating)
    ///             .command(SubCommand::Floating(EnDisTog::Disable)),
    ///     )
    ///     .command(CriterialessCommand::Bindsym(
    ///         Default::default(),
    ///         SymKey::key("a"),
    ///         SubCommand::Exit.into(),
    ///     ));
    /// let cmd: &str = cmd.as_ref();
    /// assert_eq!(
    ///     normalize_whitespace(cmd),
    ///     "workspace 5;border none;[floating]floating disable;bindsym a exit"
    /// );
    /// ```
    pub fn command(mut self, command: impl Into<Command>) -> Self {
        let command = command.into();
        if !self.commands.is_empty() {
            self.rep.push(';');
        }
        self.rep.push_str(command.to_string().as_ref());
        self.commands.push(command);
        self
    }
}

// TODO https://github.com/JelteF/derive_more/issues/219
// #[derive(AsRef)]
/// A Command that can be added to a [`CommandList`] or run directly
#[derive(Display, From)]
pub enum Command {
    // #[as_ref(forward)]
    /// A Command that contains criteria
    #[from(types(SubCommand))]
    Criteria(CriteriaCommand),
    /// A Command without Criteria
    #[from(types(CriterialessCommand))]
    Criterialess(Box<CriterialessCommand>),
    // #[from(types("&str"))]
    /// Untyped Command
    #[from(forward)]
    Raw(String),
}

#[derive(AsRef, Display, Default, Clone)]
#[display(fmt = "{rep}")]
/// A command with an optional Criteria
pub struct CriteriaCommand {
    // To be able to implement `AsRef<str>`
    #[as_ref(forward)]
    rep: String,
    criteria: Option<CriteriaList>,
    commands: Vec<SubCommand>,
}

impl From<SubCommand> for CriteriaCommand {
    fn from(cmd: SubCommand) -> Self {
        Self {
            rep: cmd.to_string(),
            commands: vec![cmd],
            criteria: Default::default(),
        }
    }
}

impl CriteriaCommand {
    /// Get the commands in CriteriaCommand
    pub fn get_commands(&self) -> &[SubCommand] {
        &self.commands
    }
    /// At a new command
    pub fn command(mut self, command: SubCommand) -> Self {
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
    pub fn criteria(mut self, criteria: Criteria) -> Self {
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
