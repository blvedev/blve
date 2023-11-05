use super::transform_info::IfBlockInfo;

// TODO: 2つの共通のフィールドを持つ構造体を作る
#[derive(Debug)]
pub enum ElmAndReactiveInfo {
    ElmAndVariableRelation(ElmAndVariableContentRelation),
    ElmAndReactiveAttributeRelation(ElmAndReactiveAttributeRelation),
}

#[derive(Debug, Clone)]
pub struct ElmAndVariableContentRelation {
    pub elm_id: String,
    pub variable_names: Vec<String>,
    pub content_of_element: String,
    pub ctx: Vec<String>,
    pub elm_loc: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct ElmAndReactiveAttributeRelation {
    pub elm_id: String,
    pub reactive_attr: Vec<ReactiveAttr>,
    pub ctx: Vec<String>,
    pub elm_loc: Vec<usize>,
}

pub fn sort_elm_and_reactive_info(_self: &mut Vec<ElmAndReactiveInfo>) {
    _self.sort_by(|a, b| {
        let a_elm_loc = match a {
            ElmAndReactiveInfo::ElmAndVariableRelation(elm_and_var) => elm_and_var.elm_loc.clone(),
            ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(elm_and_reactive_attr) => {
                elm_and_reactive_attr.elm_loc.clone()
            }
        };
        let b_elm_loc = match b {
            ElmAndReactiveInfo::ElmAndVariableRelation(elm_and_var) => elm_and_var.elm_loc.clone(),
            ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(elm_and_reactive_attr) => {
                elm_and_reactive_attr.elm_loc.clone()
            }
        };
        a_elm_loc.cmp(&b_elm_loc)
    });
}

impl ElmAndVariableContentRelation {
    pub fn generate_ctx_num(&self, if_blocks_infos: &Vec<IfBlockInfo>) -> usize {
        let mut ctx_num: u64 = 0;
        for (index, if_blk) in if_blocks_infos.iter().enumerate() {
            if self.ctx.contains(&if_blk.parent_id) {
                let blk_num: u64 = (2 as u64).pow(index as u32);
                ctx_num = ctx_num | blk_num;
            }
        }
        ctx_num as usize
    }
}

impl ElmAndReactiveAttributeRelation {
    pub fn generate_ctx_num(&self, if_blocks_infos: &Vec<IfBlockInfo>) -> usize {
        let mut ctx_num: u64 = 0;
        for (index, if_blk) in if_blocks_infos.iter().enumerate() {
            if self.ctx.contains(&if_blk.parent_id) {
                let blk_num: u64 = (2 as u64).pow(index as u32);
                ctx_num = ctx_num | blk_num;
            }
        }
        ctx_num as usize
    }
}

#[derive(Debug, Clone)]
pub struct ReactiveAttr {
    pub attribute_key: String,
    pub content_of_attr: String,
    pub variable_names: Vec<String>,
}
