

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/app/_layout.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.a405de0d.js","_app/immutable/chunks/scheduler.e108d1fd.js","_app/immutable/chunks/index.96d1e77e.js","_app/immutable/chunks/stores.cfa11fc4.js","_app/immutable/chunks/singletons.50711588.js","_app/immutable/chunks/index.0378bb41.js"];
export const stylesheets = ["_app/immutable/assets/2.672fdaf1.css"];
export const fonts = [];
