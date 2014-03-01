// See LICENSE file for copyright and license details.

use cgmath::vector::{
    Vec3,
    Vec2,
};
use visualizer::gl_helpers::{
    uniform_mat4f,
    set_clear_color,
    clear_screen,
    get_vec2_from_pixel,
};
use core::map::MapPosIter;
use visualizer::camera::Camera;
use visualizer::geom::Geom;
use visualizer::mesh::Mesh;
use core::core_types::{
    MInt,
    Size2,
    MapPos,
};
use visualizer::gl_types::{
    VertexCoord,
    Color3,
    MFloat,
    MatId,
};
use visualizer::shader::Shader;

fn build_hex_map_mesh(
    geom: &Geom,
    map_size: Size2<MInt>
) -> (~[VertexCoord], ~[Color3]) {
    let mut c_data = ~[];
    let mut v_data = ~[];
    for tile_pos in MapPosIter::new(map_size) {
        let pos3d = geom.map_pos_to_world_pos(tile_pos);
        for num in range(0 as MInt, 6) {
            let vertex = geom.index_to_hex_vertex(num);
            let next_vertex = geom.index_to_hex_vertex(num + 1);
            let col_x = tile_pos.x as MFloat / 255.0;
            let col_y = tile_pos.y as MFloat / 255.0;
            let color = Color3{r: col_x, g: col_y, b: 1.0};
            v_data.push(pos3d + vertex);
            c_data.push(color);
            v_data.push(pos3d + next_vertex);
            c_data.push(color);
            v_data.push(pos3d + Vec3::zero());
            c_data.push(color);
        }
    }
    (v_data, c_data)
}

fn get_mesh(geom: &Geom, map_size: Size2<MInt>, shader: &Shader) -> Mesh {
    let (vertex_data, color_data) = build_hex_map_mesh(geom, map_size);
    let mut mesh = Mesh::new(vertex_data);
    mesh.set_color(color_data);
    mesh.prepare(shader);
    mesh
}

pub struct TilePicker {
    shader: Shader,
    map_mesh: Mesh,
    mvp_mat_id: MatId,
    win_size: Size2<MInt>,
}

impl TilePicker {
    pub fn new(
        win_size: Size2<MInt>,
        geom: &Geom,
        map_size: Size2<MInt>
    ) -> ~TilePicker {
        let shader = Shader::new("pick.vs.glsl", "pick.fs.glsl");
        let mvp_mat_id = MatId(shader.get_uniform("mvp_mat"));
        let map_mesh = get_mesh(geom, map_size, &shader);
        let tile_picker = ~TilePicker {
            map_mesh: map_mesh,
            shader: shader,
            mvp_mat_id: mvp_mat_id,
            win_size: win_size,
        };
        tile_picker
    }

    pub fn set_win_size(&mut self, win_size: Size2<MInt>) {
        self.win_size = win_size;
    }

    pub fn pick_tile(
        &mut self,
        camera: &Camera,
        mouse_pos: Vec2<MInt>
    ) -> Option<MapPos> {
        self.shader.activate();
        uniform_mat4f(self.mvp_mat_id, &camera.mat());
        set_clear_color(0.0, 0.0, 0.0);
        clear_screen();
        self.map_mesh.draw(&self.shader);
        get_vec2_from_pixel(self.win_size, mouse_pos)
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab: