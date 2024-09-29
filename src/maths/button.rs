

extern crate piston_window;
pub use piston_window::*;
#[path = "../textures/Draw_func.rs"] mod Draw_func;
use crate::Taskbar::Draw_func::Point2d;
use crate::Taskbar::Draw_func::Color;
#[path = "Draw_text.rs"] mod Draw_text;


pub fn is_point_in_rectangle(point: &Point2d, rect_point: &Point2d,size: &Point2d) -> bool {
    let x = point.point_x;
    let y = point.point_y;
    let rect_x = rect_point.point_x;
    let rect_y = rect_point.point_y;
    let rect_width = size.point_x;
    let rect_height = size.point_y;

    if x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height {
        return true;
    }

    false
}




pub fn Slider_button(c: &Context, g: &mut G2d,position:Point2d, sections:Vec<String>,size:Point2d, selected:usize,mouse_pressed:bool,mouse_position:[f64; 2],text_size:f64) -> usize
{
    let modual_length = size.point_y/sections.len() as f64;

    let mut boxselected = selected;

    for (index,section) in sections.iter().enumerate()
    {
        if(boxselected == index){
        rectangle([0.2,0.1, 0.2, 1.0], // red
              [position.point_x +(index as f64*modual_length), position.point_y, modual_length, size.point_x],
              c.transform, g);
          }else
          {
              rectangle([0.1,0.1, 0.1, 1.0], // red
                    [position.point_x +(index as f64*modual_length), position.point_y, modual_length, size.point_x],
                    c.transform, g);
          }
        Draw_text::draw_text(&c, g, section, &Point2d{point_x: position.point_x +(index as f64*modual_length) +10.0, point_y:position.point_y+10.0}, &Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            transperency: 1.0,
        }, text_size);

             //cheak if mouse is pressing in rect
             if(is_point_in_rectangle(&Point2d{point_x:mouse_position[0],point_y:mouse_position[1]},&Point2d{point_x: position.point_x +(index as f64*modual_length), point_y:position.point_y},&Point2d{point_x: modual_length, point_y:size.point_x}))
             {
                 if(mouse_pressed)
                 {
                     boxselected = index;
                 }
             }




    }


return boxselected;

}
