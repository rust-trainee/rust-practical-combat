/// 定义枚举体
#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line.splitn(2, ' ').collect();

    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "Update" | "update" => (Event::Update, rest),
        "Delete" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}
fn main() {
    let log = r#"BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"prize\": 40.00}
DELETE 342:LO/22111"#;
    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
