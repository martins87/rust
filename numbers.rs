use std::io;

fn main() {
    let mut stdin_input = String::new();
    let mut str_number = String::new();

    // reads from stdin
    match io::stdin().read_line(&mut stdin_input) {
        Ok(n) => {
            str_number = substr(stdin_input, 0, n-1);
        }
        Err(error) => println!("Error: {}", error),
    }

    // checks if data is valid
    match str_number.parse::<i64>() {
        Ok(_ok) => {
            // process the number and gives result
            process_number(str_number);
        }
        Err(e) => println!("Data entered is not a number: {}.", e), 
    }  

}

fn process_number(_number: String) {

    // checks for overflow
    if _number.len() > 18 {
        println!("Number too big. Max length: 18");
        return;
    }

    // converts to number to prevent inputs like 019 => turns into 19
    let num: i64 = _number.parse::<i64>().unwrap();
    let number: String = num.to_string();
    let mut i: usize = 0;

    print!("-> ");

    // zero
    if num == 0 {
        println!("zero");
        return;
    }

    // quadrillions
    if number.len() > 15 {
        let temp: i64 = substr(num.to_string(), i, number.len()-15).parse::<i64>().unwrap();
        if temp != 0 {
            process_3_digits( temp.to_string() );
            print!("quadrillion, ");
        }
        i = number.len()-15;
    }

    // trillions
    if number.len() > 12 {
        let temp: i64 = substr(num.to_string(), i, number.len()-12).parse::<i64>().unwrap();
        if temp != 0 {
            process_3_digits( temp.to_string() );
            print!("trillion, ");
        }
        i = number.len()-12;
    }

    // billions
    if number.len() > 9 {
        let temp: i64 = substr(num.to_string(), i, number.len()-9).parse::<i64>().unwrap();
        if temp != 0 {
            process_3_digits( temp.to_string() );
            print!("billion, ");
        }
        i = number.len()-9;
    }

    // millions
    if number.len() > 6 {
        let temp: i64 = substr(num.to_string(), i, number.len()-6).parse::<i64>().unwrap();
        if temp != 0 {
            process_3_digits( temp.to_string() );
            print!("million, ");
        }
        i = number.len()-6;
    }

    // thousands
    if number.len() > 3 {
        let temp: i64 = substr(num.to_string(), i, number.len()-3).parse::<i64>().unwrap();
        if temp != 0 {
            process_3_digits( temp.to_string() );
            print!("thousand, ");
        }
        i = number.len()-3;
    }

    // hundreds
    if true {
        let temp: i64 = substr(num.to_string(), i, number.len()).parse::<i64>().unwrap();
        if temp != 0 {
            process_3_digits( temp.to_string() );
        }
    }

    println!("");

}

fn unity(_num: String) -> String {
    let number: i8 = _num.parse::<i8>().unwrap();
    
    if number == 1 {
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
    } else if number == 9 {
        return String::from("nine");
    } else {
        return String::from("");
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

fn process_0_to_99(_num: String) {

    let num: i64 = _num.parse::<i64>().unwrap();

    // 00
    if num == 0 {
        return;
    }

    // unity
    if num < 10 {
        print!("{} ", unity(_num));
        return;
    }

    // tens
    if num < 20 {
        print!("{} ", tens(_num));
        return;
    }

    // twenties
    if num == 20 {
        print!("twenty ");
        return;
    }
    if (num >= 20) && (num < 30) {
        print!("twenty {} ", unity(substr(_num, 1, 2)) );
        return;
    }

    // thirties
    if num == 30 {
        print!("thirty ");
        return;
    }
    if (num >= 30) && (num < 40) {
        print!("thirty {} ", unity(substr(_num, 1, 2)) );
        return;
    }

    // forties
    if num == 40 {
        print!("fourty ");
        return;
    }
    if (num >= 40) && (num < 50) {
        print!("fourty {} ", unity(substr(_num, 1, 2)) );
        return;
    }
    
    // fifties
    if num == 50 {
        print!("fifty ");
        return;
    }
    if (num >= 50) && (num < 60) {
        print!("fifty {} ", unity(substr(_num, 1, 2)) );
        return;
    }

    // sixties
    if num == 60 {
        print!("sixty ");
        return;
    }
    if (num >= 60) && (num < 70) {
        print!("sixty {} ", unity(substr(_num, 1, 2)) );
        return;
    }

    // seventies
    if num == 70 {
        print!("seventy ");
        return;
    }
    if (num >= 70) && (num < 80) {
        print!("seventy {} ", unity(substr(_num, 1, 2)) );
        return;
    }

    // eighties
    if num == 80 {
        print!("eighty ");
        return;
    }
    if (num >= 80) && (num < 90) {
        print!("eighty {} ", unity(substr(_num, 1, 2)) );
        return;
    }

    // nineties
    if num == 90 {
        print !("ninety ");
        return;
    }
    if (num >= 90) && (num < 100) {
        print !("ninety {} ", unity(substr(_num, 1, 2)) );
        return;
    }

}

fn process_3_digits(_number: String) {
    // converts to number to prevent inputs like 019, 007 or 000
    let num: i64 = _number.parse::<i64>().unwrap();
    let number: String = num.to_string();

    // from 0 to 99
    if num < 100 {	
        process_0_to_99(number);	
        return;	
    }
    
    if number.len() <= 3 {	
        print!("{} hundred ", unity(substr(num.to_string(), 0, 1)));
        process_0_to_99(substr(number, 1, 3));	
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
