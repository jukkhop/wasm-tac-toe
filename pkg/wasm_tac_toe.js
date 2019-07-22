import * as wasm from './wasm_tac_toe_bg.wasm';

function __wbg_elem_binding0(arg0, arg1) {
    wasm.__wbg_function_table.get(5)(arg0, arg1);
}
/**
*/
export function main() {
    wasm.main();
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm(arg0, arg1);
    return addHeapObject(ret);
};

export const __wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

export const __widl_instanceof_Window = function(arg0) {
    const ret = getObject(arg0) instanceof Window;
    return ret;
};

export const __widl_instanceof_CanvasRenderingContext2D = function(arg0) {
    const ret = getObject(arg0) instanceof CanvasRenderingContext2D;
    return ret;
};

export const __widl_f_begin_path_CanvasRenderingContext2D = function(arg0) {
    getObject(arg0).beginPath();
};

export const __widl_f_stroke_CanvasRenderingContext2D = function(arg0) {
    getObject(arg0).stroke();
};

export const __widl_f_set_stroke_style_CanvasRenderingContext2D = function(arg0, arg1) {
    getObject(arg0).strokeStyle = getObject(arg1);
};

export const __widl_f_arc_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    try {
        getObject(arg0).arc(arg1, arg2, arg3, arg4, arg5);
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_line_to_CanvasRenderingContext2D = function(arg0, arg1, arg2) {
    getObject(arg0).lineTo(arg1, arg2);
};

export const __widl_f_move_to_CanvasRenderingContext2D = function(arg0, arg1, arg2) {
    getObject(arg0).moveTo(arg1, arg2);
};

export const __widl_f_clear_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
};

export const __widl_f_fill_text_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
    try {
        getObject(arg0).fillText(getStringFromWasm(arg1, arg2), arg3, arg4);
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_set_font_CanvasRenderingContext2D = function(arg0, arg1, arg2) {
    getObject(arg0).font = getStringFromWasm(arg1, arg2);
};

export const __widl_f_scale_CanvasRenderingContext2D = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).scale(arg1, arg2);
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).getElementById(getStringFromWasm(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __widl_instanceof_HTMLCanvasElement = function(arg0) {
    const ret = getObject(arg0) instanceof HTMLCanvasElement;
    return ret;
};

export const __widl_f_get_context_HTMLCanvasElement = function(arg0, arg1, arg2) {
    try {
        const ret = getObject(arg0).getContext(getStringFromWasm(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_set_width_HTMLCanvasElement = function(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
};

export const __widl_f_set_height_HTMLCanvasElement = function(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
};

export const __widl_f_now_Performance = function(arg0) {
    const ret = getObject(arg0).now();
    return ret;
};

export const __widl_f_request_animation_frame_Window = function(arg0, arg1) {
    try {
        const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_document_Window = function(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __widl_f_inner_width_Window = function(arg0) {
    try {
        const ret = getObject(arg0).innerWidth;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_inner_height_Window = function(arg0) {
    try {
        const ret = getObject(arg0).innerHeight;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_device_pixel_ratio_Window = function(arg0) {
    const ret = getObject(arg0).devicePixelRatio;
    return ret;
};

export const __widl_f_performance_Window = function(arg0) {
    const ret = getObject(arg0).performance;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_newnoargs_368b05293a3f44de = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbg_call_1fc553129cb17c3c = function(arg0, arg1) {
    try {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_globalThis_8df2c73db5eac245 = function() {
    try {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_self_937dd9f384d2384a = function() {
    try {
        const ret = self.self;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_window_425d3fa09c43ece4 = function() {
    try {
        const ret = window.window;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_global_2c090b42ef2744b9 = function() {
    try {
        const ret = global.global;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

export const __wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export const __wbg_random_694320ddb679dc1e = function() {
    const ret = Math.random();
    return ret;
};

export const __wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg0);
    if (typeof(obj) === 'number') return obj;
    getUint8Memory()[arg1] = 1;
    const ret = 0;
    return ret;
};

export const __wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm(arg0, arg1));
};

export const __wbindgen_closure_wrapper78 = function(arg0, arg1, arg2) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = () => {
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return __wbg_elem_binding0(a, state.b, );
        } finally {
            if (--state.cnt === 0) wasm.__wbg_function_table.get(6)(a, state.b);
            else state.a = a;
        }
    }
    ;
    real.original = state;
    const ret = real;
    return addHeapObject(ret);
};

wasm.__wbindgen_start();

