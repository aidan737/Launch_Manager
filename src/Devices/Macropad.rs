#[path = "../textures/macropad.rs"] mod Macropad_sprite;
#[path = "../maths/button.rs"] mod Button;
#[path = "../textures/Draw_func.rs"] mod Draw_func;
#[path = "../maths/Draw_text.rs"] mod Draw_text;
#[path = "popup.rs"] mod Popups;

use crate::Taskbar::Draw_func::Point2d;
use crate::Taskbar::Draw_func::Color;
extern crate piston_window;
pub use piston_window::*;
use std::env;

use std::fs;
use std::io;
use std::sync::{Arc, Mutex};

static IS_LIGHTING: Mutex<bool> = Mutex::new(true);







pub static MODE_FILE_PROFILES: Mutex<Vec<String>> = Mutex::new(vec![]);




static MODE1_HIGHTS: Mutex<[f64; 9]> = Mutex::new([200.0,200.0,200.0,200.0,200.0,200.0,200.0,200.0,200.0]);
static MODE2_HIGHTS: Mutex<[f64; 9]> = Mutex::new([200.0,200.0,200.0,200.0,200.0,200.0,200.0,200.0,200.0]);
static MODE3_HIGHTS: Mutex<[f64; 9]> = Mutex::new([200.0,200.0,200.0,200.0,200.0,200.0,200.0,200.0,200.0]);

pub static MODE1_PROFILE: Mutex<Vec<String>> = Mutex::new(vec![]);
pub static MODE2_PROFILE: Mutex<Vec<String>> = Mutex::new(vec![]);
pub static MODE3_PROFILE: Mutex<Vec<String>> = Mutex::new(vec![]);

static MODE_SELECTED: Mutex<usize> = Mutex::new(0);




static LIGHTING_MODE: Mutex<usize> = Mutex::new(0);




static CHANGING_MACRO: Mutex<bool> = Mutex::new(false);
static MACRONUMBER: Mutex<usize> = Mutex::new(0);


static MOUSE_DOWN: Mutex<bool> = Mutex::new(false);

pub fn render_macropad(c: &Context, g: &mut G2d,mouse_position:[f64; 2], pressed:bool,port: &String,keypressed:Key,scroll_delta:f64)
{
    let mut macro_number = MODE_SELECTED.lock().unwrap();
    let mut mode1 = MODE1_PROFILE.lock().unwrap();
    let mut mode2 = MODE2_PROFILE.lock().unwrap();
    let mut mode3 = MODE3_PROFILE.lock().unwrap();
    let mut number_selected = 0;
    if(mode1.len() != 9)
    {
        drop(mode1);
        drop(mode2);
        drop(mode3);
        drop(macro_number);
        load_profile();
    }else
    {
        if(*macro_number == 0){
            number_selected = Macropad_sprite::draw_Macropad_Large(&c,g, 275.0,90.0,mouse_position,&*mode1);
        }
        if(*macro_number == 1){
            number_selected = Macropad_sprite::draw_Macropad_Large(&c,g, 275.0,90.0,mouse_position,&*mode2);
        }
        if(*macro_number == 2){
            number_selected = Macropad_sprite::draw_Macropad_Large(&c,g, 275.0,90.0,mouse_position,&*mode3);
        }
        drop(mode1);
        drop(mode2);
        drop(mode3);
        drop(macro_number);
    }






    Select_File_to_load(&c, g,mouse_position, pressed,keypressed,scroll_delta);
    Upload_button(&c, g,mouse_position,pressed,port);
    renderlayerbuttons(&c, g,mouse_position,pressed);
    renderlightingmode(&c, g,mouse_position,pressed);




let mut profile_open = PROFILE_OPEN.lock().unwrap();
let mut mouse_down = MOUSE_DOWN.lock().unwrap();




    let mut is_macro_open = CHANGING_MACRO.lock().unwrap();
    let mut macronumber = MACRONUMBER.lock().unwrap();
    if(pressed &&*mouse_down== false && *profile_open == false){
        if(*is_macro_open == false){
        if(number_selected > 0){
            *is_macro_open = true;
            *macronumber = number_selected;
            *mouse_down = true;
        }
    }
    }


    drop(mouse_down);
drop(profile_open);



    if(*is_macro_open){
        drop(is_macro_open);
        draw_2d_window_Macro(&c, g,mouse_position,pressed,*macronumber,keypressed,scroll_delta);
    }

let mut mouse_down = MOUSE_DOWN.lock().unwrap();
    if(pressed == false){
        *mouse_down = false;
    }
    if(Popups::render_popups(&c, g,mouse_position,pressed) == true)
    {
        *mouse_down = true;

    }

}


pub static LOADED_FILE: Mutex<String> = Mutex::new(String::new());
use std::fs::File;
use std::io::{BufRead, BufReader};
fn load_profile()
{
    let mut mode1 = MODE1_PROFILE.lock().unwrap();
    let mut mode2 = MODE2_PROFILE.lock().unwrap();
    let mut mode3 = MODE3_PROFILE.lock().unwrap();
    let mut mode1_HIGHTS = MODE1_HIGHTS.lock().unwrap();
    let mut mode2_HIGHTS = MODE2_HIGHTS.lock().unwrap();
    let mut mode3_HIGHTS = MODE3_HIGHTS.lock().unwrap();

        load_all_profiles();
        let number = 9;
        for num in 1..=number { // inclusive range
            mode1.push("a".to_string());
            mode2.push("b".to_string());
            mode3.push("c".to_string());
        }
        let mut file_loaded = LOADED_FILE.lock().unwrap();
            let file = File::open("Currently_loaded.txt").expect("Failed to open file");
              let reader = BufReader::new(file);

              for line in reader.lines() {
                  let line = line.unwrap(); // Handle potential errors
                  // Process the line here
                  *file_loaded = line;
              }

              let file_profile = File::open("macropad_profiles/".to_owned()+&*file_loaded.as_str()+"/Profile.txt").expect("Failed to open file");
              let reader2 = BufReader::new(file_profile);

              for line in reader2.lines() {
                  let line = line.unwrap(); // Handle potential errors
                  let separator = "-";

                  // Split the string into a vector of substrings
                  let split_strings: Vec<&str> = line.split(separator).collect();
                  for (index,string) in split_strings.iter().enumerate() {
                  // Process the line here
                  if(index < 9)
                  {
                      mode1[index] =  convert_ascii_to_letters(string).to_string();

                  }else
                  {
                      if(index < 18)
                      {
                          mode2[index-9] =  convert_ascii_to_letters(string).to_string();

                      }else
                      {
                          if(index < 27)
                          {
                              mode3[index-18] =  convert_ascii_to_letters(string).to_string();

                          }else
                          {
                              //switching to numbers
                              if(index < 36)
                              {
                                  mode1_HIGHTS[index-27] =  string.parse().unwrap();

                              }else
                              {
                                  if(index < 45)
                                  {
                                      mode2_HIGHTS[index-36] =  string.parse().unwrap();

                                  }else
                                  {
                                      if(index < 54)
                                      {
                                          mode3_HIGHTS[index-45] =  string.parse().unwrap();

                                      }

                                  }

                              }

                          }

                      }

                  }
              }






}
}





fn load_profile_change()
{
    let mut mode1 = MODE1_PROFILE.lock().unwrap();
    let mut mode2 = MODE2_PROFILE.lock().unwrap();
    let mut mode3 = MODE3_PROFILE.lock().unwrap();
    let mut mode1_HIGHTS = MODE1_HIGHTS.lock().unwrap();
    let mut mode2_HIGHTS = MODE2_HIGHTS.lock().unwrap();
    let mut mode3_HIGHTS = MODE3_HIGHTS.lock().unwrap();


        let mut file_loaded = LOADED_FILE.lock().unwrap();

              let file_profile = File::open("macropad_profiles/".to_owned()+&*file_loaded.as_str()+"/Profile.txt").expect("Failed to open file");
              let reader2 = BufReader::new(file_profile);

              for line in reader2.lines() {
                  let line = line.unwrap(); // Handle potential errors
                  let separator = "-";

                  // Split the string into a vector of substrings
                  let split_strings: Vec<&str> = line.split(separator).collect();
                  for (index,string) in split_strings.iter().enumerate() {
                  // Process the line here
                  if(index < 9)
                  {
                      mode1[index] =  convert_ascii_to_letters(string).to_string();

                  }else
                  {
                      if(index < 18)
                      {
                          mode2[index-9] =  convert_ascii_to_letters(string).to_string();

                      }else
                      {
                          if(index < 27)
                          {
                              mode3[index-18] =  convert_ascii_to_letters(string).to_string();

                          }else
                          {
                              //switching to numbers
                              if(index < 36)
                              {
                                  mode1_HIGHTS[index-27] =  string.parse().unwrap();

                              }else
                              {
                                  if(index < 45)
                                  {
                                      mode2_HIGHTS[index-36] =  string.parse().unwrap();

                                  }else
                                  {
                                      if(index < 54)
                                      {
                                          mode3_HIGHTS[index-45] =  string.parse().unwrap();

                                      }

                                  }

                              }

                          }

                      }

                  }
              }






}
}




fn convert_ascii_to_letters(ascii_string: &str) -> String {
    if(ascii_string != ""){
    ascii_string
        .split(',')
        .map(|num| {
            match num.parse::<u8>() {
                Ok(ascii_value) => char::from_u32(ascii_value as u32).unwrap_or_default(),
                Err(_) => '\0', // Replace invalid characters with a null character
            }
        })
        .collect()
    }else
    {
        return "".to_string();

    }
}
fn load_file(file_name:&String)
{


let string_to_save =file_name;

       if file_name.starts_with('/') {
    let string_to_save = remove_first_letter(file_name);
}
    let file_path = "Currently_loaded.txt";
    let mut file_loaded = LOADED_FILE.lock().unwrap();
    *file_loaded = string_to_save.clone();
    drop(file_loaded);
// Open the file for writing, creating it if it doesn't exist, and truncating it if it does
let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(file_path)
    .expect("Failed to open file");

// Write the string to the file
    match file.write_all(string_to_save.as_bytes()) {
          Ok(_) => {
              Popups::add_popup("File Loaded".to_string());;

          }
          Err(e) => {
              Popups::add_popup("Load Failed".to_string());;

          }
      }

    load_profile_change();

}


fn remove_first_letter(s: &str) -> String {
    if s.is_empty() {
        return s.to_string(); // Return an empty string if the input is empty
    }
    s[1..].to_string()
}

use std::fs::OpenOptions;
use std::io::Write;
fn save_file()
{
    let mut file_loaded = LOADED_FILE.lock().unwrap();
    let string_to_save = Convert_vecs_to_string();
    let file_path = "macropad_profiles/".to_owned()+&*file_loaded.as_str()+"/Profile.txt";

// Open the file for writing, creating it if it doesn't exist, and truncating it if it does
let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(file_path)
    .expect("Failed to open file");

// Write the string to the file
file.write_all(string_to_save.as_bytes())
    .expect("Failed to write to file");



}














static is_key_down: Mutex<bool> = Mutex::new(false);
static WINDOW_POSITION: Mutex<Point2d> = Mutex::new(Point2d{point_x:100.0,point_y:100.0});
static IS_WINDOW_HELD: Mutex<bool> = Mutex::new(false);
static MACRO_MODE_SELECTED: Mutex<usize> = Mutex::new(0);
fn draw_2d_window_Macro(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool,button_number:usize,keypressed:Key,scroll_delta:f64)
{
    let mut position = WINDOW_POSITION.lock().unwrap();

    let width = 200.0;
    let hight = 200.0;
    let mut window_held = IS_WINDOW_HELD.lock().unwrap();
    if(*window_held && mouse_position[1] >40.0)
    {
        position.point_x = mouse_position[0]- (width/2.0);
        position.point_y = mouse_position[1]- (hight/20.0);
    }
    let mut macro_mode = MACRO_MODE_SELECTED.lock().unwrap();



    //draw backing
    rectangle([0.2,0.2,0.2, 1.0], // red
              [position.point_x, position.point_y, width, hight],
              c.transform, g);

    rectangle([0.1,0.1,0.1, 1.0], // red
            [position.point_x, position.point_y, width, hight/10.0],
                c.transform, g);
   rectangle([1.0,0.5,0.5, 1.0], // red
            [position.point_x +180.0, position.point_y, width/10.0, hight/10.0],
                c.transform, g);



    *macro_mode=Button::Slider_button(&c, g,Point2d{point_x: position.point_x +5.0 , point_y: position.point_y+ 30.0},Vec::from(["Hight".to_string(), "type".to_string(), "Select".to_string()]),Point2d{point_x: 20.0, point_y:190.0}, *macro_mode,button_down,mouse_position,1.0);





    let mut mouse_down = MOUSE_DOWN.lock().unwrap();
    if(button_down && *mouse_down == false){

        *mouse_down = true;
        if(Button::is_point_in_rectangle(
                      &Point2d {
                       point_x: mouse_position[0] ,
                       point_y: mouse_position[1],
                      },
                      &Point2d {
                          point_x: position.point_x +180.0 ,
                          point_y: position.point_y,
                         },
                      &Point2d {
                       point_x: width/10.0,
                       point_y: hight/10.0,
                      } )){
                              let mut is_macro_open = CHANGING_MACRO.lock().unwrap();
                *is_macro_open = false;
                save_file();

        }
    }
    if(button_down){

        if(Button::is_point_in_rectangle(
                      &Point2d {
                       point_x: mouse_position[0] ,
                       point_y: mouse_position[1],
                      },
                      &Point2d {
                          point_x: position.point_x ,
                          point_y: position.point_y,
                         },
                      &Point2d {
                       point_x: width,
                       point_y: hight/10.0,
                      } )){
                *window_held = true;
                *mouse_down = true;

        }

    }else
    {
    *window_held = false;

}









    let mut mode1 = MODE1_PROFILE.lock().unwrap();
    let mut mode1_HIGHTS = MODE1_HIGHTS.lock().unwrap();
    let mut mode_selected =MODE_SELECTED.lock().unwrap();
    if(*mode_selected == 1)
    {
        mode1= MODE2_PROFILE.lock().unwrap();
        mode1_HIGHTS = MODE2_HIGHTS.lock().unwrap();
    }
    if(*mode_selected == 2)
    {
        mode1= MODE3_PROFILE.lock().unwrap();
        mode1_HIGHTS = MODE3_HIGHTS.lock().unwrap();
    }


if(*macro_mode == 1){






    let mut updated_str = mode1[button_number-1].to_string();
    mode1[button_number-1] = draw_textbox_profile(&c, g,&Point2d{point_x: position.point_x+10.0, point_y:position.point_y+70.0}, &Point2d{point_x: 180.0, point_y:30.0},keypressed,&updated_str,mouse_position, button_down,&1);



}



if(*macro_mode == 0){
    let mut updated_int = mode1_HIGHTS[button_number-1].to_string();
    mode1_HIGHTS[button_number - 1] = match draw_textbox_numbers(
        &c,
        g,
        &Point2d { point_x: position.point_x + 10.0, point_y: position.point_y + 70.0 },
        &Point2d { point_x: 180.0, point_y: 30.0 },
        keypressed,
        &updated_int,
        &2,
        mouse_position,
        button_down
    ).parse::<f64>() {
        Ok(value) => value,
        Err(e) => {

            // Handle error gracefully, e.g., return a default value or continue with the program
            0.0 // Or any other appropriate default value
        }
    };
}


drop(mouse_down);


if(*macro_mode == 2){

        let mut updated_str = mode1[button_number-1].to_string();
    mode1[button_number-1] =Draw_box_Buttons(&c, g,mouse_position, button_down, *position,scroll_delta,updated_str);





}










}

static SCROLL_VALUE_MACRO: Mutex<usize> = Mutex::new(0);
static Mouse2: Mutex<bool> = Mutex::new(false);
fn Draw_box_Buttons(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool, position:Point2d,scroll_delta:f64,input: String ) ->String
{
    let Keys = vec![
    "a0",
    "a1",
    "a2",
    "b0",
    "b1",
    "b2",
    "c0",
    "c1",
    "c2",
    "d0",
    "d1",
    "d2",
    "e0",
    "e1",
    "e2",
    "f0",
    "f1",
    "f2",
    "g0",
    "g1",
    "g2",
    "h0",
    "h1",
    "h2",
    "i0",
    "i1",
    "i2",
    "j0",
    "j1",
    "j2",
    "k0",
    "k1",
    "k2",
    "l0",
    "l1",
    "l2",
    "m0",
    "m1",
    "m2",
    "n0",
    "n1",
    "n2",
    "o0",
    "o1",
    "o2",
    "p0",
    "p1",
    "p2",
    "q0",
    "q1",
    "q2",
    "r0",
    "r1",
    "r2",
    "s0",
    "s1",
    "s2",
    "t0",
    "t1",
    "t2",
    "u0",
    "u1",
    "u2",
    "v0",
    "v1",
    "v2",
    "w0",
    "w1",
    "w2",
    "x0",
    "x1",
    "x2",
    "y0",
    "y1",
    "y2",
    "z0",
    "z1",
    "z2",
    "00",
    "01",
    "02",
    "10",
    "11",
    "12",
    "20",
    "21",
    "22",
    "30",
    "31",
    "32",
    "40",
    "41",
    "42",
    "50",
    "51",
    "52",
    "60",
    "61",
    "62",
    "70",
    "71",
    "72",
    "80",
    "81",
    "82",
    "90",
    "91",
    "92",
    "shift0",
    "shift1",
    "shift2",
    "ctrl0",
    "ctrl1",
    "ctrl2",
    "caps0",
    "caps1",
    "caps2",
    "tab0",
    "tab1",
    "tab2",
    "alt0",
    "alt1",
    "alt2",
    "esc0",
    "esc1",
    "esc2",
    "fn0",
    "fn1",
    "fn2",
    "insert0",
    "insert1",
    "insert2",
    "delete0",
    "delete1",
    "delete2",
    "pause0",
    "pause1",
    "pause2",
    "calc0",
    "calc1",
    "calc2",
    "enter0",
    "enter1",
    "enter2",
    "Back0",
    "Back1",
    "Back2",
    "Volup0",
    "Volup1",
    "Volup2",
    "Voldown0",
    "Voldown1",
    "Voldown2",
    "space0",
    "space1",
    "space2",
    "!0",
    "!1",
    "!2",
    "@0",
    "@1",
    "@2",
    "#0",
    "#1",
    "#2",
    "$0",
    "$1",
    "$2",
    "%0",
    "%1",
    "%2",
    "^0",
    "^1",
    "^2",
    "&0",
    "&1",
    "&2",
    "*0",
    "*1",
    "*2",
    "(0",
    "(1",
    "(2",
    ")0",
    ")1",
    ")2",
    "_0",
    "_1",
    "_2",
    "-0",
    "-1",
    "-2",
    "=0",
    "=1",
    "=2",
    "+0",
    "+1",
    "+2",
    "/0",
    "/1",
    "/2",
    "?0",
    "?1",
    "?2",
    ":0",
    ":1",
    ":2",
    ";0",
    ";1",
    ";2",
    "$0",
    "$1",
    "$2",
    "}0",
    "}1",
    "}2",
    "{0",
    "{1",
    "{2",
    "[0",
    "[1",
    "[2",
    "]0",
    "]1",
    "]2",
    "`0",
    "`1",
    "`2",
    "~0",
    "~1",
    "~2",
    // Add other get information functions as needed
    ];






let mut mouse_down = Mouse2.lock().unwrap();


let input_string_original =insert_commas(&input);
let mut input_string =insert_commas(&input);
let mut groups: Vec<&str> = input_string.split(",").collect();
let mut groups2: Vec<&str> = input_string.split(",").collect();



    if(groups.len() != 1){

 // Loop through each group

    for (i, chars) in groups2.iter().enumerate() {
        if i % 2 == 0 {
            let positionx = position.point_x + (40.0*(i/2) as f64);
            let positiony =position.point_y + 50.0;
            let letter = ascii_number_string_to_char(chars);
            let char_count = letter.clone().expect("REASON").chars().count();





                if(Button::is_point_in_rectangle(
                     &Point2d {
                      point_x: mouse_position[0],
                      point_y: mouse_position[1],
                     },
                     &Point2d {
                         point_x: positionx,
                         point_y: positiony,
                    },
                    &Point2d {
                         point_x: 30.0,
                         point_y: 30.0,
                    }) ){
                        rectangle([0.5,0.5,0.5, 1.0], // red
                                 [positionx ,positiony , 30.0, 30.0],
                                     c.transform, g);
                                             //cheaking if it is a char or a function
                            if(button_down && *mouse_down ==false){

                                groups.remove(i+1);
                                groups.remove(i);


                                *mouse_down = true;
                                    break;
                            }


                    }else
                    {

                        rectangle([0.3,0.3,0.3, 1.0], // red
                                 [positionx ,positiony , 30.0, 30.0],
                                     c.transform, g);
                    }






                         let last_digit =get_last_digit(&ascii_number_string_to_char(groups[i+1]).expect("REASON")).expect("REASON");
                         if(last_digit == '0' ||last_digit == '1'){
                         rectangle([1.0,0.5,0.5, 1.0], // red
                                  [positionx +30.0,positiony , 5.0, 10.0],
                                     c.transform, g);
                         }
                             if(last_digit == '0' ||last_digit == '2'){
                         rectangle([1.0,0.5,0.5, 1.0], // red
                                     [positionx +30.0,positiony +20.0, 5.0, 10.0],
                                     c.transform, g);
                                 }

                         if(char_count > 3)
                         {

                                          Draw_text::draw_text(&c, g, &letter.expect("REASON").to_string(),&Point2d {
                                               point_x: positionx+(10.0 - (char_count*2) as f64),
                                               point_y: positiony+10.0,
                                              },  &Color {
                                              red: 1.0,
                                              green: 1.0,
                                              blue: 1.0,
                                              transperency: 1.0,
                                          }, 1.0);
                         }
                         else
                         {

                                          Draw_text::draw_text(&c, g, &letter.expect("REASON").to_string(),&Point2d {
                                               point_x: positionx+(10.0- (char_count*2) as f64),
                                               point_y: positiony+10.0,
                                              },  &Color {
                                              red: 1.0,
                                              green: 1.0,
                                              blue: 1.0,
                                              transperency: 1.0,
                                          }, 2.0);
                         }

        }
    }

input_string = groups.join(",");

}







    let mut scroll_value = SCROLL_VALUE_MACRO.lock().unwrap();
    if(Button::is_point_in_rectangle(
            &Point2d {
            point_x: mouse_position[0],
            point_y: mouse_position[1],
            },
            &Point2d {
            point_x: position.point_x,
            point_y: position.point_y,
            },
            &Point2d {
            point_x: 200.0,
            point_y: 200.0,
            })){
    if(scroll_delta <0.0){
    *scroll_value += 1;
    }
    if(scroll_delta >0.0 && *scroll_value !=0){
    *scroll_value -= 1;
    }


    if(Keys.len()> 15){
    if(*scroll_value>(Keys.len() /5)-1)
    {
        *scroll_value = (Keys.len() /5)-1;
    }
}
}
if(Keys.len()/5< 4){
    *scroll_value = 0;
}

    for (index2,key) in Keys.iter().enumerate()
    {
        if(index2<15+(*scroll_value *5)&& index2>=(*scroll_value *5)){
            let index = index2-(*scroll_value *5);
        let whole_index = (index/5);
        let positionx = (35 * (index - (5*whole_index)))as f64+position.point_x +15.0;
        let positiony =(35 *whole_index)as f64+position.point_y+90.0;
        let char_count = key.chars().count();
        rectangle([0.3,0.3,0.3, 1.0], // red
                 [positionx ,positiony , 35.0, 30.0],
                     c.transform, g);
        let last_digit =get_last_digit(key).expect("REASON");
        if(last_digit == '0' ||last_digit == '1'){
        rectangle([1.0,0.5,0.5, 1.0], // red
                 [positionx +30.0,positiony , 5.0, 10.0],
                    c.transform, g);
        }
            if(last_digit == '0' ||last_digit == '2'){
        rectangle([1.0,0.5,0.5, 1.0], // red
                    [positionx +30.0,positiony +20.0, 5.0, 10.0],
                    c.transform, g);
                }



                     if(button_down){
                     if(Button::is_point_in_rectangle(
                             &Point2d {
                             point_x: mouse_position[0],
                             point_y: mouse_position[1],
                             },
                             &Point2d {
                             point_x: positionx,
                             point_y: positiony,
                             },
                             &Point2d {
                             point_x: 30.0,
                             point_y: 30.0,
                             }) && *mouse_down ==false){

                                 //cheaking if it is a char or a function
                                 if(char_count ==2)
                                 {
                                    if(input_string !=""){
                                        input_string+= &(",".to_owned() +&insert_commas(&key));
                                    }else
                                    {
                                        input_string+= &insert_commas(&key);
                                    }
                                }



                                     *mouse_down = true;

                             }
                         }







        if(char_count > 3)
        {

                         Draw_text::draw_text(&c, g, &remove_last_char(key).to_string(),&Point2d {
                              point_x: positionx+(10.0 - (char_count*2) as f64),
                              point_y: positiony+10.0,
                             },  &Color {
                             red: 1.0,
                             green: 1.0,
                             blue: 1.0,
                             transperency: 1.0,
                         }, 1.0);
        }
        else
        {

                         Draw_text::draw_text(&c, g, &remove_last_char(key).to_string(),&Point2d {
                              point_x: positionx+(10.0- (char_count*2) as f64),
                              point_y: positiony+10.0,
                             },  &Color {
                             red: 1.0,
                             green: 1.0,
                             blue: 1.0,
                             transperency: 1.0,
                         }, 2.0);
        }


        }

    }

    if(input_string_original != input_string)
    {
        if(input_string !=""){
        return convert_ascii_to_letters(&input_string);
    }else
    {
        return input_string;
    }
    }
    if(button_down == false)
    {
             *mouse_down = false;
    }
return input;



}
fn get_last_digit(s: &str) -> Option<char> {
    if s.is_empty() {
        return None; // Handle empty strings
    }

    let last_char = s.chars().last().unwrap(); // Get the last character
    if last_char.is_digit(10) {
        Some(last_char) // If it's a digit, return it
    } else {
        None // If it's not a digit, return None
    }
}
fn remove_last_char(s: &str) -> String {
    if s.is_empty() {
        return s.to_string(); // Return an empty string if the input is empty
    }

    s[..s.len() - 1].to_string() // Create a new string from the beginning up to the second-to-last character
}
fn ascii_number_string_to_char(ascii_number_string: &str) -> Option<String> {
    // Parse the ASCII number string into an integer
    let ascii_number = ascii_number_string.parse::<u8>().ok()?;

    // Check if the ASCII number is within the valid range (0-127)
    if 0 <= ascii_number && ascii_number <= 127 {
        // Convert the ASCII number to a character and then to a string
        let ascii_char = char::from_u32(ascii_number as u32).unwrap();
        Some(ascii_char.to_string())
    } else {
        None
    }
}
fn string_to_ascii_number(ascii_string: &str) -> Option<u8> {
    // Parse the ASCII string into a u8 (unsigned 8-bit integer)
    match ascii_string.parse::<u8>() {
        Ok(ascii_number) => {
            // Ensure the parsed ASCII number is within the valid range (0-127)
            if ascii_number >= 0 && ascii_number <= 127 {
                Some(ascii_number)
            } else {
                None
            }
        }
        Err(_) => None, // Handle parsing errors gracefully
    }
}



fn Draw_textbox_repeationspeed(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool)
{



}


fn renderlayerbuttons(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool)
{
    let mut mode = MODE_SELECTED.lock().unwrap();
    let old_mode = (*mode);
    let mut is_macro_open = CHANGING_MACRO.lock().unwrap();
    let mut mouse_down = MOUSE_DOWN.lock().unwrap();


    if(*mouse_down == false && button_down == true &&*is_macro_open == false){
    *mode =Button::Slider_button(&c, g,Point2d{point_x: 300.0, point_y:580.0},Vec::from(["layer 1".to_string(), "layer 2".to_string(), "layer 3".to_string()]),Point2d{point_x: 30.0, point_y:400.0}, *mode,button_down,mouse_position,2.0);
    if(old_mode != *mode){
    *mouse_down = true;
    }
    }else
    {
    Button::Slider_button(&c, g,Point2d{point_x: 300.0, point_y:580.0},Vec::from(["layer 1".to_string(), "layer 2".to_string(), "layer 3".to_string()]),Point2d{point_x: 30.0, point_y:400.0}, *mode,button_down,mouse_position,2.0);
    }


}

fn renderlightingmode(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool)
{
    let mut mode = LIGHTING_MODE.lock().unwrap();
        let old_mode = (*mode);
    let mut mouse_down = MOUSE_DOWN.lock().unwrap();
    let mut is_macro_open = CHANGING_MACRO.lock().unwrap();
    if(*mouse_down == false && button_down == true&&*is_macro_open == false){
    *mode =Button::Slider_button(&c, g,Point2d{point_x: 350.0, point_y:50.0},Vec::from(["keymap".to_string(), "lighting".to_string()]),Point2d{point_x: 30.0, point_y:300.0}, *mode,button_down,mouse_position,2.0);
    if(old_mode != *mode){
    *mouse_down = true;
    }
    }else
    {
        Button::Slider_button(&c, g,Point2d{point_x: 350.0, point_y:50.0},Vec::from(["keymap".to_string(), "lighting".to_string()]),Point2d{point_x: 30.0, point_y:300.0}, *mode,button_down,mouse_position,2.0);
    }


}







static UPLOAD_DOWN: Mutex<bool> = Mutex::new(false);

fn Upload_button(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool,port_name:  &String)
{
  let vertices: [[f64; 2]; 3] = [[940.0,540.0 ], [940.0,590.0], [980.0, 565.0]];

 polygon( [0.5, 0.0, 0.5,1.0 ], &vertices, c.transform, g);
let mut button = UPLOAD_DOWN.lock().unwrap();
if(button_down)
{
    if(Button::is_point_in_rectangle(&Point2d{point_x:mouse_position[0],point_y:mouse_position[1]},&Point2d{point_x: 940.0, point_y:540.0},&Point2d{point_x: 40.0, point_y:50.0})&& *button == false)
    {


         //upload to arduino
         *button = true;
         upload_to_com(port_name);
         save_file();
     }
 }
else
{
    *button = false;
}


}


use std::time::Duration;
fn upload_to_com(port_name:  &String)
{
    let mut port = serialport::new(port_name, 9600).timeout(Duration::from_millis(500)).open().expect("Failed to open port");
     port.write_data_terminal_ready(true);

//the r is for changing data

    println!("{}",Convert_vecs_to_string());
     let binding = "r ".to_owned()+&Convert_vecs_to_string();
     let output = binding.as_bytes();
     match port.write(output) {
           Ok(_) => {
               Popups::add_popup("Upload Successful".to_string());

           }
           Err(e) => {
               Popups::add_popup("Upload Failed".to_string());

           }
       }




}













fn key_to_char(key: Key) -> Option<char> {
    match key {
        Key::A => Some('a'),
        Key::B => Some('b'),
        Key::C => Some('c'),
        Key::D => Some('d'),
        Key::E => Some('e'),
        Key::F => Some('f'),
        Key::G => Some('g'),
        Key::H => Some('h'),
        Key::I => Some('i'),
        Key::J => Some('j'),
        Key::K => Some('k'),
        Key::L => Some('l'),
        Key::M => Some('m'),
        Key::N => Some('n'),
        Key::O => Some('o'),
        Key::P => Some('p'),
        Key::Q => Some('q'),
        Key::R => Some('r'),
        Key::S => Some('s'),
        Key::T => Some('t'),
        Key::U => Some('u'),
        Key::V => Some('v'),
        Key::W => Some('w'),
        Key::X => Some('x'),
        Key::Y => Some('y'),
        Key::Z => Some('z'),
        Key::D0 => Some('0'),
        Key::D1 => Some('1'),
        Key::D2 => Some('2'),
        Key::D3 => Some('3'),
        Key::D4 => Some('4'),
        Key::D5 => Some('5'),
        Key::D6 => Some('6'),
        Key::D7 => Some('7'),
        Key::D8 => Some('8'),
        Key::D9 => Some('9'),
        Key::Space => Some(' '),
        // Add any other keys you want to support
        _ => None,
    }
}



static textbox_held: Mutex<usize> = Mutex::new(0);





fn draw_textbox_numbers(c: &Context, g: &mut G2d,position: &Point2d, size: &Point2d, keypressed: Key,current_str: &String, index:&usize,mouse_position:[f64; 2], mouse:bool) -> String
{

    let mut updated_str = current_str.to_string();
    let mut key_down = is_key_down.lock().unwrap();
    let mut textbox_held_index = textbox_held.lock().unwrap();


    if(mouse){
        if(updated_str == "0".to_string())
        {
            updated_str= "".to_string();
        }
        if(Button::is_point_in_rectangle(
            &Point2d {
             point_x: mouse_position[0],
             point_y: mouse_position[1],
            },
            position,
            size)){
               *textbox_held_index = (index +1)
            }
            if(!Button::is_point_in_rectangle(
                &Point2d {
                 point_x: mouse_position[0] ,
                 point_y: mouse_position[1],
                },
                position,
                size)){
                    if(  *textbox_held_index == (index +1)){
                   *textbox_held_index = 0;
                    }
                }
        }

    if((index +1)== *textbox_held_index){
    match keypressed {
        Key::Backspace => {
            if(*key_down == false){
            updated_str.pop();
            *key_down = true;
            }
        },
        Key::Unknown => {
            *key_down = false;
        },
        Key::Return => {
            // Do nothing, or handle Enter key as needed
        },
        _ => {
            if(*key_down == false){

            if let Some(ch) = num_to_char(keypressed) {
                *key_down = true;
                //checking for only 1 fullstop in number
                if keypressed == Key::Period {
                    if(count_char_occurrences(&updated_str, '.') == 0){
                    updated_str.push('.');
                    }
                }
                else
                {
                    updated_str.push(ch);

                }
            }

        }
        },
    }



}else{
if(updated_str == "".to_string())
{
    updated_str= "0".to_string();
}
}


rectangle([0.5,0.2,0.2, 1.0], // red
      [position.point_x, position.point_y, size.point_x, size.point_y],
      c.transform, g);

Draw_text::draw_text(&c, g, &updated_str, position,  &Color {
    red: 100.0,
    green: 0.0,
    blue: 0.0,
    transperency: 1.0,
}, 2.0);
   return(updated_str);
}
fn count_char_occurrences(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
}
fn num_to_char(key: Key) -> Option<char> {
    match key {
        Key::D0 => Some('0'),
        Key::D1 => Some('1'),
        Key::D2 => Some('2'),
        Key::D3 => Some('3'),
        Key::D4 => Some('4'),
        Key::D5 => Some('5'),
        Key::D6 => Some('6'),
        Key::D7 => Some('7'),
        Key::D8 => Some('8'),
        Key::D9 => Some('9'),
        Key::Period => Some('.'),

        // Add any other keys you want to support
        _ => None,
    }
}







fn draw_textbox(c: &Context, g: &mut G2d,position: &Point2d, size: &Point2d, keypressed: Key,current_str: &String,mouse_position:[f64; 2], mouse:bool,index:&usize) -> String
{
    let mut textbox_held_index = textbox_held.lock().unwrap();
    let mut updated_str = current_str.to_string();
    let mut key_down = is_key_down.lock().unwrap();
    if(mouse){
        if(Button::is_point_in_rectangle(
            &Point2d {
             point_x: mouse_position[0],
             point_y: mouse_position[1],
            },
            position,
            size)){
               *textbox_held_index = (index +1)
            }
            if(!Button::is_point_in_rectangle(
                &Point2d {
                 point_x: mouse_position[0] ,
                 point_y: mouse_position[1],
                },
                position,
                size)){
                    if(  *textbox_held_index == (index +1)){
                   *textbox_held_index = 0;
                    }
                }

        }
        if((index +1)== *textbox_held_index){
    match keypressed {
        Key::Backspace => {
            if(*key_down == false){
            updated_str.pop();
            *key_down = true;
            }
        },
        Key::Unknown => {
            *key_down = false;
        },
        Key::Return => {
            // Do nothing, or handle Enter key as needed
        },
        _ => {
            if(*key_down == false){
            if let Some(ch) = key_to_char(keypressed) {
                *key_down = true;
                updated_str.push(ch);
            }
        }
        },
    }
}
    rectangle([0.5,0.2,0.2, 1.0], // red
          [position.point_x, position.point_y, size.point_x, size.point_y],
          c.transform, g);

    Draw_text::draw_text(&c, g, &updated_str, position,  &Color {
        red: 100.0,
        green: 0.0,
        blue: 0.0,
        transperency: 1.0,
    }, 2.0);
   return(updated_str);
}




fn draw_textbox_profile(c: &Context, g: &mut G2d,position: &Point2d, size: &Point2d, keypressed: Key,current_str: &String,mouse_position:[f64; 2], mouse:bool,index:&usize) -> String
{
    let mut textbox_held_index = textbox_held.lock().unwrap();
    let mut updated_str = current_str.to_string();
    let mut key_down = is_key_down.lock().unwrap();
    if(mouse){
        if(Button::is_point_in_rectangle(
            &Point2d {
             point_x: mouse_position[0],
             point_y: mouse_position[1],
            },
            position,
            size)){
               *textbox_held_index = (index +1)
            }
            if(!Button::is_point_in_rectangle(
                &Point2d {
                 point_x: mouse_position[0] ,
                 point_y: mouse_position[1],
                },
                position,
                size)){
                    if(  *textbox_held_index == (index +1)){
                   *textbox_held_index = 0;
                    }
                }

        }
        if((index +1)== *textbox_held_index){
    match keypressed {
        Key::Backspace => {
            if(*key_down == false){
            updated_str.pop();
            updated_str.pop();
            *key_down = true;
            }
        },
        Key::Unknown => {
            *key_down = false;
        },
        Key::Return => {
            // Do nothing, or handle Enter key as needed
        },
        _ => {
            if(*key_down == false){
            if let Some(ch) = key_to_char(keypressed) {
                *key_down = true;
                updated_str.push(ch);
                updated_str.push('0');
            }
        }
        },
    }
}
    rectangle([0.5,0.2,0.2, 1.0], // red
          [position.point_x, position.point_y, size.point_x, size.point_y],
          c.transform, g);

    Draw_text::draw_text(&c, g, &extract_every_second_char(&updated_str), position,  &Color {
        red: 100.0,
        green: 0.0,
        blue: 0.0,
        transperency: 1.0,
    }, 2.0);
   return(updated_str);
}



fn extract_every_second_char(input_string: &str) -> String {
    let mut output_string = String::new();

    for (i, char) in input_string.chars().enumerate() {
        if i % 2 == 0 {
            output_string.push(char);
        }
    }

    output_string
}





static IS_SELECT_WINDOW_OPEN: Mutex<bool> = Mutex::new(false);


static SCROLL_VALUE_FILE: Mutex<usize> = Mutex::new(0);
fn Select_File_to_load(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool,keypressed:Key,scroll_delta:f64)
{

    let mut is_macro_open = CHANGING_MACRO.lock().unwrap();
    let mut mouse_down = MOUSE_DOWN.lock().unwrap();
let mut profile_open = PROFILE_OPEN.lock().unwrap();
    let mut isopen = IS_SELECT_WINDOW_OPEN.lock().unwrap();
    if(*isopen)
    {
        rectangle([0.1,0.1,0.1, 1.0], // red
                 [800.0, 45.0, 180.0, 160.0],
                     c.transform, g);
    let mut modes = MODE_FILE_PROFILES.lock().unwrap();
    let mut scroll_value = SCROLL_VALUE_FILE.lock().unwrap();
if(Button::is_point_in_rectangle(
        &Point2d {
        point_x: mouse_position[0],
        point_y: mouse_position[1],
        },
        &Point2d {
        point_x: 800.0,
        point_y: 45.0,
        },
        &Point2d {
        point_x: 180.0,
        point_y: 160.0,
        })){
    if(scroll_delta <0.0){
    *scroll_value += 1;
    }
    if(scroll_delta >0.0 && *scroll_value !=0){
    *scroll_value -= 1;
    }


    if(modes.len()> 4){
    if(*scroll_value>modes.len() - 4)
    {
        *scroll_value = modes.len() - 4;
    }
}
if(modes.len()< 5){
    *scroll_value = 0;
}
}

    for (index2,mode) in (*modes).iter().enumerate()
    {




        if(index2 >= *scroll_value && index2 <= 4 +*scroll_value){

            let index = index2 -*scroll_value;
            rectangle([1.0,1.5,1.5, 1.0], // red
                     [810.0, 70.0 + (20.0*index as f64), 160.0, 15.0],
                         c.transform, g);
                         Draw_text::draw_text(&c, g, &mode,&Point2d {
                              point_x: 810.0,
                              point_y: 70.0 + (20.0*index as f64),
                             },  &Color {
                             red: 100.0,
                             green: 0.0,
                             blue: 0.0,
                             transperency: 1.0,
                         }, 2.0);


                         if(button_down && *mouse_down == false&&*is_macro_open == false){
                             if(Button::is_point_in_rectangle(
                                     &Point2d {
                                     point_x: mouse_position[0],
                                     point_y: mouse_position[1],
                                     },
                                     &Point2d {
                                     point_x: 810.0,
                                     point_y: 70.0 + (20.0*index as f64),
                                     },
                                     &Point2d {
                                     point_x: 160.0,
                                     point_y: 15.0,
                                     })){

                                    load_file(mode);

                                    *isopen = false;



                                         *mouse_down = true;
                                     }


                             }


                         }
    }







    rectangle([1.0,1.5,1.5, 1.0], // red
             [810.0, 180.0, 160.0, 15.0],
                 c.transform, g);
                 Draw_text::draw_text(&c, g, &"new profile".to_string(),&Point2d {
                      point_x: 810.0,
                      point_y: 180.0,
                     },  &Color {
                     red: 100.0,
                     green: 0.0,
                     blue: 0.0,
                     transperency: 1.0,
                 }, 2.0);






            if(button_down && *mouse_down == false&&*is_macro_open == false){
                if(Button::is_point_in_rectangle(
                        &Point2d {
                        point_x: mouse_position[0],
                        point_y: mouse_position[1],
                        },
                        &Point2d {
                        point_x: 810.0,
                        point_y: 180.0,
                        },
                        &Point2d {
                        point_x: 160.0,
                        point_y: 15.0,
                        })){

                            drop(modes);



                            *profile_open = true;

                            *mouse_down = true;
                        }


                }





    }










    rectangle([1.0,0.5,0.5, 1.0], // red
             [800.0, 50.0, 20.0, 4.0],
                 c.transform, g);
    rectangle([1.0,0.5,0.5, 1.0], // red
            [800.0, 58.0, 20.0, 4.0],
            c.transform, g);
   rectangle([1.0,0.5,0.5, 1.0], // red
            [800.0, 66.0, 20.0, 4.0],
            c.transform, g);

            if(button_down && *mouse_down == false&&*is_macro_open == false){
                if(Button::is_point_in_rectangle(
                    &Point2d {
                     point_x: mouse_position[0],
                     point_y: mouse_position[1],
                    },
                    &Point2d {
                     point_x: 800.0,
                     point_y: 50.0,
                    },
                    &Point2d {
                     point_x: 20.0,
                     point_y: 30.0,
                    })){
                    *isopen = !*isopen;
                    *mouse_down = true;
                    }


                }

drop(mouse_down);
                if(*profile_open){
                    drop(profile_open);
                    Add_profile(&c,g,mouse_position,button_down,keypressed);
                }




}



fn load_all_profiles()
{

        let mut modes = MODE_FILE_PROFILES.lock().unwrap();
    let dir = env::current_dir();

let directory_string = dir.expect("REASON").to_string_lossy().into_owned()+"\\macropad_profiles";
        println!("{}",directory_string);
     for entry in fs::read_dir(&directory_string).expect("Failed to read directory") {
         match entry {
             Ok(entry) => {
                 let path = entry.path();
                 if path.is_dir() {


                   let mut result:String=String::from("empty");


                   match read_file_contents(&(path.to_str().unwrap().to_owned() +"\\Profile.txt")) {
                      Ok(contents) =>result =contents,
                      Err(error) => println!("Error reading file: {}", error),
                    }
                    //add to list
                    let string = path.to_str().unwrap().to_owned();
                    let result = string.split(&directory_string).collect::<Vec<&str>>().join("");
                    modes.push(result);
                    println!("{}",path.to_str().unwrap().to_owned());
                 }
             },
             Err(_) => continue, // skip errors
         }
     }


}
fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
  // Attempt to read the file contents
  match fs::read_to_string(file_path) {
    Ok(contents) => Ok(contents),
    Err(error) => Err(error),
  }
}








use std::path::Path;
static PROFILE_NAME: Mutex<String> = Mutex::new(String::new());
static PROFILE_OPEN: Mutex<bool> = Mutex::new(false);


fn Add_profile(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool,keypressed:Key)
{

    let mut name = PROFILE_NAME.lock().unwrap();
    rectangle([0.1,0.1,0.1, 1.0], // red
             [350.0, 250.0, 150.0, 100.0],
                 c.transform, g);
    *name = draw_textbox(&c, g,&Point2d{point_x: 370.0, point_y: 250.0}, &Point2d{point_x: 120.0, point_y:30.0},keypressed,&*name,mouse_position, button_down,&3);



    rectangle([1.1,0.1,0.1, 1.0], // red
             [350.0, 300.0, 40.0, 25.0],
                 c.transform, g);
    rectangle([0.1,1.1,0.1, 1.0], // red
                [440.0, 300.0, 40.0, 25.0],
                c.transform, g);

                let mut mouse_down = MOUSE_DOWN.lock().unwrap();
                if(button_down && *mouse_down == false){

                    if(Button::is_point_in_rectangle(
                        &Point2d {
                         point_x: mouse_position[0],
                         point_y: mouse_position[1],
                        },
                        &Point2d {
                         point_x: 350.0,
                         point_y: 300.0,
                        },
                        &Point2d {
                         point_x: 40.0,
                         point_y: 25.0,
                        })){
                            let mut profile_open = PROFILE_OPEN.lock().unwrap();
                            *profile_open= false;

                            *mouse_down = true;
                        }

                        if(Button::is_point_in_rectangle(
                            &Point2d {
                             point_x: mouse_position[0] ,
                             point_y: mouse_position[1],
                            },
                            &Point2d {
                             point_x: 440.0,
                             point_y: 300.0,
                            },
                            &Point2d {
                             point_x: 40.0,
                             point_y: 25.0,
                            })){
                                let mut modes = MODE_FILE_PROFILES.lock().unwrap();
                            let dir = env::current_dir();
                            let directory_string = dir.expect("REASON").to_string_lossy().into_owned()+"\\macropad_profiles\\"+name.as_str();
                            let folder_path = Path::new(&directory_string);
                            if !folder_path.exists() {
                                fs::create_dir(folder_path).expect("Failed to create folder");
                            }

                            let file_path = folder_path.join("Profile.txt");
                            fs::write(file_path, "97,105,100,97,110,32,108,97,100,97,97,97,97-99,97,114,103,111,32,114,117,110,97,97,99,97,97,97-108-106-109-97-97-97-97-98-98-98-98-97-98-98-98-98-99-99-99-99-99-99-99-99-106-200-200-200-200-200-150-150-150-200-200-200-200-200-200-200-200-200-200-200-200-200-200-200-200-200-200-200-").expect("Failed to write to file");
                            modes.push(name.to_string());

                            *mouse_down = true;
                            let mut profile_open = PROFILE_OPEN.lock().unwrap();
                            *profile_open= false;
                            }



                    }



}






fn Convert_vecs_to_string() -> String
{
    let mut mode1 = MODE1_PROFILE.lock().unwrap();
    let mut mode1_HIGHTS = MODE1_HIGHTS.lock().unwrap();
    let mut mode2 = MODE2_PROFILE.lock().unwrap();
    let mut mode2_HIGHTS = MODE2_HIGHTS.lock().unwrap();
    let mut mode3 = MODE3_PROFILE.lock().unwrap();
    let mut mode3_HIGHTS = MODE3_HIGHTS.lock().unwrap();

    let mut return_str:String = "".to_string();

    for mode in (*mode1).iter()
    {
        let mut result = insert_commas(mode);


        return_str += &result;
        return_str += "-"
    }
    for mode in (*mode2).iter()
    {
            let mut result = insert_commas(mode);
        return_str += &result;
        return_str += "-"
    }
    for mode in (*mode3).iter()
    {
            let mut result = insert_commas(mode);
        return_str += &result;
        return_str += "-"
    }



    for hight in (*mode1_HIGHTS).iter()
    {

        return_str += &hight.to_string();
        return_str += "-"
    }
    for hight in (*mode2_HIGHTS).iter()
    {

        return_str += &hight.to_string();
        return_str += "-"
    }
    for hight in (*mode3_HIGHTS).iter()
    {

        return_str += &hight.to_string();
        return_str += "-"
    }
    return return_str;

}
fn insert_commas(input: &str) -> String {
    input.chars()
          .map(|c| c as u8)
          .map(|num| num.to_string())
          .collect::<Vec<String>>()
          .join(",")
}
