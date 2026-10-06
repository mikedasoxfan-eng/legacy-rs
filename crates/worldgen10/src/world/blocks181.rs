//! Neighbour-dependent collision and selection for the new thin blocks.
use super::World;
use crate::{block::*, entity::AxisAlignedBB as BB};

impl World {
    pub fn set_piston_extension_texture181(&mut self,texture:i32) {self.piston_extension_texture_override181=texture;}
    pub fn clear_piston_extension_texture181(&mut self) {self.piston_extension_texture_override181=-1;}
    pub fn piston_extension_texture181(&self,side:i32,metadata:i32)->Result<i32,()> {
        crate::block::visual181::piston_extension_texture181(side,metadata,self.piston_extension_texture_override181)
    }
    pub fn piston_extension_selected_box181(&self,x:i32,y:i32,z:i32)->BB {
        self.block_bounds[PISTON_EXTENSION as usize].translated(x as f64,y as f64,z as f64)
    }
    pub fn piston_extension_boxes181(&mut self,x:i32,y:i32,z:i32,query:BB)->Vec<BB> {
        let metadata=self.get_block_metadata(x,y,z) as usize;
        let boxes=crate::entity::block_table::MULTI_COLLISION[PISTON_EXTENSION as usize][metadata]
            .iter().map(|b|b.translated(x as f64,y as f64,z as f64)).filter(|b|b.intersects(query)).collect();
        self.block_bounds[PISTON_EXTENSION as usize]=BB::new(0.0,0.0,0.0,1.0,1.0,1.0);
        boxes
    }
    pub fn door_selected_box181(&mut self,id:u8,x:i32,y:i32,z:i32)->BB {
        assert!(matches!(id,64|71));
        self.set_block_bounds_based_on_state(id,x,y,z);
        self.block_bounds[id as usize].translated(x as f64,y as f64,z as f64)
    }
    pub fn set_door_rotation181(&mut self,id:u8,state:i32) {
        assert!(matches!(id,64|71));
        self.block_bounds[id as usize]=crate::block::visual181::door_rotation_bounds181(state);
    }
    pub fn set_plate_item_bounds181(&mut self,id:u8) {
        assert!(matches!(id,70|72));
        self.block_bounds[id as usize]=BB::new(0.0,0.375,0.0,1.0,0.625,1.0);
    }
    pub fn mark_blocks_dirty181(&mut self, range: [i32; 6]) {
        if self.record_block_update_ranges181 {self.block_update_ranges181.push(range);}
        self.renderers_dirty = true;
    }
    pub fn drain_block_update_ranges181(&mut self) -> Vec<[i32; 6]> {
        std::mem::take(&mut self.block_update_ranges181)
    }
    pub fn cake_selected_box181(&mut self, x: i32, y: i32, z: i32) -> BB {
        crate::block::visual181::cake_box181(x,y,z,self.get_block_metadata(x,y,z),true)
    }
    pub fn set_cake_item_bounds181(&mut self) {
        self.block_bounds[CAKE as usize] = crate::block::visual181::cake_bounds181(0);
    }
    /// Both registered sign classes refresh state bounds before selection.
    /// The standing sign's state hook leaves its singleton bounds untouched.
    pub fn sign_selected_box181(&mut self, id: u8, x: i32, y: i32, z: i32) -> BB {
        assert!(matches!(id,63|68));
        self.set_block_bounds_based_on_state(id,x,y,z);
        self.block_bounds[id as usize].translated(x as f64,y as f64,z as f64)
    }
    /// BlockStairs selection delegates to the model singleton's current bounds.
    /// Unlike collision/ray state bounds, this method does not reset either singleton.
    pub fn stairs_selected_box181(&self, id: u8, x: i32, y: i32, z: i32) -> BB {
        let model=crate::block::visual181::stairs_model181(id).expect("stairs block");
        self.block_bounds[model as usize].translated(x as f64,y as f64,z as f64)
    }
    pub fn trapdoor_can_place_on_side181(&mut self, x: i32, y: i32, z: i32, side: i32) -> bool {
        if side == 0 || side == 1 { return false; }
        let (dx,dz)=match side {2=>(0,1),3=>(0,-1),4=>(1,0),5=>(-1,0),_=>(0,0)};
        self.trapdoor_support10(x.wrapping_add(dx),y,z.wrapping_add(dz))
    }
    /// Original item bounds are singleton fields, consumed by selection/rendering.
    pub fn set_trapdoor_item_bounds181(&mut self) {
        self.block_bounds[96] = BB::new(0.0, 0.40625, 0.0, 1.0, 0.59375, 1.0);
    }
    /// Button/lever neighbor callbacks first check all attachment candidates,
    /// then only the original enumerated metadata orientations.
    pub fn attached_control_neighbor181(&mut self,id:u8,x:i32,y:i32,z:i32) {
        let sides=[(-1,0,0),(1,0,0),(0,0,-1),(0,0,1),(0,-1,0)];
        let count=if id==69{5}else{4};
        // canPlaceBlockAt short-circuits, then the neighbor callback repeats all
        // four horizontal queries (and two floor queries for a lever).
        let can_attach=sides[..count].iter().any(|&(dx,dy,dz)|self.is_block_normal_cube(x+dx,y+dy,z+dz));
        let meta=self.get_block_metadata(x,y,z);
        let orientation=meta&7;
        let mut unsupported=!can_attach;
        if can_attach {
            for (k,&(dx,dy,dz)) in sides[..4].iter().enumerate() {
                unsupported |= !self.is_block_normal_cube(x+dx,y+dy,z+dz) && orientation==k as i32+1;
            }
            if id==69 {
                unsupported |= !self.is_block_normal_cube(x,y-1,z) && orientation==5;
                unsupported |= !self.is_block_normal_cube(x,y-1,z) && orientation==6;
            }
        }
        if unsupported {
            super::behavior::drop_block_as_item(self,id,x,y,z,meta);
            self.set_block_with_notify(x,y,z,0);
        }
    }
    /// BlockStep.onBlockAdded runs for both slab instances, including double slabs.
    pub fn slab_added181(&mut self,_x:i32,_y:i32,_z:i32) {}
    /// BlockTrapDoor.onPoweredBlockChange.
    pub fn trapdoor_powered181(&mut self,x:i32,y:i32,z:i32,powered:bool) {
        let meta=self.get_block_metadata(x,y,z);
        if (meta&4!=0)!=powered {
            self.set_block_metadata_with_notify(x,y,z,meta^4);
            self.auxiliary_effect(1003,(x,y,z),0);
        }
    }
    /// BlockDoor.onPoweredBlockChange recurses from the upper half and emits one sound.
    pub fn door_powered181(&mut self,id:u8,x:i32,y:i32,z:i32,powered:bool) {
        let meta=self.get_block_metadata(x,y,z);
        if meta&8!=0 {
            if self.get_block_id(x,y-1,z)==id { self.door_powered181(id,x,y-1,z,powered); }
        } else if (meta&4!=0)!=powered {
            if self.get_block_id(x,y+1,z)==id { self.set_block_metadata_with_notify(x,y+1,z,(meta^4)+8); }
            self.set_block_metadata_with_notify(x,y,z,meta^4);
            self.mark_blocks_dirty181([x,y-1,z,x,y,z]);
            self.auxiliary_effect(1003,(x,y,z),0);
        }
    }
    /// BlockTrapDoor.onNeighborBlockChange continues checking power after removal.
    pub fn trapdoor_neighbor181(&mut self,x:i32,y:i32,z:i32,changed:u8) {
        if self.multiplayer_world { return; }
        let meta=self.get_block_metadata(x,y,z);
        let (dx,dz)=[(0,1),(0,-1),(1,0),(-1,0)][(meta&3) as usize];
        if !self.trapdoor_support10(x+dx,y,z+dz) {
            self.set_block_with_notify(x,y,z,0);
            super::behavior::drop_block_as_item(self,96,x,y,z,meta);
        }
        if changed>0 {let powered=self.is_block_indirectly_getting_powered(x,y,z);
            if powered || crate::entity::block_hooks::provides_power(changed) {self.trapdoor_powered181(x,y,z,powered);}
        }
    }
    /// BlockFence.func_35303_f: gates join fences in the new renderer.
    pub fn fence_connects181(&mut self, x: i32, y: i32, z: i32) -> bool {
        self.fence_connects10(85,x,y,z)
    }
    fn pane_connections(&mut self, id: u8, x: i32, y: i32, z: i32) -> [bool; 4] {
        [(0, -1), (0, 1), (-1, 0), (1, 0)].map(|(dx, dz)| {
            let neighbor = self.get_block_id(x + dx, y, z + dz);
            opaque_cube_lookup(neighbor) || neighbor == id || neighbor == 20
        })
    }
    pub(crate) fn pane_boxes(&mut self, id: u8, x: i32, y: i32, z: i32) -> Vec<BB> {
        let [n, s, w, e] = self.pane_connections(id, x, y, z);
        let any = n || s || w || e;
        let mut boxes = Vec::new();
        if w && e || !any {
            boxes.push(BB::new(0.0, 0.0, 0.4375, 1.0, 1.0, 0.5625));
        } else if w {
            boxes.push(BB::new(0.0, 0.0, 0.4375, 0.5, 1.0, 0.5625));
        } else if e {
            boxes.push(BB::new(0.5, 0.0, 0.4375, 1.0, 1.0, 0.5625));
        }
        if n && s || !any {
            boxes.push(BB::new(0.4375, 0.0, 0.0, 0.5625, 1.0, 1.0));
        } else if n {
            boxes.push(BB::new(0.4375, 0.0, 0.0, 0.5625, 1.0, 0.5));
        } else if s {
            boxes.push(BB::new(0.4375, 0.0, 0.5, 0.5625, 1.0, 1.0));
        }
        boxes
    }
    pub(crate) fn thin_block_bounds(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let b = match id {
            IRON_BARS | GLASS_PANE => {
                let [n, s, w, e] = self.pane_connections(id, x, y, z);
                let any = n || s || w || e;
                BB::new(
                    if w || !any { 0.0 } else { 0.4375 },
                    0.0,
                    if n || !any { 0.0 } else { 0.4375 },
                    if e || !any { 1.0 } else { 0.5625 },
                    1.0,
                    if s || !any { 1.0 } else { 0.5625 },
                )
            }
            VINE => {
                let m = self.get_block_metadata(x, y, z);
                let mut b = BB::new(1.0, 1.0, 1.0, 0.0, 0.0, 0.0);
                if m & 2 != 0 {
                    b = BB::new(0.0, 0.0, 0.0, 0.0625, 1.0, 1.0);
                }
                if m & 8 != 0 {
                    b.min_x = b.min_x.min(0.9375);
                    b.max_x = 1.0;
                    b.min_y = 0.0;
                    b.max_y = 1.0;
                    b.min_z = 0.0;
                    b.max_z = 1.0;
                }
                if m & 4 != 0 {
                    b.max_z = b.max_z.max(0.0625);
                    b.min_z = 0.0;
                    b.min_x = 0.0;
                    b.max_x = 1.0;
                    b.min_y = 0.0;
                    b.max_y = 1.0;
                }
                if m & 1 != 0 {
                    b.min_z = b.min_z.min(0.9375);
                    b.max_z = 1.0;
                    b.min_x = 0.0;
                    b.max_x = 1.0;
                    b.min_y = 0.0;
                    b.max_y = 1.0;
                }
                if m == 0 && super::plants181::vine_support(self.get_block_id(x, y + 1, z)) {
                    b = BB::new(0.0, 0.9375, 0.0, 1.0, 1.0, 1.0);
                }
                b
            }
            _ => return false,
        };
        self.block_bounds[id as usize] = b;
        true
    }
}

impl crate::World {
    pub fn silverfish_destroyed181(&mut self,x:i32,y:i32,z:i32) {
        let mut e=crate::entity::Entity::living(self,"Silverfish").unwrap();
        e.set_location_and_angles(x as f64+0.5,y as f64,z as f64+0.5,0.0,0.0);
        let id=e.entity_id;self.entity_joined_world(e);
        if let Some(mut e)=self.entities.remove(&id) {e.explosion_particle(self);self.entities.insert(id,e);}
    }
}
