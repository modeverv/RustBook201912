use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    {
        let v: Vec<i32> = Vec::new();
    }
    {
        let mut v = vec![1, 2, 3];
        v.push(3);
        let thi = &v[2];
        if let Some(n) = &v.get(1) {
            println!("{}", n);
        }
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{}", i);
        }
    }
    {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        for c in &row {
            match c {
                SpreadsheetCell::Int(n) => println!("{}", n),
                SpreadsheetCell::Float(n) => println!("{}", n),
                SpreadsheetCell::Text(t) => println!("{}", t),
            }
        }
        let text = "🍣あaho🍺";
        //for (_, c) in text.char_indices() {
        for c in text.chars() {
            println!("{}", c);
        }
    }
    // HashMap
    {
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 10);
    }
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        {
            for (k, v) in &scores {
                println!("{} - {}", k, v);
            }
        }
        {
            let score = scores.get(&"Blue".to_string());
            println!("{:?}", score);
            if let Some(n) = score {
                println!("{}", n);
            }
        }
        //let e = scores.entry(&String::from("Blue"));
    }
    {
        let (mean, median,mode) = sample1(&vec!(1,3,4,4,4,4,4,5,3,2,1));
        println!("{} {} {}",mean, median,mode);
    }
}

/// 整数のリストが与えられ、
/// ベクタを使って
/// mean(平均値)、
/// median(ソートされた時に真ん中に来る値)、 
/// mode(最も頻繁に出現する値; 
/// ハッシュマップがここでは有効活用できるでしょう)を返してください。
fn sample1(vec: &Vec<i32>) -> (f64,i32,i32) {
    let len:i32 = vec.len() as i32;
    let mut sum:f64 = 0.0;
    for i in vec.iter() {
        sum += (*i) as f64;
    }
    let mean = sum / (len as f64);
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();
    let median = sorted_vec.get(len as usize / 2).unwrap();
    let mut map = HashMap::new();
    for i in vec.iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max_pair = (0,0);
    for (k,v) in map {
        if v > max_pair.1 {
            max_pair = (*k, v);
        }
    }
    (mean, median.clone(),max_pair.0)
}
/// 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。
/// 各単語の最初の子音は、 単語の終端に移り、
/// "ay"が足されます。
/// 従って、"first"は"irst-fay"になります。
/// ただし、 母音で始まる単語には、
/// お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。 
/// UTF-8エンコードに関する詳細を心に留めておいてください！

/// ハッシュマップとベクタを使用して、
/// ユーザに会社の部署に雇用者の名前を追加させられる
/// テキストインターフェイスを作ってください。 
/// 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や
/// "Add Amir to Sales"(販売部門にアミールを追加)などです。 
/// それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で
/// 並べ替えられた会社の全人間の一覧を扱わせてあげてください。

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
