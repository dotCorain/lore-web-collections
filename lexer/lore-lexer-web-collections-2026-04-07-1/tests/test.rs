use std::fs;
use lore_lexer_web_collections_d1::Parser;
use lore_protocol_web_collections::LineTypes;

#[test]
fn test_parser() {
    let raw = fs::read_to_string("./examples/examples.lore").unwrap();
        
    let lines = raw.lines();
    
    for (n, line) in lines.enumerate() {
        let raw_line = line.trim().to_string();
    
        let line = Parser::parse_line(&raw_line);
    
        display_line(n, line);
    }
}

fn display_line(n: usize, line: LineTypes) {
    let content = match line {
        LineTypes::Empty => "<Empty>".to_string(),
        LineTypes::Atom(atom) => format!("atom {}", atom),
        LineTypes::UrlLink(name, url) => format!("#[url] {} = {}", name, url),
        LineTypes::LoreLink(name, lore) => format!("#[lore] {} = {}", name, lore),
        LineTypes::Placeholder => "<Placeholder>".to_string(),
        LineTypes::Comment(comment) => format!("# {}", comment),
        LineTypes::DomainTitle(title) => format!("+ {}", title),
    };

    println!("[{}] {}", n, content);
}