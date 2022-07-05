use crate::NewOperands;

pub fn total(
    previous_op: String,
    current_op: String,
    operator: String,
) -> Result<NewOperands, &'static str> {
    let current: f64 = match current_op.parse() {
        Ok(num) => num,
        Err(_) => return Err("Opps looks like you forgot something, try again!"),
    };
    let previous: f64 = match previous_op.parse() {
        Ok(num) => num,
        Err(_) => return Err("Opps looks like you forgot something, try again!"),
    };
    let new_previous = String::from("");
    let new_operator = String::from("");
    return match operator.as_str() {
        "+" => Ok(NewOperands {
            previous: new_previous,
            current: (previous + current).to_string(),
            operator: new_operator,
        }),
        "-" => Ok(NewOperands {
            previous: new_previous,
            current: (previous - current).to_string(),
            operator: new_operator,
        }),
        "/" => Ok(NewOperands {
            previous: new_previous,
            current: (previous / current).to_string(),
            operator: new_operator,
        }),
        "*" => Ok(NewOperands {
            previous: new_previous,
            current: (previous * current).to_string(),
            operator: new_operator,
        }),
        _ => Err("Opps cannot compute that one, try again!"),
    };
}

#[cfg(test)]
mod tests {
    use crate::compute::compute;

    #[test]
    fn operator_fails() {
        let result = compute(String::from("2"), String::from("2"), String::from("x"));
        assert_eq!(result, Err("Opps cannot compute that one, try again!"));
    }

    #[test]
    fn addition_works() {
        let result = compute(String::from("2"), String::from("2"), String::from("+"));
        assert_eq!(result, Ok(4.0));
    }

    #[test]
    fn addition_fails() {
        let result = compute(String::from("2"), String::from(""), String::from("+"));
        assert_eq!(
            result,
            Err("Opps looks like you forgot something, try again!")
        );
    }

    #[test]
    fn minus_works() {
        let result = compute(String::from("2"), String::from("2"), String::from("-"));
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn minus_fails() {
        let result = compute(String::from(""), String::from("2"), String::from("-"));
        assert_eq!(
            result,
            Err("Opps looks like you forgot something, try again!")
        );
    }

    #[test]
    fn divide_works() {
        let result = compute(String::from("10"), String::from("2"), String::from("/"));
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn divide_fails() {
        let result = compute(String::from(""), String::from("2"), String::from("/"));
        assert_eq!(
            result,
            Err("Opps looks like you forgot something, try again!")
        );
    }

    #[test]
    fn multiply_works() {
        let result = compute(String::from("10"), String::from("2"), String::from("*"));
        assert_eq!(result, Ok(20.0));
    }

    #[test]
    fn multiply_fails() {
        let result = compute(String::from(""), String::from("2"), String::from("*"));
        assert_eq!(
            result,
            Err("Opps looks like you forgot something, try again!")
        );
    }
}
