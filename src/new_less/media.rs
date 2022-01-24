use crate::extend::string::StringExtend;
use serde::{Serialize};
use crate::extend::enum_extend::EnumExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::token::lib::Token;
use crate::new_less::token::media::TokenMeidaAllow;


///
/// 媒体查询
///
#[derive(Debug, Clone, Serialize)]
pub struct MediaQuery {
  pub origin_txt: String,
  charlist: Vec<String>,
}

impl MediaQuery {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String) -> Result<Self, String> {
    let mut obj = Self {
      origin_txt: txt.clone(),
      charlist: txt.trim().to_string().tocharlist(),
    };
    match obj.parse() {
      Ok(_) => {
        Ok(obj)
      }
      Err(msg) => {
        Err(msg)
      }
    }
  }
  
  ///
  /// 打印错误信息
  ///
  pub fn errormsg(&mut self, index: &usize) -> Result<(), String> {
    let char = self.charlist.get(*index).unwrap().clone();
    Err(format!("select text {}, char {} is not allow,index is {}", self.origin_txt, char, index))
  }
  
  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }
  
  
  pub fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist;
    let mut temp: String = "".to_string();
    
    if charlist.len() < 6 || charlist[0..6].poly().as_str() != "@media" {
      return Err(format!("select_txt not match media query"));
    }
    let mut word_vec = vec!["@media".to_string()];
    let mut index = 6;
    
    // 循环解析
    while index < charlist.len() {
      let char = charlist.get(index).unwrap().to_string();
      if Token::is_token(&char) {
        if Token::is_space_token(&char) {
          // todo! 验收词
          word_vec.push(temp.clone())
        } else if TokenMeidaAllow::is(&char) {
          match TokenMeidaAllow::try_from(char.as_str()).unwrap() {
            TokenMeidaAllow::LeftBrackets => {
              // todo ! 跳变分析 feature
            }
            _ => {
              return Err(self.errormsg(&index).err().unwrap());
            }
          }
        } else {
          return Err(self.errormsg(&index).err().unwrap());
        }
      } else {
        temp += &char;
      }
      index += 1;
    }
    Err("".to_string())
  }
}