/*
    Rust cause i didn't want to learn about memory leaks in cpp or c

    Made by iamSkev#4260
*/


use std::io::{self, Write};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;

enum Net {
    Value{value: BigDecimal},
    Tick{value: String}
}

struct Stats {
    amt_to_get: BigDecimal, 
    current_amt: BigDecimal, 
    apt: BigDecimal,
    tick: String
}

fn get_tick_time(tick: &String) -> Option<BigDecimal> {
    let tick_times = HashMap::from([
        
        (String::from("fs"), BigDecimal::from_str("1").unwrap()),
        
        (String::from("bt"), BigDecimal::from_str("1.25").unwrap()),
        
        (String::from("psy"), BigDecimal::from_str("1.5").unwrap()),
        
        (String::from("idk"), BigDecimal::from_str("0").unwrap())
        ]);
    
    tick_times.get(tick).cloned()
        
}

fn get_tick_options() -> [&'static str; 4] {
    ["fs", "bt", "psy", "idk"]
}

fn get_suffix_amount(suffix: &String) -> Option<BigDecimal> {
    let suffix_amount = HashMap::from([
        
        (String::from("k"), BigDecimal::from_str("1_000").unwrap()),
        
        (String::from("m"), BigDecimal::from_str("1_000_000").unwrap()),
        
        (String::from("b"), BigDecimal::from_str("1_000_000_000").unwrap()),
        
        (String::from("t"), BigDecimal::from_str("1_000_000_000_000").unwrap()),
        
        (String::from("qa"), BigDecimal::from_str("1_000_000_000_000_000").unwrap()),
        
        (String::from("qi"), BigDecimal::from_str("1_000_000_000_000_000_000").unwrap())
        ]);
    
    suffix_amount.get(suffix).cloned()

}

fn correct_grammar(time_amount: &BigDecimal, time_name: &str, compare_with: &BigDecimal) -> String {
    let correct_string: String;

    if time_amount > compare_with{
        correct_string = format!("{} {}s", time_amount, time_name);
    } else {
        correct_string = format!("{} {}", time_amount, time_name)
    }

    correct_string
}

fn converter(stat: &String) -> Option<BigDecimal> {

    let mut true_stat: BigDecimal = BigDecimal::from(0);

    let mut suffix: String = String::new();
    let mut number: String = stat.to_string();

    let number_regex: Regex = Regex::new(r"[^a-zA-Z]").unwrap();
    let suffix_regex: Regex = Regex::new(r"[a-zA-Z]+").unwrap();
    
    if suffix_regex.is_match(stat) {
        
        suffix = number_regex.replace_all(stat, "").to_string();
        number = suffix_regex.replace_all(stat, "").to_string();
        
        let suffix_amount = get_suffix_amount(&suffix);

        if suffix_amount.is_none() {
            return None
        } else if suffix_amount.is_some() && number == String::new() {
            return None
        }
    }

    let converted_number = BigDecimal::from_str(number.trim());

    if converted_number.is_err(){
        return None
    } else if converted_number.is_ok() && suffix != String::new() {
        let suffix_amount = get_suffix_amount(&suffix);
        if suffix_amount.is_none() {
            return None
        } else {
            true_stat = &converted_number.unwrap() * &suffix_amount.unwrap();
        }
    } else if converted_number.is_ok() && suffix == String::new() {
        true_stat = converted_number.unwrap();
    }

    Some(true_stat)
}

fn round(var: &BigDecimal ) -> BigDecimal {
    let (mut b, e) = var.clone().into_bigint_and_exponent();
    for _ in 0..e {b /= 10;}

    BigDecimal::from(b.to_owned())
}

impl Stats {
    fn calculate(&self) -> String{
        
        let tick = &self.tick;
        let amt = &self.amt_to_get;
        let current_amt = &self.current_amt;
        let apt = &self.apt;
    
    
        let time_result: String;
        let mut amt_of_time = vec![];

        let tick_amt = get_tick_time(tick);

        let amt_to_get = &(amt - current_amt);

        let one = BigDecimal::from(1);
        let zero = BigDecimal::from(0);
        let sixty = BigDecimal::from(60);
        let thousand = BigDecimal::from(1000);
        
        let stats = (((amt_to_get / apt) * &tick_amt.unwrap()) / &sixty) / &sixty;
        let rounded_stats = round(&stats);
        
        let day_in_hours = BigDecimal::from_str("24").unwrap();
        

        let hours: BigDecimal;
        let mut days: BigDecimal;

        
        
        
        
        
        
        if  rounded_stats >= day_in_hours {
            
            days = &rounded_stats / &day_in_hours;
            
            days = round(&days);

            hours = &rounded_stats - (&days * &day_in_hours);

            if days >= one {
                amt_of_time.push(correct_grammar(&days, "day", &one));
            }

            if hours >= one {
                amt_of_time.push(correct_grammar(&hours, "hour", &one));
            }
        } else if rounded_stats > zero {
            hours = rounded_stats.clone();
            amt_of_time.push(correct_grammar(&hours, "hour", &one));
        }

        
        
        
        
        

        let minutes: BigDecimal = (&stats - &rounded_stats) * &sixty;
        
        let rounded_minutes: BigDecimal = round(&minutes);

        let seconds: BigDecimal = (&minutes - &rounded_minutes) * &sixty;
        let rounded_seconds: BigDecimal = round(&seconds);

        let milliseconds: BigDecimal = (&seconds - &rounded_seconds) * &thousand;
        let rounded_milliseconds: BigDecimal = round(&milliseconds);

        let microseconds: BigDecimal = (&milliseconds - &rounded_milliseconds) * &thousand;
        let rounded_microseconds: BigDecimal = round(&microseconds);

        let nanoseconds: BigDecimal = (&microseconds - &rounded_microseconds) * &thousand;
        let rounded_nanoseconds: BigDecimal = round(&nanoseconds);

        if rounded_minutes > zero {
            amt_of_time.push(correct_grammar(&rounded_minutes, "minute", &one));
        }

        if rounded_seconds > zero {
            amt_of_time.push(correct_grammar(&rounded_seconds, "second", &one));
        }

        if rounded_milliseconds > zero {
            amt_of_time.push(correct_grammar(&rounded_milliseconds, "millisecond", &one));
        }

        if rounded_microseconds > zero {
            amt_of_time.push(correct_grammar(&rounded_microseconds, "microsecond", &one));
        }

        if rounded_nanoseconds > zero {
            amt_of_time.push(correct_grammar(&rounded_nanoseconds, "nanosecond", &one));
        }

        if amt_of_time.len() > 1 {
        time_result = amt_of_time[0..(amt_of_time.len()-1)].join(", ") + &format!(" and {}", &amt_of_time[(&amt_of_time.len()-1)]);
        } else if amt_of_time.len() == 1 {
            time_result = amt_of_time[0].clone();
        } else {
            time_result = String::from("now");
        }

        time_result
    }

    fn new(amt_to_get: BigDecimal, current_amt: BigDecimal, apt: BigDecimal,tick: String) -> Stats{

        Stats {amt_to_get, current_amt, apt, tick}

    }
}

fn net(message: &str, suffix: bool) -> Net {
    loop {
        if suffix {

            println!("\ni'm sorry but you'd have to type that out again since that doesn't look right.");
            println!("You can either add a suffix to your stat or type it out manually. e.g., 69m or 69000000");
            println!("Here are the suffixes:
            
            K (Thousand)
            M (Million)
            B (Billion)
            T (Trillion)
            Qa (Quadrillion)
            Qi (Quintillion)\n");
            print!("{}", message);
            let _ = io::stdout().flush();
            
            let mut input = String::new();

            let result = io::stdin().read_line(&mut input);
            
            if result.is_ok() {
                
                let value = converter(&input.trim().to_string());


                if value.is_some() {
                    return Net::Value{value: value.unwrap()}
                } else {
                    continue;
                }

            } else {
                continue
            }

            } else {
                
                println!("\nI'm sorry but you'll have to specify which stat you're training between: 

                fs (Fist Strength)
                bt (Body Toughness) 
                psy (Psychic Power)
                idk (i don't know)\n");
                print!("{}", message);
                
                let _ = io::stdout().flush();
                let mut input = String::new();
                let result = io::stdin().read_line(&mut input);
                
                if result.is_ok() {
                    let value = get_tick_time(&input.trim().to_string());
                
                    if value.is_some() {
                        return Net::Tick{value: input.trim().to_string()}
                    } else {
                        continue;
                    }

                } else if result.is_err() {
                    continue
                }
        }
    }
    

}

fn ask_questions() -> String {
    let mut total_time = String::from("
Since you picked idk (i don't know) when i asked you about what you're training, i calculated the time for all 3 of the stats.
Here they are:
    ");

    let mut goal = String::new();
    let mut converted_goal = BigDecimal::from(0);
    
    print!("How much is your goal?: ");
    
    let _ = io::stdout().flush();
    
    let result = io::stdin().read_line(&mut goal);
    
    if result.is_ok() {
        let value = converter(&goal);
        if value.is_some() {
            converted_goal = value.unwrap();
        } else if let Net::Value { value } = net("How much is your goal?: ", true) {
            converted_goal = value;
        }
    } else if let Net::Value { value } = net("How much is your goal?: ", true) {
        converted_goal = value;
    }

    let mut current_amount = String::new();
    let mut converted_current_amount = BigDecimal::from(0);
    
    print!("\nHow much do you currently have?: ");
    
    let _ = io::stdout().flush();
    
    let result = io::stdin().read_line(&mut current_amount);
    
    if result.is_ok() {
        let value = converter(&current_amount);
        if value.is_some() {
            converted_current_amount = value.unwrap();
        } else if let Net::Value { value } = net("How much is your goal?: ", true) {
            converted_current_amount = value;
        }
    } else if let Net::Value { value } = net("\nHow much do you currently have?: ", true) {
        converted_current_amount = value;
    }

    let mut per_tick = String::new();
    let mut converted_per_tick= BigDecimal::from(0);
    
    print!("\nHow much do you get per tick?: ");
    
    let _ = io::stdout().flush();
    
    let result = io::stdin().read_line(&mut per_tick);
    
    if result.is_ok() {
        let value = converter(&per_tick);
        if value.is_some() {
            converted_per_tick = value.unwrap();
        } else if let Net::Value { value } = net("\nHow much do you get per tick?: ", true) {
            converted_per_tick = value;
        }
    } else if let Net::Value { value } = net("\nHow much do you get per tick?: ", true) {
        converted_per_tick = value;
    }

    let mut tick_time = String::new();
    let mut converted_tick_time = String::new();
    
    print!("\nWhat stat are you training?(fs, bt, psy, idk): ");
    
    let _ = io::stdout().flush();
    
    let result = io::stdin().read_line(&mut tick_time);
    
    if result.is_ok() {
        let value = get_tick_time(&tick_time.trim().to_ascii_lowercase());
        if value.is_some() {
            converted_tick_time = tick_time.trim().to_ascii_lowercase();
        } else if let Net::Tick { value } = net("\nWhat stat are you training?(fs, bt, psy, idk): ", false) {
            converted_tick_time = value.trim().to_ascii_lowercase();
        }
    } else if let Net::Tick{ value } = net("\nWhat stat are you training?(fs, bt, psy, idk): ", false) {
        converted_tick_time = value.trim().to_ascii_lowercase();
    }


    if converted_tick_time == "idk" {
        for tick in get_tick_options() {

            match tick {
                "fs" => {
                    let stat = Stats::new(converted_goal.clone(), converted_current_amount.clone(), converted_per_tick.clone(), tick.clone().to_string());
                    total_time += &(format!("\nFist Strength:: it would take roughly around {}", stat.calculate())).to_string();
                },
                "bt" => {
                    let stat = Stats::new(converted_goal.clone(), converted_current_amount.clone(), converted_per_tick.clone(), tick.clone().to_string());
                    total_time += &(format!("\nBody Toughness: it would take roughly around {}", stat.calculate())).to_string();
                },
                "psy" => {
                    let stat = Stats::new(converted_goal.clone(), converted_current_amount.clone(), converted_per_tick.clone(), tick.clone().to_string());
                    total_time += &(format!("\nPsychic Power: it would take roughly around {}", stat.calculate())).to_string();
                },
                _ => break
            }
        }
        
        total_time
    } else {
        let stat = Stats::new(converted_goal, converted_current_amount, converted_per_tick, converted_tick_time);
        format!("\nIt would take rougly around {}.", stat.calculate())
    }

}

fn main() {
    loop {
        println!("{}", ask_questions());
        println!("Good Luck. And as always. Quack.");
    }   
}