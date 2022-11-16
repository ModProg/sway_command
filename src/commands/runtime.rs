use derive_more::Display;

use super::{to_string_or_empty, when, GapsDirection, Output, Workspace, EnDisTog, EnDisable};

#[derive(Display)]
pub enum SubCommand {
    /// Set border style for focused window
    ///
    /// Default is normal with border thickness 2.
    #[display(fmt = "border {_0}")]
    Border(Border),
    /// Exit sway and end your Wayland session
    #[display(fmt = "exit")]
    Exit,
    /// Make focused view floating, non-floating, or the opposite of what it is
    /// now
    #[display(fmt = "floating {_0}")]
    Floating(EnDisTog),
    /// Changes focused node
    #[display(fmt = "focus {_0}")]
    Focus(Focus),
    /// Makes focused view fullscreen, non-fullscreen, or the opposite of what
    /// it is now.
    ///
    /// If global is specified, the view will be fullscreen across all outputs.
    #[display(fmt = "focus {_0} {_1}")]
    Fullscreen(EnDisTog, FullscreenGlobal),
    /// Changes the inner or outer gaps for either all workspaces (`true`) or
    /// the current workspace (`false`). outer gaps can be altered per side
    /// with top, right, bottom, and left or per direction with horizontal
    /// and vertical.
    #[display(fmt = "gaps {_0} {_1} {_2}, {_3}")]
    Gaps(GapsDirection, GapsWorkspaces, GapsModification, u32),
    /// Set/unset an idle inhibitor for the view
    ///
    /// This can also be used with criteria to set an idle inhibitor for any
    /// existing view or with for_window to set idle inhibitors for future
    /// views.
    #[display(fmt = "inhibit_idle {_0}")]
    InhibitIdle(InhibitIdle),
    /// Sets the layout mode of the focused container
    #[display(fmt = "layout {_0}")]
    Layout(Layout),
    ///  Controls when the relevant application is told to render this window,
    /// as a positive number of milliseconds before the next time sway
    /// composites the output. A smaller number leads to fresher rendered frames
    /// being composited by sway and lower perceived input latency, but if set
    /// too low, the application may not finish rendering before sway composites
    /// the output, leading to delayed frames.
    ///
    /// When set to off, the relevant application is told to render this window
    /// immediately after display refresh. How much time is left for rendering
    /// before sway composites the output at that point depends on the output
    /// max_render_time setting.
    ///
    /// To set this up for optimal latency:
    /// 1.   Set up output max_render_time (see sway-output(5)).
    /// 2.   Put the target application in full-screen and have it continuously
    /// render something.
    /// 3.   Start by setting max_render_time 1. If the application drops
    /// frames, increment by 1.
    ///
    /// This setting only has an effect if a per-output max_render_time is in
    /// effect on the output the window is currently on. See sway-output(5) for
    /// further details.
    #[display(fmt = "max_render_time {_0}")]
    MaxRenderTime(MaxRenderTime),
    #[display(fmt = "move {_0}")]
    Move(Move),
    /// A no operation command that can be used to override default behaviour.
    /// The optional comment argument is ignored, but logged for debugging
    /// purposes.
    #[display(fmt = "nop {}", "_0.as_deref().unwrap_or_default()")]
    Nop(Option<String>),
    /// Reloads the sway config file and applies any changes. The config file is
    /// located at path specified by the command line arguments when started,
    /// otherwise according to the priority stated in sway(1).
    #[display(fmt = "reload")]
    Reload,
    /// Rename either <old_name> workspace to the <new_name>
    #[display(fmt = "rename workspace {_0} to {_0}")]
    RenameWorkspace(String, String),
    /// Rename the focused workspace to the <new_name>
    #[display(fmt = "rename workspace to {_0}")]
    RenameFocusedWorkspace(String),
    #[display(fmt = "resize")]
    Resize(Resize),
    /// Shows a window from the scratchpad
    ///
    /// Repeatedly using this command will cycle through the windows in the
    /// scratchpad.
    #[display(fmt = "scratchpad show")]
    ScratchpadShow,
    /// Enables or disables the ability of clients to inhibit keyboard shortcuts
    /// for a view. This is primarily useful for virtualization and remote
    /// desktop software. It affects either the currently focused view or a set
    /// of views selected by criteria. Subcommand disable additionally
    /// deactivates any active inhibitors for the given view(s). Criteria are
    /// particularly useful with the for_window command to configure a class of
    /// views differently from the per-seat defaults established by the seat
    /// subcommand of the same name. See sway-input(5) for more ways to affect
    /// inhibitors.
    #[display(fmt = "shortcuts_inhibitor {_0}")]
    ShortcutsInhibitor(EnDisable),
    /// Splits the current container, vertically or horizontally.
    #[display(fmt = "split {_0}")]
    Split(Split),
    /// "Sticks" a floating window to the current output so that it shows up on
    /// all workspaces
    #[display(fmt = "sticky {_0}")]
    Sticky(EnDisTog),
    /// Swaps the position, geometry, and fullscreen status of two containers.
    ///
    /// The first container can be selected either by criteria or focus. The
    /// second container can be selected by id, con_id, or mark. id can only be
    /// used with xwayland views. If the first container has focus, it will
    /// retain focus unless it is moved to a different workspace or the second
    /// container becomes fullscreen on the same workspace as the first
    /// container. In either of those cases, the second container will gain
    /// focus.
    #[display(fmt = "sticky {_0}")]
    Swap(Swap),
    /// Sets the format of window titles. The following placeholders may be
    /// used:
    ///
    ///     - %title - The title supplied by the window
    ///     - %app_id - The wayland app ID (applicable to wayland windows only)
    ///     - %class - The X11 classname (applicable to xwayland windows only)
    ///     - %instance - The X11 instance (applicable to xwayland windows only)
    ///     - %shell - The protocol the window is using (typically xwayland or
    ///       xdg_shell)
    ///
    /// This command is typically used with for_window criteria. For example:
    ///
    ///    for_window [title="."] title_format "<b>%title</b> (%app_id)"
    ///
    /// Note that markup requires pango to be enabled via the font command.
    ///
    /// The default format is "%title".
    #[display(fmt = "title_format {_0}")]
    TitleFormat(String),
}

#[derive(Display)]
pub enum Border {
    #[display(fmt = "none")]
    None,
    /// A border of thickness n and a title bar
    #[display(fmt = "normal {}", "to_string_or_empty(_0)")]
    Normal(Option<u32>),
    /// Allows the client to draw its own decorations
    #[display(fmt = "csd")]
    ClientSideDecorations,
    /// A border without title bar n pixels thick
    #[display(fmt = "pixel {}", "to_string_or_empty(_0)")]
    Pixel(Option<u32>),
    /// Cycles through the available border styles
    #[display(fmt = "toggle")]
    Toggle,
}

#[derive(Display)]
pub enum Focus {
    /// Moves focus to the container that matches the specified criteria
    #[display(fmt = "")]
    This,
    /// Moves focus to the next container in the specified direction.
    #[display(fmt = "up")]
    Up,
    /// Moves focus to the next container in the specified direction.
    #[display(fmt = "right")]
    Right,
    /// Moves focus to the next container in the specified direction.
    #[display(fmt = "down")]
    Down,
    /// Moves focus to the next container in the specified direction.
    #[display(fmt = "left")]
    Left,
    ///  Moves focus to the previous container in the current layout. Pass
    /// `true` to focus  the last active child of the newly focused
    /// container instead of the container it self.
    #[display(fmt = "prev {}", "when(!_0, \"sibling\")")]
    Prev(bool),
    ///  Moves focus to the next container in the current layout. Pass
    /// `true` to focus  the last active child of the newly focused
    /// container instead of the container it self.
    #[display(fmt = "next {}", "when(!_0, \"sibling\")")]
    Next(bool),
    /// Moves focus to the last-focused child of the focused container
    #[display(fmt = "child")]
    Child,
    /// Moves focus to the parent of the focused container
    #[display(fmt = "parent")]
    Parent,
    /// Moves focus to the specified output
    #[display(fmt = "output {_0}")]
    Output(FocusOutput),
    /// Sets focus to the last focused tiling container
    #[display(fmt = "tiling")]
    Tiling,
    /// Sets focus to the last focused floating container
    #[display(fmt = "floating")]
    Floating,
    /// Moves focus between the floating and tiled layers
    #[display(fmt = "mode_toggle")]
    ModeToggle,
}

#[derive(Display)]
pub enum FocusOutput {
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
    /// Named output
    Name(String),
}

#[derive(Display)]
pub enum FullscreenGlobal {
    #[display(fmt = "global")]
    Global,
    #[display(fmt = "")]
    No,
}

#[derive(Display)]
pub enum GapsWorkspaces {
    #[display(fmt = "all")]
    All,
    #[display(fmt = "current")]
    Current,
}

#[derive(Display)]
pub enum GapsModification {
    #[display(fmt = "set")]
    Set,
    #[display(fmt = "plus")]
    Plus,
    #[display(fmt = "minus")]
    Minus,
    #[display(fmt = "toggle")]
    Toggle,
}

#[derive(Display)]
pub enum InhibitIdle {
    /// Will inhibit idle when the view is focused by any seat
    #[display(fmt = "focus")]
    Focus,
    /// Will inhibit idle when the view is fullscreen (or a descendant of a
    /// fullscreen container) and is visible
    #[display(fmt = "fullscreen")]
    Fullscreen,
    /// Will inhibit idle until the view is closed (or the inhibitor is
    /// unset/changed)
    #[display(fmt = "open")]
    Open,
    /// Will inhibit idle when the view is visible on any output
    #[display(fmt = "none")]
    None,
    /// Will inhibit idle when the view is visible on any output
    #[display(fmt = "visible")]
    Visible,
}

#[derive(Display)]
pub enum Layout {
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "splith")]
    Splith,
    #[display(fmt = "splitv")]
    Splitv,
    #[display(fmt = "stacking")]
    Stacking,
    #[display(fmt = "tabbed")]
    Tabbed,
    /// Cycles the layout mode of the focused container though a preset list of
    /// layouts.
    #[display(fmt = "toggle {_0}")]
    Toggle(LayoutToggle),
}

#[derive(Display)]
pub enum LayoutToggle {
    /// Cycles through stacking, tabbed and the last split layout.     None,
    None,
    /// Cycles through splith and splitv.
    #[display(fmt = "split")]
    Split,
    /// Cycles through every layout.
    #[display(fmt = "all")]
    All,
    /// Cycles the layout mode of the focused container through a list of
    /// layouts
    #[display(
        fmt = "{}",
        "_0.iter().map(ToString::to_string).collect::<Vec<_>>().join(\" \")"
    )]
    Options(Vec<LayoutToggleOptions>),
}

#[derive(Display)]
pub enum LayoutToggleOptions {
    #[display(fmt = "split")]
    Split,
    #[display(fmt = "tabbed")]
    Tabbed,
    #[display(fmt = "stacking")]
    Stacking,
    #[display(fmt = "splitv")]
    Splitv,
    #[display(fmt = "splith")]
    Splith,
}

#[derive(Display)]
pub enum MaxRenderTime {
    #[display(fmt = "off")]
    Off,
    Msec(u32),
}

#[derive(Display)]
pub enum Move {
    /// Moves the focused container in the direction specified. Pixels are
    /// ignored when moving tiled containers
    #[display(fmt = "left {_0} px")]
    Left(i32),
    /// Moves the focused container in the direction specified. Pixels are
    /// ignored when moving tiled containers
    #[display(fmt = "right {_0} px")]
    Right(i32),
    /// Moves the focused container in the direction specified. Pixels are
    /// ignored when moving tiled containers
    #[display(fmt = "up {_0} px")]
    Up(i32),
    /// Moves the focused container in the direction specified. Pixels are
    /// ignored when moving tiled containers
    #[display(fmt = "down {_0} px")]
    Down(i32),
    /// Moves the focused container to the specified position in the workspace
    ///
    /// The position can be specified in pixels or percentage points.
    #[display(fmt = "position {_0} {_0}")]
    Position(Length, Length),
    /// Moves the focused container to the specified position relative to all
    /// outputs
    #[display(fmt = "absolute position {_0} px {_0} px")]
    AbsolutePosition(u32, u32),
    /// Moves the focused container to be centered on the workspace
    #[display(fmt = "position center")]
    PositionCenter,
    /// Moves to the center of all outputs
    #[display(fmt = "absolute position center")]
    AbsolutePositionCenter,
    /// Moves the focused container to be centered on the cursor
    #[display(fmt = "position cursor")]
    PositionCursor,
    /// Moves the focused container to the specified mark
    #[display(fmt = "container to mark")]
    Mark(String),
    /// Moves the focused container to the specified workspace
    #[display(fmt = "container to workspace {_0}")]
    Workspace(Workspace),
    /// Moves the focused container to the specified workspace
    #[display(fmt = "--no-auto-back-and-forth container to workspace {_0}")]
    WorkspaceNoAutoBackAndForth(Workspace),
    /// Moves the focused container to the scratchpad
    #[display(fmt = "container to scratchpad")]
    Scratchpad,
    /// Moves the focused container to the specified output
    #[display(fmt = "container to output {_0}")]
    ContainerToOutput(Output),
    /// Moves the focused workspace to the specified output
    #[display(fmt = "workspace to output {_0}")]
    WorkspaceToOutput(Output),
}

#[derive(Display)]
pub enum Resize {
    /// Resizes the currently focused container by amount, specified in pixels
    /// or percentage points. If the units are omitted, floating containers are
    /// resized in px and tiled containers by ppt. amount will default to 10 if
    /// omitted.
    #[display(fmt = "grow width {_0}")]
    GrowWidth(Length),
    /// Resizes the currently focused container by amount, specified in pixels
    /// or percentage points. If the units are omitted, floating containers are
    /// resized in px and tiled containers by ppt. amount will default to 10 if
    /// omitted.
    #[display(fmt = "shrink width {_0}")]
    ShrinkWidth(Length),
    /// Resizes the currently focused container by amount, specified in pixels
    /// or percentage points. If the units are omitted, floating containers are
    /// resized in px and tiled containers by ppt. amount will default to 10 if
    /// omitted.
    #[display(fmt = "grow height {_0}")]
    GrowHeight(Length),
    /// Resizes the currently focused container by amount, specified in pixels
    /// or percentage points. If the units are omitted, floating containers are
    /// resized in px and tiled containers by ppt. amount will default to 10 if
    /// omitted.
    #[display(fmt = "shrink height {_0}")]
    ShrinkHeight(Length),
    /// Sets the height of the container to height, specified in pixels or
    /// percentage points. If the units are omitted, floating containers are
    /// resized in px and tiled containers by ppt. If height is 0, the container
    /// will not be resized.
    #[display(fmt = "set height {_0}")]
    SetHeight(Length),
    /// Sets the width of the container to height, specified in pixels or
    /// percentage points. If the units are omitted, floating containers are
    /// resized in px and tiled containers by ppt. If height is 0, the container
    /// will not be resized.
    #[display(fmt = "set width {_0}")]
    SetWidth(Length),
    /// Sets the width and height of the container to width and height,
    /// specified in pixels or percentage points. If the units are omitted,
    /// floating containers are resized in px and tiled containers by ppt. If
    /// width or height is 0, the container will not be resized on that axis.
    #[display(fmt = "set width {_0} height {_0}")]
    Set(Length, Length),
}

#[derive(Display)]
pub enum Split {
    Vertical,
    Horizontal,
    /// The effect of a previous split is undone if the current container is the
    /// only child of a split parent.
    None,
    /// The current container is split opposite to the parent container's layout
    Toggle,
}

#[derive(Display)]
pub enum Swap {
    /// can only be used with xwayland views
    #[display(fmt = "id {_0}")]
    Id(String),
    #[display(fmt = "con_id {_0}")]
    ConId(String),
    #[display(fmt = "mark {_0}")]
    Mark(String),
}

#[derive(Display)]
pub enum Length {
    #[display(fmt = "{_0} px")]
    Px(u32),
    #[display(fmt = "{_0} ppt")]
    Ppt(u32),
    #[display(fmt = "{_0}")]
    Default(u32),
}
