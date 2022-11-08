use derive_more::Display;

mod config;
pub use config::*;

mod runtime;
pub use runtime::*;

mod standalone;
pub use standalone::*;

mod font;
pub use font::*;

#[derive(Display)]
pub enum Workspace {
    #[display(fmt = "_0")]
    Name(WorkspaceName),
    /// Also matches a workspace with the same number, even if it has a
    /// different name
    #[display(fmt = "number _0")]
    Number(WorkspaceName),
    /// Moves the focused container to the previous workspace on this output, or
    /// if no workspaces remain, the previous output
    #[display(fmt = "prev")]
    Prev,
    /// Moves the focused container to the next workspace on this output, or if
    /// no workspaces remain, the next output
    #[display(fmt = "next")]
    Next,
    /// Moves the focused container to the current workspace
    /// on this output
    #[display(fmt = "current")]
    Current,
    /// Moves the focused container to the previous workspace on this output,
    /// wrapping around if already at the first or last workspace
    #[display(fmt = "prev_on_output")]
    PrevOnOutput,
    /// Moves the focused container to the next workspace on this output,
    /// wrapping around if already at the first or last workspace
    #[display(fmt = "next_on_output")]
    NextOnOutput,
    /// Moves the focused container to previously focused workspace
    #[display(fmt = "back_and_forth")]
    BackAndForth,
}

#[derive(Display)]
pub enum WorkspaceName {
    Simple(String),
    #[display(fmt = "{_0}:{_1}")]
    WithNumber(u32, String),
}

#[derive(Display)]
pub enum Output {
    /// Next output in the specified direction
    #[display(fmt = "up")]
    Up,
    /// Next output in the specified direction
    #[display(fmt = "right")]
    Right,
    /// Next output in the specified direction
    #[display(fmt = "down")]
    Down,
    /// Next output in the specified direction
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "current")]
    Current,
    /// Named output
    Name(String),
}

#[derive(Display)]
pub enum GapsDirection {
    #[display(fmt = "inner")]
    Inner,
    #[display(fmt = "outer")]
    Outer,
    #[display(fmt = "horizontal")]
    Horizontal,
    #[display(fmt = "vertical")]
    Vertical,
    #[display(fmt = "top")]
    Top,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "bottom")]
    Bottom,
    #[display(fmt = "left")]
    Left,
}

#[derive(Display)]
pub enum YesNo {
    #[display(fmt = "yes")]
    Yes,
    #[display(fmt = "no")]
    No,
}

#[derive(Display)]
pub enum EnDisable {
    #[display(fmt = "enable")]
    Enable,
    #[display(fmt = "disable")]
    Disable,
}

#[derive(Display)]
pub enum EnDisTog {
    #[display(fmt = "enable")]
    Enable,
    #[display(fmt = "disable")]
    Disable,
    #[display(fmt = "toggle")]
    Toggle,
}

fn when(condition: bool, then: &str) -> &str {
    if condition {
        then
    } else {
        ""
    }
}
fn then_or_empty<T>(value: &Option<T>, then: fn(&T) -> String) -> String {
    value.as_ref().map(then).unwrap_or_default()
}

fn to_string_or_empty(value: &Option<impl ToString>) -> String {
    value.as_ref().map(ToString::to_string).unwrap_or_default()
}

fn separated(values: impl IntoIterator<Item = impl ToString>, seperator: impl ToString) -> String {
    values
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(&seperator.to_string())
}
