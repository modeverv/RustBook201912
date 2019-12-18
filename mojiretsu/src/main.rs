fn main() {
    println!("Hello, world!");
    // 00 
    println!("{}",reverse("abc"));
    // 01. 「パタトクカシーー」
    println!("{}",odd_chars("パタトクカシーー"));
    // 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
    println!("{}",concat_alternately("パトカー", "タクシー"));
    // 03. 円周率
    println!("{:?}",pi_from_text());
    // 04. 元素記号
    println!("{:?}",element_symbols());
    // contains
    {
        let a = "hello world";
        if a.contains("world"){
            println!("{}","contains");
        }
    }
}
/// 00. 文字列の逆順
///　逆に（末尾から先頭に向かって）並べた文字列を得よ．
/// 一番最初の問題ですが、案外素直にいきませんでした...一度Charsに変換してから逆順にし、Stringに再変換して返しています。
fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}
/// 01. 「パタトクカシーー」
/// 「パタトクカシーー」という文字列の
/// 1,3,5,7文字目を取り出して連結した文字列を得よ．
fn odd_chars(text: &str) -> String {
    text.chars().step_by(2).collect()
}
/// 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
/// 「パトカー」＋「タクシー」の文字を先頭から
/// 交互に連結して文字列「パタトクカシーー」を得よ．
fn concat_alternately(left: &str,right: &str)->String {
    let mut concated = String::new();
    left.chars().zip(right.chars()).for_each(|(l,r)| {
        concated.push(l);
        concated.push(r);
    });
    concated
}
/// 03. 円周率
/// "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."
/// という文を単語に分解し，
/// 各単語の（アルファベットの）文字数を先頭から
/// 出現順に並べたリストを作成せよ．
fn pi_from_text() -> Vec<usize> {
    let text = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    text.replace(",", "")
        .replace(".", "")
        .split_whitespace()
        .map(|s| s.len())
        .collect()
}
/// 04. 元素記号
/// "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."
/// という文を単語に分解し，
/// 1, 5, 6, 7, 8, 9, 15, 16, 19番目の単語は先頭の1文字，
/// それ以外の単語は先頭に2文字を取り出し，
/// 取り出した文字列から単語の位置（先頭から何番目の単語か）
/// への連想配列（辞書型もしくはマップ型）を作成せよ．
use std::collections::HashMap;
pub fn element_symbols() -> HashMap<String, usize> {
    let text = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let mut result = HashMap::new();
    text.replace(".", "").split_whitespace().enumerate().for_each(|(i, s)| {
        let symbol = match i {
            0 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 18 => s.get(0..1).unwrap(),
            _ => s.get(0..2).unwrap()
        };
        result.insert(symbol.to_string(), i + 1);
    });
    result
}