use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    time::{SystemTime, UNIX_EPOCH},
};

use num::iter::Range;

fn main() {
    let input = include_str!("./input1.txt");

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part_1(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 1 - Time: {:?} Output {:?}", end - start, output);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part_2(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 2 - Time: {:?} Output {:?}", end - start, output);
}

#[derive(Debug)]
struct Rule {
    part: char,
    compare: u64,
    comparitor: Option<Ordering>,
    result: String,
}

impl Rule {
    fn new(rule_raw: &str) -> Self {
        let split = rule_raw.split(':').collect::<Vec<_>>();
        let part = split[0].chars().nth(0).unwrap();
        let comparitor = split[0].chars().nth(1).unwrap();
        let compare = split[0][2..]
            .chars()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let result = split[1].to_string();

        Rule {
            part,
            compare,
            comparitor: Self::comparitor(comparitor),
            result,
        }
    }

    fn comparitor(c: char) -> Option<Ordering> {
        match c {
            '<' => Some(Ordering::Less),
            '>' => Some(Ordering::Greater),
            _ => None,
        }
    }

    fn compare(&self, part: &Part) -> Result<(), String> {
        match self.part {
            'x' => {
                if part.x.cmp(&self.compare) == self.comparitor.unwrap() {
                    return Err(self.result.clone());
                }

                Ok(())
            }
            'm' => {
                if part.m.cmp(&self.compare) == self.comparitor.unwrap() {
                    return Err(self.result.clone());
                }

                Ok(())
            }
            'a' => {
                if part.a.cmp(&self.compare) == self.comparitor.unwrap() {
                    return Err(self.result.clone());
                }

                Ok(())
            }
            's' => {
                if part.s.cmp(&self.compare) == self.comparitor.unwrap() {
                    return Err(self.result.clone());
                }

                Ok(())
            }
            _ => Err("No match".to_string()),
        }
    }

    fn compare_possible(
        &self,
        part: ((u64, u64), (u64, u64), (u64, u64), (u64, u64)),
    ) -> RuleResult {
        if self.comparitor.unwrap() == Ordering::Greater {
            return RuleResult::GT(self.part.to_string(), self.compare, self.result.clone());
        } else {
            return RuleResult::LT(self.part.to_string(), self.compare, self.result.clone());
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    name: String,
    result: String,
}

impl Workflow {
    fn new(rule: &str) -> Self {
        let mut rule_holder: Vec<Rule> = vec![];

        let name = rule.split("{").nth(0).unwrap().to_string();
        let rules = rule
            .split(&['{', '}'])
            .nth(1)
            .unwrap()
            .split(",")
            .collect::<Vec<_>>();

        let workflow_result = rules.last().unwrap().to_string();

        for r in 0..rules.len() - 1 {
            rule_holder.push(Rule::new(rules[r]));
        }

        Workflow {
            rules: rule_holder,
            name,
            result: workflow_result,
        }
    }

    fn check_rules(&self, part: Part) -> String {
        for rule in &self.rules {
            match rule.compare(&part) {
                Ok(_) => {
                    continue;
                }
                Err(e) => {
                    return e;
                }
            }
        }

        self.result.clone()
    }

    fn check_rule_possible(
        &self,
        part: ((u64, u64), (u64, u64), (u64, u64), (u64, u64)),
        rule: usize,
    ) -> RuleResult {
        if rule < self.rules.len() {
            self.rules[rule].compare_possible(part)
        } else {
            match self.result.as_str() {
                "A" => RuleResult::ACCEPTED,
                "R" => RuleResult::REJECTED,
                s => RuleResult::GOTO(s.to_string()),
            }
        }
    }
}

enum RuleResult {
    ACCEPTED,
    REJECTED,
    GOTO(String),
    GT(String, u64, String),
    LT(String, u64, String),
}

#[derive(Debug, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new(part_raw: &str) -> Self {
        let elements = part_raw
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .collect::<Vec<_>>();
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;

        for e in elements {
            let (category, value) = e.split_once('=').unwrap();
            let value = value.parse::<u64>().unwrap();
            match category {
                "x" => x = value,
                "m" => m = value,
                "a" => a = value,
                "s" => s = value,
                _ => {}
            }
        }

        Part { x, m, a, s }
    }

    fn total(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Production {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Production {
    fn parse(input: &str, include_parts: bool) -> Self {
        let input = input.replace("\r\n", "\n");
        let input = input.split("\n\n").collect::<Vec<_>>();

        let mut workflows = HashMap::new();
        let mut parts = Vec::new();

        for workflow in input[0].lines() {
            let workflow = Workflow::new(workflow);
            workflows.insert(workflow.name.clone(), workflow);
        }

        if include_parts {
            for part in input[1].lines() {
                parts.push(Part::new(part));
            }
        }

        Production { workflows, parts }
    }

    fn qa_parts(&self) -> u64 {
        let mut accepted: Vec<u64> = Vec::new();

        for part in &self.parts {
            let mut next = String::from("in");
            loop {
                match self.workflows[&next].check_rules(part.clone()).as_str() {
                    "A" => {
                        accepted.push(part.total());
                        break;
                    }
                    "R" => {
                        break;
                    }
                    s => next = s.to_string(),
                }
            }
        }
        accepted.iter().sum()
    }

    fn possibilities(&self) -> u64 {
        // let mut parts = vec![];

        let mut part_stack: Vec<(
            (u64, u64),
            (u64, u64),
            (u64, u64),
            (u64, u64),
            String,
            usize,
        )> = vec![(
            (1, 4000),
            (1, 4000),
            (1, 4000),
            (1, 4000),
            String::from("in"),
            0,
        )];
        let mut accepted: Vec<((u64, u64), (u64, u64), (u64, u64), (u64, u64))> = Vec::new();

        while let Some(part) = part_stack.pop() {
            let (x, m, a, s, wf_key, rule) = part;

            if wf_key == "A" {
                accepted.push((x, m, a, s));
                continue;
            } else if wf_key == "R" {
                continue;
            }

            // Invalid bounds check
            if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
                continue;
            }

            match self.workflows[&wf_key].check_rule_possible((x, m, a, s), rule) {
                RuleResult::ACCEPTED => {
                    accepted.push((x, m, a, s));
                    continue;
                }
                RuleResult::REJECTED => {
                    continue;
                }
                RuleResult::GOTO(new_wf_key) => {
                    part_stack.push((x, m, a, s, new_wf_key, 0));
                    continue;
                }
                RuleResult::GT(prop, val, to) => match prop.as_str() {
                    "x" => {
                        part_stack.push(((val + 1, x.1), m, a, s, to, 0));
                        part_stack.push(((x.0, val), m, a, s, wf_key, rule + 1));
                    }
                    "m" => {
                        part_stack.push((x, (val + 1, m.1), a, s, to, 0));
                        part_stack.push((x, (m.0, val), a, s, wf_key, rule + 1));
                    }
                    "a" => {
                        part_stack.push((x, m, (val + 1, a.1), s, to, 0));
                        part_stack.push((x, m, (a.0, val), s, wf_key, rule + 1));
                    }
                    "s" => {
                        part_stack.push((x, m, a, (val + 1, s.1), to, 0));
                        part_stack.push((x, m, a, (s.0, val), wf_key, rule + 1));
                    }
                    _ => {
                        panic!("unknown prop {}", prop)
                    }
                },
                RuleResult::LT(prop, val, to) => match prop.as_str() {
                    "x" => {
                        part_stack.push(((x.0, val - 1), m, a, s, to, 0));
                        part_stack.push(((val, x.1), m, a, s, wf_key, rule + 1));
                    }
                    "m" => {
                        part_stack.push((x, (m.0, val - 1), a, s, to, 0));
                        part_stack.push((x, (val, m.1), a, s, wf_key, rule + 1));
                    }
                    "a" => {
                        part_stack.push((x, m, (a.0, val - 1), s, to, 0));
                        part_stack.push((x, m, (val, a.1), s, wf_key, rule + 1));
                    }
                    "s" => {
                        part_stack.push((x, m, a, (s.0, val - 1), to, 0));
                        part_stack.push((x, m, a, (val, s.1), wf_key, rule + 1));
                    }
                    _ => {
                        panic!("unknown prop {}", prop)
                    }
                },
            }
        }

        let total = 0;
        accepted
            .iter()
            .map(|(x, m, a, s)| {
                (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
            })
            .sum()
    }
}

fn part_1(input: &str) -> u64 {
    Production::parse(input, true).qa_parts()
}

fn part_2(input: &str) -> u64 {
    Production::parse(input, false).possibilities()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, 19114)
    }

    #[test]
    fn test_part_2() {
        let result = part_2(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, 167409079868000)
    }
}
