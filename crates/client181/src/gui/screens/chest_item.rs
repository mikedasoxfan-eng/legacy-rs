//! ChestItemRenderHelper and the closed ModelChest used by GUI item icons.
use crate::render::entity::{fixed::Fixed,geometry::ModelRenderer};
pub fn append(g:&mut Fixed) {
    g.texture=31;g.push();g.material=[1.0;4];g.translate(0.0,1.0,1.0);g.scale(1.0,-1.0,-1.0);
    // TileEntityChestRenderer: turn about the block centre by the facing (0 for the item's chest).
    g.translate(0.5,0.5,0.5);g.rotate(0.0,0.0,1.0,0.0);g.translate(-0.5,-0.5,-0.5);
    for (uv,origin,size,point) in [([0,0],[0.0,-5.0,-14.0],[14,5,14],[1.0,7.0,15.0]),([0,0],[-1.0,-2.0,-15.0],[2,4,1],[8.0,7.0,15.0]),([0,19],[0.0,0.0,0.0],[14,10,14],[1.0,6.0,1.0])] {
        let mut part=ModelRenderer::new(uv[0],uv[1]);part.add_box(origin[0],origin[1],origin[2],size[0],size[1],size[2],0.0);
        g.push();g.translate(point[0]/16.0,point[1]/16.0,point[2]/16.0);
        for face in part.faces {let n=face.normal();g.begin();g.set_normal(n.x,n.y,n.z);
            g.quad(face.vertices.map(|v|v.pos.map(|n|(n as f32)/16.0)),face.vertices.map(|v|[v.uv[0],v.uv[1]*0.5]));g.end();}
        g.pop();
    }
    g.pop();
}
