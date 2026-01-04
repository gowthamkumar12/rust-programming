pub fn is_armstrong_number(num: u32) -> bool {
    let mut number : u32 = num;
    let     n      : u32 = order(num);
    let mut result : u32 = 0;

    while number > 0 {
        let digit = number % 10;
        number = number / 10;

        result += digit.pow(n);
    }

    if result == num {
        return true;
    }

    false
}

fn order(num: u32) -> u32 {
    let mut number: u32 = num;
    let mut n     : u32 = 0;

    while number > 0 {
        number = number / 10;
        n += 1;
    }

    n
}
