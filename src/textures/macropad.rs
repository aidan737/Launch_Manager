
#[path = "Draw_func.rs"] mod Draw_sprite;
#[path = "../maths/button.rs"] mod Button;
use Draw_sprite::*;
extern crate piston_window;

pub use piston_window::*;



pub fn draw_Macropad_icon(c: &Context, g: &mut G2d,index: usize)
{

    rectangle([0.6,0.6, 0.6, 1.0], // red
              [(80.0*index as f64)+10.0, 625.0, 60.0, 60.0],
              c.transform, g);

    rectangle([0.05,0.05, 0.05, 1.0], // red
              [(80.0*index as f64)+15.0, 630.0, 10.0, 10.0],
              c.transform, g);
    rectangle([0.05,0.05, 0.05, 1.0], // red
              [(80.0*index as f64)+15.0, 650.0, 10.0, 10.0],
              c.transform, g);

    rectangle([0.05,0.05, 0.05, 1.0], // red
            [(80.0*index as f64) +15.0, 670.0, 10.0, 10.0],
            c.transform, g);

    rectangle([0.05,0.05, 0.05, 1.0], // red
            [(80.0*index as f64)+35.0, 630.0, 10.0, 10.0],
             c.transform, g);
    rectangle([0.05,0.05, 0.05, 1.0], // red
            [(80.0*index as f64)+35.0, 650.0, 10.0, 10.0],
            c.transform, g);
    rectangle([0.05,0.05, 0.05, 1.0], // red
            [(80.0*index as f64)+35.0, 670.0, 10.0, 10.0],
             c.transform, g);

             rectangle([0.05,0.05, 0.05, 1.0], // red
                     [(80.0*index as f64)+55.0, 630.0, 10.0, 10.0],
                      c.transform, g);
             rectangle([0.05,0.05, 0.05, 1.0], // red
                     [(80.0*index as f64)+55.0, 650.0, 10.0, 10.0],
                     c.transform, g);
             rectangle([0.05,0.05, 0.05, 1.0], // red
                     [(80.0*index as f64)+55.0, 670.0, 10.0, 10.0],
                      c.transform, g);



}



fn draw_button(c: &Context, g: &mut G2d,position:Point2d, letter: char)
{
        rectangle([0.2,0.2,0.2, 1.0], // red
        [position.point_x, position.point_y, 140.0, 140.0],
            c.transform, g);
            Draw_sprite::draw_text(&c, g, &letter.to_string(), &Point2d {
             point_x: position.point_x + 40.0,
             point_y: position.point_y + 40.0,
            },  &Color {
                red: 100.0,
                green: 0.0,
                blue: 0.0,
                transperency: 1.0,
            }, 9.0);
}



pub fn draw_Macropad_Large(c: &Context, g: &mut G2d,position_x:f64, position_y:f64,mouse_position:[f64; 2],leters: &Vec<String>) -> usize
{



    let scale = 7.0;
    let addition = 150.0;


    let mut number:usize = 0;




    draw_button(&c,g,Point2d {
     point_x: position_x ,
     point_y:position_y,
 }, get_first_char(&leters[0]));


    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x ,
                      point_y:position_y,
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 1;


    }






    draw_button(&c,g,Point2d {
        point_x: position_x +addition,
        point_y:position_y,
    }, get_first_char(&leters[1]));


    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x +addition,
                      point_y:position_y,
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 2;


    }


    draw_button(&c,g,Point2d {
        point_x: position_x +(addition*2.0),
        point_y:position_y,
    }, get_first_char(&leters[2]));

    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x +(addition*2.0),
                      point_y:position_y,
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 3;


    }


    draw_button(&c,g,Point2d {
        point_x: position_x ,
        point_y:position_y+addition,
    }, get_first_char(&leters[3]));


    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x,
                      point_y:position_y+addition,
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 4;


    }

    draw_button(&c,g,Point2d {
        point_x: position_x +addition,
        point_y:position_y+addition,
    }, get_first_char(&leters[4]));

    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x+addition,
                      point_y:position_y+addition,
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 5;


    }
    draw_button(&c,g,Point2d {
        point_x: position_x +(addition*2.0),
        point_y:position_y+addition,
    }, get_first_char(&leters[5]));


    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x+(addition*2.0),
                      point_y:position_y+addition,
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 6;


    }

    draw_button(&c,g,Point2d {
        point_x: position_x ,
        point_y:position_y+(addition*2.0),
    }, get_first_char(&leters[6]));


    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x,
                      point_y:position_y+(addition*2.0),
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 7;


    }


    draw_button(&c,g,Point2d {
        point_x: position_x +addition,
        point_y:position_y+(addition*2.0),
    }, get_first_char(&leters[7]));



    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x+addition,
                      point_y:position_y+(addition*2.0),
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 8;


    }

    draw_button(&c,g,Point2d {
        point_x: position_x + (addition*2.0),
        point_y:position_y + (addition*2.0),
    }, get_first_char(&leters[8]));


    if(is_point_in_rectangle(
                  &Point2d {
                   point_x: mouse_position[0] ,
                   point_y: mouse_position[1],
                  },
                  &Point2d {
                      point_x: position_x+ (addition*2.0),
                      point_y:position_y+(addition*2.0),
                     },
                  &Point2d {
                   point_x: 140.0,
                   point_y: 140.0,
                  } )){
            number = 9;


    }


    return number;
}






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


fn get_first_char(s: &String) -> char {
    s.chars().next().unwrap_or(' ')
}
