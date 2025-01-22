/* automatically generated by rust-bindgen 0.70.1 */

#[doc = " Base class for managed objects. Only descendent types of `GarbageCollected`\n can be constructed using `MakeGarbageCollected()`. Must be inherited from as\n left-most base class.\n\n Types inheriting from GarbageCollected must provide a method of\n signature `void Trace(cppgc::Visitor*) const` that dispatchs all managed\n pointers to the visitor and delegates to garbage-collected base classes.\n The method must be virtual if the type is not directly a child of\n GarbageCollected and marked as final.\n\n \\code\n // Example using final class.\n class FinalType final : public GarbageCollected<FinalType> {\n  public:\n   void Trace(cppgc::Visitor* visitor) const {\n     // Dispatch using visitor->Trace(...);\n   }\n };\n\n // Example using non-final base class.\n class NonFinalBase : public GarbageCollected<NonFinalBase> {\n  public:\n   virtual void Trace(cppgc::Visitor*) const {}\n };\n\n class FinalChild final : public NonFinalBase {\n  public:\n   void Trace(cppgc::Visitor* visitor) const final {\n     // Dispatch using visitor->Trace(...);\n     NonFinalBase::Trace(visitor);\n   }\n };\n \\endcode"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cppgc_GarbageCollected {
  pub _address: u8,
}
pub type cppgc_GarbageCollected_IsGarbageCollectedTypeMarker =
  ::std::os::raw::c_void;
pub type cppgc_GarbageCollected_ParentMostGarbageCollectedType<T> = T;
#[repr(C)]
pub struct cppgc_Visitor__bindgen_vtable(::std::os::raw::c_void);
#[doc = " Visitor passed to trace methods. All managed pointers must have called the\n Visitor's trace method on them.\n\n \\code\n class Foo final : public GarbageCollected<Foo> {\n  public:\n   void Trace(Visitor* visitor) const {\n     visitor->Trace(foo_);\n     visitor->Trace(weak_foo_);\n   }\n  private:\n   Member<Foo> foo_;\n   WeakMember<Foo> weak_foo_;\n };\n \\endcode"]
#[repr(C)]
#[derive(Debug)]
pub struct cppgc_Visitor {
  pub vtable_: *const cppgc_Visitor__bindgen_vtable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cppgc_Visitor_Key {
  pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of cppgc_Visitor_Key"]
    [::std::mem::size_of::<cppgc_Visitor_Key>() - 1usize];
  ["Alignment of cppgc_Visitor_Key"]
    [::std::mem::align_of::<cppgc_Visitor_Key>() - 1usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of cppgc_Visitor"][::std::mem::size_of::<cppgc_Visitor>() - 8usize];
  ["Alignment of cppgc_Visitor"]
    [::std::mem::align_of::<cppgc_Visitor>() - 8usize];
};
#[repr(C)]
pub struct cppgc_NameProvider__bindgen_vtable(::std::os::raw::c_void);
#[doc = " NameProvider allows for providing a human-readable name for garbage-collected\n objects.\n\n There's two cases of names to distinguish:\n a. Explicitly specified names via using NameProvider. Such names are always\n    preserved in the system.\n b. Internal names that Oilpan infers from a C++ type on the class hierarchy\n    of the object. This is not necessarily the type of the actually\n    instantiated object.\n\n Depending on the build configuration, Oilpan may hide names, i.e., represent\n them with kHiddenName, of case b. to avoid exposing internal details."]
#[repr(C)]
#[derive(Debug)]
pub struct cppgc_NameProvider {
  pub vtable_: *const cppgc_NameProvider__bindgen_vtable,
}
#[doc = " Name that is used when hiding internals."]
#[allow(unsafe_code)]
pub const cppgc_NameProvider_kHiddenName: &::std::ffi::CStr =
  unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"InternalNode\0") };
#[doc = " Name that is used in case compiler support is missing for composing a name\n from C++ types."]
#[allow(unsafe_code)]
pub const cppgc_NameProvider_kNoNameDeducible: &::std::ffi::CStr =
  unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"<No name>\0") };
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of cppgc_NameProvider"]
    [::std::mem::size_of::<cppgc_NameProvider>() - 8usize];
  ["Alignment of cppgc_NameProvider"]
    [::std::mem::align_of::<cppgc_NameProvider>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_CTypeInfo {
  pub type_: v8_CTypeInfo_Type,
  pub sequence_type_: v8_CTypeInfo_SequenceType,
  pub flags_: v8_CTypeInfo_Flags,
}
pub const v8_CTypeInfo_Type_kVoid: v8_CTypeInfo_Type = 0;
pub const v8_CTypeInfo_Type_kBool: v8_CTypeInfo_Type = 1;
pub const v8_CTypeInfo_Type_kUint8: v8_CTypeInfo_Type = 2;
pub const v8_CTypeInfo_Type_kInt32: v8_CTypeInfo_Type = 3;
pub const v8_CTypeInfo_Type_kUint32: v8_CTypeInfo_Type = 4;
pub const v8_CTypeInfo_Type_kInt64: v8_CTypeInfo_Type = 5;
pub const v8_CTypeInfo_Type_kUint64: v8_CTypeInfo_Type = 6;
pub const v8_CTypeInfo_Type_kFloat32: v8_CTypeInfo_Type = 7;
pub const v8_CTypeInfo_Type_kFloat64: v8_CTypeInfo_Type = 8;
pub const v8_CTypeInfo_Type_kPointer: v8_CTypeInfo_Type = 9;
pub const v8_CTypeInfo_Type_kV8Value: v8_CTypeInfo_Type = 10;
pub const v8_CTypeInfo_Type_kSeqOneByteString: v8_CTypeInfo_Type = 11;
pub const v8_CTypeInfo_Type_kApiObject: v8_CTypeInfo_Type = 12;
pub const v8_CTypeInfo_Type_kAny: v8_CTypeInfo_Type = 13;
pub type v8_CTypeInfo_Type = u8;
pub const v8_CTypeInfo_SequenceType_kScalar: v8_CTypeInfo_SequenceType = 0;
pub const v8_CTypeInfo_SequenceType_kIsSequence: v8_CTypeInfo_SequenceType = 1;
pub const v8_CTypeInfo_SequenceType_kIsTypedArray: v8_CTypeInfo_SequenceType =
  2;
pub const v8_CTypeInfo_SequenceType_kIsArrayBuffer: v8_CTypeInfo_SequenceType =
  3;
pub type v8_CTypeInfo_SequenceType = u8;
pub const v8_CTypeInfo_Flags_kNone: v8_CTypeInfo_Flags = 0;
pub const v8_CTypeInfo_Flags_kAllowSharedBit: v8_CTypeInfo_Flags = 1;
pub const v8_CTypeInfo_Flags_kEnforceRangeBit: v8_CTypeInfo_Flags = 2;
pub const v8_CTypeInfo_Flags_kClampBit: v8_CTypeInfo_Flags = 4;
pub const v8_CTypeInfo_Flags_kIsRestrictedBit: v8_CTypeInfo_Flags = 8;
pub type v8_CTypeInfo_Flags = u8;
pub type v8_CTypeInfo_Identifier = u32;
extern "C" {
  #[link_name = "\u{1}_ZN2v89CTypeInfo20kCallbackOptionsTypeE"]
  pub static v8_CTypeInfo_kCallbackOptionsType: v8_CTypeInfo_Type;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of v8_CTypeInfo"][::std::mem::size_of::<v8_CTypeInfo>() - 3usize];
  ["Alignment of v8_CTypeInfo"]
    [::std::mem::align_of::<v8_CTypeInfo>() - 1usize];
  ["Offset of field: v8_CTypeInfo::type_"]
    [::std::mem::offset_of!(v8_CTypeInfo, type_) - 0usize];
  ["Offset of field: v8_CTypeInfo::sequence_type_"]
    [::std::mem::offset_of!(v8_CTypeInfo, sequence_type_) - 1usize];
  ["Offset of field: v8_CTypeInfo::flags_"]
    [::std::mem::offset_of!(v8_CTypeInfo, flags_) - 2usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_FastApiTypedArrayBase {
  pub length_: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of v8_FastApiTypedArrayBase"]
    [::std::mem::size_of::<v8_FastApiTypedArrayBase>() - 8usize];
  ["Alignment of v8_FastApiTypedArrayBase"]
    [::std::mem::align_of::<v8_FastApiTypedArrayBase>() - 8usize];
  ["Offset of field: v8_FastApiTypedArrayBase::length_"]
    [::std::mem::offset_of!(v8_FastApiTypedArrayBase, length_) - 0usize];
};
extern "C" {
  #[link_name = "\u{1}_ZNK2v821FastApiTypedArrayBase13ValidateIndexEm"]
  pub fn v8_FastApiTypedArrayBase_ValidateIndex(
    this: *const v8_FastApiTypedArrayBase,
    index: usize,
  );
}
impl v8_FastApiTypedArrayBase {
  #[inline]
  pub unsafe fn ValidateIndex(&self, index: usize) {
    v8_FastApiTypedArrayBase_ValidateIndex(self, index)
  }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_FastApiTypedArray {
  pub _base: v8_FastApiTypedArrayBase,
  pub data_: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_FastApiArrayBufferView {
  pub data: *mut ::std::os::raw::c_void,
  pub byte_length: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of v8_FastApiArrayBufferView"]
    [::std::mem::size_of::<v8_FastApiArrayBufferView>() - 16usize];
  ["Alignment of v8_FastApiArrayBufferView"]
    [::std::mem::align_of::<v8_FastApiArrayBufferView>() - 8usize];
  ["Offset of field: v8_FastApiArrayBufferView::data"]
    [::std::mem::offset_of!(v8_FastApiArrayBufferView, data) - 0usize];
  ["Offset of field: v8_FastApiArrayBufferView::byte_length"]
    [::std::mem::offset_of!(v8_FastApiArrayBufferView, byte_length) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_FastOneByteString {
  pub data: *const ::std::os::raw::c_char,
  pub length: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of v8_FastOneByteString"]
    [::std::mem::size_of::<v8_FastOneByteString>() - 16usize];
  ["Alignment of v8_FastOneByteString"]
    [::std::mem::align_of::<v8_FastOneByteString>() - 8usize];
  ["Offset of field: v8_FastOneByteString::data"]
    [::std::mem::offset_of!(v8_FastOneByteString, data) - 0usize];
  ["Offset of field: v8_FastOneByteString::length"]
    [::std::mem::offset_of!(v8_FastOneByteString, length) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_CFunctionInfo {
  pub return_info_: v8_CTypeInfo,
  pub repr_: v8_CFunctionInfo_Int64Representation,
  pub arg_count_: ::std::os::raw::c_uint,
  pub arg_info_: *const v8_CTypeInfo,
}
pub const v8_CFunctionInfo_Int64Representation_kNumber:
  v8_CFunctionInfo_Int64Representation = 0;
pub const v8_CFunctionInfo_Int64Representation_kBigInt:
  v8_CFunctionInfo_Int64Representation = 1;
pub type v8_CFunctionInfo_Int64Representation = u8;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of v8_CFunctionInfo"]
    [::std::mem::size_of::<v8_CFunctionInfo>() - 16usize];
  ["Alignment of v8_CFunctionInfo"]
    [::std::mem::align_of::<v8_CFunctionInfo>() - 8usize];
  ["Offset of field: v8_CFunctionInfo::return_info_"]
    [::std::mem::offset_of!(v8_CFunctionInfo, return_info_) - 0usize];
  ["Offset of field: v8_CFunctionInfo::repr_"]
    [::std::mem::offset_of!(v8_CFunctionInfo, repr_) - 3usize];
  ["Offset of field: v8_CFunctionInfo::arg_count_"]
    [::std::mem::offset_of!(v8_CFunctionInfo, arg_count_) - 4usize];
  ["Offset of field: v8_CFunctionInfo::arg_info_"]
    [::std::mem::offset_of!(v8_CFunctionInfo, arg_info_) - 8usize];
};
extern "C" {
  #[link_name = "\u{1}_ZNK2v813CFunctionInfo12ArgumentInfoEj"]
  pub fn v8_CFunctionInfo_ArgumentInfo(
    this: *const v8_CFunctionInfo,
    index: ::std::os::raw::c_uint,
  ) -> *const v8_CTypeInfo;
}
extern "C" {
  #[link_name = "\u{1}_ZN2v813CFunctionInfoC1ERKNS_9CTypeInfoEjPS2_NS0_19Int64RepresentationE"]
  pub fn v8_CFunctionInfo_CFunctionInfo(
    this: *mut v8_CFunctionInfo,
    return_info: *const v8_CTypeInfo,
    arg_count: ::std::os::raw::c_uint,
    arg_info: *const v8_CTypeInfo,
    repr: v8_CFunctionInfo_Int64Representation,
  );
}
impl v8_CFunctionInfo {
  #[inline]
  pub unsafe fn ArgumentInfo(
    &self,
    index: ::std::os::raw::c_uint,
  ) -> *const v8_CTypeInfo {
    v8_CFunctionInfo_ArgumentInfo(self, index)
  }
  #[inline]
  pub unsafe fn new(
    return_info: *const v8_CTypeInfo,
    arg_count: ::std::os::raw::c_uint,
    arg_info: *const v8_CTypeInfo,
    repr: v8_CFunctionInfo_Int64Representation,
  ) -> Self {
    let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
    v8_CFunctionInfo_CFunctionInfo(
      __bindgen_tmp.as_mut_ptr(),
      return_info,
      arg_count,
      arg_info,
      repr,
    );
    __bindgen_tmp.assume_init()
  }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_CFunction {
  pub address_: *const ::std::os::raw::c_void,
  pub type_info_: *const v8_CFunctionInfo,
}
pub const v8_CFunction_OverloadResolution_kImpossible:
  v8_CFunction_OverloadResolution = 0;
pub const v8_CFunction_OverloadResolution_kAtRuntime:
  v8_CFunction_OverloadResolution = 1;
pub const v8_CFunction_OverloadResolution_kAtCompileTime:
  v8_CFunction_OverloadResolution = 2;
pub type v8_CFunction_OverloadResolution = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v8_CFunction_ArgUnwrap {
  pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of v8_CFunction"][::std::mem::size_of::<v8_CFunction>() - 16usize];
  ["Alignment of v8_CFunction"]
    [::std::mem::align_of::<v8_CFunction>() - 8usize];
  ["Offset of field: v8_CFunction::address_"]
    [::std::mem::offset_of!(v8_CFunction, address_) - 0usize];
  ["Offset of field: v8_CFunction::type_info_"]
    [::std::mem::offset_of!(v8_CFunction, type_info_) - 8usize];
};
extern "C" {
  #[link_name = "\u{1}_ZN2v89CFunctionC1EPKvPKNS_13CFunctionInfoE"]
  pub fn v8_CFunction_CFunction(
    this: *mut v8_CFunction,
    address: *const ::std::os::raw::c_void,
    type_info: *const v8_CFunctionInfo,
  );
}
impl v8_CFunction {
  #[inline]
  pub unsafe fn new(
    address: *const ::std::os::raw::c_void,
    type_info: *const v8_CFunctionInfo,
  ) -> Self {
    let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
    v8_CFunction_CFunction(__bindgen_tmp.as_mut_ptr(), address, type_info);
    __bindgen_tmp.assume_init()
  }
}
#[repr(u32)]
#[doc = " Features reported via the SetUseCounterCallback callback. Do not change\n assigned numbers of existing items; add new features to the end of this\n list.\n Dead features can be marked `V8_DEPRECATE_SOON`, then `V8_DEPRECATED`, and\n then finally be renamed to `kOBSOLETE_...` to stop embedders from using\n them."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum v8_Isolate_UseCounterFeature {
  kUseAsm = 0,
  kBreakIterator = 1,
  kOBSOLETE_LegacyConst = 2,
  kOBSOLETE_MarkDequeOverflow = 3,
  kOBSOLETE_StoreBufferOverflow = 4,
  kOBSOLETE_SlotsBufferOverflow = 5,
  kOBSOLETE_ObjectObserve = 6,
  kForcedGC = 7,
  kSloppyMode = 8,
  kStrictMode = 9,
  kOBSOLETE_StrongMode = 10,
  kRegExpPrototypeStickyGetter = 11,
  kRegExpPrototypeToString = 12,
  kRegExpPrototypeUnicodeGetter = 13,
  kOBSOLETE_IntlV8Parse = 14,
  kOBSOLETE_IntlPattern = 15,
  kOBSOLETE_IntlResolved = 16,
  kOBSOLETE_PromiseChain = 17,
  kOBSOLETE_PromiseAccept = 18,
  kOBSOLETE_PromiseDefer = 19,
  kHtmlCommentInExternalScript = 20,
  kHtmlComment = 21,
  kSloppyModeBlockScopedFunctionRedefinition = 22,
  kForInInitializer = 23,
  kOBSOLETE_ArrayProtectorDirtied = 24,
  kArraySpeciesModified = 25,
  kArrayPrototypeConstructorModified = 26,
  kOBSOLETE_ArrayInstanceProtoModified = 27,
  kArrayInstanceConstructorModified = 28,
  kOBSOLETE_LegacyFunctionDeclaration = 29,
  kOBSOLETE_RegExpPrototypeSourceGetter = 30,
  kOBSOLETE_RegExpPrototypeOldFlagGetter = 31,
  kDecimalWithLeadingZeroInStrictMode = 32,
  kLegacyDateParser = 33,
  kDefineGetterOrSetterWouldThrow = 34,
  kFunctionConstructorReturnedUndefined = 35,
  kAssigmentExpressionLHSIsCallInSloppy = 36,
  kAssigmentExpressionLHSIsCallInStrict = 37,
  kPromiseConstructorReturnedUndefined = 38,
  kOBSOLETE_ConstructorNonUndefinedPrimitiveReturn = 39,
  kOBSOLETE_LabeledExpressionStatement = 40,
  kOBSOLETE_LineOrParagraphSeparatorAsLineTerminator = 41,
  kIndexAccessor = 42,
  kErrorCaptureStackTrace = 43,
  kErrorPrepareStackTrace = 44,
  kErrorStackTraceLimit = 45,
  kWebAssemblyInstantiation = 46,
  kDeoptimizerDisableSpeculation = 47,
  kOBSOLETE_ArrayPrototypeSortJSArrayModifiedPrototype = 48,
  kFunctionTokenOffsetTooLongForToString = 49,
  kWasmSharedMemory = 50,
  kWasmThreadOpcodes = 51,
  kOBSOLETE_AtomicsNotify = 52,
  kOBSOLETE_AtomicsWake = 53,
  kCollator = 54,
  kNumberFormat = 55,
  kDateTimeFormat = 56,
  kPluralRules = 57,
  kRelativeTimeFormat = 58,
  kLocale = 59,
  kListFormat = 60,
  kSegmenter = 61,
  kStringLocaleCompare = 62,
  kOBSOLETE_StringToLocaleUpperCase = 63,
  kStringToLocaleLowerCase = 64,
  kNumberToLocaleString = 65,
  kDateToLocaleString = 66,
  kDateToLocaleDateString = 67,
  kDateToLocaleTimeString = 68,
  kAttemptOverrideReadOnlyOnPrototypeSloppy = 69,
  kAttemptOverrideReadOnlyOnPrototypeStrict = 70,
  kOBSOLETE_OptimizedFunctionWithOneShotBytecode = 71,
  kRegExpMatchIsTrueishOnNonJSRegExp = 72,
  kRegExpMatchIsFalseishOnJSRegExp = 73,
  kOBSOLETE_DateGetTimezoneOffset = 74,
  kStringNormalize = 75,
  kCallSiteAPIGetFunctionSloppyCall = 76,
  kCallSiteAPIGetThisSloppyCall = 77,
  kOBSOLETE_RegExpMatchAllWithNonGlobalRegExp = 78,
  kRegExpExecCalledOnSlowRegExp = 79,
  kRegExpReplaceCalledOnSlowRegExp = 80,
  kDisplayNames = 81,
  kSharedArrayBufferConstructed = 82,
  kArrayPrototypeHasElements = 83,
  kObjectPrototypeHasElements = 84,
  kNumberFormatStyleUnit = 85,
  kDateTimeFormatRange = 86,
  kDateTimeFormatDateTimeStyle = 87,
  kBreakIteratorTypeWord = 88,
  kBreakIteratorTypeLine = 89,
  kInvalidatedArrayBufferDetachingProtector = 90,
  kInvalidatedArrayConstructorProtector = 91,
  kInvalidatedArrayIteratorLookupChainProtector = 92,
  kInvalidatedArraySpeciesLookupChainProtector = 93,
  kInvalidatedIsConcatSpreadableLookupChainProtector = 94,
  kInvalidatedMapIteratorLookupChainProtector = 95,
  kInvalidatedNoElementsProtector = 96,
  kInvalidatedPromiseHookProtector = 97,
  kInvalidatedPromiseResolveLookupChainProtector = 98,
  kInvalidatedPromiseSpeciesLookupChainProtector = 99,
  kInvalidatedPromiseThenLookupChainProtector = 100,
  kInvalidatedRegExpSpeciesLookupChainProtector = 101,
  kInvalidatedSetIteratorLookupChainProtector = 102,
  kInvalidatedStringIteratorLookupChainProtector = 103,
  kInvalidatedStringLengthOverflowLookupChainProtector = 104,
  kInvalidatedTypedArraySpeciesLookupChainProtector = 105,
  kWasmSimdOpcodes = 106,
  kVarRedeclaredCatchBinding = 107,
  kWasmRefTypes = 108,
  kOBSOLETE_WasmBulkMemory = 109,
  kOBSOLETE_WasmMultiValue = 110,
  kWasmExceptionHandling = 111,
  kInvalidatedMegaDOMProtector = 112,
  kFunctionPrototypeArguments = 113,
  kFunctionPrototypeCaller = 114,
  kTurboFanOsrCompileStarted = 115,
  kAsyncStackTaggingCreateTaskCall = 116,
  kDurationFormat = 117,
  kInvalidatedNumberStringNotRegexpLikeProtector = 118,
  kOBSOLETE_RegExpUnicodeSetIncompatibilitiesWithUnicodeMode = 119,
  kImportAssertionDeprecatedSyntax = 120,
  kLocaleInfoObsoletedGetters = 121,
  kLocaleInfoFunctions = 122,
  kCompileHintsMagicAll = 123,
  kInvalidatedNoProfilingProtector = 124,
  kWasmMemory64 = 125,
  kWasmMultiMemory = 126,
  kWasmGC = 127,
  kWasmImportedStrings = 128,
  kSourceMappingUrlMagicCommentAtSign = 129,
  kTemporalObject = 130,
  kWasmModuleCompilation = 131,
  kInvalidatedNoUndetectableObjectsProtector = 132,
  kWasmJavaScriptPromiseIntegration = 133,
  kWasmReturnCall = 134,
  kWasmExtendedConst = 135,
  kWasmRelaxedSimd = 136,
  kWasmTypeReflection = 137,
  kWasmExnRef = 138,
  kWasmTypedFuncRef = 139,
  kInvalidatedStringWrapperToPrimitiveProtector = 140,
  kDocumentAllLegacyCall = 141,
  kDocumentAllLegacyConstruct = 142,
  kConsoleContext = 143,
  kWasmImportedStringsUtf8 = 144,
  kUseCounterFeatureCount = 145,
}
#[repr(C)]
#[derive(Debug)]
pub struct RustObj {
  pub _base_1: cppgc_NameProvider,
  pub data: [usize; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of RustObj"][::std::mem::size_of::<RustObj>() - 24usize];
  ["Alignment of RustObj"][::std::mem::align_of::<RustObj>() - 8usize];
  ["Offset of field: RustObj::data"]
    [::std::mem::offset_of!(RustObj, data) - 8usize];
};
extern "C" {
  #[link_name = "\u{1}_ZNK7RustObj5TraceEPN5cppgc7VisitorE"]
  pub fn RustObj_Trace(this: *const RustObj, visitor: *mut cppgc_Visitor);
}
impl RustObj {
  #[inline]
  pub unsafe fn Trace(&self, visitor: *mut cppgc_Visitor) {
    RustObj_Trace(self, visitor)
  }
}
extern "C" {
  #[link_name = "\u{1}_ZN7RustObjD1Ev"]
  pub fn RustObj_RustObj_destructor(this: *mut RustObj);
}
extern "C" {
  #[link_name = "\u{1}_ZNK7RustObj20GetHumanReadableNameEv"]
  pub fn RustObj_GetHumanReadableName(
    this: *mut ::std::os::raw::c_void,
  ) -> *const ::std::os::raw::c_char;
}
#[doc = " Types defined here will be compiled with bindgen\n and made available in `crate::binding` in rust."]
pub const v8__ScriptOrigin_SIZE: usize = 40;
pub const cppgc__Member_SIZE: usize = 16;
pub const cppgc__WeakMember_SIZE: usize = 16;
pub const v8__TracedReference_SIZE: usize = 8;
pub const v8__Eternal_SIZE: usize = 8;
pub const v8__String__ValueView_SIZE: usize = 32;
pub const v8__String__kMaxLength: ::std::os::raw::c_int = 536870888;
pub const v8__TypedArray__kMaxByteLength: usize = 9007199254740991;
pub const v8__Uint8Array__kMaxLength: usize = 9007199254740991;
pub const v8__Uint8ClampedArray__kMaxLength: usize = 9007199254740991;
pub const v8__Int8Array__kMaxLength: usize = 9007199254740991;
pub const v8__Uint16Array__kMaxLength: usize = 4503599627370495;
pub const v8__Int16Array__kMaxLength: usize = 4503599627370495;
pub const v8__Uint32Array__kMaxLength: usize = 2251799813685247;
pub const v8__Int32Array__kMaxLength: usize = 2251799813685247;
pub const v8__Float32Array__kMaxLength: usize = 2251799813685247;
pub const v8__Float64Array__kMaxLength: usize = 1125899906842623;
pub const v8__BigUint64Array__kMaxLength: usize = 1125899906842623;
pub const v8__BigInt64Array__kMaxLength: usize = 1125899906842623;
pub type v8__CFunction = v8_CFunction;
pub type v8__CFunctionInfo = v8_CFunctionInfo;
pub type v8__FastApiArrayBufferView = v8_FastApiArrayBufferView;
pub type v8__FastOneByteString = v8_FastOneByteString;
pub type v8__FastApiTypedArray = v8_FastApiTypedArray;
#[doc = " Features reported via the SetUseCounterCallback callback. Do not change\n assigned numbers of existing items; add new features to the end of this\n list.\n Dead features can be marked `V8_DEPRECATE_SOON`, then `V8_DEPRECATED`, and\n then finally be renamed to `kOBSOLETE_...` to stop embedders from using\n them."]
pub use self::v8_Isolate_UseCounterFeature as v8__Isolate__UseCounterFeature;
pub const v8__MAJOR_VERSION: u32 = 13;
pub const v8__MINOR_VERSION: u32 = 0;
pub const v8__BUILD_NUMBER: u32 = 245;
pub const v8__PATCH_LEVEL: u32 = 12;
#[allow(unsafe_code)]
pub const v8__VERSION_STRING: &::std::ffi::CStr = unsafe {
  ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"13.0.245.12-rusty\0")
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
  ["Size of template specialization: cppgc_GarbageCollected_open0_RustObj_close0"] [:: std :: mem :: size_of :: < cppgc_GarbageCollected > () - 1usize] ;
  ["Align of template specialization: cppgc_GarbageCollected_open0_RustObj_close0"] [:: std :: mem :: align_of :: < cppgc_GarbageCollected > () - 1usize] ;
};
