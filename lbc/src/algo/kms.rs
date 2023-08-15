use bip39::{Language, Mnemonic, MnemonicType};

use serde_json::Value;

use xuanmi_base_support::*;

/**
 * 创建助记词
 * argument 0: lang, 语言类型，可选值: en, zh-cn
 * argument 1: word_count, 单词数量，可选值: 12, 15, 18, 21, 24
 */
pub fn create_mnemonic(lang_code: String, size: u8) -> Outcome<Value> {
    let word_count = MnemonicType::for_word_count(size.into()).catch_()?;
    let language = Language::from_language_code(lang_code.as_str())
        .ok_or(bip39::ErrorKind::InvalidWord)
        .catch_()?;
    let mnemonic = Mnemonic::new(word_count, language);
    Ok(Value::String(mnemonic.phrase().to_string()))
}
