use std::collections::{BTreeSet, HashMap};

pub type PuzzleInput = (HashMap<String, Workflow>, Vec<Point>);

#[derive(Default, Debug)]
pub struct Point {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Point {
    fn value(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Copy, Clone, Debug)]
pub enum XMAS {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    GreaterThan,
    LessThan,
}

type Label = String;

#[derive(Debug)]
pub enum Verdict {
    Accepted,
    Rejected,
    GoTo(Label),
}
#[derive(Debug)]
pub struct Predicate {
    token: XMAS,
    operator: Operator,
    value: i32,
}

#[derive(Debug)]
pub struct Rule {
    predicate: Option<Predicate>,
    verdict: Verdict,
}

impl Rule {
    fn eval(&self, point: &Point) -> bool {
        match &self.predicate {
            None => true,
            Some(predicate) => {
                let point_val = match predicate.token {
                    XMAS::X => point.x,
                    XMAS::M => point.m,
                    XMAS::A => point.a,
                    XMAS::S => point.s,
                };

                match predicate.operator {
                    Operator::GreaterThan => point_val > predicate.value,
                    Operator::LessThan => point_val < predicate.value,
                }
            }
        }
    }
}

pub struct Workflow {
    label: Label,
    rules: Vec<Rule>,
}

pub fn part1(input: &PuzzleInput) -> usize {
    let (workflows, points) = input;

    let mut ret = 0usize;
    for point in points {
        let mut cur_label = "in".to_string();
        'eval: loop {
            let cur_wf = workflows.get(&cur_label).unwrap();
            for rule in &cur_wf.rules {
                if rule.eval(point) {
                    match &rule.verdict {
                        Verdict::Accepted => {
                            ret += point.value() as usize;
                            break 'eval;
                        }
                        Verdict::Rejected => {
                            break 'eval;
                        }
                        Verdict::GoTo(next_label) => {
                            cur_label = next_label.clone();
                            break;
                        }
                    }
                }
            }
        }
    }
    ret
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Range {
    s: i32,
    e: i32,
}
impl Default for Range {
    fn default() -> Self {
        Self { s: 1, e: 4000 }
    }
}
impl Range {
    fn size(&self) -> usize {
        i32::max(0, self.e - self.s + 1) as usize
    }
}
#[derive(Default, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
struct PointRange {
    xmas: [Range; 4],
}

impl PointRange {
    fn value(&self) -> usize {
        self.xmas.iter().map(Range::size).product()
    }

    fn empty(&self) -> bool {
        self.xmas.iter().any(|x| x.size() == 0)
    }

    fn shore(&mut self, tok: XMAS, operator: Operator, threshold: i32, inv: i32) {
        let tok = tok as usize;
        let range = self.xmas.get_mut(tok).unwrap();
        match operator {
            Operator::LessThan => {
                range.e = i32::min(range.e, threshold - 1 + inv);
            }
            Operator::GreaterThan => {
                range.s = i32::max(range.s, threshold + 1 - inv);
            }
        }
    }

    // returns (inbounds, remainder)
    fn slice(&self, tok: XMAS, operator: Operator, threshold: i32) -> (Self, Self) {
        let mut remainder: PointRange = self.to_owned();
        let mut inbounds: PointRange = self.to_owned();

        let inv_op = match operator {
            Operator::GreaterThan => Operator::LessThan,
            Operator::LessThan => Operator::GreaterThan,
        };

        remainder.shore(tok, inv_op, threshold, 1);
        inbounds.shore(tok, operator, threshold, 0);

        (inbounds, remainder)
    }
}

pub fn part2(input: &PuzzleInput) -> usize {
    let (workflows, _) = input;
    let mut ret = 0;

    let mut queue = BTreeSet::<(String, PointRange)>::new();
    queue.insert(("in".to_string(), PointRange::default()));

    while let Some((label, range)) = queue.pop_first() {
        // Loop over all rules. Each rule can potentially split this range in multiple parts. That's
        // fine, we only need to care about rules that terminate here.
        //
        // The Q U E U E will handle all

        let workflow = workflows.get(&label).unwrap();
        let mut range = range;
        for rule in &workflow.rules {
            // We need to take care of the following cases:
            // - The predicate accounts for the full range
            // - The predicate splits the range into two parts
            //
            // The "original" range stays in this loop, the departing range is flung into the queue.

            // Case 1: The predicate is empty
            if rule.predicate.is_none() {
                match &rule.verdict {
                    Verdict::Accepted => {
                        // tally up the score
                        ret += range.value();
                    }
                    Verdict::Rejected => { /* no-op */ }
                    Verdict::GoTo(label) => {
                        queue.insert((label.clone(), range));
                    }
                }
                // Stop rule matching, take the next elem from the queue
                break;
            }

            // Case 2: we have some non-trivial predicate, time to start slicing
            let predicate = rule.predicate.as_ref().unwrap();
            let (tok, operator, threshold) = (predicate.token, predicate.operator, predicate.value);
            let (inbounds, remainder) = range.slice(tok, operator, threshold);

            // Does any part of this predicate apply?
            if !inbounds.empty() {
                match &rule.verdict {
                    Verdict::Accepted => {
                        // tally up the score
                        ret += inbounds.value();
                    }
                    Verdict::Rejected => { /* no-op */ }
                    Verdict::GoTo(label) => {
                        queue.insert((label.clone(), inbounds));
                    }
                }
            }

            // If there is no remainder, stop.
            if remainder.empty() {
                break;
            }
            range = remainder;
        }
    }
    ret
}

fn parse_verdict(v: &str) -> Verdict {
    match v {
        "A" => Verdict::Accepted,
        "R" => Verdict::Rejected,
        x => Verdict::GoTo(x.to_string()),
    }
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let data = std::fs::read_to_string(filename).unwrap();

    let (workflows, points) = data.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (label, tmp) = line.split_once('{').unwrap();
            let workflow = tmp.split_once('}').unwrap().0;

            let rules: Vec<Rule> = workflow
                .split(',')
                .map(|rule| {
                    let (predicate, verdict) = match rule.contains(':') {
                        false => (None, parse_verdict(rule)),
                        true => {
                            let (tmp, label) = rule.split_once(':').unwrap();
                            let operator = match tmp.contains('<') {
                                true => Operator::LessThan,
                                false => Operator::GreaterThan,
                            };
                            let (token, value) = tmp.split_once(['<', '>']).unwrap();
                            let token = match token {
                                "x" => XMAS::X,
                                "m" => XMAS::M,
                                "a" => XMAS::A,
                                "s" => XMAS::S,
                                _ => panic!("invalid token"),
                            };
                            let value = value.parse::<i32>().unwrap();

                            (
                                Some(Predicate {
                                    token,
                                    operator,
                                    value,
                                }),
                                parse_verdict(label),
                            )
                        }
                    };

                    Rule { predicate, verdict }
                })
                .collect();

            let workflow = Workflow {
                label: label.to_string(),
                rules,
            };
            (label.to_string(), workflow)
        })
        .collect::<HashMap<String, Workflow>>();

    let points = points
        .lines()
        .map(|line| {
            let mut point = Point::default();
            let line = line.split_once('{').unwrap().1.split_once('}').unwrap().0;
            for group in line.split(',') {
                let (tok, value) = group.split_once('=').unwrap();
                let value = value.parse().unwrap();
                match tok {
                    "x" => point.x = value,
                    "m" => point.m = value,
                    "a" => point.a = value,
                    "s" => point.s = value,
                    _ => panic!("Invalid token"),
                }
            }
            point
        })
        .collect();

    (workflows, points)
}
