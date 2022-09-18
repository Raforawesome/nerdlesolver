#![allow(unused)]
pub fn purge_leading_zeroes(s: &str) -> String {
	let mut output: String = s.to_string();
	if output == "0" {
		return output;
	}

	while output.as_bytes()[0] as char == '0' && 
		!(("+-*/").contains(output.as_bytes()[1] as char)) {
		output = output[0..output.len() - 1].to_string();
	}

	for mut i in 1..(output.len() - 1) {
		if output.as_bytes()[i] as char == '0' && 
		"=_*/".contains((output.as_bytes()[output.len() - 1]) as char) {
  				if i == output.len() - 1 {
  					return output[0..output.len() - 1].to_string();
  				}
  				
  				output = String::new() + &output[0..i] + &output[i+1..output.len()];
  				i -= 1;
  			}
	}
	output
}

pub fn equation_solver(s: String) -> bool {
	let mut equation: &str = "";
	let mut answer: &str = "";
	let mut answer_num: i32 = 0;
	let mut engine_num: i32 = 0;

	for i in 0..s.len() {
		if s.as_bytes()[i] as char == '=' {
			equation = &s[0..i];
			answer = &s[i + 1..s.len()];
		}
	}

	answer_num = match answer.parse::<i32>() {
		Ok(n) => n,
		Err(e) => return false
	};

	engine_num = match meval::eval_str(equation) {
		Ok(n) => n as i32,
		Err(e) => return false
	};

	answer_num == engine_num  && 
	answer == purge_leading_zeroes(answer) && equation == purge_leading_zeroes(equation)
}

pub fn is_integer(c: char) -> bool {
	"1234567890".contains(c)
}

pub fn remove_char(c: char, s: &str) -> String {
	if s.len() == 1 { return s.to_string(); }
	let mut buffer: String = String::new();
	for c2 in s.chars() {
		if c != c2 {
			buffer.push(c2);
		}
	}
	buffer
}

pub fn char_at(s: &str, n: usize) -> char {
    s.as_bytes()[n] as char
}

pub fn remove_options(guess: String, results: String, options: Vec<String>) -> Vec<String> {
	let mut new_options: Vec<String> = Vec::with_capacity(results.len());
	for _ in 0..(if results.len() > 8 { results.len() } else { 8 }) {
		new_options.push(String::new());
	}
	for i in 0..results.len() {
		match results.as_bytes()[i] as char {
			'0' => {
				new_options[i] = remove_char(guess.as_bytes()[i] as char, &options[i]);
				for j in 0..8 {
					if j != i && !new_options[8].contains(guess.as_bytes()[i] as char) {
						new_options[j] = remove_char(guess.as_bytes()[i] as char, &options[j]);
					}
				}
			},
			'1' => {
				new_options[i] = remove_char(guess.as_bytes()[i] as char, &options[i]);
				if !new_options[8].contains(guess.as_bytes()[i] as char) {
					new_options[8].push(guess.as_bytes()[i] as char);
				}
			},
			'2' => {
				new_options[i] = (guess.as_bytes()[i] as char).to_string();
				if !new_options[8].contains(guess.as_bytes()[i] as char) {
					new_options[8].push(guess.as_bytes()[i] as char);
				}
				break;
			},
			_ => {
				eprintln!("Error: Invalid clue format");
				std::process::exit(1);
			}
		}
	}
	new_options
}

pub fn calculate_possibility(possibilities: Vec<String>) -> String {
    let mut contains_all: bool = false;
    for i0 in 0..possibilities[0].len() {
        for i1 in 0..possibilities[1].len() {
            for i2 in 0..possibilities[2].len() {
                if "+=*/=".contains(possibilities[2].as_bytes()[i2] as char) {
                    if "+=*/=".contains(possibilities[1].as_bytes()[i1] as char) {
                        break;
                    }
                }
                
                for i3 in 0..possibilities[3].len() {
                    if "+=*/=".contains(possibilities[3].as_bytes()[i3] as char) {
                        if "+=*/=".contains(possibilities[2].as_bytes()[i2] as char) {
                            break;
                        }
                    }
                    
                    for i4 in 0..possibilities[4].len() {
                        if is_integer(char_at(&possibilities[1], i1)) && is_integer(char_at(&possibilities[2], i2)) &&
                        is_integer(char_at(&possibilities[3], i3)) {
                            break;
                        }
                        // it didn't start with a four digiit
                        if ("+-*/=".contains(char_at(&possibilities[4], i4))) {
							if ("+-*/=".contains(char_at(&possibilities[3], i3))) {
								break;
							}
						}
						
						for i5 in 0..possibilities[5].len() {
							if ("+-*/=".contains(char_at(&possibilities[5], i5))) {
								if ("+-*/=".contains(char_at(&possibilities[4], i4))) {
									break;
								}
							}
							for i6 in 0..possibilities[6].len() {
								if ("+-*/=".contains(char_at(&possibilities[6], i6))) {
									if ("+-*/=".contains(char_at(&possibilities[5], i5))) {
										break;
									}
								}
								for i7 in 0..possibilities[7].len() {
								    let mut test: String = String::new();
									test.push(char_at(&possibilities[0], i0));
									test.push(char_at(&possibilities[1], i1));
									test.push(char_at(&possibilities[2], i2));
									test.push(char_at(&possibilities[3], i3));
									test.push(char_at(&possibilities[4], i4));
									test.push(char_at(&possibilities[5], i5));
									test.push(char_at(&possibilities[6], i6));
									test.push(char_at(&possibilities[7], i7));
									
									contains_all = true;
									println!("{test}");
									
									for i in 0..possibilities[8].len() {
										if !test.contains(char_at(&possibilities[8], i)) {
											contains_all = false;
										}
									}
									if contains_all && equation_solver(test.clone())
											&& equation_solver(purge_leading_zeroes(&test)) {

										return test;
									}
								}
							}
						}
                    }
                }
            }
        }
    }
    return "Something went wrong!".to_string();
}