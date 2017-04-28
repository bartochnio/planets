extern crate sdl2;

use std::path::Path;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::*;
use std::boxed::Box;
use engine::components::ComponentUpdate;


struct TextureDrawData {
    texture_name : String,
    texture : Texture,
}



pub struct ObjectsFactory;

impl<'a> ObjectsFactory {

    pub fn create_object (obj_data : &ObjectData , tex_fact : &TextureFactory) -> GameObject {

        let pos = obj_data.start_position;
        let rec = obj_data.draw_rect;
        let tex = tex_fact.create_texture_draw_data(obj_data.texure_name.clone());

        GameObject {
            position : pos,
            draw_rect : rec,
            texture_draw_data : tex,
            components : Vec::new()
        }
    }
}

pub struct ObjectData {
    pub start_position : Point,
    pub texure_name : String,
    pub draw_rect : Rect
}


pub struct GameObject {
    position : Point,
    texture_draw_data : TextureDrawData,
    draw_rect : Rect,
    components : Vec<Box<ComponentUpdate>>,
}


pub trait Drawable {
    fn DrawToRenderer(&self, renderer : &mut Renderer);
}



impl<'a> Drawable for GameObject {

    fn DrawToRenderer(&self, renderer : &mut Renderer) {
        let source_rect = self.draw_rect;
        let dest_rect = Rect::new(self.position.x, self.position.y, self.draw_rect.width(), self.draw_rect.height());
        renderer.copy_ex(&self.texture_draw_data.texture, Some(source_rect),Some(dest_rect),0.0, None, false, false).unwrap();
    }
}



pub struct TextureFactory<'a> {
    renderer : &'a Renderer<'a>,
}

impl<'a> TextureFactory<'a> {

    pub fn new (rend : &'a Renderer) -> TextureFactory<'a> {
        TextureFactory {
            renderer : &rend
        }
    }
    
    fn create_texture_from_asset (&self, name : &String) -> Texture {
        let ASSETS_PATH = "assets/".to_string();
        let path = ASSETS_PATH + &name;
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new(&path)).unwrap();

        self.renderer.create_texture_from_surface(&temp_surface).unwrap()
    }

    fn create_texture_draw_data (&self, name : String) -> TextureDrawData {
        let tex = self.create_texture_from_asset(&name);
        TextureDrawData {
            texture : tex,
            texture_name : name,
        }
    }
}