use windows_bindgen as r;
use windows_ecma335 as w;

fn main() {
    let time = std::time::Instant::now();

    let input = r::Reader::new(vec![r::File::new(std::fs::read("crates/libs/bindgen/default/Windows.winmd").unwrap()).unwrap()]);

    let mut output = w::File::new("test");

    for ty in input.values().flat_map(|values|values.values()).flatten() {
        write_type(&mut output, ty);
    }

    let bytes = output.into_stream();
    std::fs::write("copy.winmd", bytes).unwrap();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

fn write_type(output: &mut w::File, ty: &r::Type) {
    match ty {
        r::Type::Enum(ty) => write_def(output, ty.def),
        _ => {}
    }
}

fn write_def(output: &mut w::File, def: r::TypeDef) {
    println!("{}.{}", def.namespace(), def.name());

    let flags = w::TypeAttributes(def.flags().0);

    let extends = if let Some(extends) = def.extends() {
        w::TypeDefOrRef::TypeRef(output.TypeRef(extends.namespace(), extends.name()))
    } else {
        w::TypeDefOrRef::default()
    };

    output.TypeDef(def.namespace(), def.name(), extends, flags);

    for field in def.fields() {
        let flags = w::FieldAttributes(field.flags().0);
        let signature = output.FieldSig(&convert_type(&field.ty(None)));

        output.Field(field.name(), signature, flags);
    }
}

fn convert_type(input: &r::Type) -> w::Type<'static> {
    match input {
        r::Type::Void => w::Type::Void,
        r::Type::Bool => w::Type::Bool,
        r::Type::Char => w::Type::Char,
        r::Type::I8 => w::Type::I8,
        r::Type::U8 => w::Type::U8,
        r::Type::I16 => w::Type::I16,
        r::Type::U16 => w::Type::U16,
        r::Type::I32 => w::Type::I32,
        r::Type::U32 => w::Type::U32,
        r::Type::I64 => w::Type::I64,
        r::Type::U64 => w::Type::U64,
        r::Type::F32 => w::Type::F32,
        r::Type::F64 => w::Type::F64,
        r::Type::ISize => w::Type::ISize,
        r::Type::USize => w::Type::USize,
        r::Type::String => w::Type::String,
        r::Type::Object => w::Type::Object,
        r::Type::Enum(ty) => w::Type::new(ty.def.namespace(), ty.def.name()),
        rest => panic!("{rest:?}"),
    }
}
