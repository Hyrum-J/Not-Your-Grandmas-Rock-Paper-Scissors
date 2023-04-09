//Standard Input Output 
use std::io;

//Opponents structure
struct Opponents{
    id_num: u32,
    name: String,
    wins: u32,
    loses: u32,
    w_l_ratio: f64,
}

//Players structure
struct Players{
    id_num: u32,
    name: String,
    wins: u32,
    loses: u32,
    w_l_ratio: f64,
}

//Game Stats structure
struct GameStats{
    player_rock_played: u32,
    ai_rock_played: u32,
    player_fire_played: u32,
    ai_fire_played: u32,
    player_scissors_played: u32,
    ai_scissors_played: u32,
    player_snake_played: u32,
    ai_snake_played: u32,
    player_human_played: u32,
    ai_human_played: u32,
    player_tree_played: u32,
    ai_tree_played: u32,
    player_wolf_played: u32,
    ai_wolf_played: u32,
    player_sponge_played: u32,
    ai_sponge_played: u32,
    player_paper_played: u32,
    ai_paper_played: u32,
    player_air_played: u32,
    ai_air_played: u32,
    player_water_played: u32,
    ai_water_played: u32,
    player_dragon_played: u32,
    ai_dragon_played: u32,
    player_devil_played: u32,
    ai_devil_played: u32,
    player_lightning_played: u32,
    ai_lightning_played: u32,
    player_gun_played: u32,
    ai_gun_played: u32,
    number_of_rounds: i32,
    rounds_won_by_player: i32,
    rounds_won_by_ai: i32,
    rounds_tied: i32,
    game_winner: u32, 
}

//Rock in string form
static rock: &str = "()";
//Fire in string form
static fire: &str = "M*";
//Scissors in string form
static scissor: &str = "8<";
//Snake in string form
static snake: &str = "_/*";
//Human in string form
static human: &str = "-@-";
//Tree in string form
static tree: &str = "_1_";
//Wolf in string form
static wolf: &str = "W<";
//Sponge in string form
static sponge: &str = "{}";
//Paper in string form
static paper: &str = "[]";
//Air in string form
static air: &str = "o";
//Water in string form
static water: &str = "|W|";
//Dragon in string form
static dragon: &str = "m^m*";
//Devil in string form
static devil: &str = "^0^";
//Lightning in string form
static lightning: &str = "Z";
//Gun in string form
static gun: &str = "q==";
//Value if the ai wins
static a: i32 = 1;
//Value if the player wins
static p: i32 = 2;
//Value if a tie
static t: i32 = 3;

fn main(){

    //()     M*       8<    _/*   -@-   _1_  W<    {}    []    o   |w|   m^m*   ^0^      Z     q== 
    //rock  fire  scissors snake human tree wolf sponge paper air water dragon devil lightning gun
    // 0     1       2       3     4     5    6    7      8    9  10     11     12     13      14
    //Steve, the first opponent
    let steve = Opponents {
        id_num: 1,
        name: "Steve".to_string(),
        wins: 5,
        loses: 2,
        w_l_ratio: 0.0,
    };

    //John, the second opponent
    let john = Opponents {
        id_num: 2,
        name: "John".to_string(),
        wins: 2,
        loses: 1,
        w_l_ratio: 0.0,
    };

    //Me, the most feirce opponent
    let hyrum = Opponents {
        id_num: 3,
        name: "Hyrum".to_string(),
        wins: 10,
        loses: 1,
        w_l_ratio: 0.0,
    };

    //Tony, the forth opponent
    let tony = Opponents {
        id_num: 4,
        name: "Tony".to_string(),
        wins: 4,
        loses: 2,
        w_l_ratio: 0.0,
    };

    //Andrew, the last opponent
    let andrew = Opponents {
        id_num: 5,
        name: "Andrew".to_string(),
        wins: 7,
        loses: 4,
        w_l_ratio: 0.0,
    };

    //Hyrum the first player
    let hy = Players{
        id_num: 1,
        name: "Hyrum".to_string(),
        wins: 0,
        loses: 0,
        w_l_ratio: 0.0,
    };

    //Abbie the second player
    let abbie = Players{
        id_num: 2,
        name: "Abbie".to_string(),
        wins: 0,
        loses: 0,
        w_l_ratio: 0.0,
    };

    //Guest the final player option
    let guest = Players{
        id_num: 3,
        name: "Guest".to_string(),
        wins: 0,
        loses: 0,
        w_l_ratio: 0.0,
    };

    //The game stats being initialized for the current game
    let mut current_game_stats = GameStats{
        player_rock_played: 0,
        ai_rock_played: 0,
        player_fire_played: 0,
        ai_fire_played: 0,
        player_scissors_played: 0,
        ai_scissors_played: 0,
        player_snake_played: 0,
        ai_snake_played: 0,
        player_human_played: 0,
        ai_human_played: 0,
        player_tree_played: 0,
        ai_tree_played: 0,
        player_wolf_played: 0,
        ai_wolf_played: 0,
        player_sponge_played: 0,
        ai_sponge_played: 0,
        player_paper_played: 0,
        ai_paper_played: 0,
        player_air_played: 0,
        ai_air_played: 0,
        player_water_played: 0,
        ai_water_played: 0,
        player_dragon_played: 0,
        ai_dragon_played: 0,
        player_devil_played: 0,
        ai_devil_played: 0,
        player_lightning_played: 0,
        ai_lightning_played: 0,
        player_gun_played: 0,
        ai_gun_played: 0,
        number_of_rounds: 0,
        rounds_won_by_player: 0,
        rounds_won_by_ai: 0,
        rounds_tied: 0,
        game_winner: 2, 
    };

    //Whether or not someone has won
    let mut win: bool = false;
    //What the player chooses converted into numbers
    let mut player_decision: i32 = 0;
    //Track what the AI has chosen
    let mut ai_decision: f64 = 0.0;
    //Round number
    let mut counter: i32 = 1;
    //Track the amount of time the player has won
    let mut player_win: i32 = 0;
    //Track the amount of time the AI has won
    let mut ai_win: i32 = 0;

    let mut temp: i32 = 0;
    //Mutable string for the input to be stored
    let mut user_input = String::new();
    //Mutable string for the player choice to be stored
    let mut player_choice = String::new();
    //A string to keep the selected player's name
    let mut selected_player_name: String = "".to_string();
    //Mutable string for the opponent to be stored
    let mut opponent_choice = String::new();
    //A string to keep the selected opponents name
    let mut selected_opponent_name: String = "".to_string();
    
    //Welcomes them to the game
    println!("Welcome to rock paper scissors!");
    //Starts the game
    println!("To begin play, type start");
    //Gives the user the rules
    println!("To learn how to play, type help");
    //To Exit
    println!("To Exit, Type exit");

    //Gets the user input
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    //Gets rid of spaces
    let user_input = user_input.trim();
    //Makes it all uppercase letters
    let user_input = user_input.to_uppercase();

    //If the user types Start
    if user_input == "START"
    {
        //Player display
        println!("Please Select a Player: ");
        println!("{}: {}", hy.id_num, hy.name);
        println!("{}: {}", abbie.id_num, abbie.name);
        println!("{}: {}", guest.id_num, guest.name);

        //Player selection
        io::stdin().read_line(&mut player_choice).expect("Failed to read line");

        //Trims it down
        let player_choice = player_choice.trim();

        //Selects a player based off of the input
        if player_choice == hy.id_num.to_string()
        {
            println!("Selected Player: {}", hy.name);
            println!("Wins: {}", hy.wins);
            println!("Losses: {}", hy.loses);
            selected_player_name = hy.name;
        }
        else if player_choice == abbie.id_num.to_string()
        {
            println!("Selected Player: {}", abbie.name);
            println!("Wins: {}", abbie.wins);
            println!("Losses: {}", abbie.loses);
            selected_player_name = abbie.name;
        }
        else if player_choice == guest.id_num.to_string()
        {
            println!("Selected Player: {}", guest.name);
            println!("Wins: {}", guest.wins);
            println!("Losses: {}", guest.loses);
            selected_player_name = guest.name;
        }
        else 
        {
            println!("Exiting, you should have selected a user");
            win = true;
        };

        //Opponent display
        println!("Please Select an Opponent: ");
        println!("{}: {}", steve.id_num, steve.name);
        println!("{}: {}", john.id_num, john.name);
        println!("{}: {}", hyrum.id_num, hyrum.name);
        println!("{}: {}", tony.id_num, tony.name);
        println!("{}: {}", andrew.id_num, andrew.name);

        //Opponent selection
        io::stdin().read_line(&mut opponent_choice).expect("Failed to read line");

        //trims it down
        let opponent_choice = opponent_choice.trim();

        //Prints out a line based on who they selected.
        //Matched based on id number
        if opponent_choice == steve.id_num.to_string()
        {
            println!("Selected Opponent: {}", steve.name);
            println!("Wins: {}", steve.wins);
            println!("Losses: {}",  steve.loses);
            selected_opponent_name = steve.name;
        }
        else if opponent_choice == john.id_num.to_string()
        {
            println!("Selected Opponent: {}", john.name);
            println!("Wins: {}", john.wins);
            println!("Losses: {}",  john.loses);
            selected_opponent_name = john.name;
        }
        else if opponent_choice == hyrum.id_num.to_string()
        {
            println!("Selected Opponent: {}", hyrum.name);
            println!("Wins: {}", hyrum.wins);
            println!("Losses: {}",  hyrum.loses);
            selected_opponent_name = hyrum.name;
        }
        else if opponent_choice == tony.id_num.to_string()
        {
            println!("Selected Opponent: {}", tony.name);
            println!("Wins: {}", tony.wins);
            println!("Losses: {}",  tony.loses);
            selected_opponent_name = tony.name;
        }
        else if opponent_choice == andrew.id_num.to_string()
        {
            println!("Selected Opponent: {}", andrew.name);
            println!("Wins: {}", andrew.wins);
            println!("Losses: {}",  andrew.loses);
            selected_opponent_name = andrew.name;
        }
        else 
        {
            println!("Exiting, you should have selected a user");
            win = true;
        }

        //Runs while no one has won
        while !win
        {
            //It presents them with this text and has them select their choice.
            println!("Please select Rock(R), Fire(F), Scissors(SC), Snake(SN),");
            println!("Human(H), Tree(T), Wolf(WO), Sponge(SP), Paper(P), Air(A)");
            println!("Water(WA), Dragon(DR), Devil(DE), Lightning(L), or Gun(G)");
            
            //The players choice for rock, paper, and scissor
            let mut player_choice = String::new();

            //Reading the player's choice
            io::stdin().read_line(&mut player_choice).expect("Falied to read line");
            
            //Gets rid of spaces
            let player_choice = player_choice.trim();
            //Makes the choices uppercase
            let player_choice = player_choice.to_uppercase();

            //Turns letters into number
            if player_choice == "R"
            {
                //Rock equals 0
                player_decision = 0;
            }
            else if player_choice == "F"
            {
                //Fire equals 1
                player_decision = 1;
            }
            else if player_choice == "SC"
            {
                //Scissors equals 2
                player_decision = 2;
            }
            else if player_choice == "SN"
            {
                //Snake equals 3
                player_decision = 3;
            }
            else if player_choice == "H"
            {
                //Human equals 4
                player_decision = 4;
            }
            else if player_choice == "T"
            {
                //Tree equals 5
                player_decision = 5;
            }
            else if player_choice == "WO"
            {
                //Wolf equals 6
                player_decision = 6;
            }
            else if player_choice == "SP"
            {
                //Sponge equals 7
                player_decision = 7;
            }
            else if player_choice == "P"
            {
                //Paper equals 8
                player_decision = 8;
            }
            else if player_choice == "A"
            {
                //Air equals 9
                player_decision = 9;
            }
            else if player_choice == "WA"
            {
                //Water equals 10
                player_decision = 10;
            }
            else if player_choice == "DR"
            {
                //Dragon equals 11
                player_decision = 11;
            }
            else if player_choice == "DE"
            {
                //Devil equals 12
                player_decision = 12;
            }
            else if player_choice == "L"
            {
                //Lightning equals 13
                player_decision = 13;
            }
            else if player_choice == "G"
            {
                //Gun equals 14
                player_decision = 14;
            }
            else
            {
                //Sets to random number if no inputs match
                player_decision = 15;
            }
            //Makes the AI desicions based on the round number
            ai_decision = aiDecision(&counter);
            //Updates the round
            counter = counter + 1;

            //Decides who won. 
            if player_decision == 0
            {
                //Player chose rock
                temp = p_rock(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 1
            {
                //Player chose fire
                temp = p_fire(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 2
            {
                //Player chose scissors
                temp = p_scissors(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 3
            {
                //Player chose snake
                temp = p_snake(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 4
            {
                //Player chose human
                temp = p_human(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 5
            {
                //Player chose tree
                temp = p_tree(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 6
            {
                //Player chose wolf
                temp = p_wolf(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 7
            {
                //Player chose sponge
                temp = p_sponge(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 8
            {
                //Player chose paper
                temp = p_paper(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 9
            {
                //Player chose air
                temp = p_air(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 10
            {
                //Player chose water
                temp = p_water(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 11
            {
                //Player chose dragon
                temp = p_dragon(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 12
            {
                //Player chose devil
                temp = p_devil(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 13
            {
                //Player chose lightning
                temp = p_lightning(&ai_decision, &mut current_game_stats);
            }
            else if player_decision == 14
            {
                //Player chose violence
                temp = p_gun(&ai_decision, &mut current_game_stats);
            }
            else
            {
                //If the player does not select a valid choice
                println!("Please write a valid responce");
            }

            if temp == a
            {
                //Updates ai score
                ai_win += 1;
            }
            else if temp == p
            {
                //Update player score
                player_win += 1;
            }

            //Prints out both scores
            println!("Score: {} - {}", player_win, ai_win);

            //If either the player or the ai wins, the loop will end
            if player_win == 5
            {
                win = true;
                current_game_stats.game_winner = 0;
            }
            else if ai_win == 5
            {
                win = true;
                current_game_stats.game_winner = 1;
            }
        }
    }
    else if user_input == "HELP"
    {
        //The help menu
        println!("This is a game of advanced rock, paper, scissors. There are 14 different options. 
These include Rock, Fire, Scissors, Snake, Human, Tree, Wolf, Sponge, Paper, Air, Water, Dragon, Devil, Lightning, and Gun. 
You will choose one, and the AI will choose one. It is a best of three.");
        main();
    }
    //If the user types exit
    else if user_input == "EXIT"
    {
        //It presents them with this texts, and the program ends
        println!("Thank you, Goodbye");
    }

    if player_win == 2
    {
        //If it ends with the player winning two
        println!("Congrtulations, you won!");
    }
    else if ai_win == 2
    {
        //If it ends with the ai winning two
        println!("Better luck next time, but the AI has won.");
    }

    //Sets the final of the game stats
    current_game_stats.number_of_rounds = counter - 1;
    current_game_stats.rounds_won_by_player = player_win;
    current_game_stats.rounds_won_by_ai = ai_win;
    current_game_stats.rounds_tied = current_game_stats.number_of_rounds - (current_game_stats.rounds_won_by_player + current_game_stats.rounds_won_by_ai);

    //Prints out all of the game stats in an easy to read way
    println!("Game Stats: ");
    println!("  Times Played: ");
    println!("      Rock:  P: {} A: {}", current_game_stats.player_rock_played, current_game_stats.ai_rock_played);
    println!("      Fire:  P: {} A: {}", current_game_stats.player_fire_played, current_game_stats.ai_fire_played);
    println!("      Scissors:  P: {} A: {}", current_game_stats.player_scissors_played, current_game_stats.ai_scissors_played);
    println!("      Snake:  P: {} A: {}", current_game_stats.player_snake_played, current_game_stats.ai_snake_played);
    println!("      Human:  P: {} A: {}", current_game_stats.player_human_played, current_game_stats.ai_human_played);
    println!("      Tree:  P: {} A: {}", current_game_stats.player_tree_played, current_game_stats.ai_tree_played);
    println!("      Wolf:  P: {} A: {}", current_game_stats.player_wolf_played, current_game_stats.ai_wolf_played);
    println!("      Sponge:  P: {} A: {}", current_game_stats.player_sponge_played, current_game_stats.ai_sponge_played);
    println!("      Paper:  P: {} A: {}", current_game_stats.player_paper_played, current_game_stats.ai_paper_played);
    println!("      Air:  P: {} A: {}", current_game_stats.player_air_played, current_game_stats.ai_air_played);
    println!("      Water:  P: {} A: {}", current_game_stats.player_water_played, current_game_stats.ai_water_played);
    println!("      Dragon:  P: {} A: {}", current_game_stats.player_dragon_played, current_game_stats.ai_dragon_played);
    println!("      Devil:  P: {} A: {}", current_game_stats.player_devil_played, current_game_stats.ai_devil_played);
    println!("      Lightning:  P: {} A: {}", current_game_stats.player_lightning_played, current_game_stats.ai_lightning_played);
    println!("      Gun:  P: {} A: {}", current_game_stats.player_gun_played, current_game_stats.ai_gun_played);
    println!("  Round Stats: ");
    println!("      Total Round: {}", current_game_stats.number_of_rounds);
    println!("      Rounds Won:  P: {} A: {}", current_game_stats.rounds_won_by_player, current_game_stats.rounds_won_by_ai);
    println!("      Rounds Tied: {}", current_game_stats.rounds_tied);
    //Getting whether the player or opponent won
    if current_game_stats.game_winner == 0
    {
        println!("  Game Winner: {}", selected_player_name);
    }
    else if current_game_stats.game_winner == 1
    {
        println!("  Game Winner: {}", selected_opponent_name);
    }

}

fn who_won(who: i32) -> i32
{
    //Returns a value based off who won
    let mut temp: i32 = 0;

    if who == 1
    {
        //When the ai wins, prints out this line and returns 1
        println!("Ai won this round");
        temp = 1;
    }
    else if who == 2
    {
        //When the player wins, prints out this line and returns 2
        println!("You won this round");
        temp = 2
    }
    else if who == 3
    {
        //When there is a tie, prints out this line and returns 3
        println!("This round was a tie");
        temp = 3;
    }
    else 
    {
        //Returns 4
        temp = 4;
    }
    //Returns it 
    return temp;
}

//()     M*       8<    _/*   -@-   _1_  W<    {}    []    o   |w|   m^m*   ^0^      Z     q== 
//rock  fire  scissors snake human tree wolf sponge paper air water dragon devil lightning gun
// 0     1       2       3     4     5    6   7       8    9  10     11     12     13      14

//For the next 14 functions, here is the summary. It compares what the player picks to what the ai picks. It then prints out the match up
//Using the global strings at the top. It then calls the function who won with a value of either a, p, or t for AI, Player, or Tie. 
//Finally, it returns a value to main of who won, so that main can update the score accordingly. 
fn p_rock(ai: &f64, current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;
    current_game.player_rock_played = current_game.player_rock_played + 1;

    if *ai == 0.0
    {
        println!("{} - {}", rock, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(t);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", rock, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(p);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", rock, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(p);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", rock, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(p);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", rock, human);
        current_game.ai_human_played +=1;
        temp = who_won(p);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", rock, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(p);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", rock, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(p);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", rock, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(p);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", rock, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(a);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", rock, air);
        current_game.ai_air_played +=1;
        temp = who_won(a);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", rock, water);
        current_game.ai_water_played +=1;
        temp = who_won(a);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", rock, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(a);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", rock, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(a);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", rock, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(a);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", rock, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(a);
    }
    return temp;
}

fn p_fire(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_fire_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", fire, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(a);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", fire, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(t);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", fire, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(p);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", fire, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(p);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", fire, human);
        current_game.ai_human_played +=1;
        temp = who_won(p);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", fire, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(p);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", fire, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(p);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", fire, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(p);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", fire, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(p);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", fire, air);
        current_game.ai_air_played +=1;
        temp = who_won(a);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", fire, water);
        current_game.ai_water_played +=1;
        temp = who_won(a);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", fire, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(a);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", fire, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(a);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", fire, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(a);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", fire, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(a);
    }
    return temp;
}

fn p_scissors(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_scissors_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", scissor, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(a);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", scissor, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(a);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", scissor, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(t);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", scissor, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(p);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", scissor, human);
        current_game.ai_human_played +=1;
        temp = who_won(p);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", scissor, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(p);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", scissor, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(p);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", scissor, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(p);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", scissor, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(p);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", scissor, air);
        current_game.ai_air_played +=1;
        temp = who_won(p);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", scissor, water);
        current_game.ai_water_played +=1;
        temp = who_won(a);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", scissor, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(a);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", scissor, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(a);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", scissor, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(a);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", scissor, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(a);
    }
    return temp;
}

fn p_snake(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_snake_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", snake, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(a);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", snake, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(a);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", snake, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(a);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", snake, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(t);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", snake, human);
        current_game.ai_human_played +=1;
        temp = who_won(p);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", snake, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(p);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", snake, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(p);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", snake, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(p);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", snake, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(p);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", snake, air);
        current_game.ai_air_played +=1;
        temp = who_won(p);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", snake, water);
        current_game.ai_water_played +=1;
        temp = who_won(p);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", snake, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(a);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", snake, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(a);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", snake, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(a);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", snake, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(a);
    }
    return temp;
}

fn p_human(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_human_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", human, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(a);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", human, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(a);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", human, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(a);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", human, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(a);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", human, human);
        current_game.ai_human_played +=1;
        temp = who_won(t);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", human, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(p);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", human, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(p);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", human, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(p);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", human, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(p);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", human, air);
        current_game.ai_air_played +=1;
        temp = who_won(p);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", human, water);
        current_game.ai_water_played +=1;
        temp = who_won(p);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", human, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(p);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", human, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(a);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", human, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(a);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", human, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(a);
    }
    return temp;
}

fn p_tree(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_tree_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", tree, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(a);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", tree, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(a);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", tree, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(a);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", tree, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(a);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", tree, human);
        current_game.ai_human_played +=1;
        temp = who_won(a);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", tree, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(t);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", tree, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(p);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", tree, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(p);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", tree, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(p);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", tree, air);
        current_game.ai_air_played +=1;
        temp = who_won(p);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", tree, water);
        current_game.ai_water_played +=1;
        temp = who_won(p);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", tree, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(p);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", tree, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(p);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", tree, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(a);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", tree, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(a);
    }
    return temp;
}

fn p_wolf(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_wolf_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", wolf, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(a);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", wolf, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(a);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", wolf, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(a);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", wolf, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(a);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", wolf, human);
        current_game.ai_human_played +=1;
        temp = who_won(a);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", wolf, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(a);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", wolf, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(t);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", wolf, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(p);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", wolf, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(p);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", wolf, air);
        current_game.ai_air_played +=1;
        temp = who_won(p);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", wolf, water);
        current_game.ai_water_played +=1;
        temp = who_won(p);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", wolf, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(p);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", wolf, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(p);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", wolf, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(p);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", wolf, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(a);
    }
    return temp;
}

fn p_sponge(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_sponge_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", sponge, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(a);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", sponge, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(a);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", sponge, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(a);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", sponge, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(a);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", sponge, human);
        current_game.ai_human_played +=1;
        temp = who_won(a);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", sponge, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(a);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", sponge, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(a);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", sponge, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(t);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", sponge, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(p);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", sponge, air);
        current_game.ai_air_played +=1;
        temp = who_won(p);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", sponge, water);
        current_game.ai_water_played +=1;
        temp = who_won(p);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", sponge, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(p);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", sponge, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(p);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", sponge, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(p);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", sponge, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(p);
    }
    return temp;
}

fn p_paper(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_paper_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", paper, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(p);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", paper, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(a);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", paper, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(a);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", paper, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(a);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", paper, human);
        current_game.ai_human_played +=1;
        temp = who_won(a);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", paper, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(a);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", paper, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(a);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", paper, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(a);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", paper, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(t);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", paper, air);
        current_game.ai_air_played +=1;
        temp = who_won(p);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", paper, water);
        current_game.ai_water_played +=1;
        temp = who_won(p);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", paper, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(p);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", paper, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(p);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", paper, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(p);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", paper, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(p);
    }
    return temp;
}

fn p_air(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_air_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", air, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(p);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", air, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(p);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", air, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(a);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", air, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(a);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", air, human);
        current_game.ai_human_played +=1;
        temp = who_won(a);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", air, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(a);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", air, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(a);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", air, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(a);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", air, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(a);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", air, air);
        current_game.ai_air_played +=1;
        temp = who_won(t);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", air, water);
        current_game.ai_water_played +=1;
        temp = who_won(p);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", air, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(p);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", air, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(p);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", air, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(p);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", air, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(p);
    }
    return temp;
}

fn p_water(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_water_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", water, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(p);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", water, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(p);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", water, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(p);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", water, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(a);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", water, human);
        current_game.ai_human_played +=1;
        temp = who_won(a);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", water, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(a);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", water, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(a);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", water, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(a);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", water, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(a);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", water, air);
        current_game.ai_air_played +=1;
        temp = who_won(a);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", water, water);
        current_game.ai_water_played +=1;
        temp = who_won(t);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", water, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(p);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", water, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(p);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", water, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(p);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", water, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(p);
    }
    return temp;
}

fn p_dragon(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_dragon_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", dragon, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(p);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", dragon, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(p);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", dragon, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(p);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", dragon, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(p);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", dragon, human);
        current_game.ai_human_played +=1;
        temp = who_won(a);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", dragon, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(a);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", dragon, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(a);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", dragon, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(a);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", dragon, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(a);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", dragon, air);
        current_game.ai_air_played +=1;
        temp = who_won(a);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", dragon, water);
        current_game.ai_water_played +=1;
        temp = who_won(a);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", dragon, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(t);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", dragon, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(p);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", dragon, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(p);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", dragon, gun);
        current_game.ai_gun_played +=1;
        temp =who_won(p);
    }
    return temp;
}

fn p_devil(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_devil_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", devil, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(p);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", devil, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(p);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", devil, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(p);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", devil, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(p);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", devil, human);
        current_game.ai_human_played +=1;
        temp = who_won(p);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", devil, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(a);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", devil, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(a);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", devil, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(a);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", devil, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(a);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", devil, air);
        current_game.ai_air_played +=1;
        temp = who_won(a);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", devil, water);
        current_game.ai_water_played +=1;
        temp = who_won(a);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", devil, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(a);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", devil, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(t);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", devil, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(p);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", devil, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(p);
    }
    return temp;
}

fn p_lightning(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_lightning_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", lightning, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(p);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", lightning, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(p);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", lightning, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(p);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", lightning, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(p);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", lightning, human);
        current_game.ai_human_played +=1;
        temp = who_won(p);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", lightning, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(p);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", lightning, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(a);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", lightning, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(a);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", lightning, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(a);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", lightning, air);
        current_game.ai_air_played +=1;
        temp = who_won(a);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", lightning, water);
        current_game.ai_water_played +=1;
        temp = who_won(a);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", lightning, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(a);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", lightning, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(a);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", lightning, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(t);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", lightning, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(p);
    }
    return temp;
}

fn p_gun(ai: &f64, mut current_game: &mut GameStats) -> i32
{
    let mut temp: i32 = 0;

    current_game.player_gun_played += 1;

    if *ai == 0.0
    {
        println!("{} - {}", gun, rock);
        current_game.ai_rock_played +=1;
        temp = who_won(p);
    }   
    else if *ai == 1.0
    {
        println!("{} - {}", gun, fire);
        current_game.ai_fire_played +=1;
        temp = who_won(p);
    } 
    else if *ai == 2.0
    {
        println!("{} - {}", gun, scissor);
        current_game.ai_scissors_played +=1;
        temp = who_won(p);
    }
    else if *ai == 3.0
    {
        println!("{} - {}", gun, snake);
        current_game.ai_snake_played +=1;
        temp = who_won(p);
    }
    else if *ai == 4.0
    {
        println!("{} - {}", gun, human);
        current_game.ai_human_played +=1;
        temp = who_won(p);
    }
    else if *ai == 5.0
    {
        println!("{} - {}", gun, tree);
        current_game.ai_tree_played +=1;
        temp = who_won(p);
    }
    else if *ai == 6.0
    {
        println!("{} - {}", gun, wolf);
        current_game.ai_wolf_played +=1;
        temp = who_won(p);
    }
    else if *ai == 7.0
    {
        println!("{} - {}", gun, sponge);
        current_game.ai_sponge_played +=1;
        temp = who_won(a);
    }
    else if *ai == 8.0
    {
        println!("{} - {}", gun, paper);
        current_game.ai_paper_played +=1;
        temp = who_won(a);
    }
    else if *ai == 9.0
    {
        println!("{} - {}", gun, air);
        current_game.ai_air_played +=1;
        temp = who_won(a);
    }
    else if *ai == 10.0
    {
        println!("{} - {}", gun, water);
        current_game.ai_water_played +=1;
        temp = who_won(a);
    }
    else if *ai == 11.0
    {
        println!("{} - {}", gun, dragon);
        current_game.ai_dragon_played +=1;
        temp = who_won(a);
    }
    else if *ai == 12.0
    {
        println!("{} - {}", gun, devil);
        current_game.ai_devil_played +=1;
        temp = who_won(a);
    }
    else if *ai == 13.0
    {
        println!("{} - {}", gun, lightning);
        current_game.ai_lightning_played +=1;
        temp = who_won(a);
    }
    else if *ai == 14.0
    {
        println!("{} - {}", gun, gun);
        current_game.ai_gun_played +=1;
        temp = who_won(t);
    }
    return temp;
}

//Makes the decisions
fn aiDecision(counter: &i32) -> f64
{
    //What will be returned
    let mut temp: f64 = 0.0;

    if *counter == 1 
    {
        //Rock
        temp = 0.0;
    }
    else if *counter == 2
    {
        //Paper
        temp = 14.0;
    }
    else if *counter == 3
    {
        //Scissors
        temp = 12.0;
    }
    else if *counter == 4 
    {
        //Paper
        temp = 9.0;
    }
    else if *counter == 5
    {
        //Scissors
        temp = 8.0;
    }
    else if *counter == 6
    {
        //Rock
        temp = 3.0;
    }
    else if *counter == 7 
    {
        //Rock
        temp = 7.0;
    }
    else if *counter == 8
    {
        //Paper
        temp = 11.0;
    }
    else if *counter == 9
    {
        //Paper
        temp = 10.0;
    }
    else if *counter == 10 
    {
        //Rock
        temp = 13.0;
    }
    else if *counter == 11
    {
        //Scissors
        temp = 2.0;
    }
    else if *counter == 12
    {
        //Rock
        temp = 0.0;
    }
    else 
    {
        //Paper
        temp = 11.0;
    }

    //Returns the selected decision
    return temp;
}