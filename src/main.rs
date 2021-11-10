use std::io;

fn main() {
    let mut location: &str = "bed";
    let mut room: &str = "start_room";
    let mut first_question_asked: bool = false;
    let mut state_instance = State {
        location: String::from(location),
        room: String::from(room),
        first_question_asked: first_question_asked,
    };
    question(&mut state_instance, "You wake up in a room you do not recognize. What do you do?");
    println!("Next question?");
}

struct State {
    location: String,
    room: String,
    first_question_asked: bool,
}

fn question(state: &mut State, q: &str) {
    state.location = "asdsadas";
    let mut guess = String::new();
    let mut x = 0;
    while x == 0 {
        println!("{}", q);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let split = guess.split(" ");
        let vec: Vec<&str> = split.collect();

        x = can_command_be_done(vec[0]);
    }
}

fn can_command_be_done(command: &str) -> i8 {
    let options = vec!["go", "climb"];
    if !options.contains(&command) {
        println!("Dont know how to: {}", command);
        0
    } else {
        1
    }
}
