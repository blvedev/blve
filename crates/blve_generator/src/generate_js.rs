use std::collections::HashSet;

use blve_parser::DetailedBlock;

use crate::{
    orig_html_struct::structs::{Node, NodeContent},
    structs::{
        transform_info::{
            sort_if_blocks, ActionAndTarget, IfBlockInfo, NeededIdName, TextNodeRendererGroup,
            VariableNameAndAssignedNumber,
        },
        transform_targets::{sort_elm_and_reactive_info, NodeAndReactiveInfo},
    },
    transformers::{html_utils::check_html_elms, js_utils::analyze_js},
};

pub fn generate_js_from_blocks(
    blocks: &DetailedBlock,
    no_export: Option<bool>,
    runtime_path: Option<String>,
) -> Result<(String, Option<String>), String> {
    let no_export = match no_export.is_none() {
        true => false,
        false => no_export.unwrap(),
    };
    let runtime_path = match runtime_path.is_none() {
        true => "blve/dist/runtime".to_string(),
        false => runtime_path.unwrap(),
    };

    // Analyze JavaScript
    let (variables, variable_names, js_output) = analyze_js(blocks);

    // Clone HTML as mutable reference
    let mut needed_id = vec![];

    let mut elm_and_var_relation = vec![];
    let mut action_and_target = vec![];
    let mut if_blocks_info = vec![];
    let mut text_node_renderer = vec![];

    let mut new_node = Node::new_from_dom(&blocks.detailed_language_blocks.dom)?;

    // Analyze HTML
    check_html_elms(
        &variable_names,
        &mut new_node,
        &mut needed_id,
        &mut elm_and_var_relation,
        &mut action_and_target,
        None,
        &mut vec![],
        &mut if_blocks_info,
        &mut text_node_renderer,
        &vec![],
        &vec![0],
        1,
    )?;

    sort_if_blocks(&mut if_blocks_info);
    sort_elm_and_reactive_info(&mut elm_and_var_relation);

    let html_str = new_node.to_string();

    // Generate JavaScript
    let html_insert = format!("elm.innerHTML = `{}`;", html_str);

    let mut txt_node_renderer = TextNodeRendererGroup::new(&if_blocks_info, &text_node_renderer);
    let create_anchor_statements = gen_create_anchor_statements(&mut txt_node_renderer, &vec![]);
    let ref_getter_expression = gen_ref_getter_from_needed_ids(&needed_id);
    let event_listener_codes = create_event_listener(action_and_target);
    let mut codes = vec![js_output, html_insert, ref_getter_expression];

    // TODO:他の処理同様、ここも関数に切り出す
    if if_blocks_info.len() > 0 {
        let mut variables_to_declare = HashSet::new();
        for if_block_info in &if_blocks_info {
            variables_to_declare.insert(if_block_info.if_block_id.clone());
            let new_ctx_under_if = {
                let mut ctx = if_block_info.ctx.clone();
                ctx.push(if_block_info.if_block_id.clone());
                ctx
            };
            for needed_id in needed_id.iter() {
                if needed_id.ctx == new_ctx_under_if {
                    variables_to_declare.insert(needed_id.node_id.clone());
                }
            }
        }

        if variables_to_declare.len() != 0 {
            let decl = format!(
                "let {};",
                itertools::join(
                    variables_to_declare.iter().map(|v| format!("{}Ref", v)),
                    ", "
                )
            );
            codes.push(decl);
        }
    }

    codes.extend(create_anchor_statements);
    codes.extend(event_listener_codes);
    let render_if = gen_render_if_statements(&if_blocks_info, &needed_id);
    codes.extend(render_if);
    codes.push("refs[4] = 0".to_string());
    let update_func_code =
        gen_update_func_statement(elm_and_var_relation, variables, if_blocks_info);
    codes.push(update_func_code);
    let full_code = gen_full_code(codes, no_export, runtime_path);
    let css_code = blocks.detailed_language_blocks.css.clone();

    Ok((full_code, css_code))
}

fn gen_full_code(codes: Vec<String>, no_export: bool, runtime_path: String) -> String {
    let func_decl = if no_export {
        "const App = ".to_string()
    } else {
        "export default ".to_string()
    };

    // codesにcreate_indentを適用して、\nでjoinする -> code
    let code = codes
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        r#"import {{ reactiveValue, getElmRefs, addEvListener, genUpdateFunc, escapeHtml, replaceInnerText, replaceText, replaceAttr, insertEmpty,insertContent }} from '{}'

{}function(elm) {{
    const refs = [null, false, 0, 0, 0];
{}
}}"#,
        runtime_path, func_decl, code,
    )
}

// TODO: インデントの種類を入力によって変えられるようにする
fn create_indent(string: &str) -> String {
    let mut output = "".to_string();
    let indent = "    ";
    for (i, line) in string.lines().into_iter().enumerate() {
        match line == "" {
            true => {}
            false => {
                output.push_str(indent);
                output.push_str(line);
            }
        }
        if i != string.lines().into_iter().count() - 1 {
            output.push_str("\n");
        }
    }
    output
}

fn gen_ref_getter_from_needed_ids(needed_ids: &Vec<NeededIdName>) -> String {
    let mut ref_getter_str = "const [".to_string();
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.ctx.len() == 0)
            .map(|id| format!("{}Ref", id.node_id))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str(),
    );
    ref_getter_str.push_str("] = getElmRefs([");
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.ctx.len() == 0)
            .map(|id| format!("\"{}\"", id.id_name))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str(),
    );
    let delete_id_bool_map = needed_ids
        .iter()
        .filter(|id: &&NeededIdName| id.ctx.len() == 0)
        .map(|id| id.to_delete)
        .collect::<Vec<bool>>();
    let delete_id_map = gen_binary_map_from_bool(delete_id_bool_map);
    ref_getter_str.push_str(format!("], {map});", map = delete_id_map).as_str());
    ref_getter_str
}

fn gen_binary_map_from_bool(bools: Vec<bool>) -> u32 {
    let mut result = 0;
    for (i, &value) in bools.iter().enumerate() {
        if value {
            result |= 1 << (bools.len() - i - 1);
        }
    }
    result
}

fn create_event_listener(actions_and_targets: Vec<ActionAndTarget>) -> Vec<String> {
    let mut result = vec![];
    for action_and_target in actions_and_targets {
        result.push(format!(
            "addEvListener({}Ref, \"{}\", {});",
            action_and_target.target,
            action_and_target.action_name,
            action_and_target.action.to_string()
        ));
    }
    result
}

fn gen_update_func_statement(
    elm_and_variable_relations: Vec<NodeAndReactiveInfo>,
    variable_name_and_assigned_numbers: Vec<VariableNameAndAssignedNumber>,
    if_blocks_infos: Vec<IfBlockInfo>,
) -> String {
    let mut replace_statements = vec![];

    for (index, if_block_info) in if_blocks_infos.iter().enumerate() {
        let if_blk_rendering_cond = if if_block_info.ctx.len() != 0 {
            format!(
                "(!((refs[3] & {0}) ^ {0})) && ",
                if_block_info.generate_ctx_num(&if_blocks_infos)
            )
        } else {
            "".to_string()
        };

        let dep_vars = &if_block_info.condition_dep_vars;

        // TODO: データバインディングと同じコードを使っているので共通化する
        let dep_vars_assined_numbers = variable_name_and_assigned_numbers
            .iter()
            .filter(|v| {
                dep_vars
                    .iter()
                    .map(|d| *d == v.name)
                    .collect::<Vec<bool>>()
                    .contains(&true)
            })
            .map(|v| v.assignment)
            .collect::<Vec<u32>>();

        let combined_number = get_combined_binary_number(dep_vars_assined_numbers);

        replace_statements.push(format!(
            "{}refs[2] & {} && ( {} ? {} : ({}, {}, {}) );",
            if_blk_rendering_cond,
            combined_number,
            if_block_info.condition,
            format!("render{}Elm()", &if_block_info.if_block_id),
            format!("{}Ref.remove()", &if_block_info.if_block_id),
            format!("{}Ref = null", &if_block_info.if_block_id),
            format!("refs[3] ^= {}", index + 1),
        ));
    }

    for elm_and_variable_relation in elm_and_variable_relations {
        match elm_and_variable_relation {
            NodeAndReactiveInfo::ElmAndReactiveAttributeRelation(elm_and_attr_relation) => {
                let _elm_and_attr_relation = elm_and_attr_relation.clone();
                for c in elm_and_attr_relation.reactive_attr {
                    let dep_vars_assined_numbers = variable_name_and_assigned_numbers
                        .iter()
                        .filter(|v| {
                            c.variable_names
                                .iter()
                                .map(|d| *d == v.name)
                                .collect::<Vec<bool>>()
                                .contains(&true)
                        })
                        .map(|v| v.assignment)
                        .collect::<Vec<u32>>();

                    let if_blk_rendering_cond = if elm_and_attr_relation.ctx.len() != 0 {
                        format!(
                            "(!((refs[3] & {0}) ^ {0})) && ",
                            _elm_and_attr_relation.generate_ctx_num(&if_blocks_infos)
                        )
                    } else {
                        "".to_string()
                    };

                    replace_statements.push(format!(
                        "{}refs[2] & {:?} && replaceAttr(\"{}\", {}, {}Ref);",
                        if_blk_rendering_cond,
                        get_combined_binary_number(dep_vars_assined_numbers),
                        c.attribute_key,
                        c.content_of_attr,
                        elm_and_attr_relation.elm_id
                    ));
                }
            }
            NodeAndReactiveInfo::ElmAndVariableRelation(elm_and_variable_relation) => {
                let depending_variables = elm_and_variable_relation.dep_vars.clone();
                let target_id = elm_and_variable_relation.elm_id.clone();

                let dep_vars_assined_numbers = variable_name_and_assigned_numbers
                    .iter()
                    .filter(|v| {
                        depending_variables
                            .iter()
                            .map(|d| *d == v.name)
                            .collect::<Vec<bool>>()
                            .contains(&true)
                    })
                    .map(|v| v.assignment)
                    .collect::<Vec<u32>>();
                let under_if_blk = elm_and_variable_relation.ctx.len() != 0;

                let if_blk_rendering_cond = if under_if_blk {
                    format!(
                        "(!((refs[3] & {0}) ^ {0})) && ",
                        elm_and_variable_relation.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    "".to_string()
                };

                let combined_number = get_combined_binary_number(dep_vars_assined_numbers);

                let to_update_cond = if under_if_blk {
                    format!(
                        "(refs[2] & {:?} && ((refs[4] & {1}) ^ {1}) )",
                        combined_number,
                        elm_and_variable_relation.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    format!("refs[2] & {:?}", combined_number)
                };

                replace_statements.push(format!(
                    "{}{} && replaceInnerText(`{}`, {}Ref);",
                    if_blk_rendering_cond,
                    to_update_cond,
                    elm_and_variable_relation.content_of_element.trim(),
                    target_id
                ));
            }
            NodeAndReactiveInfo::TextAndVariableContentRelation(txt_and_var_content) => {
                // TODO: Elementとほとんど同じなので、共通化

                let depending_variables = txt_and_var_content.dep_vars.clone();
                let target_id = txt_and_var_content.text_node_id.clone();

                let dep_vars_assined_numbers = variable_name_and_assigned_numbers
                    .iter()
                    .filter(|v| {
                        depending_variables
                            .iter()
                            .map(|d| *d == v.name)
                            .collect::<Vec<bool>>()
                            .contains(&true)
                    })
                    .map(|v| v.assignment)
                    .collect::<Vec<u32>>();
                let under_if_blk = txt_and_var_content.ctx.len() != 0;

                let if_blk_rendering_cond = if under_if_blk {
                    format!(
                        "(!((refs[3] & {0}) ^ {0})) && ",
                        txt_and_var_content.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    "".to_string()
                };

                let combined_number = get_combined_binary_number(dep_vars_assined_numbers);

                let to_update_cond = if under_if_blk {
                    format!(
                        "(refs[2] & {:?} && ((refs[4] & {1}) ^ {1}) )",
                        combined_number,
                        txt_and_var_content.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    format!("refs[2] & {:?}", combined_number)
                };

                replace_statements.push(format!(
                    "{}{} && replaceText(`{}`, {}Text);",
                    if_blk_rendering_cond,
                    to_update_cond,
                    txt_and_var_content.content_of_element.trim(),
                    target_id
                ));
            }
        }
    }

    let code = replace_statements
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");

    let result = format!(
        r#"refs[0] = genUpdateFunc(() => {{
{code}
}});"#,
        code = code
    );

    result
}

fn gen_create_anchor_statements(
    text_node_renderer: &mut TextNodeRendererGroup,
    ctx_condition: &Vec<String>,
) -> Vec<String> {
    let mut create_anchor_statements = vec![];
    text_node_renderer.sort_by_rendering_order();
    for render in &text_node_renderer.renderers {
        match render {
            crate::structs::transform_info::TextNodeRenderer::ManualRenderer(txt_renderer) => {
                if &txt_renderer.ctx != ctx_condition {
                    continue;
                }
                let anchor_id = match &txt_renderer.target_anchor_id {
                    Some(anchor_id) => format!("{}Ref", anchor_id),
                    None => "null".to_string(),
                };
                let create_anchor_statement = format!(
                    "const {}Text = insertContent(`{}`,{}Ref,{});",
                    &txt_renderer.text_node_id,
                    &txt_renderer.content.trim(),
                    &txt_renderer.parent_id,
                    anchor_id
                );
                create_anchor_statements.push(create_anchor_statement);
            }
            crate::structs::transform_info::TextNodeRenderer::IfBlockRenderer(if_block) => {
                match if_block.distance_to_next_elm > 1 {
                    true => {
                        if &if_block.ctx != ctx_condition {
                            continue;
                        }
                        let anchor_id = match &if_block.target_anchor_id {
                            Some(anchor_id) => format!("{}Ref", anchor_id),
                            None => "null".to_string(),
                        };
                        let create_anchor_statement = format!(
                            "const {}Anchor = insertEmpty({}Ref,{});",
                            if_block.if_block_id, if_block.parent_id, anchor_id
                        );
                        create_anchor_statements.push(create_anchor_statement);
                    }
                    false => {}
                }
            }
        }
    }
    create_anchor_statements
}

fn gen_render_if_statements(
    if_block_info: &Vec<IfBlockInfo>,
    needed_ids: &Vec<NeededIdName>,
) -> Vec<String> {
    let mut render_if = vec![];

    for (index, if_block) in if_block_info.iter().enumerate() {
        let (name, js_gen_elm_code_arr) = match &if_block.elm.content {
            NodeContent::Element(elm) => elm.generate_element_on_js(&if_block.if_block_id),
            _ => panic!(),
        };
        let insert_elm = match if_block.distance_to_next_elm > 1 {
            true => format!(
                "{}Ref.insertBefore({}, {}Anchor);",
                if_block.parent_id, name, if_block.if_block_id
            ),
            false => match if_block.target_anchor_id {
                Some(_) => format!(
                    "{}Ref.insertBefore({}, {}Ref);",
                    if_block.parent_id,
                    name,
                    if_block.target_anchor_id.as_ref().unwrap().clone()
                ),
                None => format!("{}Ref.insertBefore({}, null);", if_block.parent_id, name),
            },
        };

        // TODO:一連の生成コードを、need_idのmethodとして関数にまとめる
        let current_blk_ctx = {
            let mut new_ctx = if_block.ctx.clone();
            new_ctx.push(if_block.if_block_id.clone());
            new_ctx
        };
        let filtered = needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.ctx == current_blk_ctx)
            .filter(|id: &&NeededIdName| id.node_id != if_block.if_block_id);

        let ref_getter_str = if filtered.clone().count() > 0 {
            // TODO:format!などを使ってもっとみやすいコードを書く
            let mut ref_getter_str = "\n[".to_string();

            ref_getter_str.push_str(
                filtered
                    .clone()
                    .map(|id| format!("{}Ref", id.node_id))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_str(),
            );
            ref_getter_str.push_str("] = getElmRefs([");
            ref_getter_str.push_str(
                filtered
                    .clone()
                    .map(|id| format!("\"{}\"", id.id_name))
                    .collect::<Vec<String>>()
                    .join(",")
                    .as_str(),
            );
            let delete_id_bool_map = needed_ids
                .iter()
                .filter(|id: &&NeededIdName| id.ctx == current_blk_ctx)
                .map(|id| id.to_delete)
                .collect::<Vec<bool>>();
            let delete_id_map = gen_binary_map_from_bool(delete_id_bool_map);
            ref_getter_str.push_str(format!("], {map});", map = delete_id_map).as_str());

            ref_getter_str
        } else {
            "".to_string()
        };

        let children = if_block.find_children(&if_block_info);

        let child_block_rendering_exec = if children.len() != 0 {
            let mut rendering_statement = "\n".to_string();
            let mut child_block_rendering_exec = vec![];
            for child_if in children {
                child_block_rendering_exec.push(format!(
                    "{} && render{}Elm()",
                    child_if.condition, &child_if.if_block_id
                ));
            }
            rendering_statement.push_str(child_block_rendering_exec.join("\n").as_str());
            rendering_statement
        } else {
            "".to_string()
        };

        // TODO: 一連の処理を関数にまとめる
        // TODO: CreateIndentを複数行に対応させる
        let js_gen_elm_code = js_gen_elm_code_arr
            .iter()
            .map(|c| create_indent(c))
            .collect::<Vec<String>>()
            .join("\n");
        let blk_num: u64 = (2 as u64).pow(index as u32);
        // TODO: {}の前後に改行があったりなかったりするので、統一する
        render_if.push(format!(
            r#"const render{}Elm = () => {{
{}
{}
{}{}{}
}}"#,
            &if_block.if_block_id,
            js_gen_elm_code,
            create_indent(insert_elm.as_str()),
            create_indent(format!("refs[3] |= {}, refs[4] |= {};", blk_num, blk_num).as_str()),
            create_indent(ref_getter_str.as_str()),
            create_indent(child_block_rendering_exec.as_str())
        ));
        if if_block.ctx.len() == 0 {
            render_if.push(format!(
                "{} && render{}Elm()",
                if_block.condition, &if_block.if_block_id
            ));
        }
    }
    render_if
}

/// Returns a binary number that is the result of ORing all the numbers in the argument.
/// ```
/// let numbers = vec![0b0001, 0b0010, 0b0100];
/// let result = get_combined_binary_number(numbers);
/// assert_eq!(result, 0b0111);
/// ```
fn get_combined_binary_number(numbers: Vec<u32>) -> u32 {
    let mut result = 0;
    for (_, &value) in numbers.iter().enumerate() {
        result |= value;
    }
    result
}
