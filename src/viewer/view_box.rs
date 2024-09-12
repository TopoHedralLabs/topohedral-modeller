//! This module contains the code for viewing a 2D and 3D box.
//!
//! 
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::boxing::ABox;
use super::common::{tv ,Viewable, Convert};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use topohedral_viewer::{Color, CellType, d3::Client3D, d3::Mesh3D, d3::Mesh, d3::CuboidDescriptor};
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

pub struct ABoxViewOptions
{
    pub color: Color,
}

impl Viewable for ABox<3>
{
    type Options = ABoxViewOptions;
    fn view(
        &mut self,
        port: usize,
        opts: &Self::Options,
    )
    {

        let mesh = Mesh::create_cuboid(&CuboidDescriptor{
            origin: self.origin().convert(), 
            x_axis: tv::Vec3::x(), 
            y_axis: tv::Vec3::y(),
            z_axis: tv::Vec3::z(),
            lenx: self.length(0) as f32, 
            leny: self.length(1) as f32,
            lenz: self.length(2) as f32,
            line_color: opts.color,
            tri_color: opts.color,  
            cell_type: CellType::Line,
        });

        match Client3D::new(port) {
            Ok(mut client) => {
                match client.add_mesh(mesh) {
                    Ok(mesh_id) => {
                        //{{{ trace
                        info!("mesh_id: {}", mesh_id);
                        //}}}
                    }
                    Err(err) => {
                        //{{{ trace
                        error!("Failed to add mesh with error: {}", err);
                        //}}}
                    }
                }
            }
            Err(err) => {
                //{{{ trace
                error!("Failed to connect to client with error: {}", err);
                //}}}
            }
        };
    }
}
