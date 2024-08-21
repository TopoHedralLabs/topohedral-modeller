use crate::boxing::ABox;

use topohedral_viewer::{Color, d3::{State3D,CuboidDescriptor, Mesh, State, Vertex, VertexDescriptor}, Colormap, ColormapError};

use super::common::{Vec3f32, Vecf64ToVecf32, Viewable3D};
use crate::common::Vec3;

pub struct ABoxViewOptions
{
    pub color: Color,
}

impl Viewable3D for ABox
{
    type Options = ABoxViewOptions;
    fn view(
        &mut self,
        port: usize,
        opts: &Self::Options,
    )
    {
        let color = opts.color;
        let normal = Vec3f32::zeros();
        let xmin = self.xmin();
        let xmax = self.xmax();
        let ymin = self.ymin();
        let ymax = self.ymax();
        let zmin = self.zmin();
        let zmax = self.zmax();
        let v0 = Vec3::new(xmin, ymin, zmin);
        let v1 = Vec3::new(xmax, ymin, zmin);
        let v2 = Vec3::new(xmax, ymax, zmin);
        let v3 = Vec3::new(xmin, ymax, zmin);

        let num_lines = 12;
        let mut mesh = Mesh::from_num_lines(num_lines);

        mesh.append_vertex(&Vertex::new(&VertexDescriptor {
            position: v0.convert(),
            normal: normal,
            line_color: color,
            triangle_color: color,
        }));
        mesh.append_vertex(&Vertex::new(&VertexDescriptor {
            position: v1.convert(),
            normal: normal,
            line_color: color,
            triangle_color: color,
        }));
        mesh.append_vertex(&Vertex::new(&VertexDescriptor {
            position: v2.convert(),
            normal: normal,
            line_color: color,
            triangle_color: color,
        }));
        mesh.append_vertex(&Vertex::new(&VertexDescriptor {
            position: v3.convert(),
            normal: normal,
            line_color: color,
            triangle_color: color,
        }));

        mesh.append_indices(&[0, 1]);
        mesh.append_indices(&[1, 2]);
        mesh.append_indices(&[2, 3]);
        mesh.append_indices(&[3, 0]);

        if self.is_3d()
        {
            let v4 = Vec3::new(xmin, ymin, zmax);
            let v5 = Vec3::new(xmax, ymin, zmax);
            let v6 = Vec3::new(xmax, ymax, zmax);
            let v7 = Vec3::new(xmin, ymax, zmax);

            mesh.append_vertex(&Vertex::new(&VertexDescriptor {
                position: v4.convert(),
                normal: normal,
                line_color: color,
                triangle_color: color,
            }));
            mesh.append_vertex(&Vertex::new(&VertexDescriptor {
                position: v5.convert(),
                normal: normal,
                line_color: color,
                triangle_color: color,
            }));
            mesh.append_vertex(&Vertex::new(&VertexDescriptor {
                position: v6.convert(),
                normal: normal,
                line_color: color,
                triangle_color: color,
            }));
            mesh.append_vertex(&Vertex::new(&VertexDescriptor {
                position: v7.convert(),
                normal: normal,
                line_color: color,
                triangle_color: color,
            }));

            mesh.append_indices(&[0, 4]);
            mesh.append_indices(&[1, 5]);
            mesh.append_indices(&[2, 6]);
            mesh.append_indices(&[3, 7]);
            mesh.append_indices(&[4, 5]);
            mesh.append_indices(&[5, 6]);
            mesh.append_indices(&[6, 7]);
            mesh.append_indices(&[7, 4]);
        }

        // state.add_mesh(mesh);
    }
}
