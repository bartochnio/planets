use std::sync::Arc;
use std::collections::HashMap;
use sdl2::surface::Surface;


type SurfaceArc<'a> = Arc<Surface<'a>>;

pub struct SurfaceLoader<'a> {
    surfaces : HashMap<String, SurfaceArc<'a>>

}

impl<'a> SurfaceLoader<'a> {
    pub fn new<'b> () -> SurfaceLoader<'b> {
        SurfaceLoader {
            surfaces : HashMap::new()
        }
    }

    pub fn alocate_or_get_surface (&mut self, path_name : &String) -> SurfaceArc {
        if self.surfaces.contains_key(path_name) {
            let surface = self.surfaces.get(path_name).unwrap();
            surface.clone()
        }
        else {
            let new_surface = Surface::load_bmp(path_name);
            let arc = Arc::new(new_surface.unwrap());
            self.surfaces.insert(path_name.clone(), arc.clone());
            arc.clone()
        }
    }

    pub fn dealocate_surface(&mut self, path_name : &String) -> Result<&'static str, &'static str> {
        if self.surfaces.contains_key(path_name) {

            // Cloning here becouse other wise i cant remove it from the map
            let wrap = self.surfaces.get(path_name).unwrap().clone();
            let arc_count = Arc::strong_count(&wrap);

            // 2 is last two arc reference. 1. for a one in the hasmap
            // And one for aboved cloned arc 'wrap'
            if arc_count == 2 {
                self.surfaces.remove(path_name);
                Result::Ok("Dealocation")
            }
            else {
                Result::Ok("Removing ref")
            }
        }
        else {
            Result::Err("Looks like it's dealocated already")
        }
    }

}
