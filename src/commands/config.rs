use derive_more::Display;

/// The following commands may only be used in the configuration file.
#[derive(Display)]
pub enum ConfigCommand {
    //  sway-output(5)
    // TODO sway-bar(5)
    // TODO quote string containing commands
    /// For details on bar subcommands, see sway-bar(5).
    #[display(
        fmt = "bar {} {}",
        "_0.as_deref().unwrap_or_default()",
        "_1.join(\" \")"
    )]
    Bar(Option<String>, Vec<String>),
    /// Sets the default container layout for tiled containers.
    #[display(fmt = "default_orientation {}", "_0")]
    DefaultOrientation(DefaultOrientation),
    /// Includes another file from path. path can be either a full path or a
    /// path relative to the parent config, and expands shell syntax (see
    /// wordexp(3) for details). The same include file can only be included
    /// once; subsequent attempts will be ignored.
    #[display(fmt = "include {}", _0)]
    Include(String),
    /// Executes custom background command. Default is swaybg. Refer to
    /// swayoutput(5) for more information.
    ///
    /// It can be disabled by setting the command to a single dash:
    /// swaybg_command -
    #[display(fmt = "swaybg_command {}", _0)]
    SwaybgCommand(String),
    /// Executes custom command for swaynag. Default is swaynag. Additional
    /// arguments may be appended to the end. This should only be used to
    /// either direct sway to call swaynag from a custom path or to
    /// provide additional arguments. This should be placed at the
    /// top of the config for the best results.
    ///
    /// It can be disabled by setting the command to a single dash:
    /// swaynag_command -
    #[display(fmt = "swaynag_command {}", _0)]
    SwaynagCommand(String),
    /// Specifies the initial layout for new containers in an empty
    /// workspace.
    #[display(fmt = "workspace_layout {}", _0)]
    WorkspaceLayout(WorkspaceLayout),
    /// Enables or disables Xwayland support, which allows X11 applications
    /// to be used. enable will lazily load Xwayland so Xwayland
    /// will not be launched until the first client attempts to
    /// connect. In some cases, such as slower machines, it may be
    /// desirable to have Xwayland started immediately by using
    /// force instead of enable.
    #[display(fmt = "xwayland {}", _0)]
    Xwayland(Xwayland),
}

#[derive(Display)]
pub enum DefaultOrientation {
    #[display(fmt = "horizontal")]
    Horizontal,
    #[display(fmt = "vertical")]
    Vertical,
    #[display(fmt = "auto")]
    Auto,
}

#[derive(Display)]
pub enum WorkspaceLayout {
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "stacking")]
    Stacking,
    #[display(fmt = "tabbed")]
    Tabbed,
}

#[derive(Display)]
pub enum Xwayland {
    #[display(fmt = "enable")]
    Enable,
    #[display(fmt = "disable")]
    Disable,
    #[display(fmt = "force")]
    Force,
}
