

extern crate piston_window;
pub use piston_window::*;
use std::sync::{Arc, Mutex};
pub static POPUPS: Mutex<Vec<String>> = Mutex::new(vec![]);
#[path = "../maths/Draw_text.rs"] mod Draw_text;
#[path = "../maths/button.rs"] mod Button;
use crate::Taskbar::Draw_func::Point2d;
use crate::Taskbar::Draw_func::Color;

pub fn add_popup(name:String)
{
    let mut popups = POPUPS.lock().unwrap();
    popups.push(name);

}



pub fn render_popups(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool) -> bool
{
    let mut popups = POPUPS.lock().unwrap();
    let mut return_hover = false;
    if(popups.len() > 0)
    {
        if(Button::is_point_in_rectangle(
            &Point2d {
             point_x: mouse_position[0],
             point_y: mouse_position[1],
            },
            &Point2d {
             point_x: 400.0,
             point_y: 50.0,
            },
            &Point2d {
             point_x: 200.0,
             point_y: 70.0,
            })){
                rectangle([0.5,0.1,0.1, 1.0], // red
                      [400.0, 50.0, 200.0, 70.0],
                      c.transform, g);
                       return_hover = true;
                      if(button_down)
                      {
                         popups.swap_remove(0);
                         return return_hover;
                      }



            }else{


        rectangle([0.1,0.1,0.1, 1.0], // red
              [400.0, 50.0, 200.0, 70.0],
              c.transform, g);
          }



        Draw_text::draw_text(&c, g, &popups[0], &Point2d {
               point_x: 400.0,
               point_y: 70.0,
              },  &Color {
                  red: 100.0,
                  green: 0.0,
                  blue: 0.0,
                  transperency: 1.0,
              }, 2.0);

    }
    return return_hover;
}
