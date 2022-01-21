use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::new_less::token::lib::{Token};

///
/// Select 合词字符串
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenSelect {
  #[strum(serialize = ".")]
  ClassToken,
  
  #[strum(serialize = "#")]
  IdToken,
  
  #[strum(serialize = "[")]
  AttrBegin,
  
  #[strum(serialize = "]")]
  AttrEnd,
  
  #[strum(serialize = "*")]
  WildCard,
  
  #[strum(serialize = ":")]
  Colon,
}


///
/// Select 允许的连接符
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenCombina {
  #[strum(serialize = ",")]
  Comma,
  
  #[strum(serialize = " ")]
  Space,
  
  #[strum(serialize = "\n")]
  NewLineOs,
  
  #[strum(serialize = "\r")]
  NewLineWindos,
  
  #[strum(serialize = ">")]
  ExtendChar,
  
  #[strum(serialize = "|")]
  ColumnChar,
  
  #[strum(serialize = "+")]
  BrotherNextChar,
  
  #[strum(serialize = "~")]
  BrotherMatchChar,
  
}

///
/// Select 允许的 安全字符符
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenAllow {
  #[strum(serialize = r"\")]
  LeftSlant,
  
  #[strum(serialize = "_")]
  Underscore,
  
  #[strum(serialize = "-")]
  Dash,
}

impl EnumExtend for TokenSelect {}

impl EnumExtend for TokenCombina {}

impl EnumExtend for TokenAllow {}

impl StringInto for TokenSelect {}

impl StringInto for TokenCombina {}

impl StringInto for TokenAllow {}

pub trait SelectTokenParse {
  fn token_selector_forbidden() -> Vec<String>;
}

impl SelectTokenParse for Token {
  fn token_selector_forbidden() -> Vec<String> {
    let tokenlist = Token::get_token();
    let mut list_select = TokenSelect::enum_vec();
    let mut list_combina = TokenCombina::enum_vec();
    let mut list = vec![];
    list.append(&mut list_select);
    list.append(&mut list_combina);
    let mut other = vec![];
    tokenlist.into_iter().for_each(|token| {
      match list.clone().into_iter().find(|x| *x == token) {
        None => {
          other.push(token);
        }
        Some(_) => {}
      }
    });
    other
  }
}
