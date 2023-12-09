use std::fs;

enum Packet {
    Val(u32),
    List(Vec<Packet>)
}

fn parse_pack(packet: &str) -> Packet {
    let mut stack = vec![];
    let s = packet.chars();

    let mut many = vec![];
    let mut single = None;

    for ch in packet.chars() {
        match ch {
            ',' => {
                if let Some(value) = single.take() {
                    many.push(Packet::Val(value));
                }
            }

            '[' => {
                stack.push((many, single));
                many = vec![];
                single = None;
            }

            ']' => {
                if let Some(value) = single.take() {
                    many.push(Packet::Val(value));
                }

                let packet = Packet::List(many);
                (many, single) = stack.pop().unwrap();
                many.push(packet);
            }
            x => {
                single = Some(match single.take() {
                    None => ch.to_digit(10).unwrap(),
                    Some(val) => val * 10 + (ch.to_digit(10).unwrap()),
                });
            }
        }
    }

    if let Some(value) = single.take() {
        many.push(Packet::Val(value));
    }

    return (Packet::List(many));
}

fn part1() {
    let content = fs::read_to_string("data/test.txt").unwrap();
    let pairs:Vec<Vec<&str>> = content.split("\n\n").into_iter().map(|x| x.split('\n').collect()).collect();

    for pair in pairs {
        let one = parse_pack(pair[0]);
        let two = parse_pack(pair[1]);
    }
    
    println!("part1: {val}");
}

fn part2() {
    let content = fs::read_to_string("data/test.txt").unwrap();
    
    println!("part2: {val}");
}

pub fn main() {
    println!("\nDay 13");
    part1();
    part2();
}