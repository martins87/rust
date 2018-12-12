// Number scales: http://www.statman.info/conversions/number_scales.html

use std::io;

fn main() {
    let mut str_number = String::new();
    let mut stdin_input = String::new();

    // reads from stdin
    match io::stdin().read_line(&mut stdin_input) {
        Ok(n) => {
            str_number = substr(stdin_input, 0, n-1);
        }
        Err(error) => println!("Error: {}", error),
    }

    // process the number and gives result
    process_number(str_number);


    //println!("{}", );
}

// 1355823 -> "one million, three hundred fifty five thousand, eight hundred twenty three​​​
fn process_number(_number: String) {

    // checks for overflow
    if _number.len() > 18 {
        println!("Number too big. Max length: 18");
        return;
    }

    // converts to number to prevent inputs like 019 => turns into 19
    let num: i64 = _number.parse::<i64>().unwrap();
    let number: String = num.to_string();

    // from 0 to 99
    if num < 100 {
        process0to99(_number);
        return;
    }

    // hundreds
    if num == 100 {
        println!("one hundred");
        return;
    }
    if number.len() <= 3 {
        print!("{} {}", unity(substr(number, 0, 1)), "hundred " );
        process0to99(substr(_number, 1, 3));
        return;
    }

    // thousands
    if num == 1000 {
        println!("one thousand");
        return;
    }
    if number.len() <= 6 {
        println!("thousands");
        return;
    }

    // millions
    if num == 1000000 {
        println!("one million");
        return;
    }
    if number.len() <= 9 {
        println!("millions");
        return;
    }

    // billions
    if num == 1000000000 {
        println!("one billion");
        return;
    }
    if number.len() <= 12 {
        println!("billions");
        return;
    }

    // trillions
    if num == 1000000000000 {
        println!("one trillion");
        return;
    }
    if number.len() <= 15 {
        println!("trillions");
        return;
    }

    // quadrillions
    if num == 1000000000000000 {
        println!("one quadrillion");
        return;
    }
    if number.len() <= 18 {
        println!("quadrillions");
        return;
    }
}

fn unity(_num: String) -> String {
    let number: i8 = _num.parse::<i8>().unwrap();
    
    if number == 0 {
        return String::from("zero");
    } else if number == 1 {
        return String::from("one");
    } else if number == 2 {
        return String::from("two");
    } else if number == 3 {
        return String::from("three");
    } else if number == 4 {
        return String::from("four");
    } else if number == 5 {
        return String::from("five");
    } else if number == 6 {
        return String::from("six");
    } else if number == 7 {
        return String::from("seven");
    } else if number == 8 {
        return String::from("eight");
    } else {
        return String::from("nine");
    }
}

fn tens(_num: String) -> String {
    let number: i8 = _num.parse::<i8>().unwrap();

    if number == 10 {
        return String::from("ten");
    } else if number == 11 {
        return String::from("eleven");
    } else if number == 12 {
        return String::from("twelve");
    } else if number == 13 {
        return String::from("thirteen");
    } else if number == 14 {
        return String::from("fourteen");
    } else if number == 15 {
        return String::from("fifteen");
    } else if number == 16 {
        return String::from("sixteen");
    } else if number == 17 {
        return String::from("seventeen");
    } else if number == 18 {
        return String::from("eighteen");
    } else {
        return String::from("nineteen");
    }
}

fn process0to99(_num: String) {

    let num: i64 = _num.parse::<i64>().unwrap();

    // unity
    if num < 10 {
        println!("{}", unity(_num));
        return;
    }

    // tens
    if num < 20 {
        println!("{}", tens(_num));
        return;
    }

    // twenties
    if num == 20 {
        println!("twenty");
        return;
    }
    if (num >= 20) && (num < 30) {
        println!("twenty {}", unity(substr(_num, 1, 2)) );
        return;
    }

    // thirties
    if num == 30 {
        println!("thirty");
        return;
    }
    if (num >= 30) && (num < 40) {
        println!("thirty {}", unity(substr(_num, 1, 2)) );
        return;
    }

    // forties
    if num == 40 {
        println!("fourty");
        return;
    }
    if (num >= 40) && (num < 50) {
        println!("fourty {}", unity(substr(_num, 1, 2)) );
        return;
    }
    
    // fifties
    if num == 50 {
        println!("fifty");
        return;
    }
    if (num >= 50) && (num < 60) {
        println!("fifty {}", unity(substr(_num, 1, 2)) );
        return;
    }

    // sixties
    if num == 60 {
        println!("sixty");
        return;
    }
    if (num >= 60) && (num < 70) {
        println!("sixty {}", unity(substr(_num, 1, 2)) );
        return;
    }

    // seventies
    if num == 70 {
        println!("seventy");
        return;
    }
    if (num >= 70) && (num < 80) {
        println!("seventy {}", unity(substr(_num, 1, 2)) );
        return;
    }

    // eighties
    if num == 80 {
        println!("eighty");
        return;
    }
    if (num >= 80) && (num < 90) {
        println!("eighty {}", unity(substr(_num, 1, 2)) );
        return;
    }

    // nineties
    if num == 90 {
        println!("ninety");
        return;
    }
    if (num >= 90) && (num < 100) {
        println!("ninety {}", unity(substr(_num, 1, 2)) );
        return;
    }
}

fn substr(_string: String, _start: usize, _end: usize) -> String {
    let mut _subs = String::new();
    
    for (i, c) in _string.chars().enumerate() {
        if i < _start {
            continue;
        }
        if i == _end {
            break;
        }
        _subs.push(c);
    }
    return _subs;
}
