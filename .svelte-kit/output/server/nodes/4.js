

export const index = 4;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/app/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/4.7d884ce2.js","_app/immutable/chunks/scheduler.e108d1fd.js","_app/immutable/chunks/index.96d1e77e.js"];
export const stylesheets = [];
export const fonts = [];
