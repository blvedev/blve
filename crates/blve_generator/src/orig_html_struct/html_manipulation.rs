pub struct HtmlManipulator {
    pub target_uuid: String,
    pub manipulations: HtmlManipulation,
}

pub enum HtmlManipulation {
    RemoveChildForIfStatement(RemoveChildForIfStatement),
    SetIdForReactiveContent(SetIdForReactiveContent),
}

pub struct RemoveChildForIfStatement {
    pub child_uuid: String,
}

pub struct SetIdForReactiveContent {
    pub text: String,
    pub depenent_vars: Vec<String>,
}
