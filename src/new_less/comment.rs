use crate::extend::string::StringExtend;
use crate::new_less::block::{OriginBlock, OriginBlockType};
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::option::{OptionExtend, ParseOption};

pub trait Comment {
  fn parse_comment(&self) -> Result<Vec<OriginBlock>, String>;
  fn get_comment_blocknode(&self) -> Vec<OriginBlock>;
  fn rm_comment(&self) -> String;
}

impl Comment for FileInfo {
  fn parse_comment(&self) -> Result<Vec<OriginBlock>, String> {
    parse_comment(self.get_options(), &self.origin_charlist, &self.locmap)
  }
  
  fn get_comment_blocknode(&self) -> Vec<OriginBlock> {
    get_comment_blocknode(&self.block_node)
  }
  
  fn rm_comment(&self) -> String {
    let list = &self.get_comment_blocknode();
    return if !list.is_empty() {
      rm_comment(list, &self.origin_charlist)
    } else {
      self.origin_txt_content.clone()
    };
  }
}

impl Comment for OriginBlock {
  fn parse_comment(&self) -> Result<Vec<OriginBlock>, String> {
    parse_comment(self.get_options(), &self.origin_charlist, &self.locmap)
  }
  
  fn get_comment_blocknode(&self) -> Vec<OriginBlock> {
    return if self.block_node.is_some() {
      get_comment_blocknode(&self.block_node.as_ref().unwrap())
    } else {
      vec![]
    };
  }
  
  fn rm_comment(&self) -> String {
    let node_list = &self.get_comment_blocknode();
    return if !node_list.is_empty() {
      rm_comment(node_list, &self.origin_charlist)
    } else {
      self.content.clone()
    };
  }
}

///
/// 获取一段 文件中 注释
///
fn parse_comment(options: &ParseOption, origin_charlist: &Vec<String>, locmap: &Option<LocMap>) -> Result<Vec<OriginBlock>, String> {
  let mut blocklist: Vec<OriginBlock> = vec![];
  let mut commentlist: Vec<String> = vec![];
  
  // 是否在 注释 存入中
  let mut wirte_comment = false;
  let mut wirte_line_comment = false;
  let mut wirte_closure_comment = false;
  
  // 块等级
  let mut braces_level = 0;
  
  // 结束标记 & 开始标记
  let start_braces = "{".to_string();
  let end_braces = "}".to_string();
  // 注释的内容共
  let comment_flag = "//".to_string();
  let comment_mark_strat = "/*".to_string();
  let comment_mark_end = "*/".to_string();
  
  // 如果启用 sourcemap 则用来记录坐标
  let mut record_loc: Option<Loc> = None;
  
  let mut index = 0;
  while index < origin_charlist.len() {
    // 处理字符
    let char = origin_charlist.get(index).unwrap().clone();
    let next_char;
    if index != origin_charlist.len() - 1 {
      next_char = origin_charlist.get(index + 1).unwrap().clone();
    } else {
      next_char = "".to_string()
    }
    
    // 优先检测注释 与当前块 等级 相同 为 0
    let word = char.clone() + &next_char;
    if word == comment_flag && braces_level == 0 && !wirte_comment {
      wirte_comment = true;
      wirte_line_comment = true;
    } else if word == comment_mark_strat && braces_level == 0 && !wirte_comment {
      wirte_comment = true;
      wirte_closure_comment = true;
    }
    if braces_level == 0 &&
      wirte_comment &&
      (
        (wirte_line_comment && (&char == "\n" || &char == "\r")) ||
          (wirte_closure_comment && word == comment_mark_end)
      ) {
      wirte_comment = false;
      if wirte_line_comment {
        index += 1;
        commentlist.push(char.clone());
        wirte_line_comment = false;
      } else if wirte_closure_comment {
        index += 2;
        commentlist.push(word.clone());
        wirte_closure_comment = false;
      }
      let comment = OriginBlock::create_comment(commentlist.join(""), record_loc.unwrap(), None, options.clone(), None);
      blocklist.push(comment);
      commentlist.clear();
      record_loc = None;
      continue;
    }
    if wirte_comment {
      // 如果启用 sourcemap 则记录坐标
      if options.sourcemap && char != "\r" && char != "\n" && record_loc.is_none() {
        record_loc = Some(locmap.as_ref().unwrap().get(index).unwrap());
      }
      commentlist.push(char.clone());
    }
    // ignore 忽略 大括号区域
    if char == start_braces {
      braces_level += 1;
    }
    if char == end_braces {
      braces_level -= 1;
    }
    index += 1;
  }
  
  if braces_level != 0 {
    return Err("the content contains braces that are not closed!".to_string());
  }
  Ok(blocklist)
}

///
/// 从当中的 成熟 AST 中获取 注释节点
///
fn get_comment_blocknode(block_node: &Vec<OriginBlock>) -> Vec<OriginBlock> {
  block_node
    .into_iter()
    .filter(|&node| node.block_type == OriginBlockType::Comment)
    .map(|c| c.clone())
    .collect::<Vec<OriginBlock>>()
}

///
/// 移除注释
///
fn rm_comment(commentlist: &Vec<OriginBlock>, origin_charlist: &Vec<String>) -> String {
  return if commentlist.is_empty() {
    origin_charlist.join("")
  } else {
    let mut charlist = origin_charlist.clone();
    for cc in commentlist {
      let length = cc.content.len();
      let start = cc.loc.index;
      let end = cc.loc.index + length;
      let mut i = start;
      while i < end {
        println!(".........{}", i);
        let char = charlist.get(i).unwrap();
        if char != "\n" && char != "\r" {
          charlist[i] = " ".to_string();
        }
        i += 1;
      }
      println!("{}", charlist.join(""));
    }
    charlist.join("")
  };
}

///
/// 移除注释
///
fn pure_context(content: String) {

  let list = content.tocharlist();
  
  
}





