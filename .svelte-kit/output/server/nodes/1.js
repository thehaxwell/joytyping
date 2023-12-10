

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.9144794f.js","_app/immutable/chunks/scheduler.e108d1fd.js","_app/immutable/chunks/index.96d1e77e.js","_app/immutable/chunks/stores.cfa11fc4.js","_app/immutable/chunks/singletons.50711588.js","_app/immutable/chunks/index.0378bb41.js"];
export const stylesheets = [];
export const fonts = [];
