use derive_more::{AsRef, Display};
#[cfg(feature = "serde")]
use serde::Deserialize;

#[derive(AsRef, Display, Clone)]
#[display(fmt = "{rep}")]
pub struct CriteriaList {
    #[as_ref(forward)]
    rep: String,
    criteria: Vec<Criteria>,
}

impl CriteriaList {
    pub fn get_criteria(&self) -> &[Criteria] {
        &self.criteria
    }
    pub fn criteria(&mut self, criteria: Criteria) -> &mut Self {
        assert_eq!(self.rep.pop(), Some(']'));
        self.rep.push_str(" {criteria}]");
        self.criteria.push(criteria);
        self
    }

    pub fn new(criteria: Criteria) -> CriteriaList {
        Self {
            rep: format!("[{criteria}]"),
            criteria: vec![criteria],
        }
    }
}

#[derive(Display, Clone)]
pub enum Criteria {
    /// Compare value against the app id. Can be a regular expression. If value
    /// is __focused__, then the app id must be the same as that of the
    /// currently focused window. app_id are specific to Wayland applications.
    #[display(fmt = "app_id=\"{}\"", "_0")]
    AppId(OrFocused<String>),

    /// Compare value against the window class. Can be a regular expression. If
    /// value is __focused__, then the window class must be the same as that
    /// of the currently focused window. class are specific to X11 applications.
    #[display(fmt = "class=\"{}\"", "_0")]
    Class(OrFocused<String>),

    /// Compare against the internal container ID, which you can find via IPC.
    /// If value is __focused__, then the id must be the same as that of the
    /// currently focused window.
    #[display(fmt = "con_id=\"{}\"", "_0")]
    ConId(OrFocused<u32>),

    /// Compare against the window marks. Can be a regular expression.
    #[display(fmt = "con_mark=\"{}\"", "_0")]
    ConMark(String),

    /// Matches floating windows.
    #[display(fmt = "floating")]
    Floating,

    /// Compare value against the X11 window ID. Must be numeric.
    #[display(fmt = "id=\"{}\"", "_0")]
    Id(u32),

    /// Compare value against the window instance. Can be a regular expression.
    /// If value is __focused__, then the window instance must be the same
    /// as that of the currently focused window.
    #[display(fmt = "instance=\"{}\"", "_0")]
    Instance(OrFocused<String>),

    /// Compare value against the window's process ID. Must be numeric.
    #[display(fmt = "Pid=\"{}\"", "_0")]
    Pid(u32),

    /// Compare value against the window shell, such as "xdg_shell" or
    /// "xwayland".  Can be a regular expression. If value is __focused__, then
    /// the shell must be the same as that of the currently focused window.
    #[display(fmt = "shell=\"{}\"", "_0")]
    Shell(OrFocused<String>),

    /// Matches tiling windows.
    #[display(fmt = "tiling")]
    Tiling,

    /// Compare against the window title. Can be a regular expression. If value
    /// is __focused__, then the window title must be the same as that of
    /// the currently focused window.
    #[display(fmt = "title=\"{}\"", "_0")]
    Title(OrFocused<String>),

    /// Compares the urgent state of the window. Can be "first", "last",
    /// "latest", "newest", "oldest" or "recent".
    // TODO make enum
    #[display(fmt = "urgent=\"{}\"", "_0")]
    Urgent(Urgent),

    /// Compare against the window role (WM_WINDOW_ROLE). Can be a regular
    /// expression. If value is __focused__, then the window role must be the
    /// same as that of the currently focused window.
    #[display(fmt = "window_role=\"{}\"", "_0")]
    WindowRole(OrFocused<String>),

    /// Compare against the window type (_NET_WM_WINDOW_TYPE). Possible values
    /// are normal, dialog, utility, toolbar, splash, menu, dropdown_menu,
    /// popup_menu, tooltip and notification.
    // TODO make enum
    #[display(fmt = "window_type=\"{}\"", "_0")]
    WindowType(WindowType),

    /// Compare against the workspace name for this view. Can be a regular
    /// expression. If the value is __focused__, then all the views on the
    /// currently focused workspace matches.
    #[display(fmt = "workspace=\"{}\"", "_0")]
    Workspace(OrFocused<String>),
}

#[derive(Display, Debug, Clone)]
pub enum OrFocused<T> {
    #[display(fmt = "__focused__")]
    Focused,
    #[display(fmt = "{}", "_0")]
    Value(T),
}

impl<T> From<T> for OrFocused<T> {
    fn from(t: T) -> Self {
        OrFocused::Value(t)
    }
}

#[derive(Display)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[serde(rename_all = "snake_case")]
#[derive(Clone)]
pub enum Urgent {
    #[display(fmt = "first")]
    First,
    #[display(fmt = "last")]
    Last,
    #[display(fmt = "latest")]
    Latest,
    #[display(fmt = "newest")]
    Newest,
    #[display(fmt = "oldest")]
    Oldest,
    #[display(fmt = "recent")]
    Recent,
}

#[derive(Display)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[serde(rename_all = "snake_case")]
#[derive(Clone)]
pub enum WindowType {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "dialog")]
    Dialog,
    #[display(fmt = "utility")]
    Utility,
    #[display(fmt = "toolbar")]
    Toolbar,
    #[display(fmt = "splash")]
    Splash,
    #[display(fmt = "menu")]
    Menu,
    #[display(fmt = "dropdown_menu")]
    DropdownMenu,
    #[display(fmt = "popup_menu")]
    PopupMenu,
    #[display(fmt = "tooltip")]
    Tooltip,
    #[display(fmt = "notification")]
    Notification,
}

#[test]
fn test() {
    assert_eq!("first", Urgent::First.to_string());
}
