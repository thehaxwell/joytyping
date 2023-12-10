import * as universal from '../entries/pages/_page.ts.js';

export const index = 3;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+page.ts";
export const imports = ["_app/immutable/nodes/3.b1996811.js","_app/immutable/chunks/scheduler.e108d1fd.js","_app/immutable/chunks/index.96d1e77e.js","_app/immutable/chunks/index.0378bb41.js"];
export const stylesheets = ["_app/immutable/assets/3.6dad140f.css"];
export const fonts = [];
