use std::fs::File;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use serde_pickle;
use serde_pickle::DeOptions;
use regex::Regex;

fn main() -> io::Result<()> {
    // File paths
    let file1 = "data/mytext.txt";
    let out = "data/output/";

    // Loading IPA dictionary
    let ipa_file = File::open("data/ipa.p").expect("Unable to open ipa.p file");
    let ipa: HashMap<String, String> = serde_pickle::from_reader(ipa_file, DeOptions::default()).expect("Unable to parse ipa.p file");

   // Reading input file
   let mut file_data = String::new();
   File::open(file1)?.read_to_string(&mut file_data)?;

   // Tokenizing the data using a regular expression
    // Tokenizing the data using a regular expression
    let re = Regex::new(r"\w+|[^\w\s]").unwrap();
    let tokens: Vec<&str> = re.find_iter(&file_data).map(|mat| mat.as_str()).collect();

   // Generating IPA data
   let ipa_data: Vec<String> = tokens.iter()
       .map(|&tok| ipa.get(&tok.to_lowercase()).unwrap_or(&"".to_string()).clone())
       .collect();

   // Writing the output to HTML file
   let mut file2 = File::create(format!("{}myipa.html", out))?;

   writeln!(file2, r#"<!doctype html>
<html lang="en">
<head>
<title>IPA</title>
<meta charset="utf-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="description" content="">
<meta name="author" content="">
</head>
<body>"#)?;

for (t, d) in tokens.iter().zip(ipa_data.iter()) {
    if d != "" {
        writeln!(file2, "<b>{}</b><span style=\"color:blue;\">/{}/</span> ", t, d)?;
    } else {
        writeln!(file2, "{} ", t)?;
    }
}

   writeln!(file2, "</body></html>")?;
   
   Ok(())
}