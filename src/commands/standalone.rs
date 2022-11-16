use std::num::NonZeroU32;

use derive_more::Display;
use vec1::Vec1;

use super::{EnDisTog, WorkspaceName, YesNo};
use crate::{
    commands::{
        separated, then_or_empty, to_string_or_empty, when, Font, GapsDirection, Output, Workspace,
    },
    criteria::{Criteria, CriteriaList},
    Command,
};

#[derive(Display)]
pub enum CriterialessCommand {
    #[display(fmt = "assign {_0} → workspace {_0}")]
    AssignWorkspace(CriteriaList, Workspace),
    #[display(fmt = "assign {_0} → output {_0}")]
    AssignOutput(CriteriaList, Output),
    /// Binds key combo to execute the sway command command when pressed
    ///
    /// You may use XKB key names here (wev(1) is a good tool for discovering
    /// these).
    ///
    /// With the flag --release, the command is executed when the key combo is
    /// re‐ leased. If input-device is given, the binding will only be
    /// executed for that input device and will be executed instead of any
    /// binding that is generic to all devices. If a group number is given, then
    /// the binding will only be available for that group. By default, if
    /// you overwrite a binding, swaynag will give you a warning. To si‐
    /// lence this, use the --no-warn flag.
    ///
    /// Unless the flag --locked is set, the command will not be run when a
    /// screen locking program is active. If there is a matching binding
    /// with and without --locked, the one with will be preferred when locked
    /// and the one without will be preferred when unlocked. If there are
    /// matching bindings and one has both --input-device and --locked
    /// and the other has neither, the former will be preferred even when
    /// unlocked.
    ///
    /// Unless the flag --inhibited is set, the command will not be run when a
    /// keyboard shortcuts inhibitor is active for the currently focused
    /// window. Such inhibitors are usually requested by remote desktop and
    /// virtualization software to enable the user to send keyboard
    /// shortcuts to the remote or virtual session. The --inhibited flag
    /// allows one to define bindings which will be exempt from pass-through to
    /// such software. The same preference logic as for --locked applies.
    ///
    /// Unless the flag --no-repeat is set, the command will be run repeatedly
    /// when the key is held, according to the repeat settings specified in
    /// the input configuration.
    ///
    /// Bindings to keysyms are layout-dependent. This can be changed with the
    /// --to-code flag. In this case, the keysyms will be translated into
    /// the corresponding keycodes in the first configured layout.
    ///
    /// Mouse bindings operate on the container under the cursor instead of the
    /// container that has focus. Mouse buttons can either be specified in
    /// the form button[1-9] or by using the name of the event code (ex BTN_LEFT
    /// or BTN_RIGHT). For the former option, the buttons will be mapped to
    /// their values in X11 (1=left, 2=middle, 3=right, 4=scroll up,
    /// 5=scroll down, 6=scroll left, 7=scroll right, 8=back, 9=forward). For
    /// the latter option, you can find the event names using libinput
    /// debug-events.
    ///
    /// The priority for matching bindings is as follows: input device, group,
    /// and locked state.
    ///
    /// --whole-window, --border, and --exclude-titlebar are mouse-only options
    /// which affect the region in which the mouse bindings can be
    /// triggered.  By default, mouse bindings are only triggered when over the
    /// title bar. With the --border option, the border of the window will
    /// be included in this region. With the --whole-window option,
    /// the cursor can be anywhere over a window including the title, border,
    /// and content. --exclude-titlebar can be used in conjunction with any
    /// other option to specify that the titlebar should be excluded from the
    /// region of consideration.
    ///
    /// If --whole-window is given, the command can be triggered when the cursor
    /// is over an empty workspace. Using a mouse binding over a layer
    /// surface's exclusive region is not currently possible.
    #[display(fmt = "bindsym {_0} {_1} {_2}")]
    Bindsym(BindFlags, SymKey, Command),
    /// Like [`CriterialessCommand::Bindsym`] but for key/button codes
    #[display(fmt = "bindcode {_0} {_1} {_2}")]
    Bindcode(BindFlags, SymCode, Command),
    /// Binds <switch> to execute the sway command command on state changes
    ///
    /// Supported switches are lid (laptop lid) and tablet (tablet mode)
    /// switches. Valid values for state are on, off and toggle. These switches
    /// are on when the device lid is shut and when tablet mode is active
    /// respectively. toggle is also supported to run a command
    /// both when the switch is toggled on or off.
    ///
    /// Unless the flag --locked is set, the command will not be run when a
    /// screen locking program is active. If there is a matching binding
    /// with and without --locked, the one with will be preferred when locked
    /// and the one without will be preferred when unlocked.
    ///
    /// If the --reload flag is given, the binding will also be executed when
    /// the config is reloaded. toggle bindings will not be executed on
    /// reload. The --locked flag will operate as normal so if the config is
    /// reloaded while locked and --locked is not given, the binding will
    /// not be executed.
    ///
    /// By default, if you overwrite a binding, swaynag will give you a warning.
    /// To silence this, use the --no-warn flag.
    #[display(fmt = "bindswitch {_0} {_1}:{_2} {_3}")]
    Bindswitch(BindswitchFlags, Switch, SwitchState, Command),
    /// This command is ignored and is only present for i3 compatibility.
    // TODO feature for i3 things
    #[display(fmt = "client.background {_0}")]
    ClientBackground(String),
    /// Configures the color of window borders and title bars.
    ///
    /// The first three colors are required. When omitted indicator will use a
    /// sane default and child_border will use the color set for background.
    /// Colors may be specified in hex, either as #RRGGBB or #RRGGBBAA.
    #[display(fmt = "client.{_0}")]
    Client(ClientClass),
    /// Set default border style for new tiled windows
    #[display(fmt = "default_border {_0}")]
    DefaultBorder(DefaultBorder),
    /// Set default border style for new floating windows
    ///
    /// This only applies to windows that are spawned in floating mode, not
    /// windows that become floating afterwards.
    #[display(fmt = "default_floating_border {_0}")]
    DefaultFloatingBorder(DefaultBorder),
    /// Executes shell command with sh
    #[display(fmt = "exec {_0}")]
    Exec(String),
    /// Like exec, but the shell command will be executed again after reload
    #[display(fmt = "exec_always {_0}")]
    ExecAlways(String),
    /// Specifies the maximum size of floating windows
    ///
    /// -1 x -1 removes the upper limit. The default is 0 x 0, which will use
    /// the width and height of the entire output layout as the maximums
    #[display(fmt = "floating_maximum_size {_0} x {_1}")]
    FloatingMaximumSize(i32, i32),
    /// Specifies the minimum size of floating windows. The default is 75 x 50.
    #[display(fmt = "floating_minimum_size {_0} x {_1}")]
    FloatingMinimumSize(i32, i32),
    /// When the modifier key is held down, you may hold left click to move
    /// windows, and right click to resize them.
    ///
    /// Setting modifier to none disables this feature.
    #[display(fmt = "floating_modifier {} x {_1}", "to_string_or_empty(_0)")]
    FloatingModifier(Option<String>, FloatingModifierMode),
    /// If set to yes, moving your mouse over a window will focus that window.
    /// If set to always, the window under the cursor will always be focused,
    /// even after switching between workspaces.
    #[display(fmt = "focus_follows_mouse {_0}")]
    FocusFollowsMouse(MouseFocus),
    /// This option determines what to do when an xwayland client requests
    /// window activation.
    ///
    /// If set to urgent, the ur‐ gent state will be set for that window. If set
    /// to focus, the window will become focused. If set to smart, the window
    /// will become focused only if it is already visible, otherwise the urgent
    /// state will be set. Default is urgent.
    #[display(fmt = "focus_on_window_activation {_0}")]
    FocusOnWindowActivation(WindowActivationFocus),
    /// This option determines what to do when attempting to focus over the edge
    /// of a container
    #[display(fmt = "focus_wrapping {_0}")]
    FocusWrapping(FocusWrapping),
    /// Sets font to use for the title bars
    ///
    /// To enable support for pango markup, preface the font name with `pango:`.
    /// For example, monospace 10 is the default font. To enable support for
    /// pango markup, pango:monospace 10 should be used instead. Regardless
    /// of whether pango markup is enabled, font should be specified as a
    /// pango font description. For more information on pango font
    /// descriptions, see <https://docs.gtk.org/Pango/type_func.FontDescription.from_string.html#description>
    #[display(fmt = "font {_0}")]
    Font(Font),
    /// If an application on another workspace sets an urgency hint, switching
    /// to this workspace may lead to immediate focus of the application, which
    /// also means the window decoration color would be immediately reset to
    /// client.fo‐ cused. This may make it unnecessarily hard to tell which
    /// window originally raised the event. This option allows one to set a
    /// timeout in ms to delay the urgency hint reset.
    #[display(fmt = "force_display_urgency_hint {_0} ms")]
    ForceDisplayUrgencyHint(u32),
    /// Thickness of the titlebar border in pixels
    #[display(fmt = "titlebar_border_thickness {_0}")]
    TitlebarBorderThickness(u32),
    /// Padding of the text in the titlebar. horizontal value affects horizontal
    /// padding of the text while vertical value affects vertical padding (space
    /// above and below text). Padding includes titlebar borders so their value
    /// should be greater than titlebar_border_thickness. If vertical value is
    /// not specified it is set to the horizon‐ tal value.
    #[display(fmt = "titlebar_padding {_0} {}", "to_string_or_empty(_1)")]
    TitlebarPadding(NonZeroU32, Option<NonZeroU32>),
    /// Whenever a window that matches criteria appears, run list of commands.
    #[display(fmt = "for_window {_0} {_1}")]
    ForWindow(Criteria, Command),
    ///  Sets default amount pixels of inner or outer gap, where the inner
    /// affects spacing around each view and outer affects the spacing around
    /// each workspace. Outer gaps are in addition to inner gaps. To reduce or
    /// remove outer gaps, outer gaps can be set to a negative value. outer gaps
    /// can also be specified per side with top, right, bottom, and left or per
    /// direction with horizontal and vertical.
    ///
    /// This affects new workspaces only, and is used when the workspace doesn't
    /// have its own gaps settings (see: workspace <ws> gaps ...).
    #[display(fmt = "gaps {_0} {_1}")]
    Gaps(GapsDirection, u32),
    /// Hides window borders adjacent to the screen edges. Default is none. The
    /// smart|smart_no_gaps options are equivalent to setting smart_borders
    /// smart|no_gaps and hide_edge_borders none.
    #[display(fmt = "hide_edge_borders {_0}")]
    HideEdgeBorders(EdgeBorders),
    /// i3-compatible [`Self::HideEdgeBorders`]
    ///
    /// Hide the title bar on tabbed and stacked containers with one child.
    #[display(fmt = "hide_edge_borders --i3 {_0}")]
    HideEdgeBordersI3(EdgeBorders),
    // TODO sway-input(5)
    /// For details on input subcommands, see sway-input(5)
    ///
    /// `*` may be used in lieu of a specific device name to configure all input
    /// devices. A list of input device names may be obtained via swaymsg -t
    /// get_inputs.
    #[display(fmt = "input {_0} {}", "separated(_1, ' ')")]
    Input(String, Vec<String>),
    /// For details on seat subcommands, see sway-input(5)
    #[display(fmt = "seat {_0} {}", "separated(_1, ' ')")]
    Seat(String, Vec<String>),
    /// Kills (closes) the currently focused container and all of its children
    #[display(fmt = "kill")]
    Kill,
    #[display(fmt = "smart_borders {_0}")]
    SmartBorders(SmartBorders),
    #[display(fmt = "smart_gaps {_0}")]
    SmartGaps(SmartGaps),
    /// Marks are arbitrary labels that can be used to identify certain windows
    /// and then jump to them at a later time
    #[display(fmt = "mark {_0} {_1}")]
    Mark(MarkModification, String),
    /// Switches to the specified mode
    ///
    /// The default mode is default.
    #[display(fmt = "mode")]
    Mode(String),
    /// The only valid mode-subcommands... are bindsym, bindcode, bindswitch,
    /// and set.
    #[display(fmt = "mode {_0} {}", "separated(_1, ' ')")]
    ModeCmds(String, Vec<String>),
    /// The only valid mode-subcommands... are bindsym, bindcode, bindswitch,
    /// and set. Mode will be interpreted as pango markup.
    #[display(fmt = "mode --pango_markup {_0} {}", "separated(_1, ' ')")]
    ModePangoMarkupCmds(String, Vec<String>),
    /// If output is specified, the mouse will be moved to new outputs as you
    /// move focus between them. If container is specified, the mouse will be
    /// moved to the middle of the container on switch. Default is output.
    #[display(fmt = "mouse_warping {_0}")]
    MouseWarping(MouseWarping),
    /// Prevents windows matching <criteria> from being focused automatically
    /// when they're created
    ///
    /// This has no effect on the first window in a workspace.
    #[display(fmt = "no_focus {_0}")]
    NoFocus(Criteria),
    /// For details on output subcommands, see sway-output(5)
    ///
    /// `*` may be used in lieu of a specific output name to configure all
    /// outputs. A list of output names may be obtained via swaymsg -t
    /// get_outputs.
    #[display(fmt = "output {_0} {}", "separated(_1, ' ')")]
    Output(String, Vec<String>),
    /// Determines what to do when a fullscreen view opens a dialog
    ///
    /// If smart (the default), the dialog will be dis‐ played. If ignore, the
    /// dialog will not be rendered. If leave_fullscreen, the view will exit
    /// fullscreen mode and the dialog will be rendered.
    #[display(fmt = "popup_during_fullscreen {_0}")]
    PopupDuringFullscreen(PopupDuringFullscreen),
    /// Sets variable $name to value
    ///
    /// You can use the new variable in the arguments of future commands. When
    /// the variable is used, it can be escaped with an additional $ (ie
    /// $$name) to have the replacement happen at run time instead of when
    /// reading the config. However, it does not always make sense for the
    /// variable to be replaced at run time since some arguments do need to
    /// be known at config time.
    ///
    /// **NOTE:** Use [`crate::Command::Raw`] for commands referencing variables
    /// variables.
    #[display(fmt = "set ${_0} {_1}")]
    Set(String, String),
    /// If show_marks is yes, marks will be displayed in the window borders. Any
    /// mark that starts with an underscore will not be drawn even if show_marks
    /// is yes. The default is yes.
    #[display(fmt = "show_marks {_0}")]
    ShowMarks(YesNo),
    /// Adjusts the opacity of the window between 0 (completely transparent) and
    /// 1 (completely opaque).
    #[display(fmt = "opacity {_0} {_1}")]
    Opacity(OpacityModification, f32),
    /// Sets whether or not tiling containers can be dragged with the mouse
    ///
    /// If enabled (default), the floating_mod can be used to drag tiling, as
    /// well as floating, containers. Using the left mouse button on title
    /// bars without the floating_mod will also allow the container to be
    /// dragged. toggle should not be used in the config file.
    #[display(fmt = "tiling_drag {_0}")]
    TilingDrag(EnDisTog),
    /// Sets the threshold that must be exceeded for a container to be dragged
    /// by its titlebar
    ///
    /// This has no effect if floating_mod is used or if tiling_drag is set to
    /// disable.  Once the threshold has been exceeded once, the drag starts and
    /// the cursor can come back inside the threshold without stopping the drag.
    /// threshold is multiplied by the scale of the output that the cursor on.
    /// The default is 9.
    #[display(fmt = "tiling_drag_threshold {_0}")]
    TilingDragThreshold(u32),
    /// Sets the title alignment
    ///
    /// If right is selected and show_marks is set to yes, the marks will be
    /// shown on the left side instead of the right side.
    #[display(fmt = "title_align {_0}")]
    TitleAlign(TitleAlign),
    /// Removes a binding for when <switch> changes to <state>
    #[display(fmt = "unbindswitch {_0}:{_1}")]
    Unbindswitch(Switch, SwitchState),
    /// Removes the binding for key combo that was previously bound with the
    /// given flags
    ///
    /// If input-device is given, only the binding for that input device will be
    /// unbound.
    #[display(fmt = "unbindsym {_0} {_1}")]
    Unbindsym(BindFlags, SymKey),
    /// <code> is also available for unbinding with key/button codes instead of
    /// key/button names
    #[display(fmt = "unbindcode {_0} {_1}")]
    Unbindcode(BindFlags, SymCode),
    // TODO should this not be in `runtime`
    /// Will remove identifier from the list of current marks on a window
    ///
    /// If identifier is omitted, all marks are removed.
    #[display(fmt = "unmark {_0}")]
    Unmark(String),
    // TODO should this not be in `runtime`
    /// Using enable or disable manually sets or unsets the window's urgent
    /// state. Using allow or deny controls the window's ability to set itself
    /// as urgent. By default, windows are allowed to set their own urgency.
    #[display(fmt = "unmark {_0}")]
    Urgent(Urgent),
    /// Switches to the specified workspace
    #[display(fmt = "workspace {_0}")]
    Workspace(Workspace),
    /// Specifies that workspace name should have the given gaps settings when
    /// it is created
    ///
    /// This command does not affect existing workspaces. To alter the gaps of
    /// an existing workspace, use the gaps command.
    #[display(fmt = "workspace {_0} gaps {_1} {_2}")]
    WorkspaceGaps(WorkspaceName, GapsDirection, u32),
    /// Specifies that workspace name should be shown on the specified outputs.
    /// Multiple outputs can be listed and the first available will be used. If
    /// the workspace gets placed on an output further down the list and an
    /// output that is higher on the list becomes available, the workspace will
    /// be moved to the higher priority output.
    ///
    /// This command does not affect existing workspaces. To move an existing
    /// workspace, use the move command in combination with the workspace
    /// criteria (non-empty workspaces only) or workspace command (to switch to
    /// the workspace before moving).
    #[display(fmt = "workspace {_0} output {}", "separated(_1, ' ')")]
    WorkspaceOutput(WorkspaceName, Vec1<String>),
    /// When yes, repeating a workspace switch command will switch back to the
    /// prior workspace. For example, if you are currently on workspace 1,
    /// switch to workspace 2, then invoke the workspace 2 command again, you
    /// will be returned to workspace 1. Default is no.
    #[display(fmt = "workspace_auto_back_and_forth {_0}")]
    WorkspaceAutoBackAndForth(YesNo),
}

#[derive(Display, Default)]
#[display(
    fmt = "{} {} {} {} {} {} {} {} {} {}",
    "when(*whole_window, \"--whole-window\")",
    "when(*border, \"--border\")",
    "when(*exclude_title_bar, \"--exclude-title-bar\")",
    "when(*release, \"--release\")",
    "when(*locked, \"--locked\")",
    "when(*to_code, \"--to-code\")",
    "input_device.as_ref().map(|input_device| format!(\"--input-device={}\", input_device)).unwrap_or_default()",
    "when(*no_warn, \"--no-warn\")",
    "when(*no_repeat, \"--no-repeat\")",
    "when(*inhibited, \"--inhibited\")"
)]
pub struct BindFlags {
    /// The cursor can be anywhere over a window including the title, border,
    /// and content
    pub whole_window: bool,
    /// The border of the window will be included in this region
    pub border: bool,
    /// Can be used in conjunction with any other option to specify that the
    /// titlebar should be excluded from the region of consideration
    pub exclude_title_bar: bool,
    /// Command is executed when the key combo is released
    pub release: bool,
    /// Run command when a screen locking program is active
    pub locked: bool,
    /// Bindings to keysyms are layout-dependent. This can be changed with the
    /// --to-code flag. In this case, the keysyms will be translated into the
    /// corresponding keycodes in the first configured layout.
    pub to_code: bool,
    /// The binding will only be executed for that input device and will be
    /// executed instead of any binding that is generic to all devices
    pub input_device: Option<String>,
    /// By default, if you overwrite a binding, swaynag will give you a warning.
    /// To silence this, use the --no-warn flag.
    pub no_warn: bool,
    /// Unless the flag --no-repeat is set, the command will be run repeatedly
    /// when the key is held, according to the repeat settings specified in the
    /// input configuration.
    pub no_repeat: bool,
    /// Unless the flag --inhibited is set, the command will not be run when a
    /// keyboard shortcuts inhibitor is active for the currently focused window.
    /// Such inhibitors are usually requested by remote desktop and
    /// virtualization software to enable the user to send keyboard shortcuts to
    /// the remote or virtual session. The --inhibited flag allows one to define
    /// bindings which will be exempt from pass-through to such software. The
    /// same preference logic as for --locked applies.
    pub inhibited: bool,
}

#[derive(Display)]
#[display(fmt = "{group}{modifiers}{key}")]
pub struct SymKey {
    group: Group,
    modifiers: Modifiers,
    key: String,
}

impl SymKey {
    pub fn key(key: impl Into<String>) -> Self {
        Self {
            group: Default::default(),
            modifiers: Default::default(),
            key: key.into(),
        }
    }
}

#[derive(Display)]
#[display(fmt = "{modifiers}{key}")]
pub struct SymCode {
    modifiers: Modifiers,
    key: u32,
}

#[derive(Display, Default)]
pub enum Group {
    #[default]
    #[display(fmt = "")]
    None,
    #[display(fmt = "Group1+")]
    Group1,
    #[display(fmt = "Group2+")]
    Group2,
    #[display(fmt = "Group3+")]
    Group3,
    #[display(fmt = "Group4+")]
    Group4,
}

#[derive(Display, Default)]
#[display(
    fmt = "{}{}{}{}{}{}",
    "when(*mod1, \"Mod1+\")",
    "when(*mod2, \"Mod2+\")",
    "when(*mod3, \"Mod3+\")",
    "when(*mod4, \"Mod4+\")",
    "when(*shift, \"Shift+\")",
    "when(*control, \"Control+\")"
)]
pub struct Modifiers {
    pub mod1: bool,
    pub mod2: bool,
    pub mod3: bool,
    pub mod4: bool,
    pub shift: bool,
    pub control: bool,
}

#[derive(Display)]
#[display(
    fmt = "{} {} {}",
    "when(*locked, \"--locked\")",
    "when(*no_warn, \"--no-warn\")",
    "when(*reload, \"--reload\")"
)]
pub struct BindswitchFlags {
    /// Run command when a screen locking program is active
    pub locked: bool,
    /// By default, if you overwrite a binding, swaynag will give you a warning.
    /// To silence this, use the --no-warn flag.
    pub no_warn: bool,
    /// the binding will also be executed when the config is reloaded.
    ///
    /// Toggle bindings will not be executed on reload. The --locked flag will
    /// operate as normal so if the config is reloaded while locked and --locked
    /// is not given, the binding will not be executed.
    pub reload: bool,
}

#[derive(Display)]
pub enum Switch {
    /// Laptop lid
    #[display(fmt = "lid")]
    Lid,
    /// Tablet mode
    #[display(fmt = "tablet")]
    Tablet,
}

#[derive(Display)]
pub enum SwitchState {
    #[display(fmt = "on")]
    On,
    #[display(fmt = "off")]
    Off,
    #[display(fmt = "toggle")]
    Toggle,
}

#[derive(Display)]
#[display(
    fmt = "{class} {border} {background} {text} {} {}",
    "to_string_or_empty(indicator)",
    "to_string_or_empty(&indicator.and(*child_border))"
)]
pub struct ClientClass {
    pub class: Class,
    /// The border around the title bar
    pub border: Color,
    /// The background of the title bar
    pub background: Color,
    /// The text color of the title bar
    pub text: Color,
    /// The color used to indicate where a new view will open
    ///
    /// In a tiled container, this would paint the right border of the current
    /// view if a new view would be opened to the right.
    pub indicator: Option<Color>,
    /// The border around the view itself
    ///
    /// Note: Requires [`Self::indicator`] to be set
    pub child_border: Option<Color>,
}

#[derive(Display, Clone, Copy)]
#[display(
    fmt = "#{red:X}{green:X}{blue:X}{}",
    "then_or_empty(alpha, |a| format!(\"{a:X}\"))"
)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: Option<u8>,
}

#[derive(Display)]
pub enum Class {
    /// The window that has focus
    #[display(fmt = "focused")]
    Focused,
    /// The most recently focused view within a container which is not focused
    #[display(fmt = "focused_inactive")]
    FocusedInactive,
    /// A view that has focused descendant container
    ///
    /// Tab or stack container title that is the parent of the focused container
    /// but is not directly focused. Defaults to focused_inactive if not
    /// specified and does not use the indicator and child_border colors.
    #[display(fmt = "focused_tab_title")]
    FocusedTabTitle,
    /// Ignored (present for i3 compatibility)
    #[display(fmt = "placeholder")]
    Placeholder,
    /// A view that does not have focus
    #[display(fmt = "unfocused")]
    Unfocused,
    /// A view with an urgency hint.
    ///
    /// Note: Native Wayland windows do not support urgency. Urgency only works
    /// for Xwayland windows.
    #[display(fmt = "urgent")]
    Urgent,
}

#[derive(Display)]
pub enum DefaultBorder {
    #[display(fmt = "none")]
    None,
    /// A border of thickness n and a title bar
    #[display(fmt = "normal {}", "to_string_or_empty(_0)")]
    Normal(Option<u32>),
    /// A border without title bar n pixels thick
    #[display(fmt = "pixel {}", "to_string_or_empty(_0)")]
    Pixel(Option<u32>),
}

#[derive(Display)]
pub enum FloatingModifierMode {
    /// Left click is used for moving and right click for resizing
    #[display(fmt = "normal")]
    Normal,
    /// Left click is used for resizing and right click for moving
    #[display(fmt = "inverse")]
    Inverse,
}

#[derive(Display)]
pub enum MouseFocus {
    /// Moving your mouse over a window will focus that window
    #[display(fmt = "yes")]
    Yes,
    #[display(fmt = "no")]
    No,
    /// The window under the cursor will always be focused, even after switching
    /// between workspaces
    #[display(fmt = "always")]
    Always,
}

#[derive(Display)]
pub enum WindowActivationFocus {
    /// The window will become focused only if it is already visible, otherwise
    /// the urgent state will be set
    Smart,
    /// The urgent state will be set for that window
    Urgent,
    /// The window will become focused
    Focus,
    None,
}

#[derive(Display)]
pub enum FocusWrapping {
    /// Focus will be wrapped to the opposite edge of the container, if there
    /// are no other containers in the direction
    #[display(fmt = "yes")]
    Yes,
    /// The focused container will retain focus, if there are no other
    /// containers in the direction
    #[display(fmt = "no")]
    No,
    /// Focus will be wrapped to the opposite edge of the container, even if
    /// there are other containers in the direction
    #[display(fmt = "force")]
    Force,
    /// Focus will wrap like in the yes case and additionally wrap when moving
    /// outside of workspaces boundaries
    #[display(fmt = "workspace")]
    Workspace,
}

#[derive(Display)]
pub enum EdgeBorders {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "vertical")]
    Vertical,
    #[display(fmt = "horizontal")]
    Horizontal,
    #[display(fmt = "both")]
    Both,
    #[display(fmt = "smart")]
    Smart,
    #[display(fmt = "smart_no_gaps")]
    SmartNoGaps,
}

#[derive(Display)]
pub enum SmartBorders {
    /// Borders will only be enabled if the workspace has more than one visible
    /// child
    #[display(fmt = "on")]
    On,
    /// borders will only be enabled if the workspace has more than one visible
    /// child and gaps equal to zero
    #[display(fmt = "no_gaps")]
    NoGaps,
    #[display(fmt = "off")]
    Off,
}

#[derive(Display)]
pub enum SmartGaps {
    /// Gaps will only be enabled if a workspace has more than one child
    #[display(fmt = "on")]
    On,
    #[display(fmt = "off")]
    Off,
    #[display(fmt = "toggle")]
    Toggle,
    /// outer gaps will only be enabled if a workspace has exactly one child
    #[display(fmt = "inverse_outer")]
    InverseOuter,
}

#[derive(Display)]
pub enum MarkModification {
    /// Will add identifier to the list of current marks
    Add,
    /// Will add identifier to the list of current marks, will remove mark if it
    /// is already marked
    AddToggle,
    /// Sets identifier as the only mark on a window
    Replace,
    /// Sets identifier as the only mark on a window, will remove mark if it
    /// is already marked
    ReplaceToggle,
}

#[derive(Display)]
pub enum MouseWarping {
    /// The mouse will be moved to new outputs as you move focus between them
    #[display(fmt = "output")]
    Output,
    /// The mouse will be moved to the middle of the container on switch
    #[display(fmt = "container")]
    Container,
    #[display(fmt = "none")]
    None,
}

#[derive(Display)]
pub enum PopupDuringFullscreen {
    /// the dialog will be displayed
    Smart,
    /// the dialog will not be rendered
    Ignore,
    /// the view will exit fullscreen mode and the dialog will be rendered
    LeaveFullscreen,
}

#[derive(Display)]
pub enum OpacityModification {
    #[display(fmt = "set")]
    Set,
    #[display(fmt = "plus")]
    Plus,
    #[display(fmt = "minus")]
    Minus,
}

#[derive(Display)]
pub enum TitleAlign {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "right")]
    Right,
}

#[derive(Display)]
pub enum Urgent {
    #[display(fmt = "enable")]
    Enable,
    #[display(fmt = "disable")]
    Disable,
    #[display(fmt = "allow")]
    Allow,
    #[display(fmt = "deny")]
    Deny,
}
