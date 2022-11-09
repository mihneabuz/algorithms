struct StockSpanner {
    previous: Vec<(i32, i32)>
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner {
            previous: Vec::new()
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut count = 1;
        while let Some(prev) = self.previous.pop() {
            if prev.0 > price {
                self.previous.push(prev);
                break;
            }

            count += prev.1;
        }

        self.previous.push((price, count));
        count
    }
}

fn main() {
    let mut ss = StockSpanner::new();

    for input in [100, 80, 60, 70, 60, 75, 85] {
        println!("{}", ss.next(input));
    }
}
