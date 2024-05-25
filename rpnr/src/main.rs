// Calculate rpn expressions.
// 4 2 +
// 4 2 + 3 6 + *
// 4 2 + * 3 6 + 1 *

struct RPN<'a>{
    tokens: Vec<&'a str>
}

impl<'a> RPN<'a> {
    fn eval(&self, index: usize) -> i32 {
        match self.tokens[index] {
            op @ ("+" | "-" | "*" | "/") => {
                let lookahead_offset = match self.tokens[index-1].parse::<i32>() {
                    Ok(_) => 2,
                    Err(_) => 4,
                };
                match op {
                    "+" => {
                        self.eval(index-1) + self.eval(index-lookahead_offset)
                    },
                    "-" => {
                        self.eval(index-1) - self.eval(index-lookahead_offset)
                    },
                    "*" => {
                        self.eval(index-1) * self.eval(index-lookahead_offset)
                    },
                    "/" => {
                        self.eval(index-1) / self.eval(index-lookahead_offset)
                    },
                    _ => {
                        0
                    }
                }
            }
            _ => {
                self.tokens[index].parse::<i32>().unwrap()
            }, 
        }
    }
}


fn rpn<'a>(tokens: Vec<&'a str>) -> i32 {
    let tokens_len = tokens.len();
    let rpn_calculator = RPN{tokens};
    rpn_calculator.eval(tokens_len - 1)
}
fn main() {
    let tokens: Vec<&str> = vec!["4", "2", "+"];
    println!("4 2 + = {}", rpn(tokens));

    let tokens: Vec<&str> = vec!["4", "2", "+", "3", "6", "+", "*"];
    println!("4 2 + 3 6 + * = {}", rpn(tokens));

    let tokens: Vec<&str> = vec!["4", "2", "+", "3", "6", "+", "*", "1", "*"];
    println!("4 2 + 3 6 + * 1 * = {}", rpn(tokens));
}
