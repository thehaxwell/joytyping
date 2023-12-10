export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png","robots.txt"]),
	mimeTypes: {".png":"image/png",".txt":"text/plain"},
	_: {
		client: {"start":"_app/immutable/entry/start.d0f818fd.js","app":"_app/immutable/entry/app.f0008ff7.js","imports":["_app/immutable/entry/start.d0f818fd.js","_app/immutable/chunks/scheduler.e108d1fd.js","_app/immutable/chunks/singletons.50711588.js","_app/immutable/chunks/index.0378bb41.js","_app/immutable/entry/app.f0008ff7.js","_app/immutable/chunks/scheduler.e108d1fd.js","_app/immutable/chunks/index.96d1e77e.js"],"stylesheets":[],"fonts":[]},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js'))
		],
		routes: [
			
		],
		matchers: async () => {
			
			return {  };
		}
	}
}
})();
