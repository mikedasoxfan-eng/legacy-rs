package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;import static net.minecraft.src.Oracle.*;
public final class Mode_models10 {
    static void models(String out) throws Exception {
        int[] kinds = {0,0,0,1,2,3,3,4,4,5,6,7,8,9,10,10,10,11,12,13,14,15,16,17,18,19};
        float[] inflate = {0,.35F,-.25F,0,0,0,.6F,0,.5F,0,0,0,0,0,0,2,-.1F,0,0,0,0,0,0,0,0,0};
        float[] offsets = {0,1.25F,-2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
        int[] legs = {12,12,12,12,12,12,6,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12};
        float[][] poses = {
            {0,0,0,0,0,.0625F}, {1.25F,.7F,4.5F,35,-20,.0625F},
            {-3.75F,1,17.25F,-179,89,.125F}, {12.345F,.13F,-2.5F,270,-67,.03125F},
            {0,0,0,0,0,.0625F}, {3.1415927F,.99F,.75F,-45,12,.0625F},
            {1024.5F,.4F,10000.25F,123.75F,-45.5F,.0625F}, {0,0,0,0,0,.0625F}
        };
        float[] attacks = {0,.35F,.8F,1,-10000,.01F,.6F,0};
        
        
        Field normal = Tessellator.class.getDeclaredField("normal"); normal.setAccessible(true);
        Field drawing = Tessellator.class.getDeclaredField("isDrawing"); drawing.setAccessible(true);
        DataOut o = new DataOut(); o.i32(0x3337314d); o.i32(2); o.i32(kinds.length * poses.length);
        for (int group = 0; group < kinds.length; group++) {
            ModelBase m;
            switch (kinds[group]) {
                case 0: m = new ModelBiped(inflate[group], offsets[group]); break;
                case 1: m = new ModelZombie(); break;
                case 2: m = new ModelSkeleton(); break;
                case 3: m = new ModelQuadruped(legs[group], inflate[group]); break;
                case 4: m = new ModelPig(inflate[group]); break;
                case 5: m = new ModelCow(); break;
                case 6: m = new ModelSheep1(); break;
                case 7: m = new ModelSheep2(); break;
                case 8: m = new ModelChicken(); break;
                case 9: m = new ModelSpider(); break;
                case 10: m = new ModelCreeper(inflate[group]); break;
                case 11: m = new ModelSlime(0); break;
                case 12: m = new ModelSlime(16); break;
                case 13: m = new ModelSquid(); break;
                case 14: m = new ModelWolf(); break;
                case 15: m = new ModelGhast(); break;
                case 16: m = new ModelEnderman(); break;
                case 17: m = new ModelSilverfish(); break;
                case 18: m = new ModelChest(); break;
                case 19: m = new ModelLargeChest(); break;
                default: throw new AssertionError();
            }
            EntityWolf wolf = kinds[group] == 14 ? new EntityWolf(null) : null;
            for (int c = 0; c < poses.length; c++) {
                float[] p = poses[c];
                int flags = c == 1 ? 1 : c == 2 ? 30 : c == 3 ? 63 : c == 5 ? 12 : 0;
                float partial = c * .125F;
                float[] interest = {.1F * c, .2F * c};
                float[] shake = {.15F * c, .25F * c};
                m.onGround = attacks[c]; m.isRiding = (flags & 1) != 0;
                if (m instanceof ModelBiped) {
                    ModelBiped b = (ModelBiped)m;
                    b.isSneak = (flags & 2) != 0;
                    b.field_1279_h = ((flags & 4) != 0?1:0); b.field_1278_i = ((flags & 8) != 0?1:0);
                }
                if (wolf != null) {
                    wolf.setWolfAngry((flags & 16) != 0); wolf.setIsSitting((flags & 32) != 0);
                    wolfField(wolf, "field_25054_c", interest[0]); wolfField(wolf, "field_25048_b", interest[1]);
                    wolfField(wolf, "prevTimeWolfIsShaking", shake[0]); wolfField(wolf, "timeWolfIsShaking", shake[1]);
                    m.setLivingAnimations(wolf, p[0], p[1], partial);
                }
                if(m instanceof ModelEnderman){((ModelEnderman)m).isCarrying=(flags&4)!=0;((ModelEnderman)m).isAttacking=(flags&8)!=0;}
                m.setRotationAngles(p[0],p[1],p[2],p[3],p[4],p[5]);
                o.i32(group); o.i32(kinds[group]); modelFloat(o, inflate[group]); modelFloat(o, offsets[group]); o.i32(legs[group]);
                for (float v : p) modelFloat(o,v);
                modelFloat(o,attacks[c]); o.i32(flags); modelFloat(o,partial);
                for (float v : interest) modelFloat(o,v); for (float v : shake) modelFloat(o,v);
                SortedMap<String,ModelRenderer> ps = modelParts(m); o.i32(ps.size());
                for (Map.Entry<String,ModelRenderer> e : ps.entrySet()) {
                    ModelRenderer r = e.getValue(); modelString(o,e.getKey());
                    for (String f : new String[]{"rotationPointX","rotationPointY","rotationPointZ","rotateAngleX","rotateAngleY","rotateAngleZ"})
                        modelFloat(o, ModelRenderer.class.getField(f).getFloat(r));
                    o.i32((r.mirror ? 1 : 0) | (r.showModel ? 2 : 0) | (r.isHidden ? 4 : 0));
                    PositionTextureVertex[] vs = Harness10.modelCorners(r); o.i32(vs.length);
                    for (PositionTextureVertex v : vs) modelVertex(o,v);
                    TexturedQuad[] qs = Harness10.modelFaces(r); o.i32(qs.length);
                    for (TexturedQuad q : qs) {
                        o.i32(q.vertexPositions.length);
                        for (PositionTextureVertex v : q.vertexPositions) modelVertex(o,v);
                        // The same Vec3D calls TexturedQuad.draw uses, followed by
                        // the original Tessellator's BYTE packing. No draw/GL call.
                        Vec3D edge1 = q.vertexPositions[1].vector3D.subtract(q.vertexPositions[0].vector3D);
                        Vec3D edge2 = q.vertexPositions[1].vector3D.subtract(q.vertexPositions[2].vector3D);
                        Vec3D n = edge2.crossProduct(edge1).normalize();
                        modelFloat(o,(float)n.xCoord); modelFloat(o,(float)n.yCoord); modelFloat(o,(float)n.zCoord);
                        Tessellator.instance.startDrawingQuads();
                        Tessellator.instance.setNormal((float)n.xCoord,(float)n.yCoord,(float)n.zCoord);
                        o.i32(normal.getInt(Tessellator.instance)); drawing.setBoolean(Tessellator.instance,false);
                    }
                }
            }
        }
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) { o.writeTo(os); }
        System.out.println("models: " + kinds.length + " constructors, " + poses.length + " successive poses each");
    }

 public static void main(String[] a)throws Exception{models(a[1]);}
}