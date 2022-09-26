struct Solution {}

const LETTERS: usize = 26;

struct Union {
    parent: Vec<u8>,
}

impl Union {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).map(|x| x as u8).collect(),
        }
    }

    fn find(&self, mut item: u8) -> u8 {
        while self.parent[item as usize] != item {
            item = self.parent[item as usize];
        }

        item
    }

    fn join(&mut self, x: u8, y: u8) {
        let (parent_x, parent_y) = (self.find(x), self.find(y));
        self.parent[parent_y as usize] = parent_x;
    }
}

#[derive(PartialEq)]
enum Operator {
    Equal,
    NotEqual,
}

struct Equation {
    operands: (u8, u8),
    operator: Operator,
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let equations: Vec<Equation> = equations
            .into_iter()
            .map(|s| Solution::extract(s))
            .collect();

        let mut un = Union::new(LETTERS);

        equations
            .iter()
            .filter(|eq| eq.operator.eq(&Operator::Equal))
            .map(|eq| eq.operands)
            .for_each(|(op1, op2)| un.join(op1, op2));

        equations
            .iter()
            .filter(|eq| eq.operator.eq(&Operator::NotEqual))
            .map(|eq| eq.operands)
            .all(|(op1, op2)| un.find(op1) != un.find(op2))
    }

    pub fn equations_possible_simple(equations: Vec<String>) -> bool {
        let equations: Vec<Equation> = equations
            .into_iter()
            .map(|s| Solution::extract(s))
            .collect();

        let mut equal: Vec<Vec<u8>> = (0..LETTERS).map(|i| vec![i as u8]).collect();
        let mut bucket: Vec<usize> = (0..LETTERS).collect();

        for eq in equations.iter() {
            let (op1, op2) = eq.operands;
            let (bucket1, bucket2) = (bucket[op1 as usize], bucket[op2 as usize]);

            if eq.operator == Operator::Equal {
                let mut res: Vec<u8> = Vec::new();
                res.extend(equal[bucket1].iter());
                res.extend(equal[bucket2].iter());
                res.sort();
                res.dedup();

                for op in res.iter() {
                    bucket[*op as usize] = bucket1;
                }

                equal[bucket1] = res;
            }
        }

        for eq in equations.into_iter() {
            let (op1, op2) = eq.operands;

            if eq.operator == Operator::NotEqual {
                if let Ok(_) = &equal[bucket[op1 as usize]].binary_search(&op2) {
                    return false;
                }

                if let Ok(_) = &equal[bucket[op2 as usize]].binary_search(&op1) {
                    return false;
                }
            }
        }

        true
    }

    fn extract(equation: String) -> Equation {
        let bytes = equation.as_bytes();

        Equation {
            operands: (bytes[0] - 'a' as u8, bytes[3] - 'a' as u8),
            operator: if bytes[1] == '=' as u8 {
                Operator::Equal
            } else {
                Operator::NotEqual
            },
        }
    }
}

fn main() {
    let inputs: Vec<Vec<String>> = [
        vec!["a==b", "b!=a"],
        vec!["b==a", "a==b"],
        vec!["a==b", "e==c", "b==c", "a!=e"],
    ]
    .into_iter()
    .map(|arr| arr.into_iter().map(String::from).collect())
    .collect();

    for input in inputs {
        println!("{}", Solution::equations_possible(input));
    }
}
