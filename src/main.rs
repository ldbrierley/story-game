use read_input::shortcut::input_inside;

fn main() {
    println!("Hi, come on You... I have alot to show you");
    println!("1) Why are you here?");
    println!("2) Ok i'm coming");
    match input_inside(1..=2) {
        1 => println!("I'm andrew I came to save you idiot."),
        2 => println!("Lets go!"),
        _ => unreachable!(),
    }
    println!("He undid the handcuffs on you slowly, you were in range Dr Lamption had tricked you and locked you up in this prison just to torture your very own soul.\n He knew.\n He knew the very power you could posess, he knew what happend, he knew everything even Andrew.\n Andrew took you to the exit of the prison when two gaurds stoped you.\n Did you.\n 1) Leave andrew to do all the talking.\n 2) Tell the guards you are visitor and a friend of Andrew.\n 3) Laugh at the guards.\n 4) Attack them.\n 5) Ruuuuuunn!!!");
    match input_inside(1..=5) {
        1 => println!("Andrew told the gaurd you were a visitor shyly, but the guard just laughed and arrested you Andrew to be exacuted. .\n GAME OVER."),
        2 => println!("A friend of Andrew the guard laughed, Andrew has no friends. the gaurd picked yo up and beat you, then hand cuffed you and sent you to prison. .\n GAME ."),
        3 => println!("The gaurds picked you up and beat you to death. ./n GAME OVER. "),
        4 => println!(" Both you and Andrew took down the guards with ease remembering your mashal arts skills. "),
        5 => println!("You and Andrew ran strait in to the gaurds swords and died. .\n GAME OVER. "), 
        _ => unreachable!(),
    }
}
