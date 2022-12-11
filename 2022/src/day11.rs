use std::fs;
use std::collections::VecDeque;
use scan_fmt::scan_fmt;

struct Monkey {
    items: VecDeque<i64>,
    test: i64,
    yes: usize,
    no: usize,
    oper: Vec<i64>,
    inspects: i64,
}

impl Monkey {
    fn new(itemlist: VecDeque<i64>, testnum: i64, truemon: usize, falsemon: usize, equat: Vec<i64>) -> Monkey {
        return Monkey {
            items: itemlist,
            test: testnum,
            yes: truemon,
            no: falsemon, 
            oper: equat,
            inspects: 0,
        }
    }

    fn operation(&self, item: i64) -> i64 {
        let num = if self.oper[1]==0 {item} else {self.oper[1]};
        if self.oper[0] == 1 {
            return item*num
        } else {
            return item+num
        }
    }

}

struct MonkeyList {
    list: Vec<Monkey>
}

impl MonkeyList {
    fn new() -> MonkeyList {
        return MonkeyList {
            list: Vec::new(),
        }
    }

    fn next_round(&mut self, part: i64) {
        for i in 0..self.list.len() {
            Self::take_turn(&mut self.list, i, part);
        }
    }

    fn take_turn(list: &mut Vec<Monkey>, i: usize, part: i64) {
        while let Some(pritem) = list[i].items.pop_front() {
            let item = if part==1 { pritem } else { pritem%part };
            let nitem = if part==1 {list[i].operation(item)/3} else {list[i].operation(item)%part};
            list[i].inspects += 1;
            let idx = if nitem%list[i].test==0 { list[i].yes } else { list[i].no };
            list[idx].items.push_back(nitem);
        }
    }
}

fn parse_input() -> MonkeyList {
    let content = fs::read_to_string("data/day11.txt").unwrap();
    let monkeys = match content.split("\n\n").collect::<Vec<&str>>().len() {
        1 => content.split("\n\r\n"),
        _ => content.split("\n\n"),
    };

    let mut mlist = MonkeyList::new();
    for item in monkeys {
        let monkey: Vec<&str> = item.lines().collect();
        let items: VecDeque<i64> = VecDeque::from_iter(scan_fmt!(monkey[1], "  Starting items: {/.*/}", String).unwrap().split(", ").map(|x| x.parse::<i64>().unwrap()));
        let (oper, num) = scan_fmt!(monkey[2], "  Operation: new = old {} {}", char, String).unwrap();
        let eq: Vec<i64> = vec![(if oper=='*' {1} else {0}), if num=="old" {0} else {num.parse::<i64>().unwrap()}];
        let div = scan_fmt!(monkey[3], "  Test: divisible by {d}", i64).unwrap();
        let yes = scan_fmt!(monkey[4], "    If true: throw to monkey {d}", usize).unwrap();
        let no = scan_fmt!(monkey[5], "    If false: throw to monkey {d}", usize).unwrap();
        mlist.list.push(Monkey::new(items, div, yes, no, eq));
    }
    return mlist
}

fn part1() {
    let mut mlist = parse_input();
    for _ in 0..20 {
        mlist.next_round(1);
    }
    let mut max_insp = mlist.list.iter().map(|x| x.inspects).collect::<Vec<i64>>(); 
    max_insp.sort_by(|a, b| b.cmp(a));
    println!("part1: {}", max_insp[0]*max_insp[1]);
}

fn part2() {
    let mut mlist = parse_input();
    let mut divisor = 1;
    for monk in &mlist.list {
        divisor *= monk.test;
    }
    for _ in 0..10000 {
        mlist.next_round(divisor);
    }
    let mut max_insp = mlist.list.iter().map(|x| x.inspects).collect::<Vec<i64>>(); 
    max_insp.sort_by(|a, b| b.cmp(a));
    println!("part2: {}", max_insp[0]*max_insp[1]);
}

pub fn main() {
    println!("\nDay 11");
    part1();
    part2();
}