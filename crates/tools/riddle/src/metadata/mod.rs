pub use windows_metadata::*;
use std::collections::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct QueryPosition {
    pub object: usize,
    pub guid: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SignatureKind {
    Query(QueryPosition),
    QueryOptional(QueryPosition),
    ResultValue,
    ResultVoid,
    ReturnStruct,
    ReturnValue,
    ReturnVoid,
    PreserveSig,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SignatureParamKind {
    ArrayFixed(usize),
    ArrayRelativeLen(usize),
    ArrayRelativeByteLen(usize),
    ArrayRelativePtr(usize),
    TryInto,
    IntoParam,
    OptionalPointer,
    ValueType,
    Blittable,
    Other,
}

impl SignatureParamKind {
    fn is_array(&self) -> bool {
        matches!(self, Self::ArrayFixed(_) | Self::ArrayRelativeLen(_) | Self::ArrayRelativeByteLen(_) | Self::ArrayRelativePtr(_))
    }
}

pub struct Signature {
    pub def: MethodDef,
    pub params: Vec<SignatureParam>,
    pub return_type: Type,
    pub call_flags: MethodCallAttributes,
}

pub struct SignatureParam {
    pub def: Param,
    pub ty: Type,
    pub kind: SignatureParamKind,
}

pub fn type_def_invoke_method(reader: &Reader, row: TypeDef) -> MethodDef {
    reader
        .type_def_methods(row)
        .find(|method| reader.method_def_name(*method) == "Invoke")
        .expect("`Invoke` method not found")
}

pub fn type_def_generics(reader: &Reader, def: TypeDef) -> Vec<Type> {
    reader.type_def_generics(def).map(Type::GenericParam).collect()    
}

    // TODO: namespace should not be required - it's a hack to accomodate Win32 metadata
    // TODO: this is very Rust-specific and Win32-metadata specific with all of its translation. Replace with literal signature parser that just returns slice of types.
    pub fn method_def_signature(reader: &Reader, namespace: &str, row: MethodDef, generics: &[Type]) -> Signature {
        let mut blob = reader.row_blob(row, 4);
        let call_flags = MethodCallAttributes(blob.read_usize() as u8);
        let _param_count = blob.read_usize();
        let mut return_type = reader.type_from_blob(&mut blob, None, generics);

        let mut params: Vec<SignatureParam> = reader
            .method_def_params(row)
            .filter_map(|param| {
                let param_is_const = reader.has_attribute(param, "ConstAttribute");
                if reader.param_sequence(param) == 0 {
                    if param_is_const {
                        return_type = return_type.clone().to_const_type();
                    }
                    None
                } else {
                    let is_output = reader.param_flags(param).contains(ParamAttributes::Out);
                    let mut ty = reader.type_from_blob(&mut blob, None, generics);

                    if let Some(name) = param_or_enum(reader,param) {
                        let def = reader.get_type_def(TypeName::new(namespace, &name)).next().expect("Enum not found");
                        ty = Type::PrimitiveOrEnum(Box::new(ty), Box::new(Type::TypeDef(def, Vec::new())));
                    }

                    if param_is_const || !is_output {
                        ty = ty.to_const_type();
                    }
                    if !is_output {
                        ty = ty.to_const_ptr();
                    }
                    let kind = param_kind(reader,param);
                    Some(SignatureParam { def: param, ty, kind })
                }
            })
            .collect();

        for position in 0..params.len() {
            // Point len params back to the corresponding ptr params.
            match params[position].kind {
                SignatureParamKind::ArrayRelativeLen(relative) | SignatureParamKind::ArrayRelativeByteLen(relative) => {
                    // The len params must be input only.
                    if !reader.param_flags(params[relative].def).contains(ParamAttributes::Out) && position != relative && !params[relative].ty.is_pointer() {
                        params[relative].kind = SignatureParamKind::ArrayRelativePtr(position);
                    } else {
                        params[position].kind = SignatureParamKind::Other;
                    }
                }
                SignatureParamKind::ArrayFixed(_) => {
                    if reader.has_attribute(params[position].def, "FreeWithAttribute") {
                        params[position].kind = SignatureParamKind::Other;
                    }
                }
                _ => {}
            }
        }

        let mut sets = BTreeMap::<usize, Vec<usize>>::new();

        // Finds sets of ptr params pointing at the same len param.
        for (position, param) in params.iter().enumerate() {
            match param.kind {
                SignatureParamKind::ArrayRelativeLen(relative) | SignatureParamKind::ArrayRelativeByteLen(relative) => {
                    sets.entry(relative).or_default().push(position);
                }
                _ => {}
            }
        }

        // Remove all sets.
        for (len, ptrs) in sets {
            if ptrs.len() > 1 {
                params[len].kind = SignatureParamKind::Other;
                for ptr in ptrs {
                    params[ptr].kind = SignatureParamKind::Other;
                }
            }
        }

        // Remove any byte arrays that aren't byte-sized types.
        for position in 0..params.len() {
            if let SignatureParamKind::ArrayRelativeByteLen(relative) = params[position].kind {
                if !params[position].ty.is_byte_size() {
                    params[position].kind = SignatureParamKind::Other;
                    params[relative].kind = SignatureParamKind::Other;
                }
            }
        }

        for param in &mut params {
            if param.kind == SignatureParamKind::Other {
                if signature_param_is_convertible(reader,param) {
                    if type_is_non_exclusive_winrt_interface(reader, &param.ty) {
                        param.kind = SignatureParamKind::TryInto;
                    } else {
                        param.kind = SignatureParamKind::IntoParam;
                    }
                } else {
                    let flags = reader.param_flags(param.def);
                    if param.ty.is_pointer() && (flags.contains(ParamAttributes::Optional) || reader.has_attribute(param.def, "ReservedAttribute")) {
                        param.kind = SignatureParamKind::OptionalPointer;
                    } else if reader.type_is_primitive(&param.ty) && (!param.ty.is_pointer() || reader.type_is_blittable(&param.ty.deref())) {
                        param.kind = SignatureParamKind::ValueType;
                    } else if reader.type_is_blittable(&param.ty) {
                        param.kind = SignatureParamKind::Blittable;
                    }
                }
            }
        }

        Signature { def: row, params, return_type, call_flags }
    }

    fn param_kind(reader: &Reader, row: Param) -> SignatureParamKind {
        for attribute in reader.attributes(row) {
            match reader.attribute_name(attribute) {
                "NativeArrayInfoAttribute" => {
                    for (_, value) in reader.attribute_args(attribute) {
                        match value {
                            Value::I16(value) => return SignatureParamKind::ArrayRelativeLen(value as usize),
                            Value::I32(value) => return SignatureParamKind::ArrayFixed(value as usize),
                            _ => {}
                        }
                    }
                }
                "MemorySizeAttribute" => {
                    for (_, value) in reader.attribute_args(attribute) {
                        if let Value::I16(value) = value {
                            return SignatureParamKind::ArrayRelativeByteLen(value as usize);
                        }
                    }
                }
                _ => {}
            }
        }
        SignatureParamKind::Other
    }
    // TODO: this is a terribly broken Win32 metadata attribute - need to get rid of it.
    fn param_or_enum(reader: &Reader, row: Param) -> Option<String> {
        reader.find_attribute(row, "AssociatedEnumAttribute").and_then(|attribute| {
            for (_, arg) in reader.attribute_args(attribute) {
                if let Value::String(name) = arg {
                    return Some(name);
                }
            }
            None
        })
    }

    pub fn signature_param_is_borrowed(reader: &Reader, param: &SignatureParam) -> bool {
        type_is_borrowed(reader, &param.ty)
    }

    pub fn signature_param_is_convertible(reader: &Reader, param: &SignatureParam) -> bool {
        !reader.param_flags(param.def).contains(ParamAttributes::Out) && !param.ty.is_winrt_array() && !param.ty.is_pointer() && !param.kind.is_array() && (type_is_borrowed(reader, &param.ty) || type_is_non_exclusive_winrt_interface(reader,&param.ty) || type_is_trivially_convertible(reader, &param.ty))
    }
    fn signature_param_is_retval(reader: &Reader, param: &SignatureParam) -> bool {
        // The Win32 metadata uses `RetValAttribute` to call out retval methods but it is employed
        // very sparingly, so this heuristic is used to apply the transformation more uniformly.
        if reader.has_attribute(param.def, "RetValAttribute") {
            return true;
        }
        if !param.ty.is_pointer() {
            return false;
        }
        if param.ty.is_void() {
            return false;
        }
        let flags = reader.param_flags(param.def);
        if flags.contains(ParamAttributes::In) || !flags.contains(ParamAttributes::Out) || flags.contains(ParamAttributes::Optional) || param.kind.is_array() {
            return false;
        }
        if param_kind(reader,param.def).is_array() {
            return false;
        }
        // If it's bigger than 128 bits, best to pass as a reference.
        if reader.type_size(&param.ty.deref()) > 16 {
            return false;
        }
        // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid
        // producing the `Result<Option<T>>` anti-pattern.
        !type_is_callback(reader,&param.ty.deref())
    }
    pub fn signature_kind(reader: &Reader, signature: &Signature) -> SignatureKind {
        if reader.has_attribute(signature.def, "CanReturnMultipleSuccessValuesAttribute") {
            return SignatureKind::PreserveSig;
        }
        match &signature.return_type {
            Type::Void if signature_is_retval(reader,signature) => SignatureKind::ReturnValue,
            Type::Void => SignatureKind::ReturnVoid,
            Type::HRESULT => {
                if signature.params.len() >= 2 {
                    if let Some(guid) = signature_param_is_query_guid(reader, &signature.params) {
                        if let Some(object) = signature_param_is_query_object(reader,&signature.params) {
                            if reader.param_flags(signature.params[object].def).contains(ParamAttributes::Optional) {
                                return SignatureKind::QueryOptional(QueryPosition { object, guid });
                            } else {
                                return SignatureKind::Query(QueryPosition { object, guid });
                            }
                        }
                    }
                }
                if signature_is_retval(reader,signature) {
                    SignatureKind::ResultValue
                } else {
                    SignatureKind::ResultVoid
                }
            }
            Type::TypeDef(def, _) if reader.type_def_type_name(*def) == TypeName::NTSTATUS => SignatureKind::ResultVoid,
            Type::TypeDef(def, _) if reader.type_def_type_name(*def) == TypeName::WIN32_ERROR => SignatureKind::ResultVoid,
            Type::TypeDef(def, _) if reader.type_def_type_name(*def) == TypeName::BOOL && method_def_last_error(reader, signature.def) => SignatureKind::ResultVoid,
            _ if reader.type_is_struct(&signature.return_type) => SignatureKind::ReturnStruct,
            _ => SignatureKind::PreserveSig,
        }
    }
    fn signature_is_retval(reader: &Reader, signature: &Signature) -> bool {
        signature.params.last().map_or(false, |param| signature_param_is_retval(reader,param))
            && signature.params[..signature.params.len() - 1].iter().all(|param| {
                let flags = reader.param_flags(param.def);
                !flags.contains(ParamAttributes::Out)
            })
    }
    fn signature_param_is_query_guid(reader: &Reader, params: &[SignatureParam]) -> Option<usize> {
        params.iter().rposition(|param| param.ty == Type::ConstPtr(Box::new(Type::GUID), 1) && !reader.param_flags(param.def).contains(ParamAttributes::Out))
    }
    fn signature_param_is_query_object(reader: &Reader, params: &[SignatureParam]) -> Option<usize> {
        params.iter().rposition(|param| param.ty == Type::MutPtr(Box::new(Type::Void), 2) && reader.has_attribute(param.def, "ComOutPtrAttribute"))
    }

    fn method_def_last_error(reader: &Reader, row: MethodDef) -> bool {
        if let Some(map) = reader.method_def_impl_map(row) {
            reader.impl_map_flags(map).contains(PInvokeAttributes::SupportsLastError)
        } else {
            false
        }
    }


    fn type_is_borrowed(reader: &Reader, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => !reader.type_def_is_blittable(*row),
            Type::BSTR | Type::PCSTR | Type::PCWSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => true,
            _ => false,
        }
    }

    pub fn type_is_non_exclusive_winrt_interface(reader: &Reader, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => {
                let flags = reader.type_def_flags(*row);
                if !flags.contains(TypeAttributes::WindowsRuntime) {
                    false
                } else {
                    match reader.type_def_kind(*row) {
                        TypeKind::Interface => !reader.type_def_is_exclusive(*row),
                        TypeKind::Class => reader.has_attribute(*row, "ComposableAttribute"),
                        _ => false,
                    }
                }
            }
            _ => false,
        }
    }

    fn type_is_trivially_convertible(reader: &Reader, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) =>         match reader.type_def_kind(*row) {
                TypeKind::Struct => reader.type_def_is_handle(*row),
                _ => false,
            },
            Type::PCSTR | Type::PCWSTR => true,
            _ => false,
        }
    }

    fn type_is_callback(reader: &Reader, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => type_def_is_callback(reader,*row),
            _ => false,
        }
    }

    fn type_def_is_callback(reader: &Reader, row: TypeDef) -> bool {
        !reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) && reader.type_def_kind(row) == TypeKind::Delegate
    }

    pub fn type_has_callback(reader: &Reader, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => type_def_has_callback(reader, *row),
            Type::Win32Array(ty, _) => type_has_callback(reader, ty),
            _ => false,
        }
    }
    pub fn type_def_has_callback(reader: &Reader, row: TypeDef) -> bool {
        if type_def_is_callback(reader, row) {
            return true;
        }
        if reader.type_def_kind(row) != TypeKind::Struct {
            return false;
        }
        fn check(reader: &Reader, row: TypeDef) -> bool {
            if reader.type_def_fields(row).any(|field| type_has_callback(reader, &reader.field_type(field, Some(row)))) {
                return true;
            }
            false
        }
        let type_name = reader.type_def_type_name(row);
        if type_name.namespace.is_empty() {
            check(reader, row)
        } else {
            for row in reader.get_type_def(type_name) {
                if check(reader, row) {
                    return true;
                }
            }
            false
        }
    }