use std::fs;
use std::error::Error;

mod mul_state_machine;
use mul_state_machine::MulStateMachine;

mod do_state_machine;
use do_state_machine::DoStateMachine;

mod dont_state_machine;
use dont_state_machine::DontStateMachine;

// Define the file name
const FILE_NAME: &str = "resources/input.txt";

fn parse_mul(input: &str) -> Option<i32> {
    let start = input.find("mul(")?;
    let end = start + 4 + input[start + 4..].find(')')?;
    let content = &input[start + 4..end];
    let parts = content.split(',').collect::<Vec<_>>();
    if parts.len() == 2 {
        let n: i32 = parts[0].parse().ok()?;
        let m: i32 = parts[1].parse().ok()?;
        Some(n * m)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // read file to string
    let message: String = fs::read_to_string(FILE_NAME)?;

    let mut mul_state_machine = MulStateMachine::new();
    let mut do_state_machine = DoStateMachine::new();
    let mut dont_state_machine = DontStateMachine::new();

    let mut enabled = true;

    for c in message.chars() {
        if enabled {
            mul_state_machine.transform(c);
        }
        do_state_machine.transform(c);
        dont_state_machine.transform(c);

        if do_state_machine.matched_strings.len() > 0 {
            enabled = true;
            println!("Switching to enabled state due to 'do' match");
            do_state_machine.matched_strings.clear();
        }
        if dont_state_machine.matched_strings.len() > 0 {
            enabled = false;
            println!("Switching to disabled state due to 'dont' match");
            dont_state_machine.matched_strings.clear();
        }
    }

    let sum: i32 = mul_state_machine.matched_strings.iter()
        .filter_map(|s| parse_mul(s))
        .sum();
    println!("Sum of all parsed numbers: {}", sum);

    Ok(())
}
