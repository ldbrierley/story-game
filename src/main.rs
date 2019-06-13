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
            return ();
        }
        2 => {
            println!("ok");
        }
        _ => unreachable!(),
    }
    println!(" \"NOOOOOOOOOOO!!!!!!!!!!!!!!!\".  Said Andrew. But you had already gone into the thick forest and out of the sight of Andrew");
    junction();
}


fn junction() {
    println!("Did You");
    println!("1) Go east");
    println!("2) Go south ");
    println!("3) Go north ");
    match input_inside(1..=3) {
        1 => {
            println!("You continued traveling east When you heard a weired noise.");
            println!("Did you.");
            println!("1) Go back and be a noob");
            println!("2) Continue, your not a noob");
            match input_inside(1..=2) {
                1 => {
                    println!("Pathetic.");
                    junction();
                }
                2 => {
                    println!("Suddenly you saw 15 of Dr Lamption best woriers stop you.");
                    println!("Did you.");
                    println!("1) Attack them.");
                    println!("2)ruuuuuunn!!!!");
                    match input_inside(1..=2) {
                        1 => {
                            println!(
                                "You could defeat two of Dr Lamptions gaurds but not 15 woriers."
                            );
                            println!("You died.");
                            println!("GAME OVER.");
                            junction();
                        }
                        2 => {
                            println!("You ran, because the forest was so thick they lost you.");
                            println!("You went back");
                            junction();
                        }

                        _ => unreachable!(),
                    }
                }

                _ => unreachable!(),
            }
        }
        2 => {
            println!("You countinued traveling south through the trees.");
            println!("It felt like you were walking forever. You where so tired and verry hungry. Finaly you reached the beach, hungry and tired.");
            println!("Did you");
            println!("1) Build a shelter on the beech ");
            println!("2) Go back into the forest.");
            match input_inside(1..=2) {
                1 => {
                    println!("You used your verry last bit of your energy to collect sticks and leaves and built your shelter on the beach.");
                    println!("But while you where sleeping the tide came in fast and you drowend");
                    println!("GAME OVER");
                    junction();
                }

                2 => {
                    println!("You walked back into the forest you were so hungry and tired, suddenly you fell to your knees. ");
                    println!("You had had enough.");
                    println!("Did you.");
                    println!("1) Work through the pain.");
                    println!("2) Sit there and rest.");

                    match input_inside(1..=2) {
                        1 => {
                        println!("You worked untill you felt like you were going to die.");
                        println!("And you did.");
                        println!("GAME OVER");
                        junction();
                        }
                    
                        2 => {
                            println!("You sat there.");
                            println!("It was so cold.");
                            println!("You passed out.");
                            println!("You died.");
                            println!("GAME OVER"); 
                            junction(); 

                        }


                
                _ => unreachable!(),
                    }

           }
           _ => unreachable!(),
        }
        
    }
        3 => {
            println!("You kept traveling north.  It felt like you were walking forever but finally you saw someone in a cloke");
            println!(" I am your guidance said the figure."); 
            println!("I have only one question for you. What do you want?");
            println!("Did you ask?");
            println!("1) I want power.");
            println!("2) I want to kill Dr Lamption.");
            println!("3) I want to know about Dr lamption and why he wants me.");
            match input_inside(1..=3) {
            1 => {
                println!("You want power you greedy %$£! ");
                println!("The figure killed you instantly...  You died.");
                println!("GAME OVER");
                junction();
            }
            2 => {
                println!("Dr Lamption brought me to power. HOW DARE YOU!!!!");
                println!("The figure killed you instantly...  You died.");
                println!("GAME OVER.");
                junction();
            }
            3 => {
                println!("Dr lamption wants to kill you because you would become so powerfull if you went to the cave of darkness.");
                println!("He wants to get there before you and take over the land with the power that should have gone to you");
                println!("1) Take me to the cave.");
                println!("2) I don't trust you.");
                match input_inside(1..=2) {
                    1=>{
                         println!("OK let's go!");
                    }
                    2=>{
                        println!("Well bye then.");
                        println!("He took Dr Lamption to the cave");
                        println!("GAME OVER");
                        junction(); 
                    }
                _ => unreachable!(),
                }        
                }


           _ => unreachable!(), 
        }
        println!("He took you to the cave of darkness where you saw a bright blue crystal.");
        println!("Did you");
        println!("1)Take the cristal. This looks ggoooooodd");
        println!("2)No I will save that one for Dr lamption");
        match input_inside(1..=2) {
            1 => {
                println!("You touched it and died because of GREEEEEEEDD!!!!!!");
                println!("GAME OVER");
                junction(); 
            }
            2 => { 
                println!("Good you are not greedy said the figure");
                println!("If you had touched it you would have died");
                println!("The figure took you to a pile of gold with a necklace on the top of it. The figure told you that this was the key to power.");
                println!("You were about to take it when you saw Andrew hand came up behind you and grabed it. It was just too late Andrew had it and he could do what he wants");
                println!("YOU ARE GOING TO PAY FOR LEAVING MEEEEE!!!!! he shouted.");
                println!("MORE SOOON")
            }

            _ => unreachable!(),
        }

    }
    _ => unreachable!(), 
}

}
