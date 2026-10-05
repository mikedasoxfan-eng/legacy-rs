package net.minecraft.src;

import java.io.*;
import java.lang.reflect.Field;
import java.util.*;

/** Scripted block worlds for redstone, rails, note blocks and pistons: a flat stone world
 *  (chunks -3..3, floor at y 63), commands from a text file, the original World.tick in
 *  between. Commands: place x y z id meta | remove x y z | use x y z | tick n
 *  (`#` starts a comment). The final world and a state line are written for comparison. */
class RedstoneOracle {
    static class Probe extends TileEntityNote {
        int action,tx,ty,tz,value;
        Probe(int a,int x,int y,int z,int v){note=17;action=a;tx=x;ty=y;tz=z;value=v;}
        public void updateEntity() {
            World w=worldObj;
            switch(action) {
                case 0:w.removeBlockTileEntity(tx,ty,tz);break;
                case 1:TileEntityNote n=new TileEntityNote();n.note=(byte)value;w.setBlockTileEntity(tx,ty,tz,n);break;
                case 2:w.setBlockAndMetadataWithNotify(tx,ty,tz,value,0);break;
                case 3:w.removeBlockTileEntity(tx,ty,tz);w.getBlockTileEntity(tx,ty,tz);w.getBlockTileEntity(tx,ty,tz);break;
                case 4:w.setBlockWithNotify(tx,ty,tz,0);break;
            }
            w.removeBlockTileEntity(xCoord,yCoord,zCoord);
        }
    }
    static void snapshot(World w,StringBuilder s)throws Exception {
        s.append("tiles ").append(w.loadedTileEntityList.size()).append(':');
        for(Object o:w.loadedTileEntityList) {
            TileEntity t=(TileEntity)o;
            s.append(' ').append(t.xCoord).append(',').append(t.yCoord).append(',').append(t.zCoord).append(',').append(t.isInvalid()?1:0);
            if(t instanceof TileEntityNote) {TileEntityNote n=(TileEntityNote)t;s.append(",M,").append(n.note).append(',').append(n.previousRedstoneState?1:0);}
            if(t instanceof TileEntityPiston) {
                TileEntityPiston p=(TileEntityPiston)t;
                Field f=TileEntityPiston.class.getDeclaredField("progress");f.setAccessible(true);
                s.append(",P,").append(p.getStoredBlockID()).append(',').append(p.getBlockMetadata()).append(',').append(p.func_31009_d()).append(',').append(Float.floatToRawIntBits(f.getFloat(p))).append(',').append(Float.floatToRawIntBits(p.func_31008_a(0))).append(',').append(p.func_31015_b()?1:0).append(',').append(p.func_31012_k()?1:0);
            }
        }
        s.append('\n');
        for(Object o:w.loadedEntityList) {
            Entity e=(Entity)o;s.append("entity ").append(e.entityId);
            for(double v:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ,e.boundingBox.minX,e.boundingBox.minY,e.boundingBox.minZ,e.boundingBox.maxX,e.boundingBox.maxY,e.boundingBox.maxZ}) s.append(' ').append(Double.doubleToRawLongBits(v));
            s.append('\n');
        }
    }
    static void run(String script, String out) throws Exception {
        World w = Oracle.newWorld(0);
        w.chunkProvider = new EntityOracle.FlatProvider(w);
        w.difficultySetting = 2;
        w.setAllowedMobSpawns(false, false);
        Field ids = Entity.class.getDeclaredField("nextEntityID"); ids.setAccessible(true); ids.setInt(null, 1000);
        Item.itemRand.setSeed(600); OracleHooks.reset(100); EntityOracle.mathRandom().setSeed(200);
        Field chest = BlockChest.class.getDeclaredField("random"); chest.setAccessible(true); ((Random) chest.get(Block.chest)).setSeed(500);
        w.rand.setSeed(300);
        w.updateLCG = 0;
        Field sound = World.class.getDeclaredField("soundCounter"); sound.setAccessible(true); sound.setInt(w, 0);
        PlayerOracle.HeadlessPlayer p = PlayerOracle.player(w);
        p.setLocationAndAngles(0.5, 64, 0.5, 0, 0);
        final StringBuilder events=new StringBuilder();
        w.addWorldAccess((IWorldAccess)java.lang.reflect.Proxy.newProxyInstance(IWorldAccess.class.getClassLoader(),new Class[]{IWorldAccess.class},(proxy,method,args)->{
            if(method.getName().equals("playSound") && (((String)args[0]).startsWith("tile.piston.") || ((String)args[0]).startsWith("note."))) {
                events.append("sound ").append(args[0]);
                for(int j=1;j<=3;j++) events.append(' ').append(Double.doubleToRawLongBits((Double)args[j]));
                events.append(' ').append(Float.floatToRawIntBits((Float)args[4])).append(' ').append(Float.floatToRawIntBits((Float)args[5])).append('\n');
            }
            if(method.getName().equals("spawnParticle") && args[0].equals("note")) {
                events.append("particle note");for(int j=1;j<=6;j++)events.append(' ').append(Double.doubleToRawLongBits((Double)args[j]));events.append('\n');
            }
            return null;
        }));
        int ticks = 0;
        StringBuilder steps = new StringBuilder();
        try (BufferedReader r = new BufferedReader(new FileReader(script))) {
            for (String line; (line = r.readLine()) != null; ) {
                line = line.split("#", 2)[0].trim();
                if (line.isEmpty()) continue;
                String[] a = line.split("\\s+");
                int[] v = new int[a.length];
                for (int i = 1; i < a.length; i++) v[i] = Integer.parseInt(a[i]);
                switch (a[0]) {
                    case "place": w.setBlockAndMetadataWithNotify(v[1], v[2], v[3], v[4], v[5]); break;
                    case "itemplace": {
                        p.setLocationAndAngles(v[6]/1000.0,v[7]/1000.0,v[8]/1000.0,v[9]/1000.0F,0);
                        Item.itemsList[v[5]].onItemUse(new ItemStack(v[5],1,0),p,w,v[1],v[2],v[3],v[4]);break;
                    }
                    case "placedby": {
                        p.setLocationAndAngles(v[4]/1000.0,v[5]/1000.0,v[6]/1000.0,v[7]/1000.0F,0);
                        Block.blocksList[w.getBlockId(v[1],v[2],v[3])].onBlockPlacedBy(w,v[1],v[2],v[3],p);break;
                    }
                    case "click": Block.blocksList[w.getBlockId(v[1],v[2],v[3])].onBlockClicked(w,v[1],v[2],v[3],p);break;
                    case "item": w.entityJoinedWorld(new EntityItem(w,v[1]/1000.0,v[2]/1000.0,v[3]/1000.0,new ItemStack(v[4],v[5],v[6])));break;
                    case "mob": {
                        EntityLiving e=(EntityLiving)EntityList.createEntity(v[4],w);
                        e.setLocationAndAngles(v[1]/1000.0,v[2]/1000.0,v[3]/1000.0,0,0);w.entityJoinedWorld(e);break;
                    }
                    case "reload": {
                        TileEntity te=w.getBlockTileEntity(v[1],v[2],v[3]);NBTTagCompound n=new NBTTagCompound();te.writeToNBT(n);
                        w.removeBlockTileEntity(v[1],v[2],v[3]);w.setBlockTileEntity(v[1],v[2],v[3],TileEntity.createAndLoadEntity(n));break;
                    }
                    case "properties": {
                        for(int id=1;id<=96;id++)if(Block.blocksList[id]!=null) steps.append("property ").append(id).append(' ').append(Block.blocksList[id].getMobilityFlag()).append(' ').append(Float.floatToRawIntBits(Block.blocksList[id].getHardness())).append(' ').append(Block.isBlockContainer[id]?1:0).append('\n');break;
                    }
                    case "boxes": {
                        int x=v[1],y=v[2],z=v[3];Block b=Block.blocksList[w.getBlockId(x,y,z)];
                        AxisAlignedBB box=b==null?null:b.getCollisionBoundingBoxFromPool(w,x,y,z);
                        steps.append("collision");
                        if(box==null)steps.append(" null");else for(double d:new double[]{box.minX,box.minY,box.minZ,box.maxX,box.maxY,box.maxZ})steps.append(' ').append(Double.doubleToRawLongBits(d));
                        steps.append('\n');
                        if(b!=null){b.setBlockBoundsBasedOnState(w,x,y,z);steps.append("selection");for(double d:new double[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ})steps.append(' ').append(Double.doubleToRawLongBits(d));steps.append('\n');}break;
                    }
                    case "probe": {
                        w.setBlockTileEntity(v[1],v[2],v[3],new Probe(v[4],v[5],v[6],v[7],v[8]));break;
                    }
                    case "scan":w.updateEntities();break;
                    case "lookup": {
                        TileEntity te=w.getBlockTileEntity(v[1],v[2],v[3]);
                        steps.append("lookup ").append(te==null?-1:((TileEntityNote)te).note).append('\n');break;
                    }
                    case "setnote": {
                        TileEntityNote te=new TileEntityNote();NBTTagCompound n=new NBTTagCompound();n.setByte("note",(byte)v[4]);te.readFromNBT(n);w.setBlockTileEntity(v[1],v[2],v[3],te);break;
                    }
                    case "chunkcycle": {
                        Chunk c=w.chunkProvider.provideChunk(v[1],v[2]);c.onChunkUnload();c.onChunkLoad();break;
                    }
                    case "chunkreload": {
                        EntityOracle.FlatProvider f=(EntityOracle.FlatProvider)w.chunkProvider;
                        Chunk c=f.provideChunk(v[1],v[2]);NBTTagCompound n=new NBTTagCompound();ChunkLoader.storeChunkInCompound(c,w,n);c.onChunkUnload();
                        c=ChunkLoader.loadChunkIntoWorldFromCompound(w,n);f.chunks.put(f.key(v[1],v[2]),c);c.onChunkLoad();break;
                    }
                    case "events": steps.append(events);events.setLength(0);break;
                    case "state": snapshot(w,steps);break;
                    case "remove": w.setBlockWithNotify(v[1], v[2], v[3], 0); break;
                    case "use": {
                        int id = w.getBlockId(v[1], v[2], v[3]);
                        if (id > 0) Block.blocksList[id].blockActivated(w, v[1], v[2], v[3], p);
                        break;
                    }
                    case "tick":
                        for (int t = 0; t < v[1]; t++) { w.updateEntities(); w.tick(); Oracle.drainLighting(w); ticks++; }
                        break;
                    case "check": {
                        // check x0 y0 z0 x1 y1 z1: ids and metadata of a box, every tick for n ticks
                        int n = a.length > 7 ? v[7] : 0;
                        for (int t = 0; t <= n; t++) {
                            if (t > 0) { w.updateEntities(); w.tick(); Oracle.drainLighting(w); ticks++; }
                            steps.append(ticks).append(':');
                            for (int y = v[2]; y <= v[5]; y++) for (int z = v[3]; z <= v[6]; z++) for (int x = v[1]; x <= v[4]; x++)
                                steps.append(' ').append(w.getBlockId(x, y, z)).append('/').append(w.getBlockMetadata(x, y, z));
                            steps.append('\n');
                        }
                        break;
                    }
                    default: throw new IllegalArgumentException(line);
                }
                Oracle.drainLighting(w);
            }
        }
        Oracle.dumpWorld(w, out, 0, 64, 0);
        try (PrintWriter pw = new PrintWriter(out + ".txt")) {
            pw.println("ticks " + ticks + " time " + w.getWorldTime() + " entities " + w.loadedEntityList.size());
            pw.println("rand " + w.rand.nextLong());
            pw.print(steps);
        }
    }
}
