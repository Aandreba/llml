flat_mod!(vec3);

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
flat_mod!(vec2);