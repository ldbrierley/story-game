use read_input::shortcut::input_inside;

fn main() {
    println!("Hi, come on ?You? I have alot to show you");
    println!("1) Why are you here?");
    println!("2) Ok i'm coming");
    match input_inside(1..=2) {
        1 => println!("I'm andrew I came to save you idiot."),
        2 => println!("Lets go!"),
        _ => unreachable!(),
    }
    println!(    "He undid the handcuffs on you slowly, you were in range Dr Lamption had tricked you and locked you up in this prison just to torture your very own soul  ");
    println!("HE KNEW.");
    println!("He knew the very power you could posess, he knew what happend, he knew everything EVEN Andrew.");
    println!("Andrew took you to the exit of the prison when two gaurds stoped you.");
    println!("Did you.");
    println!("1) Leave andrew to do all the talking.");
    println!("2) Tell the guards you are visitor and a friend of Andrew.");
    println!("3) Laugh at the guards.");
    println!("4) Attack them.");
    println!("5) Ruuuuuunn!!!");

    match input_inside(1..=5) {
        1 => {
            println!("Andrew told the gaurd you were a visitor shyly, but the guard just laughed and arrested you Andrew to be exacuted.");
            println!("GAME OVER.");
            return ();
        }

        2 => {
            println!("A friend of Andrew the guard laughed, Andrew has no friends. the gaurd picked you up and beat you, then hand cuffed you and sent you to be exacuted.");
            println!("GAME OVER.");
            return ();
        }
        3 => {
            println!("The gaurds picked you up and beat you to death.");
            println!("GAME OVER.");
            return ();
        }
        4 => {
            println!(" Both you and Andrew took down the guards with ease remembering your mashal arts skills. ");
        }
        5 => {
            println!("You and Andrew ran strait in to the gaurds swords and died. .\n GAME OVER. ");
            return ();
        }
        _ => unreachable!(),
    }

    println!("You and Andrew ran into the forest together. Andrew was taking you to the cave of darkness where you would unleesh your full potential.\n Before we get to the cave we must stay in the inn at Westfield village where we will sleep said Andrew ");
    println!("did you 1) follow Andrew.\n 2) Nah i'm going my own way");
    match input_inside(1..=2) {
        1 => {
            println!("You arived at the village to stay in the inn and rest there but then Dr Lamption's best woriers raided the inn killing you and Andrew in your sleep");
            println!("GAME OVER");
        }
        2 => println!("You win so far, but be aware there is more coming soon."),
        _ => unreachable!(),
    }
}
