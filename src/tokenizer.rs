#![allow(dead_code)]
#![allow(unused_variables)]

use url::{ Url };
use regex::{ Regex };

lazy_static! {
    static ref HEADER_REGEX: Regex = Regex::new(r"^(#{1,6}?) (.*?)$").unwrap();
    static ref BLOCKQUOTE_REGEX: Regex = Regex::new(r"^> ([\s\S]+)$").unwrap();
    static ref ORDERED_LIST: Regex = Regex::new(r"^[0-9]{1,}. ([\s\S]*)$").unwrap();
    static ref UNORDERED_LIST: Regex = Regex::new(r"^- ([\s\S]*)$").unwrap();
    static ref IMAGE: Regex = Regex::new(r"^!\[(.*)\]\((.*)\)$").unwrap();
    static ref CODEBLOCK: Regex = Regex::new(r"```").unwrap();
}


#[derive(Debug)]
pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self {
	    text: text
	}
    }
}


#[derive(Debug)]
pub struct List {
    pub elements: Vec<Text>,
}

impl List {
    pub fn new() -> Self {
        Self {
            elements: Vec::new()
        }
    }

    pub fn append(&mut self, text: Text) {
        self.elements.push(text);
    }
}

#[derive(Debug)]
pub enum Token {
    Header(usize, Text),
    Blockquote(Text),
    Image(Url),
    CodeBlock(Text),
    OrderedList(List),
    UnorderedList(List),
    Link(Url, Text),
    Paragraph(Text),
}

pub fn run(lines: &Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for line in lines.iter() {
        let token = if HEADER_REGEX.is_match(line) {
	    tokenize_header(line)
	} else if BLOCKQUOTE_REGEX.is_match(line) {
	    tokenize_blockquote(line)
	} else {
	    None
	};

	if let Some(tkn) = token {
            tokens.push(tkn);
	}
    }

    println!("{:?}", tokens);
    tokens
}

fn tokenize_header(line: &String) -> Option<Token> {
    let capture_group = HEADER_REGEX.captures(&line);
    match &capture_group {
        Some(group) => {
	    let matches = group
		.iter()
		.flatten()
		.map(|value| value.as_str())
		.collect::<Vec<_>>();

	    if let [_, header_type, header_text] = &matches[..] {
		let header = header_type.len();
		let text = Text::new(header_text.to_string());

		let token = Token::Header(header, text);
		return Some(token)
	    }

	    None
        },
	_ => None
    }
}

fn tokenize_blockquote(line: &String) -> Option<Token> {
    let capture_group = BLOCKQUOTE_REGEX.captures(&line);
    match &capture_group {
        Some(group) => {
	    let matches = group
		.iter()
		.flatten()
		.map(|value| value.as_str())
		.collect::<Vec<_>>();

	    if let [_, quote_text] = &matches[..] {
		let text = Text::new(quote_text.to_string());

		let token = Token::Blockquote(text);
		return Some(token)
	    }

	    None
        },
	_ => None
    }
}

fn tokenize_image() { }

fn tokenize_codeblock() { }

fn tokenize_unordered_list() { }

fn tokenize_ordered_list() { }

fn tokenize_link() { }

fn tokenize_text() { }
