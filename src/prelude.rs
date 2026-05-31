//! Re-exports most types with an extra `Lua*` prefix to prevent name clashes.

#[doc(no_inline)]
pub use crate::{
    AnyUserData as LuauAnyUserData, BorrowedBytes as LuauBorrowedBytes, BorrowedStr as LuauBorrowedStr,
    Chunk as LuauChunk, ContinuationStatus as LuauContinuationStatus, Either as LuauEither,
    Error as LuauError, ErrorContext as LuauErrorContext, ExternalError as LuauExternalError,
    ExternalResult as LuauExternalResult, FromLua, FromLuaMulti, Function as LuauFunction,
    FunctionInfo as LuauFunctionInfo, GCMode as LuauGCMode, Integer as LuauInteger, IntoLua, IntoLuaMulti,
    LightUserData as LuauLightUserData, Lua, LuaNativeFn, LuaNativeFnMut, LuaOptions,
    MetaMethod as LuauMetaMethod, MultiValue as LuauMultiValue, Nil as LuauNil, Number as LuauNumber,
    ObjectLike as LuauObjectLike, RegistryKey as LuauRegistryKey, Result as LuauResult, StdLib as LuauStdLib,
    String as LuauString, Table as LuauTable, TablePairs as LuauTablePairs,
    TablePairsOwned as LuauTablePairsOwned, TableSequence as LuauTableSequence, Thread as LuauThread,
    ThreadStatus as LuauThreadStatus, UserData as LuauUserData, UserDataFields as LuauUserDataFields,
    UserDataMetatable as LuauUserDataMetatable, UserDataMethods as LuauUserDataMethods,
    UserDataRef as LuauUserDataRef, UserDataRefMut as LuauUserDataRefMut,
    UserDataRegistry as LuauUserDataRegistry, Value as LuauValue, Variadic as LuauVariadic,
    VmState as LuauVmState, WeakLua,
};

#[cfg(not(feature = "luau"))]
#[doc(no_inline)]
pub use crate::HookTriggers as LuauHookTriggers;

#[cfg(feature = "luau")]
#[doc(no_inline)]
pub use crate::{
    CompileConstant as LuauCompileConstant, CoverageInfo as LuauCoverageInfo,
    NavigateError as LuauNavigateError, Require as LuauRequire, TextRequirer as LuauTextRequirer,
    Vector as LuauVector,
};

#[cfg(feature = "serde")]
#[doc(no_inline)]
pub use crate::{
    DeserializeOptions as LuauDeserializeOptions, LuaSerdeExt, SerializableValue as LuauSerializableValue,
    SerializeOptions as LuauSerializeOptions,
};
