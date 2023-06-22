use crate::{
    orig_html_struct::{
        html_manipulation::{HtmlManipulation, HtmlManipulator, SetIdForReactiveContent},
        structs::{Element, Node, NodeContent},
    },
    structs::{
        transform_info::{ActionAndTarget, EventBindingStatement, EventTarget, NeededIdName},
        transform_targets::{
            ElmAndReactiveAttributeRelation, ElmAndReactiveInfo, ElmAndVariableContentRelation,
            ReactiveAttr,
        },
    },
};

use super::utils::{append_v_to_vars, gen_nanoid};

// TODO:dep_vars の使い方を再考する
// RCを使用して、子から親のmutableな変数を参照できるようにする可能性も視野に入れる
pub fn check_html_elms(
    varibale_names: &Vec<String>,
    node: &mut Node,
    needed_ids: &mut Vec<NeededIdName>,
    elm_and_var_relation: &mut Vec<ElmAndReactiveInfo>,
    actions_and_targets: &mut Vec<ActionAndTarget>,
    parent_uuid: Option<&String>,
    html_manipulators: &mut Vec<HtmlManipulator>,
) -> Result<(), String> {
    match &mut node.content {
        NodeContent::Element(element) => {
            for (key, action_value) in &element.attributes.clone() {
                // if attrs.name starts with "@"
                if key.starts_with("@") {
                    let action_name = &key[1..];
                    let id = set_id_for_needed_elm(element, needed_ids);
                    if let Some(value) = &&action_value {
                        actions_and_targets.push(ActionAndTarget {
                            action_name: action_name.to_string(),
                            action: EventTarget::new(value.to_string(), varibale_names),
                            target: id,
                        })
                    }
                    element.attributes.remove(key);
                } else if key.starts_with("::") {
                    let binding_attr = &key[2..];
                    let id: String = set_id_for_needed_elm(element, needed_ids);
                    if let Some(value) = &&action_value {
                        actions_and_targets.push(ActionAndTarget {
                            action_name: "input".to_string(),
                            action: EventTarget::EventBindingStatement(EventBindingStatement {
                                statement: format!("{}.v = event.target.{}", &value, &binding_attr),
                                arg: "e".to_string(),
                            }),
                            target: id.clone(),
                        });
                        elm_and_var_relation.push(
                            ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(
                                ElmAndReactiveAttributeRelation {
                                    elm_id: id,
                                    reactive_attr: vec![ReactiveAttr {
                                        attribute_key: binding_attr.to_string(),
                                        content_of_attr: format!("{}.v", value),
                                        variable_names: vec![value.clone()],
                                    }],
                                },
                            ),
                        );
                    }
                    element.attributes.remove(key);
                } else if key.starts_with(":") {
                    // TODO: reconsider about this constraint
                    if key == ":innerHtml" {
                        Err(format!(":innerHtml is not supported"))?;
                    } else if key == ":textContent" {
                        Err(format!(":textContent is not supported"))?;
                    }
                    let id: String = set_id_for_needed_elm(element, needed_ids);
                    let raw_attr_name = &key[1..];
                    let raw_attr_value = action_value.clone();

                    let reactive_attr_info = find_reactive_attr_from_id(&id, elm_and_var_relation);

                    // if elm_and_var_relation includes elm_id

                    let reactive_attr_info = match reactive_attr_info {
                        Some(rel) => rel,
                        None => {
                            let rel2 = ElmAndReactiveAttributeRelation {
                                elm_id: id.clone(),
                                reactive_attr: vec![],
                            };
                            elm_and_var_relation
                                .push(ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(rel2));
                            find_reactive_attr_from_id(&id, elm_and_var_relation).unwrap()
                        }
                    };

                    // Check if the value is null
                    // TODO:要素のIndexを返すようにする
                    if raw_attr_value.is_none() {
                        Err(format!("value of attribute :{} is null", raw_attr_name))?;
                    }

                    let mut raw_attr_value = raw_attr_value.unwrap();

                    let (raw_attr_value, used_vars) =
                        append_v_to_vars(&mut raw_attr_value, varibale_names);

                    element.attributes.remove(key);
                    element.attributes.insert(
                        raw_attr_name.to_string(),
                        Some(format!("${{{}}}", raw_attr_value)),
                    );

                    let k = ReactiveAttr {
                        attribute_key: raw_attr_name.to_string(),
                        content_of_attr: raw_attr_value,
                        variable_names: used_vars,
                    };

                    reactive_attr_info.reactive_attr.push(k);
                }
            }

            for child_node in &mut element.children {
                check_html_elms(
                    varibale_names,
                    child_node,
                    needed_ids,
                    elm_and_var_relation,
                    actions_and_targets,
                    Some(&node.uuid),
                    html_manipulators,
                )?;
            }
            for manip in html_manipulators {
                if manip.target_uuid == node.uuid {
                    match &manip.manipulations {
                        HtmlManipulation::RemoveChildForIfStatement(remove_statement) => {
                            let id = set_id_for_needed_elm(element, needed_ids);
                            // TODO:実装する
                        }
                        HtmlManipulation::SetIdForReactiveContent(set_id) => {
                            let id = set_id_for_needed_elm(element, needed_ids);
                            elm_and_var_relation.push(ElmAndReactiveInfo::ElmAndVariableRelation(
                                ElmAndVariableContentRelation {
                                    elm_id: id,
                                    variable_names: set_id.depenent_vars.clone(),
                                    content_of_element: set_id.text.clone(),
                                },
                            ));
                        }
                    }
                }
            }
            Ok(())
        }
        NodeContent::TextNode(text) => {
            let dep_vars = replace_text_with_reactive_value(text, varibale_names);
            if dep_vars.len() > 0 {
                html_manipulators.push(HtmlManipulator {
                    target_uuid: parent_uuid.unwrap().clone(),
                    manipulations: HtmlManipulation::SetIdForReactiveContent(
                        SetIdForReactiveContent {
                            text: text.clone(),
                            depenent_vars: dep_vars,
                        },
                    ),
                });
            }
            Ok(())
        }
        crate::orig_html_struct::structs::NodeContent::Comment(_) => Ok(()),
    }
    // let node_len = *(&nodes.len().clone()) as u32;
    // for node in nodes {
    //     dep_vars = match node {
    //         Node::Element(element) => {
    //             for (key, action_value) in element.attributes.clone() {
    //                 // if attrs.name starts with "@"
    //                 if key.starts_with("@") {
    //                     let action_name = &key[1..];
    //                     let id: String = set_id_for_needed_elm(element, needed_ids);
    //                     if let Some(value) = &&action_value {
    //                         actions_and_targets.push(ActionAndTarget {
    //                             action_name: action_name.to_string(),
    //                             action: EventTarget::new(value.to_string(), varibale_names),
    //                             target: id,
    //                         })
    //                     }
    //                     element.attributes.remove(&key);
    //                 } else if key.starts_with("::") {
    //                     let binding_attr = &key[2..];
    //                     let id: String = set_id_for_needed_elm(element, needed_ids);
    //                     if let Some(value) = &&action_value {
    //                         actions_and_targets.push(ActionAndTarget {
    //                             action_name: "input".to_string(),
    //                             action: EventTarget::EventBindingStatement(EventBindingStatement {
    //                                 statement: format!(
    //                                     "{}.v = event.target.{}",
    //                                     &value, &binding_attr
    //                                 ),
    //                                 arg: "e".to_string(),
    //                             }),
    //                             target: id.clone(),
    //                         });
    //                         elm_and_var_relation.push(
    //                             ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(
    //                                 ElmAndReactiveAttributeRelation {
    //                                     elm_id: id,
    //                                     reactive_attr: vec![ReactiveAttr {
    //                                         attribute_key: binding_attr.to_string(),
    //                                         content_of_attr: format!("{}.v", value),
    //                                         variable_names: vec![value.clone()],
    //                                     }],
    //                                 },
    //                             ),
    //                         );
    //                     }
    //                     element.attributes.remove(&key);
    //                 } else if key.starts_with(":") {
    //                     if key == ":innerHtml" {
    //                         Err(format!(":innerHtml is not supported"))?;
    //                     } else if key == ":textContent" {
    //                         Err(format!(":textContent is not supported"))?;
    //                     }

    //                     let id: String = set_id_for_needed_elm(element, needed_ids);
    //                     let raw_attr_name = &key[1..];
    //                     let raw_attr_value = action_value.clone();

    //                     let reactive_attr_info =
    //                         find_reactive_attr_from_id(&id, elm_and_var_relation);

    //                     // if elm_and_var_relation includes elm_id

    //                     let reactive_attr_info = match reactive_attr_info {
    //                         Some(rel) => rel,
    //                         None => {
    //                             let rel2 = ElmAndReactiveAttributeRelation {
    //                                 elm_id: id.clone(),
    //                                 reactive_attr: vec![],
    //                             };
    //                             elm_and_var_relation.push(
    //                                 ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(rel2),
    //                             );
    //                             find_reactive_attr_from_id(&id, elm_and_var_relation).unwrap()
    //                         }
    //                     };

    //                     // Check if the value is null
    //                     // TODO:要素のIndexを返すようにする
    //                     if raw_attr_value.is_none() {
    //                         Err(format!("value of attribute :{} is null", raw_attr_name))?;
    //                     }

    //                     let mut raw_attr_value = raw_attr_value.unwrap();

    //                     let (raw_attr_value, used_vars) =
    //                         append_v_to_vars(&mut raw_attr_value, varibale_names);

    //                     element.attributes.remove(&key);
    //                     element.attributes.insert(
    //                         raw_attr_name.to_string(),
    //                         Some(format!("${{{}}}", raw_attr_value)),
    //                     );

    //                     let k = ReactiveAttr {
    //                         attribute_key: raw_attr_name.to_string(),
    //                         content_of_attr: raw_attr_value,
    //                         variable_names: used_vars,
    //                     };

    //                     reactive_attr_info.reactive_attr.push(k);
    //                 }
    //             }
    //             let var_deps = check_html_elms(
    //                 varibale_names,
    //                 &mut element.children,
    //                 needed_ids,
    //                 elm_and_var_relation,
    //                 actions_and_targets,
    //             )?;

    //             if var_deps.len() > 0 {
    //                 let id: String = set_id_for_needed_elm(element, needed_ids);
    //                 // TODO:以下のIf文のコーナーケースを考える
    //                 if element.children.len() == 1 && element.children[0].is_text() {
    //                     elm_and_var_relation.push(ElmAndReactiveInfo::ElmAndVariableRelation(
    //                         ElmAndVariableContentRelation {
    //                             elm_id: id,
    //                             variable_names: var_deps,
    //                             content_of_element: element.children[0].as_text().clone(),
    //                         },
    //                     ));
    //                 }
    //             }
    //             vec![]
    //         }
    //         Node::Text(text) => {
    //             let dep_vars = replace_text_with_reactive_value(text, varibale_names);
    //             if node_len == 1 {
    //                 dep_vars
    //             } else {
    //                 vec![]
    //             }
    //         }
    //         _ => vec![],
    //     };
    // }
}

fn set_id_for_needed_elm(element: &mut Element, needed_ids: &mut Vec<NeededIdName>) -> String {
    if let Some(Some(id)) = element.attributes.get("id") {
        let id = if needed_ids.iter().any(|x| x.id_name == id.clone()) {
            id.clone()
        } else {
            needed_ids.push(NeededIdName {
                id_name: id.clone(),
                to_delete: false,
            });
            id.clone()
        };
        id
    } else {
        let new_id = gen_nanoid();
        element
            .attributes
            .insert("id".to_string(), Some(new_id.clone()));
        needed_ids.push(NeededIdName {
            id_name: new_id.clone(),
            to_delete: true,
        });
        new_id
    }
}

// fn set_id_for_needed_elm(
//     node: Rc<RefCell<Node>>,
//     needed_ids: &mut Vec<NeededIdName>,
// ) -> Result<String, String> {
//     match &mut (*node).borrow_mut().content {
//         NodeContent::Element(elm) => {
//             if let Some(Some(id)) = elm.attributes.get("id") {
//                 let id = if needed_ids.iter().any(|x| x.id_name == id.clone()) {
//                     id.clone()
//                 } else {
//                     needed_ids.push(NeededIdName {
//                         id_name: id.clone(),
//                         to_delete: false,
//                     });
//                     id.clone()
//                 };
//                 Ok(id)
//             } else {
//                 let new_id = gen_nanoid();
//                 elm.attributes
//                     .insert("id".to_string(), Some(new_id.clone()));
//                 needed_ids.push(NeededIdName {
//                     id_name: new_id.clone(),
//                     to_delete: true,
//                 });
//                 Ok(new_id)
//             }
//         }
//         _ => Err(format!("set_id_for_needed_elm: node is not element")),
//     }
// }

// FIXME:カッコが複数でも、escapeTextは各バインディングに1つだけでいい
// 具体例:
// 現在:${escapeHtml(count.v+count.v)} count ${escapeHtml(count)} ${escapeHtml( count + count )}
// 将来的:${escapeHtml(`${count.v+count.v} count ${count} ${ count + count }`)}

// カッコが1つだけの場合、その部分のみをエスケープする
// Give: <div>    ${count} </div>
// Want: <div>    ${escapeHtml(count)} </div>
fn replace_text_with_reactive_value(code: &mut String, variables: &Vec<String>) -> Vec<String> {
    let start_tag = "${";
    let end_tag = "}";
    let mut new_code = String::new();
    let mut depending_vars = vec![];
    let mut last_end = 0;

    while let Some(start) = code[last_end..].find(start_tag) {
        let start = start + last_end;
        if let Some(end) = code[start..].find(end_tag) {
            let end = end + start;
            let pre_bracket = &code[last_end..start];
            let in_bracket = &code[start + 2..end];
            let _post_bracket = &code[end + 1..];

            new_code.push_str(pre_bracket);
            new_code.push_str(start_tag);
            let (output, dep_vars) = append_v_to_vars(in_bracket, variables);
            new_code.push_str(&escape_html(&output));
            new_code.push_str(end_tag);

            last_end = end + 1;

            depending_vars.extend(dep_vars);
        }
    }

    new_code.push_str(&code[last_end..]);
    *code = new_code;
    depending_vars
}

// test
#[cfg(test)]
mod tests {
    use super::replace_text_with_reactive_value;

    #[test]
    fn exploration() {
        let code = "escapeHtml(count2.v+count.v)";
        let mut code = code.clone().to_string();
        replace_text_with_reactive_value(
            &mut code,
            &vec!["count".to_string(), "count2".to_string()],
        );
        assert_eq!(code, "escapeHtml(count2.v+count.v)");
    }

    #[test]
    fn exploration2() {
        let code = "escapeHtml( count2.v + count.v )";
        let mut code = code.clone().to_string();
        replace_text_with_reactive_value(
            &mut code,
            &vec!["count".to_string(), "count2".to_string()],
        );
        assert_eq!(code, "escapeHtml( count2.v + count.v )");
    }

    #[test]
    fn exploration3() {
        let code = "${interval==null?'start':'clear'}";
        let mut code = code.clone().to_string();
        replace_text_with_reactive_value(&mut code, &vec!["interval".to_string()]);
        assert_eq!(
            code,
            "${escapeHtml(interval.v == null ? 'start' : 'clear')}"
        );
    }
}

fn escape_html(s: &str) -> String {
    format!("escapeHtml({})", s)
}

fn find_reactive_attr_from_id<'a>(
    id: &str,
    reactive_attrs: &'a mut Vec<ElmAndReactiveInfo>,
) -> Option<&'a mut ElmAndReactiveAttributeRelation> {
    reactive_attrs
        .iter_mut()
        .filter_map(|elm_and_var_relation| {
            if let ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(elm_and_var_relation) =
                elm_and_var_relation
            {
                Some(elm_and_var_relation)
            } else {
                None
            }
        })
        .find(|x| x.elm_id == id)
}

// pub fn check_html_elms(
//     varibale_names: &Vec<String>,
//     nodes: &mut Vec<Node>,
//     needed_ids: &mut Vec<NeededIdName>,
//     elm_and_var_relation: &mut Vec<ElmAndReactiveInfo>,
//     actions_and_targets: &mut Vec<ActionAndTarget>,
// ) -> Result<Vec<String>, String> {
//     let node_len = *(&nodes.len().clone()) as u32;
//     let mut dep_vars: Vec<String> = vec![];
//     for node in nodes {
//         dep_vars = match node {
//             Node::Element(element) => {
//                 for (key, action_value) in element.attributes.clone() {
//                     // if attrs.name starts with "@"
//                     if key.starts_with("@") {
//                         let action_name = &key[1..];
//                         let id: String = set_id_for_needed_elm(element, needed_ids);
//                         if let Some(value) = &&action_value {
//                             actions_and_targets.push(ActionAndTarget {
//                                 action_name: action_name.to_string(),
//                                 action: EventTarget::new(value.to_string(), varibale_names),
//                                 target: id,
//                             })
//                         }
//                         element.attributes.remove(&key);
//                     } else if key.starts_with("::") {
//                         let binding_attr = &key[2..];
//                         let id: String = set_id_for_needed_elm(element, needed_ids);
//                         if let Some(value) = &&action_value {
//                             actions_and_targets.push(ActionAndTarget {
//                                 action_name: "input".to_string(),
//                                 action: EventTarget::EventBindingStatement(EventBindingStatement {
//                                     statement: format!(
//                                         "{}.v = event.target.{}",
//                                         &value, &binding_attr
//                                     ),
//                                     arg: "e".to_string(),
//                                 }),
//                                 target: id.clone(),
//                             });
//                             elm_and_var_relation.push(
//                                 ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(
//                                     ElmAndReactiveAttributeRelation {
//                                         elm_id: id,
//                                         reactive_attr: vec![ReactiveAttr {
//                                             attribute_key: binding_attr.to_string(),
//                                             content_of_attr: format!("{}.v", value),
//                                             variable_names: vec![value.clone()],
//                                         }],
//                                     },
//                                 ),
//                             );
//                         }
//                         element.attributes.remove(&key);
//                     } else if key.starts_with(":") {
//                         if key == ":innerHtml" {
//                             Err(format!(":innerHtml is not supported"))?;
//                         } else if key == ":textContent" {
//                             Err(format!(":textContent is not supported"))?;
//                         }

//                         let id: String = set_id_for_needed_elm(element, needed_ids);
//                         let raw_attr_name = &key[1..];
//                         let raw_attr_value = action_value.clone();

//                         let reactive_attr_info =
//                             find_reactive_attr_from_id(&id, elm_and_var_relation);

//                         // if elm_and_var_relation includes elm_id

//                         let reactive_attr_info = match reactive_attr_info {
//                             Some(rel) => rel,
//                             None => {
//                                 let rel2 = ElmAndReactiveAttributeRelation {
//                                     elm_id: id.clone(),
//                                     reactive_attr: vec![],
//                                 };
//                                 elm_and_var_relation.push(
//                                     ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(rel2),
//                                 );
//                                 find_reactive_attr_from_id(&id, elm_and_var_relation).unwrap()
//                             }
//                         };

//                         // Check if the value is null
//                         // TODO:要素のIndexを返すようにする
//                         if raw_attr_value.is_none() {
//                             Err(format!("value of attribute :{} is null", raw_attr_name))?;
//                         }

//                         let mut raw_attr_value = raw_attr_value.unwrap();

//                         let (raw_attr_value, used_vars) =
//                             append_v_to_vars(&mut raw_attr_value, varibale_names);

//                         element.attributes.remove(&key);
//                         element.attributes.insert(
//                             raw_attr_name.to_string(),
//                             Some(format!("${{{}}}", raw_attr_value)),
//                         );

//                         let k = ReactiveAttr {
//                             attribute_key: raw_attr_name.to_string(),
//                             content_of_attr: raw_attr_value,
//                             variable_names: used_vars,
//                         };

//                         reactive_attr_info.reactive_attr.push(k);
//                     }
//                 }
//                 let var_deps = check_html_elms(
//                     varibale_names,
//                     &mut element.children,
//                     needed_ids,
//                     elm_and_var_relation,
//                     actions_and_targets,
//                 )?;

//                 if var_deps.len() > 0 {
//                     let id: String = set_id_for_needed_elm(element, needed_ids);
//                     // TODO:以下のIf文のコーナーケースを考える
//                     if element.children.len() == 1 && element.children[0].is_text() {
//                         elm_and_var_relation.push(ElmAndReactiveInfo::ElmAndVariableRelation(
//                             ElmAndVariableContentRelation {
//                                 elm_id: id,
//                                 variable_names: var_deps,
//                                 content_of_element: element.children[0].as_text().clone(),
//                             },
//                         ));
//                     }
//                 }
//                 vec![]
//             }
//             Node::Text(text) => {
//                 let dep_vars = replace_text_with_reactive_value(text, varibale_names);
//                 if node_len == 1 {
//                     dep_vars
//                 } else {
//                     vec![]
//                 }
//             }
//             _ => vec![],
//         };
//     }
//     Ok(dep_vars)
// }
