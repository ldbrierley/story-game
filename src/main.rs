use read_input::shortcut::{simple_input, valid_input};

fn main() {
    println!("Hi, come on ...., I have alot to show you");
    println!("1) Why are you here?");
    println!("2) Ok i'm coming");
    match valid_input(|x| *x < 3 && *x > 0) {
        1 => println!("I'm andrew I came to save you idiot."),
        2 => println!("Lets go!"),
        _ => unreachable!(),
    }
    println!("He undid the handcuffs on you slowly, you were in range Dr Lamption had tricked you and locked you up in this prison just to torture your very own soul.\n He knew.\n He knew the very power you could posess, he knew what happend, he knew everything even Andrew.\n Andrew took you to the exit of the prison when two gaurds stoped you.\n Did you.\n 1) Leave andrew to do all the talking.\n 2) Tell the guards you are visitor and a friend of Andrew.\n 3) Laugh at the guards.\n 4) Attack them.\n 5) Ruuuuuunn!!!")
}
