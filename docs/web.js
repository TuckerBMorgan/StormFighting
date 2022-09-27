import { push_asset, pull_assets } from './storm.js';

const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
let wasm;

const heap = new Array(32).fill(undefined);

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0 = new Float64Array();

function getFloat64Memory0() {
    if (cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_32(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd5bb7e9d09983990(arg0, arg1);
}

function __wbg_adapter_35(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4413a993fc0638d7(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_40(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd2b66e801ed204b3(arg0, arg1);
}

function __wbg_adapter_43(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h11626ca8dc761ecc(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_50(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3d78c92d15ce048a(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_53(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h43176def79fbeaaf(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_56(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h995e15d650f0d8f3(arg0, arg1);
}

function __wbg_adapter_59(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h13034c8d37438d42(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_68(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h141d93c6ee908127(arg0, arg1);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedFloat32Memory0 = new Float32Array();

function getFloat32Memory0() {
    if (cachedFloat32Memory0.byteLength === 0) {
        cachedFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_clearTimeout_5b4145302d77e5f3 = typeof clearTimeout == 'function' ? clearTimeout : notDefined('clearTimeout');
    imports.wbg.__wbg_setTimeout_02c3975efb677088 = function() { return handleError(function (arg0, arg1) {
        const ret = setTimeout(getObject(arg0), arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_process_0cc2ada8524d6f83 = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_c11acceab27a6c87 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_7ff1ce49caf23815 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_crypto_2036bed7c44c25e7 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_msCrypto_a21fc88caf1ecdc8 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_NODE_MODULE_cf6401cc1091279e = function() {
        const ret = module;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_a746e79b322b9336 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_b99eec4244a475bb = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_randomFillSync_065afffde01daa66 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_pushasset_48c90130b3fead01 = function(arg0, arg1) {
        push_asset(arg0 >>> 0, takeObject(arg1));
    };
    imports.wbg.__wbg_pullassets_8514db49f4558a3d = function() {
        const ret = pull_assets();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_WebGl2RenderingContext_d76863c237fc08d8 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof WebGL2RenderingContext;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_bindBufferBase_fc5d16f681e95ff9 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bindBufferBase(arg1 >>> 0, arg2 >>> 0, getObject(arg3));
    };
    imports.wbg.__wbg_bindVertexArray_38cd48e4e3cd558d = function(arg0, arg1) {
        getObject(arg0).bindVertexArray(getObject(arg1));
    };
    imports.wbg.__wbg_bufferData_d0fb0ae1efb9c644 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    };
    imports.wbg.__wbg_createVertexArray_4bf6b20cc35cc30c = function(arg0) {
        const ret = getObject(arg0).createVertexArray();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteVertexArray_0fefa9f54994d299 = function(arg0, arg1) {
        getObject(arg0).deleteVertexArray(getObject(arg1));
    };
    imports.wbg.__wbg_drawArraysInstanced_d2fb8c4e7f34ca48 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawArraysInstanced(arg1 >>> 0, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_getUniformBlockIndex_bddd34ae60b1b19b = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).getUniformBlockIndex(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_texImage2D_c9d4cacd47ee450d = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    }, arguments) };
    imports.wbg.__wbg_texSubImage2D_ffdce8320da6b3f1 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    }, arguments) };
    imports.wbg.__wbg_texSubImage2D_02533a6fc87c768e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
    }, arguments) };
    imports.wbg.__wbg_uniformBlockBinding_42e8cd9c613b2b10 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniformBlockBinding(getObject(arg1), arg2 >>> 0, arg3 >>> 0);
    };
    imports.wbg.__wbg_vertexAttribDivisor_5c6671731e1b48ed = function(arg0, arg1, arg2) {
        getObject(arg0).vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_vertexAttribIPointer_5099dc753acd20ab = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).vertexAttribIPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
    };
    imports.wbg.__wbg_activeTexture_6a5ed7372e6c98fd = function(arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_attachShader_01e03fc70a71557f = function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_bindBuffer_38e9701822843222 = function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindTexture_02ce01de0ddc9e6f = function(arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_blendFunc_40f96cb1e76932f3 = function(arg0, arg1, arg2) {
        getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_clear_959861de63ace980 = function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_clearColor_f542cdeed6bbcc49 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_clearDepth_5d1231db73f0b6c9 = function(arg0, arg1) {
        getObject(arg0).clearDepth(arg1);
    };
    imports.wbg.__wbg_compileShader_96ee7d1ded23ea20 = function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    };
    imports.wbg.__wbg_createBuffer_e1112a3f729fe783 = function(arg0) {
        const ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createProgram_eae10e696a39f131 = function(arg0) {
        const ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createShader_80d58e495dc9928d = function(arg0, arg1) {
        const ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createTexture_67e7eaf49b0cb974 = function(arg0) {
        const ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_cullFace_7eb704ad1df68fde = function(arg0, arg1) {
        getObject(arg0).cullFace(arg1 >>> 0);
    };
    imports.wbg.__wbg_deleteBuffer_193ed2d4519aa156 = function(arg0, arg1) {
        getObject(arg0).deleteBuffer(getObject(arg1));
    };
    imports.wbg.__wbg_deleteProgram_281c58a572f61a36 = function(arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    };
    imports.wbg.__wbg_deleteShader_f0098b092d2057ad = function(arg0, arg1) {
        getObject(arg0).deleteShader(getObject(arg1));
    };
    imports.wbg.__wbg_deleteTexture_d96de9a33de791a3 = function(arg0, arg1) {
        getObject(arg0).deleteTexture(getObject(arg1));
    };
    imports.wbg.__wbg_depthFunc_a62ae223388df11f = function(arg0, arg1) {
        getObject(arg0).depthFunc(arg1 >>> 0);
    };
    imports.wbg.__wbg_drawArrays_d463b668df3aa354 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_enable_f781dd58baddc0b9 = function(arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_21e452efa5576ee3 = function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_generateMipmap_9825fd2ce3c4dc78 = function(arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    };
    imports.wbg.__wbg_getExtension_61a9de6235ccf767 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).getExtension(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getParameter_0359eea934c15c8c = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).getParameter(arg1 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getProgramInfoLog_f9f10bda54aa2a04 = function(arg0, arg1, arg2) {
        const ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getProgramParameter_c01496a0c500dc79 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getShaderInfoLog_caa4e118eb238994 = function(arg0, arg1, arg2) {
        const ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderParameter_b2c18ddb8dde2442 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getSupportedExtensions_a46d4277f3575c75 = function(arg0) {
        const ret = getObject(arg0).getSupportedExtensions();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_getUniformLocation_63c88dcaef19f361 = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_linkProgram_d48f571a3d853bdd = function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    };
    imports.wbg.__wbg_pixelStorei_578b6ffdfc59c2a1 = function(arg0, arg1, arg2) {
        getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_shaderSource_cb1d551b4d3ef3f9 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_texParameterf_d81fd53092d74893 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameterf(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_texParameteri_0f2ba34afbf04fdc = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_uniform1i_4e9d048486f2cdb5 = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_useProgram_7e244cf9a356f3c6 = function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    };
    imports.wbg.__wbg_vertexAttribPointer_11b88f1dce1cc411 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_viewport_e1da083d1ab88187 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_instanceof_Window_42f092928baaee84 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_document_163930ea5ac4fab3 = function(arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_innerWidth_d79100a011251704 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).innerWidth;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_innerHeight_699f04b4e43f9df7 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).innerHeight;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_devicePixelRatio_27b7cca8001ea85a = function(arg0) {
        const ret = getObject(arg0).devicePixelRatio;
        return ret;
    };
    imports.wbg.__wbg_cancelAnimationFrame_b82eda69f4bd7b8a = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).cancelAnimationFrame(arg1);
    }, arguments) };
    imports.wbg.__wbg_matchMedia_df185f0a7ca43008 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).matchMedia(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_requestAnimationFrame_511a485a044a22ac = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_1b7fa72b18a2e99a = function(arg0, arg1, arg2) {
        const ret = getObject(arg0)[getStringFromWasm0(arg1, arg2)];
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_clearTimeout_e8bc01333ead4785 = function(arg0, arg1) {
        getObject(arg0).clearTimeout(arg1);
    };
    imports.wbg.__wbg_setTimeout_0a15b95a299d348f = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_destination_4dba43e94242bc41 = function(arg0) {
        const ret = getObject(arg0).destination;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_currentTime_d68457097e7c4fbf = function(arg0) {
        const ret = getObject(arg0).currentTime;
        return ret;
    };
    imports.wbg.__wbg_newwithcontextoptions_dd624867e74ad7d4 = function() { return handleError(function (arg0) {
        const ret = new lAudioContext(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createBuffer_bba3d5fa92ffa39b = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).createBuffer(arg1 >>> 0, arg2 >>> 0, arg3);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createBufferSource_80943a9e66c3ebed = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).createBufferSource();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_resume_2c41441286fa5de4 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).resume();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setbuffer_edf99ac1890646b4 = function(arg0, arg1) {
        getObject(arg0).buffer = getObject(arg1);
    };
    imports.wbg.__wbg_setonended_8fc66b95fd1e4a0b = function(arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    };
    imports.wbg.__wbg_start_000ed57a7055b13d = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).start(arg1);
    }, arguments) };
    imports.wbg.__wbg_wasClean_f8c74ec8ce31c946 = function(arg0) {
        const ret = getObject(arg0).wasClean;
        return ret;
    };
    imports.wbg.__wbg_code_773543dcd4cb793e = function(arg0) {
        const ret = getObject(arg0).code;
        return ret;
    };
    imports.wbg.__wbg_reason_7764ea72b6b491d8 = function(arg0, arg1) {
        const ret = getObject(arg1).reason;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_matches_dbfe3bf7b8354472 = function(arg0) {
        const ret = getObject(arg0).matches;
        return ret;
    };
    imports.wbg.__wbg_now_3bc14762a7083eb7 = function(arg0) {
        const ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_width_937961452e82d54f = function(arg0) {
        const ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_setwidth_a1fbf5f0011cb4ff = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_height_e61eb59263d24bfb = function(arg0) {
        const ret = getObject(arg0).height;
        return ret;
    };
    imports.wbg.__wbg_setheight_84faf97c1bd124a4 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_getContext_b03f09d1f0696e0b = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_data_f2b88856f48a1d44 = function(arg0) {
        const ret = getObject(arg0).data;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_connect_dda7970dfc6c3a33 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).connect(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_charCode_c2d343038d035715 = function(arg0) {
        const ret = getObject(arg0).charCode;
        return ret;
    };
    imports.wbg.__wbg_keyCode_8da778f15a49b1c1 = function(arg0) {
        const ret = getObject(arg0).keyCode;
        return ret;
    };
    imports.wbg.__wbg_altKey_0e62b8fd86fa21f0 = function(arg0) {
        const ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_64c2ab618ea0913c = function(arg0) {
        const ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_0f13c07d19e6ef98 = function(arg0) {
        const ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_6d900eae5aab00a9 = function(arg0) {
        const ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_key_6171b1a949dd430c = function(arg0, arg1) {
        const ret = getObject(arg1).key;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_code_be324c83d915d368 = function(arg0, arg1) {
        const ret = getObject(arg1).code;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getModifierState_3e6625dc2de4b531 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).getModifierState(getStringFromWasm0(arg1, arg2));
        return ret;
    };
    imports.wbg.__wbg_sdp_04ac2dd6c7819c6e = function(arg0, arg1) {
        const ret = getObject(arg1).sdp;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_addEventListener_b740d49aba10fcd3 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_addEventListener_f05b6d8202f0e302 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_dbf7c2a45a4bb8d8 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_appendChild_d1c2fa29cbb7f768 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_target_987b61b9edcc7732 = function(arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_cancelBubble_eae0c7b8ab36e642 = function(arg0) {
        const ret = getObject(arg0).cancelBubble;
        return ret;
    };
    imports.wbg.__wbg_preventDefault_9cd0104fc06a25d9 = function(arg0) {
        getObject(arg0).preventDefault();
    };
    imports.wbg.__wbg_stopPropagation_e48635a9da1ca25c = function(arg0) {
        getObject(arg0).stopPropagation();
    };
    imports.wbg.__wbg_matches_243298591c38e429 = function(arg0) {
        const ret = getObject(arg0).matches;
        return ret;
    };
    imports.wbg.__wbg_addListener_1763650af67e55d5 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).addListener(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_removeListener_bdd1b4e03227a5fc = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).removeListener(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_readyState_05b624d7036bcfd5 = function(arg0) {
        const ret = getObject(arg0).readyState;
        return ret;
    };
    imports.wbg.__wbg_setonopen_1b2ad0ecf44fb650 = function(arg0, arg1) {
        getObject(arg0).onopen = getObject(arg1);
    };
    imports.wbg.__wbg_setonerror_41a9c6c33cc083b1 = function(arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    };
    imports.wbg.__wbg_setonclose_2a77d30fca110e71 = function(arg0, arg1) {
        getObject(arg0).onclose = getObject(arg1);
    };
    imports.wbg.__wbg_setonmessage_4150f6b17dc77d5e = function(arg0, arg1) {
        getObject(arg0).onmessage = getObject(arg1);
    };
    imports.wbg.__wbg_setbinaryType_3a4dc9584a838ad9 = function(arg0, arg1) {
        getObject(arg0).binaryType = takeObject(arg1);
    };
    imports.wbg.__wbg_new_f681d720796d0271 = function() { return handleError(function (arg0, arg1) {
        const ret = new WebSocket(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithstrsequence_0f2e994cd4684d92 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = new WebSocket(getStringFromWasm0(arg0, arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_close_b54edd488a2b7b06 = function() { return handleError(function (arg0) {
        getObject(arg0).close();
    }, arguments) };
    imports.wbg.__wbg_send_6f0fdc71433121c8 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).send(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_send_67801ea99109ec70 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).send(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_settitle_edda68deedb593b9 = function(arg0, arg1, arg2) {
        getObject(arg0).title = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_body_3812cf22a714797d = function(arg0) {
        const ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_fullscreenElement_5fe850c452bfa1ea = function(arg0) {
        const ret = getObject(arg0).fullscreenElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_5a8447e4c16ca697 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_exitFullscreen_ceb6f3ec24c54afa = function(arg0) {
        getObject(arg0).exitFullscreen();
    };
    imports.wbg.__wbg_getBoundingClientRect_ec1ceb4bcde163ac = function(arg0) {
        const ret = getObject(arg0).getBoundingClientRect();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_requestFullscreen_0d5b69c856a74b58 = function() { return handleError(function (arg0) {
        getObject(arg0).requestFullscreen();
    }, arguments) };
    imports.wbg.__wbg_setAttribute_a99d9013414c1316 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_setPointerCapture_2c29112a8a6370e3 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).setPointerCapture(arg1);
    }, arguments) };
    imports.wbg.__wbg_style_fd62fee6d0893c16 = function(arg0) {
        const ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_bufferData_84e79627df8b23df = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    };
    imports.wbg.__wbg_texImage2D_54a11bba82040b2f = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    }, arguments) };
    imports.wbg.__wbg_texSubImage2D_05cd6d581363b632 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    }, arguments) };
    imports.wbg.__wbg_activeTexture_2ebd8a2241525651 = function(arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_attachShader_6108257f78273890 = function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_bindBuffer_268e589843f56214 = function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindTexture_f71a589f415b4ad9 = function(arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_blendFunc_f77b8103bfa55b56 = function(arg0, arg1, arg2) {
        getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_clear_019e005cf2cd4e32 = function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_clearColor_60fa6c5401bc1dcd = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_clearDepth_095849a8e49c3003 = function(arg0, arg1) {
        getObject(arg0).clearDepth(arg1);
    };
    imports.wbg.__wbg_compileShader_cb9fe79849b1756b = function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    };
    imports.wbg.__wbg_createBuffer_9e22a836c779a089 = function(arg0) {
        const ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createProgram_346bbb419153a384 = function(arg0) {
        const ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createShader_23bbd7dcf3b78c08 = function(arg0, arg1) {
        const ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createTexture_50ebe7ac9d80b166 = function(arg0) {
        const ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_cullFace_3d32509c728582f2 = function(arg0, arg1) {
        getObject(arg0).cullFace(arg1 >>> 0);
    };
    imports.wbg.__wbg_deleteBuffer_7766fc435fbced0b = function(arg0, arg1) {
        getObject(arg0).deleteBuffer(getObject(arg1));
    };
    imports.wbg.__wbg_deleteProgram_413bea2e5a6520d1 = function(arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    };
    imports.wbg.__wbg_deleteShader_9513e00b106bbe40 = function(arg0, arg1) {
        getObject(arg0).deleteShader(getObject(arg1));
    };
    imports.wbg.__wbg_deleteTexture_4426b0e14dbca414 = function(arg0, arg1) {
        getObject(arg0).deleteTexture(getObject(arg1));
    };
    imports.wbg.__wbg_depthFunc_1a1d95fa916e925e = function(arg0, arg1) {
        getObject(arg0).depthFunc(arg1 >>> 0);
    };
    imports.wbg.__wbg_drawArrays_d31bfc04b0a0a27a = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_enable_5ba851367a122fc6 = function(arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_18f93ab4f7ffa280 = function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_generateMipmap_1de402fd9c87417d = function(arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    };
    imports.wbg.__wbg_getParameter_3492c5b17a42bb1c = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).getParameter(arg1 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getProgramInfoLog_3d0387a27719c1e2 = function(arg0, arg1, arg2) {
        const ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getProgramParameter_ca48f30e46819e6e = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getShaderInfoLog_5dc966cb92c64cfa = function(arg0, arg1, arg2) {
        const ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderParameter_28fc211057a4d238 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getUniformLocation_cba8d729b48ff245 = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_linkProgram_f7bf5447dbae39de = function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    };
    imports.wbg.__wbg_pixelStorei_32252f759fc06bc0 = function(arg0, arg1, arg2) {
        getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_shaderSource_25049cac6ae74ae3 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_texParameterf_efe41b03bee7a05c = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameterf(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_texParameteri_6008ebcc7c74ef76 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_uniform1i_44946038a1da36a1 = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_useProgram_917e6a5c4c13967b = function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    };
    imports.wbg.__wbg_vertexAttribPointer_3837d89e5b3fd5c1 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_viewport_d76270a1545efe25 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_debug_07401d6dd085ab9f = function(arg0, arg1, arg2, arg3) {
        console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_error_14911e509f7486a1 = function(arg0, arg1) {
        console.error(getObject(arg0), getObject(arg1));
    };
    imports.wbg.__wbg_error_671981bc444705ed = function(arg0, arg1, arg2, arg3) {
        console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_info_9e1cab954aeb98ff = function(arg0, arg1, arg2, arg3) {
        console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_log_3ade350d48e895ee = function(arg0, arg1, arg2, arg3) {
        console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_warn_c7401f641d2733e0 = function(arg0, arg1, arg2, arg3) {
        console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_instanceof_Blob_2c39d4d0956b6160 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Blob;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_setProperty_506453e26bec5e5e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_localDescription_30de72cefd83a9f0 = function(arg0) {
        const ret = getObject(arg0).localDescription;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_iceGatheringState_1994dd23f4260266 = function(arg0) {
        const ret = getObject(arg0).iceGatheringState;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setonicegatheringstatechange_8991854b5f82ab8b = function(arg0, arg1) {
        getObject(arg0).onicegatheringstatechange = getObject(arg1);
    };
    imports.wbg.__wbg_newwithconfiguration_45758d63ecd599b5 = function() { return handleError(function (arg0) {
        const ret = new RTCPeerConnection(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createAnswer_047392c5c2956638 = function(arg0) {
        const ret = getObject(arg0).createAnswer();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createDataChannel_c1d999df087ee5e0 = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).createDataChannel(getStringFromWasm0(arg1, arg2), getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createOffer_ea65928dd36e791e = function(arg0) {
        const ret = getObject(arg0).createOffer();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setLocalDescription_a18bba7384a6cf6e = function(arg0, arg1) {
        const ret = getObject(arg0).setLocalDescription(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setRemoteDescription_2ce191dcb0768c4c = function(arg0, arg1) {
        const ret = getObject(arg0).setRemoteDescription(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_drawArraysInstancedANGLE_a579099fc3e440a6 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawArraysInstancedANGLE(arg1 >>> 0, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_vertexAttribDivisorANGLE_c4a63efa09e768e4 = function(arg0, arg1, arg2) {
        getObject(arg0).vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_x_a8e4f5cd0edae7b4 = function(arg0) {
        const ret = getObject(arg0).x;
        return ret;
    };
    imports.wbg.__wbg_y_593b0018d94a22f7 = function(arg0) {
        const ret = getObject(arg0).y;
        return ret;
    };
    imports.wbg.__wbg_clientX_875a7baae967c933 = function(arg0) {
        const ret = getObject(arg0).clientX;
        return ret;
    };
    imports.wbg.__wbg_clientY_b1e052bedf0edda5 = function(arg0) {
        const ret = getObject(arg0).clientY;
        return ret;
    };
    imports.wbg.__wbg_offsetX_abc5355cbd5f7a0e = function(arg0) {
        const ret = getObject(arg0).offsetX;
        return ret;
    };
    imports.wbg.__wbg_offsetY_7fcc5137ceb9f2de = function(arg0) {
        const ret = getObject(arg0).offsetY;
        return ret;
    };
    imports.wbg.__wbg_ctrlKey_b7f024e6569d735f = function(arg0) {
        const ret = getObject(arg0).ctrlKey;
        return ret;
    };
    imports.wbg.__wbg_shiftKey_5762e6aa3dea9406 = function(arg0) {
        const ret = getObject(arg0).shiftKey;
        return ret;
    };
    imports.wbg.__wbg_altKey_51ab3719d3bac5e7 = function(arg0) {
        const ret = getObject(arg0).altKey;
        return ret;
    };
    imports.wbg.__wbg_metaKey_91027e9b4d851bd8 = function(arg0) {
        const ret = getObject(arg0).metaKey;
        return ret;
    };
    imports.wbg.__wbg_button_d2ec06b227d5de9b = function(arg0) {
        const ret = getObject(arg0).button;
        return ret;
    };
    imports.wbg.__wbg_buttons_0161d690733ea42a = function(arg0) {
        const ret = getObject(arg0).buttons;
        return ret;
    };
    imports.wbg.__wbg_movementX_ef8b075ffc2133e7 = function(arg0) {
        const ret = getObject(arg0).movementX;
        return ret;
    };
    imports.wbg.__wbg_movementY_60d5fc4c02a277be = function(arg0) {
        const ret = getObject(arg0).movementY;
        return ret;
    };
    imports.wbg.__wbg_copyToChannel_5726061d55778aa8 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).copyToChannel(getArrayF32FromWasm0(arg1, arg2), arg3);
    }, arguments) };
    imports.wbg.__wbg_code_77c54dc2ef8082bd = function(arg0) {
        const ret = getObject(arg0).code;
        return ret;
    };
    imports.wbg.__wbg_bindVertexArrayOES_c1935dcb930ace69 = function(arg0, arg1) {
        getObject(arg0).bindVertexArrayOES(getObject(arg1));
    };
    imports.wbg.__wbg_createVertexArrayOES_551b806170245bd2 = function(arg0) {
        const ret = getObject(arg0).createVertexArrayOES();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteVertexArrayOES_bd92228f403b2cae = function(arg0, arg1) {
        getObject(arg0).deleteVertexArrayOES(getObject(arg1));
    };
    imports.wbg.__wbg_pointerId_11412361d7e551e3 = function(arg0) {
        const ret = getObject(arg0).pointerId;
        return ret;
    };
    imports.wbg.__wbg_setonopen_248a56aab314f755 = function(arg0, arg1) {
        getObject(arg0).onopen = getObject(arg1);
    };
    imports.wbg.__wbg_setonmessage_edee6204d7ada1b6 = function(arg0, arg1) {
        getObject(arg0).onmessage = getObject(arg1);
    };
    imports.wbg.__wbg_setbinaryType_8de757175895335f = function(arg0, arg1) {
        getObject(arg0).binaryType = takeObject(arg1);
    };
    imports.wbg.__wbg_send_740da131ba7840da = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).send(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_deltaX_3e477f8e4b4a6d89 = function(arg0) {
        const ret = getObject(arg0).deltaX;
        return ret;
    };
    imports.wbg.__wbg_deltaY_7f486d751b2e3d94 = function(arg0) {
        const ret = getObject(arg0).deltaY;
        return ret;
    };
    imports.wbg.__wbg_deltaMode_b996adb28fecdef9 = function(arg0) {
        const ret = getObject(arg0).deltaMode;
        return ret;
    };
    imports.wbg.__wbg_get_5c7997f237e97429 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_c71e7a4e0f91ff23 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_bfd57da4d3b6c976 = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_e1ddb03293334932 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_556a4c57efe02bcd = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_a6fa88c3302e8ad5 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_08b60aea8ab679bc = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_14408afdb5c69451 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_75b1f6151d589837 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_e2d2385b94c810da = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_3c19477360f9b641 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_eval_743a11002e381e3d = function() { return handleError(function (arg0, arg1) {
        const ret = eval(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_isArray_8d3ac415a656d809 = function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_push_92eb92b6550bd402 = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_02bbeeb60438c785 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ArrayBuffer;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_getTime_def258a47d72ac06 = function(arg0) {
        const ret = getObject(arg0).getTime();
        return ret;
    };
    imports.wbg.__wbg_getTimezoneOffset_2ee2b0ab15c99512 = function(arg0) {
        const ret = getObject(arg0).getTimezoneOffset();
        return ret;
    };
    imports.wbg.__wbg_new0_d8c0fd74ecfbd384 = function() {
        const ret = new Date();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_is_a00a7c25908fcfd4 = function(arg0, arg1) {
        const ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_resolve_a9c1dadc5ab3a423 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_beb4f7b8c6a83b00 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_47f089895771218a = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_e8e1791d59230f6e = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_5ddd455ea299a2f4 = function(arg0, arg1, arg2) {
        const ret = new Int8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_9d6824f37d383b1b = function(arg0, arg1, arg2) {
        const ret = new Int16Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_8553be3326e207ce = function(arg0, arg1, arg2) {
        const ret = new Int32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_62fab8cdca7b6db0 = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_d256fd368dc8455c = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_ff6a229de2633e38 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_8c589b0fd9987662 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_0e13ccf164777c43 = function(arg0, arg1, arg2) {
        const ret = new Uint16Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_b46ef81e1e624110 = function(arg0, arg1, arg2) {
        const ret = new Uint32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_f307afbade41ecb2 = function(arg0, arg1, arg2) {
        const ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Uint8Array_36c37b9ca15e3e0a = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Uint8Array;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_dc0752ff6d0d8cc2 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_fbf3eb17f25d3dd4 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_d43020530506f215 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper686 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 117, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper688 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 117, __wbg_adapter_35);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper690 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 117, __wbg_adapter_35);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper946 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 211, __wbg_adapter_40);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper948 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 211, __wbg_adapter_43);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper950 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 211, __wbg_adapter_43);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper952 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 211, __wbg_adapter_43);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1140 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 297, __wbg_adapter_50);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1209 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 336, __wbg_adapter_53);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1227 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 345, __wbg_adapter_56);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1924 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_59);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1926 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_59);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1928 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_59);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1930 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_59);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1932 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_68);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1934 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_59);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1936 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_59);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1938 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 587, __wbg_adapter_59);
        return addHeapObject(ret);
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat32Memory0 = new Float32Array();
    cachedFloat64Memory0 = new Float64Array();
    cachedInt32Memory0 = new Int32Array();
    cachedUint8Memory0 = new Uint8Array();

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('web_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
