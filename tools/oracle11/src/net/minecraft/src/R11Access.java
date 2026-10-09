package net.minecraft.src;
import java.lang.reflect.Field;
/** Read fixture observations without changing original game bytecode or visibility. */
final class R11Access {
    static Object get(Object instance, String name) {
        Class<?> type=instance.getClass();
        while(type!=null) {
            try { Field field=type.getDeclaredField(name); field.setAccessible(true); return field.get(instance); }
            catch(NoSuchFieldException missing) { type=type.getSuperclass(); }
            catch(IllegalAccessException error) { throw new RuntimeException(error); }
        }
        throw new IllegalArgumentException(name);
    }
}
