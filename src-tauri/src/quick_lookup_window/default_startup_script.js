window.addEventListener("load", (event) => {
var app = (function () {
	'use strict';

	/** @returns {void} */
	function noop() {}

	function run(fn) {
		return fn();
	}

	function blank_object() {
		return Object.create(null);
	}

	/**
	 * @param {Function[]} fns
	 * @returns {void}
	 */
	function run_all(fns) {
		fns.forEach(run);
	}

	/**
	 * @param {any} thing
	 * @returns {thing is Function}
	 */
	function is_function(thing) {
		return typeof thing === 'function';
	}

	/** @returns {boolean} */
	function safe_not_equal(a, b) {
		return a != a ? b == b : a !== b || (a && typeof a === 'object') || typeof a === 'function';
	}

	/** @returns {boolean} */
	function is_empty(obj) {
		return Object.keys(obj).length === 0;
	}

	/**
	 * @param {Node} target
	 * @param {Node} node
	 * @returns {void}
	 */
	function append(target, node) {
		target.appendChild(node);
	}

	/**
	 * @param {Node} target
	 * @param {Node} node
	 * @param {Node} [anchor]
	 * @returns {void}
	 */
	function insert(target, node, anchor) {
		target.insertBefore(node, anchor || null);
	}

	/**
	 * @param {Node} node
	 * @returns {void}
	 */
	function detach(node) {
		if (node.parentNode) {
			node.parentNode.removeChild(node);
		}
	}

	/**
	 * @returns {void} */
	function destroy_each(iterations, detaching) {
		for (let i = 0; i < iterations.length; i += 1) {
			if (iterations[i]) iterations[i].d(detaching);
		}
	}

	/**
	 * @template {keyof HTMLElementTagNameMap} K
	 * @param {K} name
	 * @returns {HTMLElementTagNameMap[K]}
	 */
	function element(name) {
		return document.createElement(name);
	}

	/**
	 * @template {keyof SVGElementTagNameMap} K
	 * @param {K} name
	 * @returns {SVGElement}
	 */
	function svg_element(name) {
		return document.createElementNS('http://www.w3.org/2000/svg', name);
	}

	/**
	 * @param {string} data
	 * @returns {Text}
	 */
	function text(data) {
		return document.createTextNode(data);
	}

	/**
	 * @returns {Text} */
	function space() {
		return text(' ');
	}

	/**
	 * @returns {Text} */
	function empty() {
		return text('');
	}

	/**
	 * @param {Element} node
	 * @param {string} attribute
	 * @param {string} [value]
	 * @returns {void}
	 */
	function attr(node, attribute, value) {
		if (value == null) node.removeAttribute(attribute);
		else if (node.getAttribute(attribute) !== value) node.setAttribute(attribute, value);
	}

	/**
	 * @param {Element} element
	 * @returns {ChildNode[]}
	 */
	function children(element) {
		return Array.from(element.childNodes);
	}

	/**
	 * @typedef {Node & {
	 * 	claim_order?: number;
	 * 	hydrate_init?: true;
	 * 	actual_end_child?: NodeEx;
	 * 	childNodes: NodeListOf<NodeEx>;
	 * }} NodeEx
	 */

	/** @typedef {ChildNode & NodeEx} ChildNodeEx */

	/** @typedef {NodeEx & { claim_order: number }} NodeEx2 */

	/**
	 * @typedef {ChildNodeEx[] & {
	 * 	claim_info?: {
	 * 		last_index: number;
	 * 		total_claimed: number;
	 * 	};
	 * }} ChildNodeArray
	 */

	let current_component;

	/** @returns {void} */
	function set_current_component(component) {
		current_component = component;
	}

	const dirty_components = [];
	const binding_callbacks = [];

	let render_callbacks = [];

	const flush_callbacks = [];

	const resolved_promise = /* @__PURE__ */ Promise.resolve();

	let update_scheduled = false;

	/** @returns {void} */
	function schedule_update() {
		if (!update_scheduled) {
			update_scheduled = true;
			resolved_promise.then(flush);
		}
	}

	/** @returns {void} */
	function add_render_callback(fn) {
		render_callbacks.push(fn);
	}

	// flush() calls callbacks in this order:
	// 1. All beforeUpdate callbacks, in order: parents before children
	// 2. All bind:this callbacks, in reverse order: children before parents.
	// 3. All afterUpdate callbacks, in order: parents before children. EXCEPT
	//    for afterUpdates called during the initial onMount, which are called in
	//    reverse order: children before parents.
	// Since callbacks might update component values, which could trigger another
	// call to flush(), the following steps guard against this:
	// 1. During beforeUpdate, any updated components will be added to the
	//    dirty_components array and will cause a reentrant call to flush(). Because
	//    the flush index is kept outside the function, the reentrant call will pick
	//    up where the earlier call left off and go through all dirty components. The
	//    current_component value is saved and restored so that the reentrant call will
	//    not interfere with the "parent" flush() call.
	// 2. bind:this callbacks cannot trigger new flush() calls.
	// 3. During afterUpdate, any updated components will NOT have their afterUpdate
	//    callback called a second time; the seen_callbacks set, outside the flush()
	//    function, guarantees this behavior.
	const seen_callbacks = new Set();

	let flushidx = 0; // Do *not* move this inside the flush() function

	/** @returns {void} */
	function flush() {
		// Do not reenter flush while dirty components are updated, as this can
		// result in an infinite loop. Instead, let the inner flush handle it.
		// Reentrancy is ok afterwards for bindings etc.
		if (flushidx !== 0) {
			return;
		}
		const saved_component = current_component;
		do {
			// first, call beforeUpdate functions
			// and update components
			try {
				while (flushidx < dirty_components.length) {
					const component = dirty_components[flushidx];
					flushidx++;
					set_current_component(component);
					update(component.$$);
				}
			} catch (e) {
				// reset dirty state to not end up in a deadlocked state and then rethrow
				dirty_components.length = 0;
				flushidx = 0;
				throw e;
			}
			set_current_component(null);
			dirty_components.length = 0;
			flushidx = 0;
			while (binding_callbacks.length) binding_callbacks.pop()();
			// then, once components are updated, call
			// afterUpdate functions. This may cause
			// subsequent updates...
			for (let i = 0; i < render_callbacks.length; i += 1) {
				const callback = render_callbacks[i];
				if (!seen_callbacks.has(callback)) {
					// ...so guard against infinite loops
					seen_callbacks.add(callback);
					callback();
				}
			}
			render_callbacks.length = 0;
		} while (dirty_components.length);
		while (flush_callbacks.length) {
			flush_callbacks.pop()();
		}
		update_scheduled = false;
		seen_callbacks.clear();
		set_current_component(saved_component);
	}

	/** @returns {void} */
	function update($$) {
		if ($$.fragment !== null) {
			$$.update();
			run_all($$.before_update);
			const dirty = $$.dirty;
			$$.dirty = [-1];
			$$.fragment && $$.fragment.p($$.ctx, dirty);
			$$.after_update.forEach(add_render_callback);
		}
	}

	/**
	 * Useful for example to execute remaining `afterUpdate` callbacks before executing `destroy`.
	 * @param {Function[]} fns
	 * @returns {void}
	 */
	function flush_render_callbacks(fns) {
		const filtered = [];
		const targets = [];
		render_callbacks.forEach((c) => (fns.indexOf(c) === -1 ? filtered.push(c) : targets.push(c)));
		targets.forEach((c) => c());
		render_callbacks = filtered;
	}

	const outroing = new Set();

	/**
	 * @type {Outro}
	 */
	let outros;

	/**
	 * @returns {void} */
	function group_outros() {
		outros = {
			r: 0,
			c: [],
			p: outros // parent group
		};
	}

	/**
	 * @returns {void} */
	function check_outros() {
		if (!outros.r) {
			run_all(outros.c);
		}
		outros = outros.p;
	}

	/**
	 * @param {import('./private.js').Fragment} block
	 * @param {0 | 1} [local]
	 * @returns {void}
	 */
	function transition_in(block, local) {
		if (block && block.i) {
			outroing.delete(block);
			block.i(local);
		}
	}

	/**
	 * @param {import('./private.js').Fragment} block
	 * @param {0 | 1} local
	 * @param {0 | 1} [detach]
	 * @param {() => void} [callback]
	 * @returns {void}
	 */
	function transition_out(block, local, detach, callback) {
		if (block && block.o) {
			if (outroing.has(block)) return;
			outroing.add(block);
			outros.c.push(() => {
				outroing.delete(block);
				if (callback) {
					if (detach) block.d(1);
					callback();
				}
			});
			block.o(local);
		} else if (callback) {
			callback();
		}
	}

	/** @typedef {1} INTRO */
	/** @typedef {0} OUTRO */
	/** @typedef {{ direction: 'in' | 'out' | 'both' }} TransitionOptions */
	/** @typedef {(node: Element, params: any, options: TransitionOptions) => import('../transition/public.js').TransitionConfig} TransitionFn */

	/**
	 * @typedef {Object} Outro
	 * @property {number} r
	 * @property {Function[]} c
	 * @property {Object} p
	 */

	/**
	 * @typedef {Object} PendingProgram
	 * @property {number} start
	 * @property {INTRO|OUTRO} b
	 * @property {Outro} [group]
	 */

	/**
	 * @typedef {Object} Program
	 * @property {number} a
	 * @property {INTRO|OUTRO} b
	 * @property {1|-1} d
	 * @property {number} duration
	 * @property {number} start
	 * @property {number} end
	 * @property {Outro} [group]
	 */

	// general each functions:

	function ensure_array_like(array_like_or_iterator) {
		return array_like_or_iterator?.length !== undefined
			? array_like_or_iterator
			: Array.from(array_like_or_iterator);
	}

	/** @returns {void} */
	function create_component(block) {
		block && block.c();
	}

	/** @returns {void} */
	function mount_component(component, target, anchor) {
		const { fragment, after_update } = component.$$;
		fragment && fragment.m(target, anchor);
		// onMount happens before the initial afterUpdate
		add_render_callback(() => {
			const new_on_destroy = component.$$.on_mount.map(run).filter(is_function);
			// if the component was destroyed immediately
			// it will update the `$$.on_destroy` reference to `null`.
			// the destructured on_destroy may still reference to the old array
			if (component.$$.on_destroy) {
				component.$$.on_destroy.push(...new_on_destroy);
			} else {
				// Edge case - component was destroyed immediately,
				// most likely as a result of a binding initialising
				run_all(new_on_destroy);
			}
			component.$$.on_mount = [];
		});
		after_update.forEach(add_render_callback);
	}

	/** @returns {void} */
	function destroy_component(component, detaching) {
		const $$ = component.$$;
		if ($$.fragment !== null) {
			flush_render_callbacks($$.after_update);
			run_all($$.on_destroy);
			$$.fragment && $$.fragment.d(detaching);
			// TODO null out other refs, including component.$$ (but need to
			// preserve final state?)
			$$.on_destroy = $$.fragment = null;
			$$.ctx = [];
		}
	}

	/** @returns {void} */
	function make_dirty(component, i) {
		if (component.$$.dirty[0] === -1) {
			dirty_components.push(component);
			schedule_update();
			component.$$.dirty.fill(0);
		}
		component.$$.dirty[(i / 31) | 0] |= 1 << i % 31;
	}

	/** @returns {void} */
	function init(
		component,
		options,
		instance,
		create_fragment,
		not_equal,
		props,
		append_styles,
		dirty = [-1]
	) {
		const parent_component = current_component;
		set_current_component(component);
		/** @type {import('./private.js').T$$} */
		const $$ = (component.$$ = {
			fragment: null,
			ctx: [],
			// state
			props,
			update: noop,
			not_equal,
			bound: blank_object(),
			// lifecycle
			on_mount: [],
			on_destroy: [],
			on_disconnect: [],
			before_update: [],
			after_update: [],
			context: new Map(options.context || (parent_component ? parent_component.$$.context : [])),
			// everything else
			callbacks: blank_object(),
			dirty,
			skip_bound: false,
			root: options.target || parent_component.$$.root
		});
		append_styles && append_styles($$.root);
		let ready = false;
		$$.ctx = instance
			? instance(component, options.props || {}, (i, ret, ...rest) => {
					const value = rest.length ? rest[0] : ret;
					if ($$.ctx && not_equal($$.ctx[i], ($$.ctx[i] = value))) {
						if (!$$.skip_bound && $$.bound[i]) $$.bound[i](value);
						if (ready) make_dirty(component, i);
					}
					return ret;
			  })
			: [];
		$$.update();
		ready = true;
		run_all($$.before_update);
		// `false` as a special case of no DOM component
		$$.fragment = create_fragment ? create_fragment($$.ctx) : false;
		if (options.target) {
			if (options.hydrate) {
				const nodes = children(options.target);
				// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
				$$.fragment && $$.fragment.l(nodes);
				nodes.forEach(detach);
			} else {
				// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
				$$.fragment && $$.fragment.c();
			}
			if (options.intro) transition_in(component.$$.fragment);
			mount_component(component, options.target, options.anchor);
			flush();
		}
		set_current_component(parent_component);
	}

	/**
	 * Base class for Svelte components. Used when dev=false.
	 *
	 * @template {Record<string, any>} [Props=any]
	 * @template {Record<string, any>} [Events=any]
	 */
	class SvelteComponent {
		/**
		 * ### PRIVATE API
		 *
		 * Do not use, may change at any time
		 *
		 * @type {any}
		 */
		$$ = undefined;
		/**
		 * ### PRIVATE API
		 *
		 * Do not use, may change at any time
		 *
		 * @type {any}
		 */
		$$set = undefined;

		/** @returns {void} */
		$destroy() {
			destroy_component(this, 1);
			this.$destroy = noop;
		}

		/**
		 * @template {Extract<keyof Events, string>} K
		 * @param {K} type
		 * @param {((e: Events[K]) => void) | null | undefined} callback
		 * @returns {() => void}
		 */
		$on(type, callback) {
			if (!is_function(callback)) {
				return noop;
			}
			const callbacks = this.$$.callbacks[type] || (this.$$.callbacks[type] = []);
			callbacks.push(callback);
			return () => {
				const index = callbacks.indexOf(callback);
				if (index !== -1) callbacks.splice(index, 1);
			};
		}

		/**
		 * @param {Partial<Props>} props
		 * @returns {void}
		 */
		$set(props) {
			if (this.$$set && !is_empty(props)) {
				this.$$.skip_bound = true;
				this.$$set(props);
				this.$$.skip_bound = false;
			}
		}
	}

	/**
	 * @typedef {Object} CustomElementPropDefinition
	 * @property {string} [attribute]
	 * @property {boolean} [reflect]
	 * @property {'String'|'Boolean'|'Number'|'Array'|'Object'} [type]
	 */

	// generated during release, do not modify

	const PUBLIC_VERSION = '4';

	if (typeof window !== 'undefined')
		// @ts-ignore
		(window.__svelte || (window.__svelte = { v: new Set() })).v.add(PUBLIC_VERSION);

	var d$1=Object.defineProperty;var e=(c,a)=>{for(var b in a)d$1(c,b,{get:a[b],enumerable:!0});};

	var w={};e(w,{convertFileSrc:()=>u$1,invoke:()=>d,transformCallback:()=>s$1});function l(){return window.crypto.getRandomValues(new Uint32Array(1))[0]}function s$1(r,n=!1){let e=l(),t=`_${e}`;return Object.defineProperty(window,t,{value:o=>(n&&Reflect.deleteProperty(window,t),r?.(o)),writable:!1,configurable:!0}),e}async function d(r,n={}){return new Promise((e,t)=>{let o=s$1(i=>{e(i),Reflect.deleteProperty(window,`_${a}`);},!0),a=s$1(i=>{t(i),Reflect.deleteProperty(window,`_${o}`);},!0);window.__TAURI_IPC__({cmd:r,callback:o,error:a,...n});})}function u$1(r,n="asset"){let e=encodeURIComponent(r);return navigator.userAgent.includes("Windows")?`https://${n}.localhost/${e}`:`${n}://localhost/${e}`}

	async function a$1(i){return d("tauri",i)}

	var W={};e(W,{TauriEvent:()=>c,emit:()=>D,listen:()=>E,once:()=>_});async function s(n,t){return a$1({__tauriModule:"Event",message:{cmd:"unlisten",event:n,eventId:t}})}async function m(n,t,r){await a$1({__tauriModule:"Event",message:{cmd:"emit",event:n,windowLabel:t,payload:r}});}async function a(n,t,r){return a$1({__tauriModule:"Event",message:{cmd:"listen",event:n,windowLabel:t,handler:s$1(r)}}).then(i=>async()=>s(n,i))}async function u(n,t,r){return a(n,t,i=>{r(i),s(n,i.id).catch(()=>{});})}var c=(e=>(e.WINDOW_RESIZED="tauri://resize",e.WINDOW_MOVED="tauri://move",e.WINDOW_CLOSE_REQUESTED="tauri://close-requested",e.WINDOW_CREATED="tauri://window-created",e.WINDOW_DESTROYED="tauri://destroyed",e.WINDOW_FOCUS="tauri://focus",e.WINDOW_BLUR="tauri://blur",e.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",e.WINDOW_THEME_CHANGED="tauri://theme-changed",e.WINDOW_FILE_DROP="tauri://file-drop",e.WINDOW_FILE_DROP_HOVER="tauri://file-drop-hover",e.WINDOW_FILE_DROP_CANCELLED="tauri://file-drop-cancelled",e.MENU="tauri://menu",e.CHECK_UPDATE="tauri://update",e.UPDATE_AVAILABLE="tauri://update-available",e.INSTALL_UPDATE="tauri://update-install",e.STATUS_UPDATE="tauri://update-status",e.DOWNLOAD_PROGRESS="tauri://update-download-progress",e))(c||{});async function E(n,t){return a(n,null,t)}async function _(n,t){return u(n,null,t)}async function D(n,t){return m(n,void 0,t)}

	/* src/QuickLookup/ButtonSet.svelte generated by Svelte v4.2.0 */

	function get_each_context(ctx, list, i) {
		const child_ctx = ctx.slice();
		child_ctx[7] = list[i].points;
		child_ctx[2] = list[i].labels;
		child_ctx[9] = i;
		return child_ctx;
	}

	function get_each_context_1(ctx, list, i) {
		const child_ctx = ctx.slice();
		child_ctx[10] = list[i].x;
		child_ctx[11] = list[i].y;
		child_ctx[12] = list[i].label;
		return child_ctx;
	}

	// (104:2) {#each labels as {x,y,label}}
	function create_each_block_1(ctx) {
		let text_1;
		let t_value = /*label*/ ctx[12] + "";
		let t;

		return {
			c() {
				text_1 = svg_element("text");
				t = text(t_value);
				attr(text_1, "x", /*x*/ ctx[10] + "");
				attr(text_1, "y", /*y*/ ctx[11] + "");
				attr(text_1, "fill", "black");
			},
			m(target, anchor) {
				insert(target, text_1, anchor);
				append(text_1, t);
			},
			p: noop,
			d(detaching) {
				if (detaching) {
					detach(text_1);
				}
			}
		};
	}

	// (97:0) {#each polygons as {points, labels}
	function create_each_block(ctx) {
		let polygon;
		let polygon_fill_value;
		let polygon_stroke_value;
		let each_1_anchor;
		let each_value_1 = ensure_array_like(/*labels*/ ctx[2]);
		let each_blocks = [];

		for (let i = 0; i < each_value_1.length; i += 1) {
			each_blocks[i] = create_each_block_1(get_each_context_1(ctx, each_value_1, i));
		}

		return {
			c() {
				polygon = svg_element("polygon");

				for (let i = 0; i < each_blocks.length; i += 1) {
					each_blocks[i].c();
				}

				each_1_anchor = empty();
				attr(polygon, "points", /*points*/ ctx[7]);

				attr(polygon, "fill", polygon_fill_value = /*layerIsActive*/ ctx[1] && stepIsActive(/*activeStepNum*/ ctx[0], /*index*/ ctx[9])
				? "none"
				: "lightgrey");

				attr(polygon, "stroke", polygon_stroke_value = /*layerIsActive*/ ctx[1] && stepIsActive(/*activeStepNum*/ ctx[0], /*index*/ ctx[9])
				? "blue"
				: "grey");
			},
			m(target, anchor) {
				insert(target, polygon, anchor);

				for (let i = 0; i < each_blocks.length; i += 1) {
					if (each_blocks[i]) {
						each_blocks[i].m(target, anchor);
					}
				}

				insert(target, each_1_anchor, anchor);
			},
			p(ctx, dirty) {
				if (dirty & /*layerIsActive, activeStepNum*/ 3 && polygon_fill_value !== (polygon_fill_value = /*layerIsActive*/ ctx[1] && stepIsActive(/*activeStepNum*/ ctx[0], /*index*/ ctx[9])
				? "none"
				: "lightgrey")) {
					attr(polygon, "fill", polygon_fill_value);
				}

				if (dirty & /*layerIsActive, activeStepNum*/ 3 && polygon_stroke_value !== (polygon_stroke_value = /*layerIsActive*/ ctx[1] && stepIsActive(/*activeStepNum*/ ctx[0], /*index*/ ctx[9])
				? "blue"
				: "grey")) {
					attr(polygon, "stroke", polygon_stroke_value);
				}

				if (dirty & /*polygons*/ 8) {
					each_value_1 = ensure_array_like(/*labels*/ ctx[2]);
					let i;

					for (i = 0; i < each_value_1.length; i += 1) {
						const child_ctx = get_each_context_1(ctx, each_value_1, i);

						if (each_blocks[i]) {
							each_blocks[i].p(child_ctx, dirty);
						} else {
							each_blocks[i] = create_each_block_1(child_ctx);
							each_blocks[i].c();
							each_blocks[i].m(each_1_anchor.parentNode, each_1_anchor);
						}
					}

					for (; i < each_blocks.length; i += 1) {
						each_blocks[i].d(1);
					}

					each_blocks.length = each_value_1.length;
				}
			},
			d(detaching) {
				if (detaching) {
					detach(polygon);
					detach(each_1_anchor);
				}

				destroy_each(each_blocks, detaching);
			}
		};
	}

	function create_fragment$4(ctx) {
		let svg;
		let each_value = ensure_array_like(/*polygons*/ ctx[3]);
		let each_blocks = [];

		for (let i = 0; i < each_value.length; i += 1) {
			each_blocks[i] = create_each_block(get_each_context(ctx, each_value, i));
		}

		return {
			c() {
				svg = svg_element("svg");

				for (let i = 0; i < each_blocks.length; i += 1) {
					each_blocks[i].c();
				}

				attr(svg, "viewBox", "0 0 260 170");
				attr(svg, "width", "260px");
				attr(svg, "height", "170px");
				attr(svg, "xmlns", "http://www.w3.org/2000/svg");
			},
			m(target, anchor) {
				insert(target, svg, anchor);

				for (let i = 0; i < each_blocks.length; i += 1) {
					if (each_blocks[i]) {
						each_blocks[i].m(svg, null);
					}
				}
			},
			p(ctx, [dirty]) {
				if (dirty & /*polygons, layerIsActive, stepIsActive, activeStepNum*/ 11) {
					each_value = ensure_array_like(/*polygons*/ ctx[3]);
					let i;

					for (i = 0; i < each_value.length; i += 1) {
						const child_ctx = get_each_context(ctx, each_value, i);

						if (each_blocks[i]) {
							each_blocks[i].p(child_ctx, dirty);
						} else {
							each_blocks[i] = create_each_block(child_ctx);
							each_blocks[i].c();
							each_blocks[i].m(svg, null);
						}
					}

					for (; i < each_blocks.length; i += 1) {
						each_blocks[i].d(1);
					}

					each_blocks.length = each_value.length;
				}
			},
			i: noop,
			o: noop,
			d(detaching) {
				if (detaching) {
					detach(svg);
				}

				destroy_each(each_blocks, detaching);
			}
		};
	}

	const SQ_LENGTH$2 = 20;

	function buildCrossPoints(startX, startY, sqLength, labels) {
		const pointsSrc = [
			[startX, startY],
			[startX + sqLength, startY],
			[startX + sqLength, startY + sqLength],
			[startX + 2 * sqLength, startY + sqLength],
			[startX + 2 * sqLength, startY + 2 * sqLength],
			[startX + sqLength, startY + 2 * sqLength],
			[startX + sqLength, startY + 3 * sqLength],
			[startX, startY + 3 * sqLength],
			[startX, startY + 2 * sqLength],
			[startX - sqLength, startY + 2 * sqLength],
			[startX - sqLength, startY + sqLength],
			[startX, startY + sqLength]
		];

		let availableSpaceOnOneSideOfLabel = 4;

		const labelsSrc = [
			{
				x: pointsSrc[pointsSrc.length - 1][0] + availableSpaceOnOneSideOfLabel,
				y: pointsSrc[pointsSrc.length - 1][1] - availableSpaceOnOneSideOfLabel,
				label: labels[0]
			},
			{
				x: pointsSrc[5][0] + availableSpaceOnOneSideOfLabel,
				y: pointsSrc[5][1] - availableSpaceOnOneSideOfLabel,
				label: labels[1]
			},
			{
				x: pointsSrc[7][0] + availableSpaceOnOneSideOfLabel,
				y: pointsSrc[7][1] - availableSpaceOnOneSideOfLabel,
				label: labels[2]
			},
			{
				x: pointsSrc[9][0] + availableSpaceOnOneSideOfLabel,
				y: pointsSrc[9][1] - availableSpaceOnOneSideOfLabel,
				label: labels[3]
			}
		];

		return {
			points: pointsSrc.map(point => {
				return point.join(",");
			}).join(" "),
			labels: labelsSrc
		};
	}

	function stepIsActive(activeStepNum, index) {
		if (activeStepNum == 1 && [0, 1].includes(index)) {
			return true;
		} else if (activeStepNum == 2 && [2, 3].includes(index)) {
			return true;
		} else if (activeStepNum == 3 && [4, 5].includes(index)) {
			return true;
		} else if (activeStepNum == 4 && [6, 7].includes(index)) {
			return true;
		} else return false;
	}

	function instance$4($$self, $$props, $$invalidate) {
		let { activeStepNum = 1 } = $$props;
		let { layerIsActive = false } = $$props;
		let { labels = [] } = $$props;
		let { isLeftButtonSet = true } = $$props;
		let { doubleCrossCoordinates = [] } = $$props;

		function buildDoubleCross(startX, startY, labels) {
			return [
				buildCrossPoints(isLeftButtonSet ? startX : startX + 2 * SQ_LENGTH$2, startY, SQ_LENGTH$2, labels.slice(4, 8)),
				buildCrossPoints(isLeftButtonSet ? startX + 2 * SQ_LENGTH$2 : startX, startY + SQ_LENGTH$2, SQ_LENGTH$2, labels.slice(0, 4))
			];
		}

		let polygons = [
			...buildDoubleCross(doubleCrossCoordinates[0].x, doubleCrossCoordinates[0].y, labels.slice(0, 8)),
			...buildDoubleCross(doubleCrossCoordinates[1].x, doubleCrossCoordinates[1].y, labels.slice(8, 16)),
			...buildDoubleCross(doubleCrossCoordinates[2].x, doubleCrossCoordinates[2].y, labels.slice(16, 24)),
			...buildDoubleCross(doubleCrossCoordinates[3].x, doubleCrossCoordinates[3].y, labels.slice(24, 32))
		];

		$$self.$$set = $$props => {
			if ('activeStepNum' in $$props) $$invalidate(0, activeStepNum = $$props.activeStepNum);
			if ('layerIsActive' in $$props) $$invalidate(1, layerIsActive = $$props.layerIsActive);
			if ('labels' in $$props) $$invalidate(2, labels = $$props.labels);
			if ('isLeftButtonSet' in $$props) $$invalidate(4, isLeftButtonSet = $$props.isLeftButtonSet);
			if ('doubleCrossCoordinates' in $$props) $$invalidate(5, doubleCrossCoordinates = $$props.doubleCrossCoordinates);
		};

		return [
			activeStepNum,
			layerIsActive,
			labels,
			polygons,
			isLeftButtonSet,
			doubleCrossCoordinates
		];
	}

	class ButtonSet extends SvelteComponent {
		constructor(options) {
			super();

			init(this, options, instance$4, create_fragment$4, safe_not_equal, {
				activeStepNum: 0,
				layerIsActive: 1,
				labels: 2,
				isLeftButtonSet: 4,
				doubleCrossCoordinates: 5
			});
		}
	}

	/* src/QuickLookup/LeftButtonsSet.svelte generated by Svelte v4.2.0 */

	function create_fragment$3(ctx) {
		let buttonset;
		let current;

		buttonset = new ButtonSet({
				props: {
					activeStepNum: /*activeStepNum*/ ctx[0],
					layerIsActive: /*layerIsActive*/ ctx[1],
					labels: /*labels*/ ctx[2],
					isLeftButtonSet: true,
					doubleCrossCoordinates: /*doubleCrossCoordinates*/ ctx[3]
				}
			});

		return {
			c() {
				create_component(buttonset.$$.fragment);
			},
			m(target, anchor) {
				mount_component(buttonset, target, anchor);
				current = true;
			},
			p(ctx, [dirty]) {
				const buttonset_changes = {};
				if (dirty & /*activeStepNum*/ 1) buttonset_changes.activeStepNum = /*activeStepNum*/ ctx[0];
				if (dirty & /*layerIsActive*/ 2) buttonset_changes.layerIsActive = /*layerIsActive*/ ctx[1];
				if (dirty & /*labels*/ 4) buttonset_changes.labels = /*labels*/ ctx[2];
				buttonset.$set(buttonset_changes);
			},
			i(local) {
				if (current) return;
				transition_in(buttonset.$$.fragment, local);
				current = true;
			},
			o(local) {
				transition_out(buttonset.$$.fragment, local);
				current = false;
			},
			d(detaching) {
				destroy_component(buttonset, detaching);
			}
		};
	}

	const SQ_LENGTH$1 = 20;
	const SPACING$1 = 10;

	function instance$3($$self, $$props, $$invalidate) {
		let { activeStepNum = 1 } = $$props;
		let { layerIsActive = false } = $$props;
		let { labels = [] } = $$props;

		const doubleCrossCoordinates = [
			{ x: 4 * SQ_LENGTH$1 + SPACING$1, y: 0 },
			{
				x: 8 * SQ_LENGTH$1 + 1.5 * SPACING$1,
				y: 2 * SQ_LENGTH$1 + SPACING$1 / 2
			},
			{ x: SQ_LENGTH$1, y: SQ_LENGTH$1 + SPACING$1 / 2 },
			{
				x: 5 * SQ_LENGTH$1 + SPACING$1 / 2,
				y: 3 * SQ_LENGTH$1 + SPACING$1
			}
		];

		$$self.$$set = $$props => {
			if ('activeStepNum' in $$props) $$invalidate(0, activeStepNum = $$props.activeStepNum);
			if ('layerIsActive' in $$props) $$invalidate(1, layerIsActive = $$props.layerIsActive);
			if ('labels' in $$props) $$invalidate(2, labels = $$props.labels);
		};

		return [activeStepNum, layerIsActive, labels, doubleCrossCoordinates];
	}

	class LeftButtonsSet extends SvelteComponent {
		constructor(options) {
			super();

			init(this, options, instance$3, create_fragment$3, safe_not_equal, {
				activeStepNum: 0,
				layerIsActive: 1,
				labels: 2
			});
		}
	}

	/* src/QuickLookup/RightButtonsSet.svelte generated by Svelte v4.2.0 */

	function create_fragment$2(ctx) {
		let buttonset;
		let current;

		buttonset = new ButtonSet({
				props: {
					activeStepNum: /*activeStepNum*/ ctx[0],
					layerIsActive: /*layerIsActive*/ ctx[1],
					labels: /*labels*/ ctx[2],
					isLeftButtonSet: false,
					doubleCrossCoordinates: /*doubleCrossCoordinates*/ ctx[3]
				}
			});

		return {
			c() {
				create_component(buttonset.$$.fragment);
			},
			m(target, anchor) {
				mount_component(buttonset, target, anchor);
				current = true;
			},
			p(ctx, [dirty]) {
				const buttonset_changes = {};
				if (dirty & /*activeStepNum*/ 1) buttonset_changes.activeStepNum = /*activeStepNum*/ ctx[0];
				if (dirty & /*layerIsActive*/ 2) buttonset_changes.layerIsActive = /*layerIsActive*/ ctx[1];
				if (dirty & /*labels*/ 4) buttonset_changes.labels = /*labels*/ ctx[2];
				buttonset.$set(buttonset_changes);
			},
			i(local) {
				if (current) return;
				transition_in(buttonset.$$.fragment, local);
				current = true;
			},
			o(local) {
				transition_out(buttonset.$$.fragment, local);
				current = false;
			},
			d(detaching) {
				destroy_component(buttonset, detaching);
			}
		};
	}

	const SQ_LENGTH = 20;
	const SPACING = 10;

	function instance$2($$self, $$props, $$invalidate) {
		let { activeStepNum = 1 } = $$props;
		let { layerIsActive = false } = $$props;
		let { labels = [] } = $$props;

		const doubleCrossCoordinates = [
			{ x: 5 * SQ_LENGTH + SPACING / 2, y: 0 },
			{
				x: 8 * SQ_LENGTH + 1.5 * SPACING,
				y: SQ_LENGTH + SPACING / 2
			},
			{
				x: SQ_LENGTH,
				y: 2 * SQ_LENGTH + SPACING / 2
			},
			{
				x: 4 * SQ_LENGTH + SPACING,
				y: 3 * SQ_LENGTH + SPACING
			}
		];

		$$self.$$set = $$props => {
			if ('activeStepNum' in $$props) $$invalidate(0, activeStepNum = $$props.activeStepNum);
			if ('layerIsActive' in $$props) $$invalidate(1, layerIsActive = $$props.layerIsActive);
			if ('labels' in $$props) $$invalidate(2, labels = $$props.labels);
		};

		return [activeStepNum, layerIsActive, labels, doubleCrossCoordinates];
	}

	class RightButtonsSet extends SvelteComponent {
		constructor(options) {
			super();

			init(this, options, instance$2, create_fragment$2, safe_not_equal, {
				activeStepNum: 0,
				layerIsActive: 1,
				labels: 2
			});
		}
	}

	/* src/QuickLookup/Keyboard.svelte generated by Svelte v4.2.0 */

	function create_key_block_3(ctx) {
		let leftbuttonsset;
		let current;

		leftbuttonsset = new LeftButtonsSet({
				props: {
					layerIsActive: /*layerNum*/ ctx[0] == 1,
					activeStepNum: /*stepNum*/ ctx[1],
					labels: /*layer_1_left_labels*/ ctx[2]
				}
			});

		return {
			c() {
				create_component(leftbuttonsset.$$.fragment);
			},
			m(target, anchor) {
				mount_component(leftbuttonsset, target, anchor);
				current = true;
			},
			p(ctx, dirty) {
				const leftbuttonsset_changes = {};
				if (dirty & /*layerNum*/ 1) leftbuttonsset_changes.layerIsActive = /*layerNum*/ ctx[0] == 1;
				if (dirty & /*stepNum*/ 2) leftbuttonsset_changes.activeStepNum = /*stepNum*/ ctx[1];
				if (dirty & /*layer_1_left_labels*/ 4) leftbuttonsset_changes.labels = /*layer_1_left_labels*/ ctx[2];
				leftbuttonsset.$set(leftbuttonsset_changes);
			},
			i(local) {
				if (current) return;
				transition_in(leftbuttonsset.$$.fragment, local);
				current = true;
			},
			o(local) {
				transition_out(leftbuttonsset.$$.fragment, local);
				current = false;
			},
			d(detaching) {
				destroy_component(leftbuttonsset, detaching);
			}
		};
	}

	// (20:2) {#key layer_1_right_labels}
	function create_key_block_2(ctx) {
		let rightbuttonsset;
		let current;

		rightbuttonsset = new RightButtonsSet({
				props: {
					layerIsActive: /*layerNum*/ ctx[0] == 1,
					activeStepNum: /*stepNum*/ ctx[1],
					labels: /*layer_1_right_labels*/ ctx[3]
				}
			});

		return {
			c() {
				create_component(rightbuttonsset.$$.fragment);
			},
			m(target, anchor) {
				mount_component(rightbuttonsset, target, anchor);
				current = true;
			},
			p(ctx, dirty) {
				const rightbuttonsset_changes = {};
				if (dirty & /*layerNum*/ 1) rightbuttonsset_changes.layerIsActive = /*layerNum*/ ctx[0] == 1;
				if (dirty & /*stepNum*/ 2) rightbuttonsset_changes.activeStepNum = /*stepNum*/ ctx[1];
				if (dirty & /*layer_1_right_labels*/ 8) rightbuttonsset_changes.labels = /*layer_1_right_labels*/ ctx[3];
				rightbuttonsset.$set(rightbuttonsset_changes);
			},
			i(local) {
				if (current) return;
				transition_in(rightbuttonsset.$$.fragment, local);
				current = true;
			},
			o(local) {
				transition_out(rightbuttonsset.$$.fragment, local);
				current = false;
			},
			d(detaching) {
				destroy_component(rightbuttonsset, detaching);
			}
		};
	}

	// (29:2) {#key layer_1_right_labels}
	function create_key_block_1(ctx) {
		let leftbuttonsset;
		let current;

		leftbuttonsset = new LeftButtonsSet({
				props: {
					layerIsActive: /*layerNum*/ ctx[0] == 2,
					activeStepNum: /*stepNum*/ ctx[1],
					labels: /*layer_2_left_labels*/ ctx[4]
				}
			});

		return {
			c() {
				create_component(leftbuttonsset.$$.fragment);
			},
			m(target, anchor) {
				mount_component(leftbuttonsset, target, anchor);
				current = true;
			},
			p(ctx, dirty) {
				const leftbuttonsset_changes = {};
				if (dirty & /*layerNum*/ 1) leftbuttonsset_changes.layerIsActive = /*layerNum*/ ctx[0] == 2;
				if (dirty & /*stepNum*/ 2) leftbuttonsset_changes.activeStepNum = /*stepNum*/ ctx[1];
				if (dirty & /*layer_2_left_labels*/ 16) leftbuttonsset_changes.labels = /*layer_2_left_labels*/ ctx[4];
				leftbuttonsset.$set(leftbuttonsset_changes);
			},
			i(local) {
				if (current) return;
				transition_in(leftbuttonsset.$$.fragment, local);
				current = true;
			},
			o(local) {
				transition_out(leftbuttonsset.$$.fragment, local);
				current = false;
			},
			d(detaching) {
				destroy_component(leftbuttonsset, detaching);
			}
		};
	}

	// (36:2) {#key layer_1_right_labels}
	function create_key_block(ctx) {
		let rightbuttonsset;
		let current;

		rightbuttonsset = new RightButtonsSet({
				props: {
					layerIsActive: /*layerNum*/ ctx[0] == 2,
					activeStepNum: /*stepNum*/ ctx[1],
					labels: /*layer_2_right_labels*/ ctx[5]
				}
			});

		return {
			c() {
				create_component(rightbuttonsset.$$.fragment);
			},
			m(target, anchor) {
				mount_component(rightbuttonsset, target, anchor);
				current = true;
			},
			p(ctx, dirty) {
				const rightbuttonsset_changes = {};
				if (dirty & /*layerNum*/ 1) rightbuttonsset_changes.layerIsActive = /*layerNum*/ ctx[0] == 2;
				if (dirty & /*stepNum*/ 2) rightbuttonsset_changes.activeStepNum = /*stepNum*/ ctx[1];
				if (dirty & /*layer_2_right_labels*/ 32) rightbuttonsset_changes.labels = /*layer_2_right_labels*/ ctx[5];
				rightbuttonsset.$set(rightbuttonsset_changes);
			},
			i(local) {
				if (current) return;
				transition_in(rightbuttonsset.$$.fragment, local);
				current = true;
			},
			o(local) {
				transition_out(rightbuttonsset.$$.fragment, local);
				current = false;
			},
			d(detaching) {
				destroy_component(rightbuttonsset, detaching);
			}
		};
	}

	function create_fragment$1(ctx) {
		let div0;
		let previous_key = /*layer_1_left_labels*/ ctx[2];
		let t0;
		let previous_key_1 = /*layer_1_right_labels*/ ctx[3];
		let t1;
		let div1;
		let previous_key_2 = /*layer_1_right_labels*/ ctx[3];
		let t2;
		let previous_key_3 = /*layer_1_right_labels*/ ctx[3];
		let current;
		let key_block0 = create_key_block_3(ctx);
		let key_block1 = create_key_block_2(ctx);
		let key_block2 = create_key_block_1(ctx);
		let key_block3 = create_key_block(ctx);

		return {
			c() {
				div0 = element("div");
				key_block0.c();
				t0 = space();
				key_block1.c();
				t1 = space();
				div1 = element("div");
				key_block2.c();
				t2 = space();
				key_block3.c();
				attr(div0, "class", "flex svelte-1mu3a6k");
				attr(div1, "class", "flex svelte-1mu3a6k");
			},
			m(target, anchor) {
				insert(target, div0, anchor);
				key_block0.m(div0, null);
				append(div0, t0);
				key_block1.m(div0, null);
				insert(target, t1, anchor);
				insert(target, div1, anchor);
				key_block2.m(div1, null);
				append(div1, t2);
				key_block3.m(div1, null);
				current = true;
			},
			p(ctx, [dirty]) {
				if (dirty & /*layer_1_left_labels*/ 4 && safe_not_equal(previous_key, previous_key = /*layer_1_left_labels*/ ctx[2])) {
					group_outros();
					transition_out(key_block0, 1, 1, noop);
					check_outros();
					key_block0 = create_key_block_3(ctx);
					key_block0.c();
					transition_in(key_block0, 1);
					key_block0.m(div0, t0);
				} else {
					key_block0.p(ctx, dirty);
				}

				if (dirty & /*layer_1_right_labels*/ 8 && safe_not_equal(previous_key_1, previous_key_1 = /*layer_1_right_labels*/ ctx[3])) {
					group_outros();
					transition_out(key_block1, 1, 1, noop);
					check_outros();
					key_block1 = create_key_block_2(ctx);
					key_block1.c();
					transition_in(key_block1, 1);
					key_block1.m(div0, null);
				} else {
					key_block1.p(ctx, dirty);
				}

				if (dirty & /*layer_1_right_labels*/ 8 && safe_not_equal(previous_key_2, previous_key_2 = /*layer_1_right_labels*/ ctx[3])) {
					group_outros();
					transition_out(key_block2, 1, 1, noop);
					check_outros();
					key_block2 = create_key_block_1(ctx);
					key_block2.c();
					transition_in(key_block2, 1);
					key_block2.m(div1, t2);
				} else {
					key_block2.p(ctx, dirty);
				}

				if (dirty & /*layer_1_right_labels*/ 8 && safe_not_equal(previous_key_3, previous_key_3 = /*layer_1_right_labels*/ ctx[3])) {
					group_outros();
					transition_out(key_block3, 1, 1, noop);
					check_outros();
					key_block3 = create_key_block(ctx);
					key_block3.c();
					transition_in(key_block3, 1);
					key_block3.m(div1, null);
				} else {
					key_block3.p(ctx, dirty);
				}
			},
			i(local) {
				if (current) return;
				transition_in(key_block0);
				transition_in(key_block1);
				transition_in(key_block2);
				transition_in(key_block3);
				current = true;
			},
			o(local) {
				transition_out(key_block0);
				transition_out(key_block1);
				transition_out(key_block2);
				transition_out(key_block3);
				current = false;
			},
			d(detaching) {
				if (detaching) {
					detach(div0);
					detach(t1);
					detach(div1);
				}

				key_block0.d(detaching);
				key_block1.d(detaching);
				key_block2.d(detaching);
				key_block3.d(detaching);
			}
		};
	}

	function instance$1($$self, $$props, $$invalidate) {
		let { layerNum = 1 } = $$props;
		let { stepNum = 1 } = $$props;
		let { layer_1_left_labels = [] } = $$props;
		let { layer_1_right_labels = [] } = $$props;
		let { layer_2_left_labels = [] } = $$props;
		let { layer_2_right_labels = [] } = $$props;

		$$self.$$set = $$props => {
			if ('layerNum' in $$props) $$invalidate(0, layerNum = $$props.layerNum);
			if ('stepNum' in $$props) $$invalidate(1, stepNum = $$props.stepNum);
			if ('layer_1_left_labels' in $$props) $$invalidate(2, layer_1_left_labels = $$props.layer_1_left_labels);
			if ('layer_1_right_labels' in $$props) $$invalidate(3, layer_1_right_labels = $$props.layer_1_right_labels);
			if ('layer_2_left_labels' in $$props) $$invalidate(4, layer_2_left_labels = $$props.layer_2_left_labels);
			if ('layer_2_right_labels' in $$props) $$invalidate(5, layer_2_right_labels = $$props.layer_2_right_labels);
		};

		return [
			layerNum,
			stepNum,
			layer_1_left_labels,
			layer_1_right_labels,
			layer_2_left_labels,
			layer_2_right_labels
		];
	}

	class Keyboard extends SvelteComponent {
		constructor(options) {
			super();

			init(this, options, instance$1, create_fragment$1, safe_not_equal, {
				layerNum: 0,
				stepNum: 1,
				layer_1_left_labels: 2,
				layer_1_right_labels: 3,
				layer_2_left_labels: 4,
				layer_2_right_labels: 5
			});
		}
	}

	/* src/App.svelte generated by Svelte v4.2.0 */

	function create_fragment(ctx) {
		let keyboard;
		let current;

		keyboard = new Keyboard({
				props: {
					layerNum: /*layerNum*/ ctx[1],
					stepNum: /*stepNum*/ ctx[0],
					layer_1_left_labels: /*layer_1_left_labels*/ ctx[2],
					layer_1_right_labels: /*layer_1_right_labels*/ ctx[3],
					layer_2_left_labels: /*layer_2_left_labels*/ ctx[4],
					layer_2_right_labels: /*layer_2_right_labels*/ ctx[5]
				}
			});

		return {
			c() {
				create_component(keyboard.$$.fragment);
			},
			m(target, anchor) {
				mount_component(keyboard, target, anchor);
				current = true;
			},
			p(ctx, [dirty]) {
				const keyboard_changes = {};
				if (dirty & /*layerNum*/ 2) keyboard_changes.layerNum = /*layerNum*/ ctx[1];
				if (dirty & /*stepNum*/ 1) keyboard_changes.stepNum = /*stepNum*/ ctx[0];
				keyboard.$set(keyboard_changes);
			},
			i(local) {
				if (current) return;
				transition_in(keyboard.$$.fragment, local);
				current = true;
			},
			o(local) {
				transition_out(keyboard.$$.fragment, local);
				current = false;
			},
			d(detaching) {
				destroy_component(keyboard, detaching);
			}
		};
	}

	function instance($$self, $$props, $$invalidate) {
		let stepNum = 1;
		let layerNum = 1;

		E('update-keyboard', event => {
			$$invalidate(0, stepNum = event.payload.step);
			$$invalidate(1, layerNum = event.payload.layer);
		}).then();

		let layer_1_left_labels = [
			"t",
			"o",
			"n",
			"⌫",
			"T",
			"O",
			"N",
			"",
			"l",
			"c",
			"m",
			"r",
			"L",
			"C",
			"M",
			"R",
			"w",
			"b",
			"k",
			"p",
			"W",
			"B",
			"K",
			"P",
			"z",
			"",
			"",
			"j",
			"Z",
			"",
			"",
			"J"
		];

		let layer_1_right_labels = [
			"e",
			"➟",
			"i",
			"a",
			"E",
			"↵",
			"I",
			"A",
			"h",
			"s",
			"u",
			"d",
			"H",
			"S",
			"U",
			"D",
			"g",
			"f",
			"v",
			"y",
			"G",
			"F",
			"V",
			"Y",
			"q",
			"x",
			"",
			"",
			"Q",
			"X",
			"",
			""
		];

		let layer_2_left_labels = [
			"1",
			"2",
			"3",
			"4",
			"!",
			"@",
			"#",
			"$",
			"9",
			"0",
			".",
			",",
			"(",
			")",
			">",
			"<",
			"-",
			"/",
			"=",
			"`",
			"_",
			"?",
			"+",
			"~",
			"",
			"",
			"",
			"",
			"",
			"",
			"",
			""
		];

		let layer_2_right_labels = [
			"5",
			"6",
			"7",
			"8",
			"%",
			"^",
			"&",
			"*",
			"'",
			";",
			"]",
			"[",
			"\"",
			":",
			"}",
			"{",
			"⇥",
			"\\",
			"",
			"",
			"⇤",
			"|",
			"",
			"",
			"",
			"",
			"",
			"",
			"",
			"",
			"",
			""
		];

		return [
			stepNum,
			layerNum,
			layer_1_left_labels,
			layer_1_right_labels,
			layer_2_left_labels,
			layer_2_right_labels
		];
	}

	class App extends SvelteComponent {
		constructor(options) {
			super();
			init(this, options, instance, create_fragment, safe_not_equal, {});
		}
	}

	const app = new App({
	  target: document.body,
	  props: {
	    name: 'Daffodil',
	  },
	});

	return app;

})();
});
