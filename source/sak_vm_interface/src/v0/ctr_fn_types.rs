use wasmtime::TypedFunc;

pub type CtrInitFn = TypedFunc<(), (i32, i32)>;

pub type CtrExecuteFn = TypedFunc<(i32, i32), (i32, i32, i32, i32)>;

pub type CtrUpdateFn = TypedFunc<(i32, i32), (i32, i32, i32, i32)>;
