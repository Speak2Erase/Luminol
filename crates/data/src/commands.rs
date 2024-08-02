use serde::{Deserialize, Serialize};

pub type Code = u16;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub name: String,
    pub kind: CommandKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommandKind {
    /// A command with (possibly multiple) branches.
    Branch {
        /// The relevant parameters for this branch.
        parameters: Parameters,
        /// Optional branches.
        branches: Vec<Branch>,
        /// The terminator command comes after all branches.
        terminator: Terminator,
        /// True if this command can contain a branch (like Conditional Branch)
        /// and false if it can't (like Show Choice)
        command_contains_branch: bool,
    },
    /// Multi-string command.
    /// Special in that every newline is an extra command,
    /// and every command after the first one has a continuation id.
    Multi(Code),
    Regular {
        parameters: Parameters,
    },
    /// Special case for the "Set Move Route" command
    /// where RPG Maker will insert every single move command inside the move route
    /// as a command following this one, for editor display purposes.
    ///
    /// We don't bother doing that but this is required for compatibility with RPG Maker.
    MoveRoute(Code),
    /// Special commands with no parameters!
    Blank,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub name: String,
    pub code: Code,
    pub condition: Condition,
    pub duplicate_parameter: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Condition {
    pub parameter: usize,
    pub kind: ConditionKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConditionKind {
    IsTrue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Terminator {
    pub code: Code,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Parameter {
    Parameter {
        index: usize,
        name: String,
        kind: ParameterKind,
    },
    /// Used for display purposes only.
    Label(String),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParameterKind {
    Switch,
    Variable,
    /// List of names to select from.
    /// Will be stored as the index of the selected name.
    Enum(Vec<String>),
    /// Selects between  a list of parameters.
    /// This is used for commands like "Conditional Branch"
    /// where multiple conditon types are available.
    Selector(Vec<Vec<Parameter>>),
    SelfSwitch,
    IntBool,
    Bool,
    Int,
}

pub type Parameters = Vec<Parameter>;
