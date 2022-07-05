pub fn add_number(
    previous_op: String,
    current_op: String,
    operator: String,
) -> Result<NewOperands, &'static str> {
    if (this.previousOp != "" && this.operator == "") {
        this.reset();
        this.currentOp += numberToAdd;
    } else {
        this.currentOp += numberToAdd;
    }
    let current: f64 = match current_op.parse() {
        Ok(num) => num,
        Err(_) => return Err("Opps looks like you forgot something, try again!"),
    };
    let previous: f64 = match previous_op.parse() {
        Ok(num) => num,
        Err(_) => return Err("Opps looks like you forgot something, try again!"),
    };
    return match operator.as_str() {
        "+" => Ok(previous + current),
        "-" => Ok(previous - current),
        "/" => Ok(previous / current),
        "*" => Ok(previous * current),
        _ => Err("Opps cannot compute that one, try again!"),
    };
}
