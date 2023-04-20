use sdl2::{pixels::Color, rect::Rect, event::Event, keyboard::Keycode, video::Window,};
use sdl2::render::{WindowCanvas, TextureCreator};

use rand::Rng;
use rand::rngs::ThreadRng;

use std::path::Path;
use std::time::Duration;
use std::thread::sleep;

//Standard Input Output 
use std::io;

//Value if the ai wins
static a: i32 = 1;
//Value if the player wins
static p: i32 = 2;
//Value if a tie
static t: i32 = 3;

//Main function
fn main() {

    playGame();

}

fn playGame(){
    //an instance of the sdl2 library
    let sdl_context = sdl2::init().unwrap();
    //The video of the sdl2 from sdl context
    let video_subsystem = sdl_context.video().unwrap();

    //Whether or not someone has won
    let mut win: bool = false;
    //What the player chooses converted into numbers
    let mut player_decision: i32 = 0;
    //Track what the AI has chosen
    let mut ai_decision: i32 = 0;
    //Track the amount of time the player has won
    let mut player_win: i32 = 0;
    //Track the amount of time the AI has won
    let mut ai_win: i32 = 0;

    let mut temp: i32 = 0;

    let mut i: i32 = 0;

    //Mutable string for the player choice to be stored
    let mut player_choice = String::new();
    //Mutable string for the opponent to be stored
    let mut opponent_choice = String::new();

    //Create a window for our game
    let mut window = video_subsystem.window("Not Your Grandmas Rock Paper Scissors", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    //Create a canvas on the window
    let mut canvas = window.into_canvas().build().unwrap();

    //Square size variable
    let square_size: u32 = 20;

    let mut x_pos: i32 = 100;

    let mut y_pos: i32 = 480;

    let firstX: i32 = 107;

    let secondX: i32 = 154;

    let thirdX: i32 = 201;

    let fourthX: i32 = 248;

    let fifthX: i32 = 295;

    let sixthX: i32 = 342;

    let selectedX: i32 = 225;

    let selectedY: i32 = 150;

    let optionsY: i32 = 330;

    let boxSize: u32 = 40;

    let mut group: i16 = 1;

    let mut selected: i16 = 15;

    let rockColor: Color = Color::RGB(54,69,79);
    let fireColor: Color = Color::RGB(255,0,0);
    let scissorsColor: Color = Color::RGB(211,211,211);
    let snakeColor: Color = Color::RGB(1,50,1);
    let humanColor: Color = Color::RGB(208,181,154);
    let treeColor: Color = Color::RGB(150,75,0);
    let wolfColor: Color = Color::RGB(0,0,0);
    let spongeColor: Color = Color::RGB(246,190,0);
    let paperColor: Color = Color::RGB(230,230,230);
    let airColor: Color = Color::RGB(175,175,255);
    let waterColor: Color = Color::RGB(0,50,255);
    let dragonColor: Color = Color::RGB(75,0,130);
    let devilColor: Color = Color::RGB(200,0,0);
    let lightningColor: Color = Color::RGB(200,150,255);
    let gunColor: Color = Color::RGB(50,50,50);
    let switchColor: Color = Color::RGB(0,255,200);
    let selectedColor: Color = Color::RGB(255, 255, 255);
    let mut selectedBoxColor: Color = Color::RGB(255, 255, 255);
    let mut ai_color: Color = rockColor;

    let mut player_score_text: String = "Hello World".to_string();

    //Create a square in the position of 35, 23, with a size of the square size
    let square = Rect::new(x_pos, y_pos, square_size, square_size);

    let background = Rect::new(0, 0, 800, 600);

    let boxBottom = Rect::new(50, 500, 700, 50);

    let boxTop = Rect::new(50, 50, 700, 50);

    let boxLeft = Rect::new(50, 50, 50, 450);

    let boxRight = Rect::new(700, 50, 50, 450);

    let divider = Rect::new(390, 300, 20, 200);

    let selectedBox = Rect::new(selectedX, selectedY, boxSize, boxSize);

    let ai_box = Rect::new(525, 150, boxSize, boxSize);

    let rockSquare = Rect::new(firstX, optionsY, boxSize, boxSize);

    let fireSquare = Rect::new(secondX, optionsY, boxSize, boxSize);

    let scissorSquare = Rect::new(thirdX, optionsY, boxSize, boxSize);

    let snakeSquare = Rect::new(fourthX, optionsY, boxSize, boxSize);

    let humanSquare = Rect::new(fifthX, optionsY, boxSize, boxSize);

    let treeSquare = Rect::new(firstX, optionsY, boxSize, boxSize);

    let wolfSquare = Rect::new(secondX, optionsY, boxSize, boxSize);

    let spongeSquare = Rect::new(thirdX, optionsY, boxSize, boxSize);

    let paperSquare = Rect::new(fourthX, optionsY, boxSize, boxSize);

    let airSquare = Rect::new(fifthX, optionsY, boxSize, boxSize);

    let waterSquare = Rect::new(firstX, optionsY, boxSize, boxSize);

    let dragonSquare = Rect::new(secondX, optionsY, boxSize, boxSize);

    let devilSquare = Rect::new(thirdX, optionsY, boxSize, boxSize);

    let lightningSquare = Rect::new(fourthX, optionsY, boxSize, boxSize);

    let gunSquare = Rect::new(fifthX, optionsY, boxSize, boxSize);

    let switchSquare = Rect::new(sixthX, optionsY, boxSize, boxSize);

    //Sets color to blue
    canvas.set_draw_color(Color::RGB(255,255,255));

    canvas.fill_rect(background);

    canvas.fill_rect(selectedBox);

    canvas.fill_rect(ai_box);

    canvas.set_draw_color(Color::RGB(0,0,0));

    canvas.fill_rect(boxBottom);

    canvas.fill_rect(boxTop);

    canvas.fill_rect(boxLeft);

    canvas.fill_rect(boxRight);

    canvas.fill_rect(divider);

    canvas.set_draw_color(rockColor);

    canvas.fill_rect(rockSquare);

    canvas.set_draw_color(fireColor);

    canvas.fill_rect(fireSquare);

    canvas.set_draw_color(scissorsColor);

    canvas.fill_rect(scissorSquare);

    canvas.set_draw_color(snakeColor);

    canvas.fill_rect(snakeSquare);

    canvas.set_draw_color(humanColor);

    canvas.fill_rect(humanSquare);

    canvas.set_draw_color(switchColor);

    canvas.fill_rect(switchSquare);

    canvas.set_draw_color(Color::RGB(0,255,200));

    //Fills the square with the color
    canvas.fill_rect(square);

    //Presents canvas
    canvas.present();

    while !win
    {

        //Allows events to happen 
        let mut event_pump = sdl_context.event_pump().unwrap();

        //Checks to see if the exit button is pressed
        'running: loop{
            if i < 10
            {
                ai_color = rockColor;
            }
            else if i >= 10 && i < 20
            {
                ai_color = fireColor;
            }
            else if i >= 20 && i < 30
            {
                ai_color = scissorsColor;
            }
            else if i >= 30 && i < 40
            {
                ai_color = snakeColor;
            }
            else if i >= 40 && i < 50
            {
                ai_color = humanColor;
            }
            else if i >= 50 && i < 60
            {
                ai_color = treeColor;
            }
            else if i >= 60 && i < 70
            {
                ai_color = wolfColor;
            }
            else if i >= 70 && i < 80
            {
                ai_color = spongeColor;
            }
            else if i >= 80 && i < 90
            {
                ai_color = paperColor;
            }
            else if i >= 90 && i < 100
            {
                ai_color = airColor;
            }
            else if i >= 100 && i < 110
            {
                ai_color = waterColor;
            }
            else if i >= 110 && i < 120
            {
                ai_color = dragonColor;
            }
            else if i >= 120 && i < 130
            {
                ai_color = devilColor;
            }
            else if i >= 130 && i < 140
            {
                ai_color = lightningColor;
            }
            else if i >= 140 && i < 150
            {
                ai_color = gunColor;
            }
            else
            {
                i = 0;
            }

            canvas.set_draw_color(ai_color);

            canvas.fill_rect(ai_box);

            canvas.present();

            i = i + 1;
        for event in event_pump.poll_iter()
        {
            match event {
                Event::Quit{..}|
                Event::KeyUp{keycode: Some(Keycode::Escape), ..} => {
                    win = true;
                    break 'running;
                },

                Event::KeyUp{keycode: Some(Keycode::W), ..} => {

                    for i in 1..15
                    {
                    canvas.set_draw_color(Color::RGB(0,0,0));
                    canvas.clear();

                    y_pos -= 10;

                    canvas.set_draw_color(Color::RGB(255,255,255));

                    canvas.fill_rect(background);

                    canvas.set_draw_color(Color::RGB(0,0,0));

                    canvas.fill_rect(boxBottom);

                    canvas.fill_rect(boxTop);

                    canvas.fill_rect(boxLeft);

                    canvas.fill_rect(boxRight);

                    canvas.fill_rect(divider);

                    canvas.set_draw_color(switchColor);

                    canvas.fill_rect(switchSquare);

                    canvas.set_draw_color(ai_color);

                    canvas.fill_rect(ai_box);

                    if group == 1
                    {
                        canvas.set_draw_color(rockColor);

                        canvas.fill_rect(rockSquare);

                        canvas.set_draw_color(fireColor);

                        canvas.fill_rect(fireSquare);

                        canvas.set_draw_color(scissorsColor);

                        canvas.fill_rect(scissorSquare);

                        canvas.set_draw_color(snakeColor);

                        canvas.fill_rect(snakeSquare);

                        canvas.set_draw_color(humanColor);

                        canvas.fill_rect(humanSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 0
                        {
                            canvas.fill_rect(rockSquare);

                            selectedBoxColor = rockColor;
                        }
                        else if selected == 1
                        {
                            canvas.fill_rect(fireSquare);

                            selectedBoxColor = fireColor;
                        }
                        else if selected == 2
                        {
                            canvas.fill_rect(scissorSquare);

                            selectedBoxColor = scissorsColor;
                        }
                        else if selected == 3
                        {
                            canvas.fill_rect(snakeSquare);

                            selectedBoxColor = snakeColor;
                        }
                        else if selected == 4
                        {
                            canvas.fill_rect(humanSquare);

                            selectedBoxColor = humanColor;
                        }

                    }
                    else if group == 2
                    {
                        canvas.set_draw_color(treeColor);

                        canvas.fill_rect(treeSquare);

                        canvas.set_draw_color(wolfColor);

                        canvas.fill_rect(wolfSquare);

                        canvas.set_draw_color(spongeColor);

                        canvas.fill_rect(spongeSquare);

                        canvas.set_draw_color(paperColor);

                        canvas.fill_rect(paperSquare);

                        canvas.set_draw_color(airColor);

                        canvas.fill_rect(airSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 5
                        {
                            canvas.fill_rect(treeSquare);

                            selectedBoxColor = treeColor;
                        }
                        else if selected == 6
                        {
                            canvas.fill_rect(wolfSquare);

                            selectedBoxColor = wolfColor;
                        }
                        else if selected == 7
                        {
                            canvas.fill_rect(spongeSquare);

                            selectedBoxColor = spongeColor;
                        }
                        else if selected == 8
                        {
                            canvas.fill_rect(paperSquare);

                            selectedBoxColor = paperColor;
                        }
                        else if selected == 9
                        {
                            canvas.fill_rect(airSquare);

                            selectedBoxColor = airColor;
                        }
                    }
                    else if group == 3
                    {
                        canvas.set_draw_color(waterColor);

                        canvas.fill_rect(waterSquare);

                        canvas.set_draw_color(dragonColor);

                        canvas.fill_rect(dragonSquare);

                        canvas.set_draw_color(devilColor);

                        canvas.fill_rect(devilSquare);

                        canvas.set_draw_color(lightningColor);

                        canvas.fill_rect(lightningSquare);

                        canvas.set_draw_color(gunColor);

                        canvas.fill_rect(gunSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 10
                        {
                            canvas.fill_rect(waterSquare);

                            selectedBoxColor = waterColor;
                        }
                        else if selected == 11
                        {
                            canvas.fill_rect(dragonSquare);

                            selectedBoxColor = dragonColor;
                        }
                        else if selected == 12
                        {
                            canvas.fill_rect(devilSquare);

                            selectedBoxColor = devilColor;
                        }
                        else if selected == 13
                        {
                            canvas.fill_rect(lightningSquare);

                            selectedBoxColor = lightningColor;
                        }
                        else if selected == 14
                        {
                            canvas.fill_rect(gunSquare);

                            selectedBoxColor = gunColor;
                        }
                    }

                    canvas.set_draw_color(selectedBoxColor);
                    canvas.fill_rect(selectedBox);

                    canvas.set_draw_color(Color::RGB(0, 255, 200));
                    canvas.fill_rect(Rect::new(x_pos, y_pos, square_size, square_size));

                    canvas.present();
                    std::thread::sleep(Duration::from_millis(16));
                    }

                    if x_pos > sixthX && x_pos < sixthX + 40 && y_pos > optionsY
                    {
                        group += 1;
                        if group > 3
                        {
                            group = 1;
                        }
                    }
                    else if group == 1
                    {
                        if x_pos > fifthX && x_pos < fifthX + 40 && y_pos > optionsY
                        {
                            player_decision = 4;
                            selected = 4;
                        }
                        else if x_pos > fourthX && x_pos < fourthX + 40 && y_pos > optionsY
                        {
                            player_decision = 3;
                            selected = 3;
                        }
                        else if x_pos > thirdX && x_pos < thirdX + 40 && y_pos > optionsY
                        {
                            player_decision = 2;
                            selected = 2;
                        }
                        else if x_pos > secondX && x_pos < secondX + 40 && y_pos > optionsY
                        {
                            player_decision = 1;
                            selected = 1;
                        }
                        else if x_pos > firstX && x_pos < firstX + 40 && y_pos > optionsY
                        {
                            player_decision = 0;
                            selected = 0;
                        }
                    }
                    else if group == 2
                    {
                        if x_pos > fifthX && x_pos < fifthX + 40 && y_pos > optionsY
                        {
                            player_decision = 9;
                            selected = 9;
                        }
                        else if x_pos > fourthX && x_pos < fourthX + 40 && y_pos > optionsY
                        {
                            player_decision = 8;
                            selected = 8;
                        }
                        else if x_pos > thirdX && x_pos < thirdX + 40 && y_pos > optionsY
                        {
                            player_decision = 7;
                            selected = 7;
                        }
                        else if x_pos > secondX && x_pos < secondX + 40 && y_pos > optionsY
                        {
                            player_decision = 6;
                            selected = 6;
                        }
                        else if x_pos > firstX && x_pos < firstX + 40 && y_pos > optionsY
                        {
                            player_decision = 5;
                            selected = 5;
                        }
                    }
                    else if group == 3
                    {
                        if x_pos > fifthX && x_pos < fifthX + 40 && y_pos > optionsY
                        {
                            player_decision = 14;
                            selected = 14;
                        }
                        else if x_pos > fourthX && x_pos < fourthX + 40 && y_pos > optionsY
                        {
                            player_decision = 13;
                            selected = 13;
                        }
                        else if x_pos > thirdX && x_pos < thirdX + 40 && y_pos > optionsY
                        {
                            player_decision = 12;
                            selected = 12;
                        }
                        else if x_pos > secondX && x_pos < secondX + 40 && y_pos > optionsY
                        {
                            player_decision = 11;
                            selected = 11;
                        }
                        else if x_pos > firstX && x_pos < firstX + 40 && y_pos > optionsY
                        {
                            player_decision = 10;
                            selected = 10;
                        }
                    }

                    for i in 1..15
                    {
                    canvas.set_draw_color(Color::RGB(0,0,0));
                    canvas.clear();

                    y_pos += 10;

                    canvas.set_draw_color(Color::RGB(255,255,255));

                    canvas.fill_rect(background);

                    canvas.set_draw_color(Color::RGB(0,0,0));

                    canvas.fill_rect(boxBottom);

                    canvas.fill_rect(boxTop);

                    canvas.fill_rect(boxLeft);

                    canvas.fill_rect(boxRight);

                    canvas.fill_rect(divider);

                    canvas.set_draw_color(switchColor);

                    canvas.fill_rect(switchSquare);

                    canvas.set_draw_color(ai_color);

                    canvas.fill_rect(ai_box);

                    if group == 1
                    {
                        canvas.set_draw_color(rockColor);

                        canvas.fill_rect(rockSquare);

                        canvas.set_draw_color(fireColor);

                        canvas.fill_rect(fireSquare);

                        canvas.set_draw_color(scissorsColor);

                        canvas.fill_rect(scissorSquare);

                        canvas.set_draw_color(snakeColor);

                        canvas.fill_rect(snakeSquare);

                        canvas.set_draw_color(humanColor);

                        canvas.fill_rect(humanSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 0
                        {
                            canvas.fill_rect(rockSquare);

                            selectedBoxColor = rockColor;
                        }
                        else if selected == 1
                        {
                            canvas.fill_rect(fireSquare);

                            selectedBoxColor = fireColor;
                        }
                        else if selected == 2
                        {
                            canvas.fill_rect(scissorSquare);

                            selectedBoxColor = scissorsColor;
                        }
                        else if selected == 3
                        {
                            canvas.fill_rect(snakeSquare);

                            selectedBoxColor = snakeColor;
                        }
                        else if selected == 4
                        {
                            canvas.fill_rect(humanSquare);

                            selectedBoxColor = humanColor;
                        }

                    }
                    else if group == 2
                    {
                        canvas.set_draw_color(treeColor);

                        canvas.fill_rect(treeSquare);

                        canvas.set_draw_color(wolfColor);

                        canvas.fill_rect(wolfSquare);

                        canvas.set_draw_color(spongeColor);

                        canvas.fill_rect(spongeSquare);

                        canvas.set_draw_color(paperColor);

                        canvas.fill_rect(paperSquare);

                        canvas.set_draw_color(airColor);

                        canvas.fill_rect(airSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 5
                        {
                            canvas.fill_rect(treeSquare);

                            selectedBoxColor = treeColor;
                        }
                        else if selected == 6
                        {
                            canvas.fill_rect(wolfSquare);

                            selectedBoxColor = wolfColor;
                        }
                        else if selected == 7
                        {
                            canvas.fill_rect(spongeSquare);

                            selectedBoxColor = spongeColor;
                        }
                        else if selected == 8
                        {
                            canvas.fill_rect(paperSquare);

                            selectedBoxColor = paperColor;
                        }
                        else if selected == 9
                        {
                            canvas.fill_rect(airSquare);

                            selectedBoxColor = airColor;
                        }
                    }
                    else if group == 3
                    {
                        canvas.set_draw_color(waterColor);

                        canvas.fill_rect(waterSquare);

                        canvas.set_draw_color(dragonColor);

                        canvas.fill_rect(dragonSquare);

                        canvas.set_draw_color(devilColor);

                        canvas.fill_rect(devilSquare);

                        canvas.set_draw_color(lightningColor);

                        canvas.fill_rect(lightningSquare);

                        canvas.set_draw_color(gunColor);

                        canvas.fill_rect(gunSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 10
                        {
                            canvas.fill_rect(waterSquare);

                            selectedBoxColor = waterColor;
                        }
                        else if selected == 11
                        {
                            canvas.fill_rect(dragonSquare);

                            selectedBoxColor = dragonColor;
                        }
                        else if selected == 12
                        {
                            canvas.fill_rect(devilSquare);

                            selectedBoxColor = devilColor;
                        }
                        else if selected == 13
                        {
                            canvas.fill_rect(lightningSquare);

                            selectedBoxColor = lightningColor;
                        }
                        else if selected == 14
                        {
                            canvas.fill_rect(gunSquare);

                            selectedBoxColor = gunColor;
                        }
                    }

                    canvas.set_draw_color(selectedBoxColor);
                    canvas.fill_rect(selectedBox);

                    canvas.set_draw_color(Color::RGB(0, 255, 200));
                    canvas.fill_rect(Rect::new(x_pos, y_pos, square_size, square_size));

                    canvas.present();
                    std::thread::sleep(Duration::from_millis(16));
                    }
                }

                Event::KeyDown{keycode: Some(Keycode::A), ..} => {

                    if x_pos > 100
                    {
                        canvas.set_draw_color(Color::RGB(0,0,0));
                        canvas.clear();

                        x_pos -= 10;

                        canvas.set_draw_color(Color::RGB(255,255,255));

                        canvas.fill_rect(background);

                        canvas.set_draw_color(Color::RGB(0,0,0));

                        canvas.fill_rect(boxBottom);

                        canvas.fill_rect(boxTop);

                        canvas.fill_rect(boxLeft);

                        canvas.fill_rect(boxRight);

                        canvas.fill_rect(divider);

                        canvas.set_draw_color(switchColor);

                    canvas.fill_rect(switchSquare);

                    canvas.set_draw_color(ai_color);

                    canvas.fill_rect(ai_box);


                    if group == 1
                    {
                        canvas.set_draw_color(rockColor);

                        canvas.fill_rect(rockSquare);

                        canvas.set_draw_color(fireColor);

                        canvas.fill_rect(fireSquare);

                        canvas.set_draw_color(scissorsColor);

                        canvas.fill_rect(scissorSquare);

                        canvas.set_draw_color(snakeColor);

                        canvas.fill_rect(snakeSquare);

                        canvas.set_draw_color(humanColor);

                        canvas.fill_rect(humanSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 0
                        {
                            canvas.fill_rect(rockSquare);

                            selectedBoxColor = rockColor;
                        }
                        else if selected == 1
                        {
                            canvas.fill_rect(fireSquare);

                            selectedBoxColor = fireColor;
                        }
                        else if selected == 2
                        {
                            canvas.fill_rect(scissorSquare);

                            selectedBoxColor = scissorsColor;
                        }
                        else if selected == 3
                        {
                            canvas.fill_rect(snakeSquare);

                            selectedBoxColor = snakeColor;
                        }
                        else if selected == 4
                        {
                            canvas.fill_rect(humanSquare);

                            selectedBoxColor = humanColor;
                        }

                    }
                    else if group == 2
                    {
                        canvas.set_draw_color(treeColor);

                        canvas.fill_rect(treeSquare);

                        canvas.set_draw_color(wolfColor);

                        canvas.fill_rect(wolfSquare);

                        canvas.set_draw_color(spongeColor);

                        canvas.fill_rect(spongeSquare);

                        canvas.set_draw_color(paperColor);

                        canvas.fill_rect(paperSquare);

                        canvas.set_draw_color(airColor);

                        canvas.fill_rect(airSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 5
                        {
                            canvas.fill_rect(treeSquare);

                            selectedBoxColor = treeColor;
                        }
                        else if selected == 6
                        {
                            canvas.fill_rect(wolfSquare);

                            selectedBoxColor = wolfColor;
                        }
                        else if selected == 7
                        {
                            canvas.fill_rect(spongeSquare);

                            selectedBoxColor = spongeColor;
                        }
                        else if selected == 8
                        {
                            canvas.fill_rect(paperSquare);

                            selectedBoxColor = paperColor;
                        }
                        else if selected == 9
                        {
                            canvas.fill_rect(airSquare);

                            selectedBoxColor = airColor;
                        }
                    }
                    else if group == 3
                    {
                        canvas.set_draw_color(waterColor);

                        canvas.fill_rect(waterSquare);

                        canvas.set_draw_color(dragonColor);

                        canvas.fill_rect(dragonSquare);

                        canvas.set_draw_color(devilColor);

                        canvas.fill_rect(devilSquare);

                        canvas.set_draw_color(lightningColor);

                        canvas.fill_rect(lightningSquare);

                        canvas.set_draw_color(gunColor);

                        canvas.fill_rect(gunSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 10
                        {
                            canvas.fill_rect(waterSquare);

                            selectedBoxColor = waterColor;
                        }
                        else if selected == 11
                        {
                            canvas.fill_rect(dragonSquare);

                            selectedBoxColor = dragonColor;
                        }
                        else if selected == 12
                        {
                            canvas.fill_rect(devilSquare);

                            selectedBoxColor = devilColor;
                        }
                        else if selected == 13
                        {
                            canvas.fill_rect(lightningSquare);

                            selectedBoxColor = lightningColor;
                        }
                        else if selected == 14
                        {
                            canvas.fill_rect(gunSquare);

                            selectedBoxColor = gunColor;
                        }
                    }

                    canvas.set_draw_color(selectedBoxColor);
                    canvas.fill_rect(selectedBox);

                        canvas.set_draw_color(Color::RGB(0, 255, 200));
                        canvas.fill_rect(Rect::new(x_pos, y_pos, square_size, square_size));

                        canvas.present();
                    }
                }

                Event::KeyDown{keycode: Some(Keycode::D), ..} => {

                    if x_pos < 370
                    {
                        canvas.set_draw_color(Color::RGB(0,0,0));
                        canvas.clear();

                        x_pos += 10;

                        canvas.set_draw_color(Color::RGB(255,255,255));

                        canvas.fill_rect(background);

                        canvas.set_draw_color(Color::RGB(0,0,0));

                        canvas.fill_rect(boxBottom);

                        canvas.fill_rect(boxTop);

                        canvas.fill_rect(boxLeft);

                        canvas.fill_rect(boxRight);

                        canvas.fill_rect(divider);

                        canvas.set_draw_color(switchColor);

                        canvas.fill_rect(switchSquare);

                        canvas.set_draw_color(ai_color);

                    canvas.fill_rect(ai_box);

                    if group == 1
                    {
                        canvas.set_draw_color(rockColor);

                        canvas.fill_rect(rockSquare);

                        canvas.set_draw_color(fireColor);

                        canvas.fill_rect(fireSquare);

                        canvas.set_draw_color(scissorsColor);

                        canvas.fill_rect(scissorSquare);

                        canvas.set_draw_color(snakeColor);

                        canvas.fill_rect(snakeSquare);

                        canvas.set_draw_color(humanColor);

                        canvas.fill_rect(humanSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 0
                        {
                            canvas.fill_rect(rockSquare);

                            selectedBoxColor = rockColor;
                        }
                        else if selected == 1
                        {
                            canvas.fill_rect(fireSquare);

                            selectedBoxColor = fireColor;
                        }
                        else if selected == 2
                        {
                            canvas.fill_rect(scissorSquare);

                            selectedBoxColor = scissorsColor;
                        }
                        else if selected == 3
                        {
                            canvas.fill_rect(snakeSquare);

                            selectedBoxColor = snakeColor;
                        }
                        else if selected == 4
                        {
                            canvas.fill_rect(humanSquare);

                            selectedBoxColor = humanColor;
                        }

                    }
                    else if group == 2
                    {
                        canvas.set_draw_color(treeColor);

                        canvas.fill_rect(treeSquare);

                        canvas.set_draw_color(wolfColor);

                        canvas.fill_rect(wolfSquare);

                        canvas.set_draw_color(spongeColor);

                        canvas.fill_rect(spongeSquare);

                        canvas.set_draw_color(paperColor);

                        canvas.fill_rect(paperSquare);

                        canvas.set_draw_color(airColor);

                        canvas.fill_rect(airSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 5
                        {
                            canvas.fill_rect(treeSquare);

                            selectedBoxColor = treeColor;
                        }
                        else if selected == 6
                        {
                            canvas.fill_rect(wolfSquare);

                            selectedBoxColor = wolfColor;
                        }
                        else if selected == 7
                        {
                            canvas.fill_rect(spongeSquare);

                            selectedBoxColor = spongeColor;
                        }
                        else if selected == 8
                        {
                            canvas.fill_rect(paperSquare);

                            selectedBoxColor = paperColor;
                        }
                        else if selected == 9
                        {
                            canvas.fill_rect(airSquare);

                            selectedBoxColor = airColor;
                        }
                    }
                    else if group == 3
                    {
                        canvas.set_draw_color(waterColor);

                        canvas.fill_rect(waterSquare);

                        canvas.set_draw_color(dragonColor);

                        canvas.fill_rect(dragonSquare);

                        canvas.set_draw_color(devilColor);

                        canvas.fill_rect(devilSquare);

                        canvas.set_draw_color(lightningColor);

                        canvas.fill_rect(lightningSquare);

                        canvas.set_draw_color(gunColor);

                        canvas.fill_rect(gunSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 10
                        {
                            canvas.fill_rect(waterSquare);

                            selectedBoxColor = waterColor;
                        }
                        else if selected == 11
                        {
                            canvas.fill_rect(dragonSquare);

                            selectedBoxColor = dragonColor;
                        }
                        else if selected == 12
                        {
                            canvas.fill_rect(devilSquare);

                            selectedBoxColor = devilColor;
                        }
                        else if selected == 13
                        {
                            canvas.fill_rect(lightningSquare);

                            selectedBoxColor = lightningColor;
                        }
                        else if selected == 14
                        {
                            canvas.fill_rect(gunSquare);

                            selectedBoxColor = gunColor;
                        }
                    }

                    canvas.set_draw_color(selectedBoxColor);
                    canvas.fill_rect(selectedBox);

                        canvas.set_draw_color(Color::RGB(0, 255, 200));
                        canvas.fill_rect(Rect::new(x_pos, y_pos, square_size, square_size));

                        canvas.present();
                    }
                }
                
                Event::KeyDown{keycode: Some(Keycode::Return), ..} => {

                    
                        canvas.set_draw_color(Color::RGB(0,0,0));
                        canvas.clear();

                        canvas.set_draw_color(Color::RGB(255,255,255));

                        canvas.fill_rect(background);

                        canvas.set_draw_color(Color::RGB(0,0,0));

                        canvas.fill_rect(boxBottom);

                        canvas.fill_rect(boxTop);

                        canvas.fill_rect(boxLeft);

                        canvas.fill_rect(boxRight);

                        canvas.fill_rect(divider);

                        canvas.set_draw_color(switchColor);

                        canvas.fill_rect(switchSquare);

                        canvas.set_draw_color(ai_color);

                    canvas.fill_rect(ai_box);


                    if group == 1
                    {
                        canvas.set_draw_color(rockColor);

                        canvas.fill_rect(rockSquare);

                        canvas.set_draw_color(fireColor);

                        canvas.fill_rect(fireSquare);

                        canvas.set_draw_color(scissorsColor);

                        canvas.fill_rect(scissorSquare);

                        canvas.set_draw_color(snakeColor);

                        canvas.fill_rect(snakeSquare);

                        canvas.set_draw_color(humanColor);

                        canvas.fill_rect(humanSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 0
                        {
                            canvas.fill_rect(rockSquare);

                            selectedBoxColor = rockColor;
                        }
                        else if selected == 1
                        {
                            canvas.fill_rect(fireSquare);

                            selectedBoxColor = fireColor;
                        }
                        else if selected == 2
                        {
                            canvas.fill_rect(scissorSquare);

                            selectedBoxColor = scissorsColor;
                        }
                        else if selected == 3
                        {
                            canvas.fill_rect(snakeSquare);

                            selectedBoxColor = snakeColor;
                        }
                        else if selected == 4
                        {
                            canvas.fill_rect(humanSquare);

                            selectedBoxColor = humanColor;
                        }

                    }
                    else if group == 2
                    {
                        canvas.set_draw_color(treeColor);

                        canvas.fill_rect(treeSquare);

                        canvas.set_draw_color(wolfColor);

                        canvas.fill_rect(wolfSquare);

                        canvas.set_draw_color(spongeColor);

                        canvas.fill_rect(spongeSquare);

                        canvas.set_draw_color(paperColor);

                        canvas.fill_rect(paperSquare);

                        canvas.set_draw_color(airColor);

                        canvas.fill_rect(airSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 5
                        {
                            canvas.fill_rect(treeSquare);

                            selectedBoxColor = treeColor;
                        }
                        else if selected == 6
                        {
                            canvas.fill_rect(wolfSquare);

                            selectedBoxColor = wolfColor;
                        }
                        else if selected == 7
                        {
                            canvas.fill_rect(spongeSquare);

                            selectedBoxColor = spongeColor;
                        }
                        else if selected == 8
                        {
                            canvas.fill_rect(paperSquare);

                            selectedBoxColor = paperColor;
                        }
                        else if selected == 9
                        {
                            canvas.fill_rect(airSquare);

                            selectedBoxColor = airColor;
                        }
                    }
                    else if group == 3
                    {
                        canvas.set_draw_color(waterColor);

                        canvas.fill_rect(waterSquare);

                        canvas.set_draw_color(dragonColor);

                        canvas.fill_rect(dragonSquare);

                        canvas.set_draw_color(devilColor);

                        canvas.fill_rect(devilSquare);

                        canvas.set_draw_color(lightningColor);

                        canvas.fill_rect(lightningSquare);

                        canvas.set_draw_color(gunColor);

                        canvas.fill_rect(gunSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 10
                        {
                            canvas.fill_rect(waterSquare);

                            selectedBoxColor = waterColor;
                        }
                        else if selected == 11
                        {
                            canvas.fill_rect(dragonSquare);

                            selectedBoxColor = dragonColor;
                        }
                        else if selected == 12
                        {
                            canvas.fill_rect(devilSquare);

                            selectedBoxColor = devilColor;
                        }
                        else if selected == 13
                        {
                            canvas.fill_rect(lightningSquare);

                            selectedBoxColor = lightningColor;
                        }
                        else if selected == 14
                        {
                            canvas.fill_rect(gunSquare);

                            selectedBoxColor = gunColor;
                        }
                    }

                        canvas.set_draw_color(selectedBoxColor);
                        canvas.fill_rect(selectedBox);

                        canvas.set_draw_color(Color::RGB(0, 255, 200));
                        canvas.fill_rect(Rect::new(x_pos, y_pos, square_size, square_size));

                        canvas.present();

                        break 'running;
                }

                _ => {}

                
            }
        }
        sleep(Duration::from_millis(50));
        }

    ai_decision = aiDecision();

     //Decides who won. 
     if player_decision == 0
     {
         //Player chose rock
         temp = p_rock(&ai_decision, &mut ai_color);
     }
     else if player_decision == 1
     {
         //Player chose fire
         temp = p_fire(&ai_decision, &mut ai_color);
     }
     else if player_decision == 2
     {
         //Player chose scissors
         temp = p_scissors(&ai_decision, &mut ai_color);
     }
     else if player_decision == 3
     {
         //Player chose snake
         temp = p_snake(&ai_decision, &mut ai_color);
     }
     else if player_decision == 4
     {
         //Player chose human
         temp = p_human(&ai_decision, &mut ai_color);
     }
     else if player_decision == 5
     {
         //Player chose tree
         temp = p_tree(&ai_decision, &mut ai_color);
     }
     else if player_decision == 6
     {
         //Player chose wolf
         temp = p_wolf(&ai_decision, &mut ai_color);
     }
     else if player_decision == 7
     {
         //Player chose sponge
         temp = p_sponge(&ai_decision, &mut ai_color);
     }
     else if player_decision == 8
     {
         //Player chose paper
         temp = p_paper(&ai_decision, &mut ai_color);
     }
     else if player_decision == 9
     {
         //Player chose air
         temp = p_air(&ai_decision, &mut ai_color);
     }
     else if player_decision == 10
     {
         //Player chose water
         temp = p_water(&ai_decision, &mut ai_color);
     }
     else if player_decision == 11
     {
         //Player chose dragon
         temp = p_dragon(&ai_decision, &mut ai_color);
     }
     else if player_decision == 12
     {
         //Player chose devil
         temp = p_devil(&ai_decision, &mut ai_color);
     }
     else if player_decision == 13
     {
         //Player chose lightning
         temp = p_lightning(&ai_decision, &mut ai_color);
     }
     else if player_decision == 14
     {
         //Player chose violence
         temp = p_gun(&ai_decision, &mut ai_color);
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

     canvas.set_draw_color(Color::RGB(0,0,0));
                        canvas.clear();

                        canvas.set_draw_color(Color::RGB(255,255,255));

                        canvas.fill_rect(background);

                        canvas.set_draw_color(Color::RGB(0,0,0));

                        canvas.fill_rect(boxBottom);

                        canvas.fill_rect(boxTop);

                        canvas.fill_rect(boxLeft);

                        canvas.fill_rect(boxRight);

                        canvas.fill_rect(divider);

                        canvas.set_draw_color(switchColor);

                        canvas.fill_rect(switchSquare);

                        canvas.set_draw_color(ai_color);

                    canvas.fill_rect(ai_box);


                    if group == 1
                    {
                        canvas.set_draw_color(rockColor);

                        canvas.fill_rect(rockSquare);

                        canvas.set_draw_color(fireColor);

                        canvas.fill_rect(fireSquare);

                        canvas.set_draw_color(scissorsColor);

                        canvas.fill_rect(scissorSquare);

                        canvas.set_draw_color(snakeColor);

                        canvas.fill_rect(snakeSquare);

                        canvas.set_draw_color(humanColor);

                        canvas.fill_rect(humanSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 0
                        {
                            canvas.fill_rect(rockSquare);

                            selectedBoxColor = rockColor;
                        }
                        else if selected == 1
                        {
                            canvas.fill_rect(fireSquare);

                            selectedBoxColor = fireColor;
                        }
                        else if selected == 2
                        {
                            canvas.fill_rect(scissorSquare);

                            selectedBoxColor = scissorsColor;
                        }
                        else if selected == 3
                        {
                            canvas.fill_rect(snakeSquare);

                            selectedBoxColor = snakeColor;
                        }
                        else if selected == 4
                        {
                            canvas.fill_rect(humanSquare);

                            selectedBoxColor = humanColor;
                        }

                    }
                    else if group == 2
                    {
                        canvas.set_draw_color(treeColor);

                        canvas.fill_rect(treeSquare);

                        canvas.set_draw_color(wolfColor);

                        canvas.fill_rect(wolfSquare);

                        canvas.set_draw_color(spongeColor);

                        canvas.fill_rect(spongeSquare);

                        canvas.set_draw_color(paperColor);

                        canvas.fill_rect(paperSquare);

                        canvas.set_draw_color(airColor);

                        canvas.fill_rect(airSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 5
                        {
                            canvas.fill_rect(treeSquare);

                            selectedBoxColor = treeColor;
                        }
                        else if selected == 6
                        {
                            canvas.fill_rect(wolfSquare);

                            selectedBoxColor = wolfColor;
                        }
                        else if selected == 7
                        {
                            canvas.fill_rect(spongeSquare);

                            selectedBoxColor = spongeColor;
                        }
                        else if selected == 8
                        {
                            canvas.fill_rect(paperSquare);

                            selectedBoxColor = paperColor;
                        }
                        else if selected == 9
                        {
                            canvas.fill_rect(airSquare);

                            selectedBoxColor = airColor;
                        }
                    }
                    else if group == 3
                    {
                        canvas.set_draw_color(waterColor);

                        canvas.fill_rect(waterSquare);

                        canvas.set_draw_color(dragonColor);

                        canvas.fill_rect(dragonSquare);

                        canvas.set_draw_color(devilColor);

                        canvas.fill_rect(devilSquare);

                        canvas.set_draw_color(lightningColor);

                        canvas.fill_rect(lightningSquare);

                        canvas.set_draw_color(gunColor);

                        canvas.fill_rect(gunSquare);

                        canvas.set_draw_color(selectedColor);

                        if selected == 10
                        {
                            canvas.fill_rect(waterSquare);

                            selectedBoxColor = waterColor;
                        }
                        else if selected == 11
                        {
                            canvas.fill_rect(dragonSquare);

                            selectedBoxColor = dragonColor;
                        }
                        else if selected == 12
                        {
                            canvas.fill_rect(devilSquare);

                            selectedBoxColor = devilColor;
                        }
                        else if selected == 13
                        {
                            canvas.fill_rect(lightningSquare);

                            selectedBoxColor = lightningColor;
                        }
                        else if selected == 14
                        {
                            canvas.fill_rect(gunSquare);

                            selectedBoxColor = gunColor;
                        }
                    }

                        canvas.set_draw_color(selectedBoxColor);
                        canvas.fill_rect(selectedBox);

                        canvas.set_draw_color(Color::RGB(0, 255, 200));
                        canvas.fill_rect(Rect::new(x_pos, y_pos, square_size, square_size));

                        canvas.present();

     //Prints out both scores
     println!("Score: {} - {}", player_win, ai_win);

     //If either the player or the ai wins, the loop will end
     if player_win >= 5
     {
         win = true;
     }
     else if ai_win >= 5
     {
         win = true;
     }

     sleep(Duration::from_millis(1000));

    }
 

    if player_win >= 2
    {
        //If it ends with the player winning two
        println!("Congrtulations, you won!");
    }
    else if ai_win >= 2
    {
        //If it ends with the ai winning two
        println!("Better luck next time, but the AI has won.");
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
fn p_rock(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("rock - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(t);
    }   
    else if *ai == 1
    {
        println!("rock - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(p);
    } 
    else if *ai == 2
    {
        println!("rock - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(p);
    }
    else if *ai == 3
    {
        println!("rock - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(p);
    }
    else if *ai == 4
    {
        println!("rock - Human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(p);
    }
    else if *ai == 5
    {
        println!("rock - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(p);
    }
    else if *ai == 6
    {
        println!("rock - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(p);
    }
    else if *ai == 7
    {
        println!("rock - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(p);
    }
    else if *ai == 8
    {
        println!("rock - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(a);
    }
    else if *ai == 9
    {
        println!("rock - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(a);
    }
    else if *ai == 10
    {
        println!("rock - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(a);
    }
    else if *ai == 11
    {
        println!("rock - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(a);
    }
    else if *ai == 12
    {
        println!("rock - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(a);
    }
    else if *ai == 13
    {
        println!("rock - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(a);
    }
    else if *ai == 14
    {
        println!("rock - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(a);
    }
    return temp;
}

fn p_fire(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("fire - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }   
    else if *ai == 1
    {
        println!("fire - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(t);
    } 
    else if *ai == 2
    {
        println!("fire - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(p);
    }
    else if *ai == 3
    {
        println!("fire - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(p);
    }
    else if *ai == 4
    {
        println!("fire - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(p);
    }
    else if *ai == 5
    {
        println!("fire - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(p);
    }
    else if *ai == 6
    {
        println!("fire - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(p);
    }
    else if *ai == 7
    {
        println!("fire - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(p);
    }
    else if *ai == 8
    {
        println!("fire - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(p);
    }
    else if *ai == 9
    {
        println!("fire - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(a);
    }
    else if *ai == 10
    {
        println!("fire - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(a);
    }
    else if *ai == 11
    {
        println!("fire - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(a);
    }
    else if *ai == 12
    {
        println!("fire - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(a);
    }
    else if *ai == 13
    {
        println!("fire - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(a);
    }
    else if *ai == 14
    {
        println!("fire - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(a);
    }
    return temp;
}

fn p_scissors(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("scissors - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }   
    else if *ai == 1
    {
        println!("scissor - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(a);
    } 
    else if *ai == 2
    {
        println!("scissor - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(t);
    }
    else if *ai == 3
    {
        println!("scissor - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(p);
    }
    else if *ai == 4
    {
        println!("scissor - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(p);
    }
    else if *ai == 5
    {
        println!("scissor - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(p);
    }
    else if *ai == 6
    {
        println!("scissor - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(p);
    }
    else if *ai == 7
    {
        println!("scissor - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(p);
    }
    else if *ai == 8
    {
        println!("scissor - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(p);
    }
    else if *ai == 9
    {
        println!("scissor - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(p);
    }
    else if *ai == 10
    {
        println!("scissor - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(a);
    }
    else if *ai == 11
    {
        println!("scissor - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(a);
    }
    else if *ai == 12
    {
        println!("scissor - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(a);
    }
    else if *ai == 13
    {
        println!("scissor - lighting");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(a);
    }
    else if *ai == 14
    {
        println!("scissor - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(a);
    }
    return temp;
}

fn p_snake(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("snake - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }   
    else if *ai == 1
    {
        println!("snake - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(a);
    } 
    else if *ai == 2
    {
        println!("snake - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(a);
    }
    else if *ai == 3
    {
        println!("snake - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(t);
    }
    else if *ai == 4
    {
        println!("snake - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(p);
    }
    else if *ai == 5
    {
        println!("snake - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(p);
    }
    else if *ai == 6
    {
        println!("snake - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(p);
    }
    else if *ai == 7
    {
        println!("snake - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(p);
    }
    else if *ai == 8
    {
        println!("snake - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(p);
    }
    else if *ai == 9
    {
        println!("snake - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(p);
    }
    else if *ai == 10
    {
        println!("snake - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(p);
    }
    else if *ai == 11
    {
        println!("snake - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(a);
    }
    else if *ai == 12
    {
        println!("snake - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(a);
    }
    else if *ai == 13
    {
        println!("snake - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(a);
    }
    else if *ai == 14
    {
        println!("snake - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(a);
    }
    return temp;
}

fn p_human(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("human - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }   
    else if *ai == 1
    {
        println!("human - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(a);
    } 
    else if *ai == 2
    {
        println!("human - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(a);
    }
    else if *ai == 3
    {
        println!("human - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(a);
    }
    else if *ai == 4
    {
        println!("human - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(t);
    }
    else if *ai == 5
    {
        println!("human - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(p);
    }
    else if *ai == 6
    {
        println!("human - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(p);
    }
    else if *ai == 7
    {
        println!("human - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(p);
    }
    else if *ai == 8
    {
        println!("human - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(p);
    }
    else if *ai == 9
    {
        println!("human - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(p);
    }
    else if *ai == 10
    {
        println!("human - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(p);
    }
    else if *ai == 11
    {
        println!("human - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(p);
    }
    else if *ai == 12
    {
        println!("human - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(a);
    }
    else if *ai == 13
    {
        println!("human - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(a);
    }
    else if *ai == 14
    {
        println!("human - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(a);
    }
    return temp;
}

fn p_tree(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("tree - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }   
    else if *ai == 1
    {
        println!("tree - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(a);
    } 
    else if *ai == 2
    {
        println!("tree - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(a);
    }
    else if *ai == 3
    {
        println!("tree - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(a);
    }
    else if *ai == 4
    {
        println!("tree - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(a);
    }
    else if *ai == 5
    {
        println!("tree - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(t);
    }
    else if *ai == 6
    {
        println!("tree - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(p);
    }
    else if *ai == 7
    {
        println!("tree - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(p);
    }
    else if *ai == 8
    {
        println!("tree - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(p);
    }
    else if *ai == 9
    {
        println!("tree - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(p);
    }
    else if *ai == 10
    {
        println!("tree - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(p);
    }
    else if *ai == 11
    {
        println!("tree - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(p);
    }
    else if *ai == 12
    {
        println!("tree - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(p);
    }
    else if *ai == 13
    {
        println!("tree - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(a);
    }
    else if *ai == 14
    {
        println!("tree - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(a);
    }
    return temp;
}

fn p_wolf(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("wolf - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }   
    else if *ai == 1
    {
        println!("wolf - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(a);
    } 
    else if *ai == 2
    {
        println!("wolf - scissors");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(a);
    }
    else if *ai == 3
    {
        println!("wolf - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(a);
    }
    else if *ai == 4
    {
        println!("wolf - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(a);
    }
    else if *ai == 5
    {
        println!("wolf - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(a);
    }
    else if *ai == 6
    {
        println!("wolf - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(t);
    }
    else if *ai == 7
    {
        println!("wolf - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(p);
    }
    else if *ai == 8
    {
        println!("wolf - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(p);
    }
    else if *ai == 9
    {
        println!("wolf - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(p);
    }
    else if *ai == 10
    {
        println!("wolf - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(p);
    }
    else if *ai == 11
    {
        println!("wolf - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(p);
    }
    else if *ai == 12
    {
        println!("wolf - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(p);
    }
    else if *ai == 13
    {
        println!("wolf - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(p);
    }
    else if *ai == 14
    {
        println!("wolf - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(a);
    }
    return temp;
}

fn p_sponge(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("sponge - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }   
    else if *ai == 1
    {
        println!("sponge - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(a);
    } 
    else if *ai == 2
    {
        println!("sponge - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(a);
    }
    else if *ai == 3
    {
        println!("sponge - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(a);
    }
    else if *ai == 4
    {
        println!("sponge - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(a);
    }
    else if *ai == 5
    {
        println!("spong - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(a);
    }
    else if *ai == 6
    {
        println!("sponge - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(a);
    }
    else if *ai == 7
    {
        println!("sponge - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(t);
    }
    else if *ai == 8
    {
        println!("sponge - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(p);
    }
    else if *ai == 9
    {
        println!("sponge - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(p);
    }
    else if *ai == 10
    {
        println!("sponge - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(p);
    }
    else if *ai == 11
    {
        println!("sponge - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(p);
    }
    else if *ai == 12
    {
        println!("sponge - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(p);
    }
    else if *ai == 13
    {
        println!("sponge - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(p);
    }
    else if *ai == 14
    {
        println!("sponge - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(p);
    }
    return temp;
}

fn p_paper(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("paper - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(p);
    }   
    else if *ai == 1
    {
        println!("paper - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(a);
    } 
    else if *ai == 2
    {
        println!("paper - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(a);
    }
    else if *ai == 3
    {
        println!("paper - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(a);
    }
    else if *ai == 4
    {
        println!("paper - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(a);
    }
    else if *ai == 5
    {
        println!("paper - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(a);
    }
    else if *ai == 6
    {
        println!("paper - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(a);
    }
    else if *ai == 7
    {
        println!("paper - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(a);
    }
    else if *ai == 8
    {
        println!("paper - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(t);
    }
    else if *ai == 9
    {
        println!("paper - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(p);
    }
    else if *ai == 10
    {
        println!("paper - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(p);
    }
    else if *ai == 11
    {
        println!("paper - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(p);
    }
    else if *ai == 12
    {
        println!("paper - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(p);
    }
    else if *ai == 13
    {
        println!("paper - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(p);
    }
    else if *ai == 14
    {
        println!("paper - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(p);
    }
    return temp;
}

fn p_air(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("air - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(p);
    }   
    else if *ai == 1
    {
        println!("air - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(p);
    } 
    else if *ai == 2
    {
        println!("air - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(a);
    }
    else if *ai == 3
    {
        println!("air - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(a);
    }
    else if *ai == 4
    {
        println!("air - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(a);
    }
    else if *ai == 5
    {
        println!("air - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(a);
    }
    else if *ai == 6
    {
        println!("air - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(a);
    }
    else if *ai == 7
    {
        println!("air - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(a);
    }
    else if *ai == 8
    {
        println!("air - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(a);
    }
    else if *ai == 9
    {
        println!("air - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(t);
    }
    else if *ai == 10
    {
        println!("air - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(p);
    }
    else if *ai == 11
    {
        println!("air - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(p);
    }
    else if *ai == 12
    {
        println!("air - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(p);
    }
    else if *ai == 13
    {
        println!("air - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(p);
    }
    else if *ai == 14
    {
        println!("air - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(p);
    }
    return temp;
}

fn p_water(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("water - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(p);
    }   
    else if *ai == 1
    {
        println!("water - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(p);
    } 
    else if *ai == 2
    {
        println!("water - scissors");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(p);
    }
    else if *ai == 3
    {
        println!("water - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(a);
    }
    else if *ai == 4
    {
        println!("water - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(a);
    }
    else if *ai == 5
    {
        println!("water - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(a);
    }
    else if *ai == 6
    {
        println!("water - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(a);
    }
    else if *ai == 7
    {
        println!("water - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(a);
    }
    else if *ai == 8
    {
        println!("water - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(a);
    }
    else if *ai == 9
    {
        println!("water - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(a);
    }
    else if *ai == 10
    {
        println!("water - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(t);
    }
    else if *ai == 11
    {
        println!("water - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(p);
    }
    else if *ai == 12
    {
        println!("water - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(p);
    }
    else if *ai == 13
    {
        println!("water - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(p);
    }
    else if *ai == 14
    {
        println!("water - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(p);
    }
    return temp;
}

fn p_dragon(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("dragon - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(p);
    }   
    else if *ai == 1
    {
        println!("dragon - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(p);
    } 
    else if *ai == 2
    {
        println!("dragon - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(p);
    }
    else if *ai == 3
    {
        println!("dragon - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(p);
    }
    else if *ai == 4
    {
        println!("dragon - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(a);
    }
    else if *ai == 5
    {
        println!("dragon - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(a);
    }
    else if *ai == 6
    {
        println!("dragon - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(a);
    }
    else if *ai == 7
    {
        println!("dragon - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(a);
    }
    else if *ai == 8
    {
        println!("dragon - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(a);
    }
    else if *ai == 9
    {
        println!("dragon - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(a);
    }
    else if *ai == 10
    {
        println!("dragon - water");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(a);
    }
    else if *ai == 11
    {
        println!("dragon - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(t);
    }
    else if *ai == 12
    {
        println!("dragon - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(p);
    }
    else if *ai == 13
    {
        println!("dragon - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(p);
    }
    else if *ai == 14
    {
        println!("dragon - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp =who_won(p);
    }
    return temp;
}

fn p_devil(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("devil - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(p);
    }   
    else if *ai == 1
    {
        println!("devil - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(p);
    } 
    else if *ai == 2
    {
        println!("devil - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(p);
    }
    else if *ai == 3
    {
        println!("devil - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(p);
    }
    else if *ai == 4
    {
        println!("devil - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(p);
    }
    else if *ai == 5
    {
        println!("devil - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(a);
    }
    else if *ai == 6
    {
        println!("devil - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(a);
    }
    else if *ai == 7
    {
        println!("devil - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(a);
    }
    else if *ai == 8
    {
        println!("devil - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(a);
    }
    else if *ai == 9
    {
        println!("devil - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(a);
    }
    else if *ai == 10
    {
        println!("devil - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(a);
    }
    else if *ai == 11
    {
        println!("devil - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(a);
    }
    else if *ai == 12
    {
        println!("devil - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(t);
    }
    else if *ai == 13
    {
        println!("devil - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(p);
    }
    else if *ai == 14
    {
        println!("devil - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(p);
    }
    return temp;
}

fn p_lightning(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("lightning - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(p);
    }   
    else if *ai == 1
    {
        println!("lightning - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(p);
    } 
    else if *ai == 2
    {
        println!("lightning - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(p);
    }
    else if *ai == 3
    {
        println!("lightning - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(p);
    }
    else if *ai == 4
    {
        println!("lightning - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(p);
    }
    else if *ai == 5
    {
        println!("lightning - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(p);
    }
    else if *ai == 6
    {
        println!("lightning - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(a);
    }
    else if *ai == 7
    {
        println!("lightning - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(a);
    }
    else if *ai == 8
    {
        println!("lightning - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(a);
    }
    else if *ai == 9
    {
        println!("lightning - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(a);
    }
    else if *ai == 10
    {
        println!("lightning - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(a);
    }
    else if *ai == 11
    {
        println!("lightning - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(a);
    }
    else if *ai == 12
    {
        println!("lightning - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(a);
    }
    else if *ai == 13
    {
        println!("lightning - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(t);
    }
    else if *ai == 14
    {
        println!("lightning - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(p);
    }
    return temp;
}

fn p_gun(ai: &i32, ai_Color: &mut Color) -> i32
{
    let mut temp: i32 = 0;

    if *ai == 0
    {
        println!("gun - rock");
        *ai_Color = Color::RGB(54,69,79);
        temp = who_won(p);
    }   
    else if *ai == 1
    {
        println!("gun - fire");
        *ai_Color = Color::RGB(255,0,0);
        temp = who_won(p);
    } 
    else if *ai == 2
    {
        println!("gun - scissor");
        *ai_Color = Color::RGB(211,211,211);
        temp = who_won(p);
    }
    else if *ai == 3
    {
        println!("gun - snake");
        *ai_Color = Color::RGB(1,50,1);
        temp = who_won(p);
    }
    else if *ai == 4
    {
        println!("gun - human");
        *ai_Color = Color::RGB(208,181,154);
        temp = who_won(p);
    }
    else if *ai == 5
    {
        println!("gun - tree");
        *ai_Color = Color::RGB(150,75,0);
        temp = who_won(p);
    }
    else if *ai == 6
    {
        println!("gun - wolf");
        *ai_Color = Color::RGB(0,0,0);
        temp = who_won(p);
    }
    else if *ai == 7
    {
        println!("gun - sponge");
        *ai_Color = Color::RGB(246,190,0);
        temp = who_won(a);
    }
    else if *ai == 8
    {
        println!("gun - paper");
        *ai_Color = Color::RGB(230,230,230);
        temp = who_won(a);
    }
    else if *ai == 9
    {
        println!("gun - air");
        *ai_Color = Color::RGB(175,175,255);
        temp = who_won(a);
    }
    else if *ai == 10
    {
        println!("Gun - water");
        *ai_Color = Color::RGB(0,50,255);
        temp = who_won(a);
    }
    else if *ai == 11
    {
        println!("gun - dragon");
        *ai_Color = Color::RGB(75,0,130);
        temp = who_won(a);
    }
    else if *ai == 12
    {
        println!("gun - devil");
        *ai_Color = Color::RGB(200,0,0);
        temp = who_won(a);
    }
    else if *ai == 13
    {
        println!("gun - lightning");
        *ai_Color = Color::RGB(200,150,255);
        temp = who_won(a);
    }
    else if *ai == 14
    {
        println!("gun - gun");
        *ai_Color = Color::RGB(50,50,50);
        temp = who_won(t);
    }
    return temp;
}

//Makes the decisions
fn aiDecision() -> i32
{
    //What will be returned
    let mut temp: i32 = 0;

    let mut rng: ThreadRng = rand::thread_rng();

    temp = rng.gen_range(0..15);

    //Returns the selected decision
    return temp;
}