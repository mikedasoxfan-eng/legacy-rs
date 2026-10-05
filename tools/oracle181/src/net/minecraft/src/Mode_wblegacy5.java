package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
/** Original old ChunkLoader paths, reads, saves, locks and dimension selection. */
public final class Mode_wblegacy5 {
    static void path(DataOutputStream o,File root,File file)throws Exception {
        o.writeBoolean(file!=null);
        if(file!=null)Mode_wboldformat4.text(o,root.toPath().relativize(file.toPath()).toString().replace('\\','/'));
    }
    static NBTTagCompound root(int k) {
        NBTTagCompound root=new NBTTagCompound(),n=new NBTTagCompound();root.setCompoundTag("Level",n);
        n.setInteger("xPos",k==1?-7:k);n.setInteger("zPos",k==1?11:0);n.setBoolean("TerrainPopulated",true);
        for(int j=0;j<5;j++)if(!(k==3&&j==0)&&!(k==5&&j==2))n.setByteArray(Mode_wbarrays.keys[j],Mode_wbarrays.data(j==0?32768:j==4?256:16384,k+j));
        if(k==2)root=new NBTTagCompound();if(k==4)n.setInteger("Data",1);
        if(k==6)n.setByteArray("Blocks",new byte[1]);
        return root;
    }
    public static void main(String[] a)throws Exception {
        System.setErr(new PrintStream(new OutputStream(){public void write(int b){}}));
        int init=Block.stone.blockID;File root=new File("wb181-legacy5-java");Mode_wbsavelife.clean(root);root.mkdirs();
        World w=Mode_worldblocks181.fixture(true);Method path=ChunkLoader.class.getDeclaredMethod("chunkFileForXZ",int.class,int.class);path.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(boolean create:new boolean[]{false,true}) {
                File dir=new File(root,create?"create":"readonly");dir.mkdirs();ChunkLoader loader=new ChunkLoader(dir,create);
                for(int x:new int[]{0,-1,63,64,-64,Integer.MIN_VALUE,Integer.MAX_VALUE})for(int z:new int[]{0,-1,64}) {
                    path(o,dir,(File)path.invoke(loader,x,z));
                    o.writeBoolean(new File(dir,Integer.toString(x&63,36)).exists());
                    o.writeBoolean(new File(new File(dir,Integer.toString(x&63,36)),Integer.toString(z&63,36)).exists());
                    o.writeBoolean(loader.loadChunk(w,x,z)!=null);
                }
            }
            File input=new File(root,"input");input.mkdirs();ChunkLoader reader=new ChunkLoader(input,true);
            for(int k=0;k<9;k++) {
                File file=(File)path.invoke(reader,k,0);
                if(k==7)try(FileOutputStream f=new FileOutputStream(file)){f.write(new byte[]{1,2,3});}
                else if(k!=8)CompressedStreamTools.writeGzippedCompoundToOutputStream(root(k),new FileOutputStream(file));
                Chunk c=reader.loadChunk(w,k,0);o.writeBoolean(c!=null);if(c!=null)Mode_wbarrays.snapshot(o,c);
            }
            File save=new File(root,"save");SaveHandler handler=new SaveHandler(root,"save",false);
            Field handlerField=World.class.getDeclaredField("saveHandler");handlerField.setAccessible(true);handlerField.set(w,handler);
            for(WorldProvider provider:new WorldProvider[]{WorldProvider.getProviderForDimension(0),new WorldProviderHell(),new WorldProviderSky()}) {
                IChunkLoader loader=handler.getChunkLoader(provider);o.writeBoolean(loader instanceof ChunkLoader);
                o.writeBoolean(new File(save,"DIM-1").exists());
            }
            ChunkLoader writer=(ChunkLoader)handler.getChunkLoader(WorldProvider.getProviderForDimension(0));
            Chunk chunk=new Chunk(w,new byte[32768],-67,131);chunk.isTerrainPopulated=true;
            for(int k=0;k<4;k++) {
                w.setWorldTime(77+k);File file=(File)path.invoke(writer,chunk.xPosition,chunk.zPosition);
                long old=file.exists()?file.length():0;long size=w.getWorldInfo().getSizeOnDisk();
                if(k==2)new File(save,"tmp_chunk.dat").mkdir();
                writer.saveChunk(w,chunk);
                long delta=w.getWorldInfo().getSizeOnDisk()-size;
                o.writeBoolean(delta==(k==2?-old:file.length()-old));
                o.writeBoolean(file.exists());o.writeBoolean(new File(save,"tmp_chunk.dat").exists());
                if(k==2)new File(save,"tmp_chunk.dat").delete();
                NBTTagCompound tag=CompressedStreamTools.loadGzippedCompoundFromOutputStream(new FileInputStream(file));
                ByteArrayOutputStream bytes=new ByteArrayOutputStream();CompressedStreamTools.func_1139_a(tag,new DataOutputStream(bytes));
                EntityOracle.i(o,bytes.size());o.write(bytes.toByteArray());
            }
            writer.func_814_a();writer.saveExtraData();writer.saveExtraChunkData(w,chunk);
            try(FileOutputStream lock=new FileOutputStream(new File(save,"session.lock"))){lock.write(new byte[8]);}
            try{writer.saveChunk(w,chunk);o.writeBoolean(false);}catch(MinecraftException e){o.writeBoolean(true);}
        }
    }
}
