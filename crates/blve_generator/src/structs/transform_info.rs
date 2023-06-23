use crate::{orig_html_struct::structs::Node, transformers::utils::append_v_to_vars};

#[derive(Debug, Clone)]
pub struct AddStringToPosition {
    pub position: u32,
    pub string: String,
}

#[derive(Debug)]
pub struct VariableNameAndAssignedNumber {
    pub name: String,
    pub assignment: u32,
}

#[derive(Debug)]
pub struct ActionAndTarget {
    pub action_name: String,
    pub action: EventTarget,
    pub target: String,
}

#[derive(Debug)]
pub struct NeededIdName {
    pub id_name: String,
    pub to_delete: bool,
}

#[derive(Debug)]
pub enum EventTarget {
    RefToFunction(String),
    Statement(String),
    EventBindingStatement(EventBindingStatement),
}

#[derive(Debug)]
pub struct EventBindingStatement {
    pub statement: String,
    pub arg: String,
}

impl ToString for EventTarget {
    fn to_string(&self) -> String {
        match self {
            EventTarget::RefToFunction(function_name) => function_name.clone(),
            EventTarget::Statement(statement) => format!("()=>{}", statement),
            EventTarget::EventBindingStatement(statement) => {
                format!("({})=>{}", statement.arg, statement.statement)
            }
        }
    }
}

impl EventTarget {
    pub fn new(content: String, variables: &Vec<String>) -> Self {
        // FIXME: This is a hacky way to check if the content is a statement or a function
        if content.trim().ends_with(")") {
            EventTarget::Statement(content)
        } else if word_is_one_word(content.as_str()) {
            EventTarget::RefToFunction(content)
        } else {
            EventTarget::Statement(append_v_to_vars(content.as_str(), &variables).0)
        }
    }
}

fn word_is_one_word(word: &str) -> bool {
    word.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

#[derive(Debug)]
pub struct IfBlockInfo {
    pub parent_id: String,
    pub target_if_blk_id: String,
    pub distance: u64,
    pub target_anchor_id: Option<String>,
    pub elm: Node,
    pub ref_text_node_id: Option<String>,
    pub condition: String,
    pub condition_dep_vars: Vec<String>,
    pub ctx: Vec<String>,
    pub if_block_id: String,
}
