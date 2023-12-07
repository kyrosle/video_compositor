use std::{collections::HashMap, ops::Deref, rc::Rc};

use schemars::schema::{
    InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec, SubschemaValidation,
};

use crate::type_definition::{Kind, ObjectProperty, TypeDefinition};

#[derive(Debug)]
pub struct DocPage {
    pub title: Rc<str>,
    // TODO: Consider using HashMap with ordering
    definitions: Vec<TypeDefinition>,
}

impl DocPage {
    pub fn new(title: Rc<str>) -> Self {
        Self {
            title,
            definitions: Vec::new(),
        }
    }

    pub fn add_definition(&mut self, def: TypeDefinition) {
        self.definitions.push(def);
    }

    pub fn remove_definition(&mut self, name: &Rc<str>) {
        self.definitions
            .retain(|def| def.name.as_ref() != Some(name))
    }

    pub fn contains_definition(&self, name: &Rc<str>) -> bool {
        self.definitions
            .iter()
            .any(|def| def.name.as_ref() == Some(name))
    }

    fn simplify(&mut self) {
        fn merge_descriptions(desc1: Option<Rc<str>>, desc2: Option<Rc<str>>) -> Option<Rc<str>> {
            match (desc1, desc2) {
                (Some(desc1), Some(desc2)) => {
                    let separator = if !desc1.ends_with('.') { ". " } else { " " };
                    Some(format!("{desc1}{separator}{desc2}").into())
                }
                (Some(desc1), None) => Some(desc1),
                (None, Some(desc2)) => Some(desc2),
                (None, None) => None,
            }
        }

        fn inline_definition(
            def: &mut TypeDefinition,
            inline_definitions: &HashMap<Rc<str>, TypeDefinition>,
        ) {
            match &mut def.kind {
                Kind::Ref(reference) => {
                    if let Some(inline_def) = inline_definitions.get(reference) {
                        let description = merge_descriptions(
                            def.description.clone(),
                            inline_def.description.clone(),
                        );
                        *def = inline_def.clone();
                        def.description = description;
                    }
                }
                Kind::Tuple(types) | Kind::Union(types) => types
                    .iter_mut()
                    .for_each(|def| inline_definition(def, inline_definitions)),
                Kind::Array(ty) => inline_definition(ty, inline_definitions),
                Kind::Object(properties) => properties
                    .iter_mut()
                    .for_each(|prop| inline_definition(&mut prop.type_def, inline_definitions)),
                _ => {}
            }
        }

        fn simplify_single_variant_unions(def: &mut TypeDefinition) {
            match &mut def.kind {
                Kind::Tuple(types) => types.iter_mut().for_each(simplify_single_variant_unions),
                Kind::Array(def) => simplify_single_variant_unions(def),
                Kind::Union(variants) => {
                    if variants.len() == 1 {
                        let variant_def = variants.remove(0);
                        let description = merge_descriptions(
                            def.description.clone(),
                            variant_def.description.clone(),
                        );

                        *def = variant_def.clone();
                        def.description = description;
                    } else {
                        variants.iter_mut().for_each(simplify_single_variant_unions)
                    }
                }
                Kind::Object(properties) => properties
                    .iter_mut()
                    .for_each(|prop| simplify_single_variant_unions(&mut prop.type_def)),
                _ => {}
            }
        }

        let mut inline_definitions = HashMap::new();
        for def in self.definitions.iter() {
            let should_inline = match &def.kind {
                Kind::Null
                | Kind::I32
                | Kind::F32
                | Kind::F64
                | Kind::U32
                | Kind::U16
                | Kind::Bool
                | Kind::String(_) => true,
                Kind::Union(types) => types
                    .iter()
                    .all(|def: &TypeDefinition| matches!(def.kind, Kind::String(_))),
                _ => false,
            };

            if should_inline {
                inline_definitions.insert(def.name.clone().unwrap(), def.clone());
            }
        }

        for name in inline_definitions.keys() {
            self.remove_definition(name);
        }

        for def in self.definitions.iter_mut() {
            inline_definition(def, &inline_definitions);
            simplify_single_variant_unions(def);
        }
    }

    pub fn to_markdown(&self) -> String {
        self.definitions
            .iter()
            .map(TypeDefinition::to_markdown)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn generate_component_docs(
    root_schema: &RootSchema,
    name: &str,
    variant_name: &str,
) -> DocPage {
    generate_docs_from_enum(root_schema, "type", name, variant_name)
}

pub fn generate_renderer_docs(root_schema: &RootSchema, name: &str, variant_name: &str) -> DocPage {
    generate_docs_from_enum(root_schema, "entity_type", name, variant_name)
}

pub fn generate_docs_from_enum(
    root_schema: &RootSchema,
    type_key_name: &str,
    name: &str,
    variant_name: &str,
) -> DocPage {
    let retrieve_type_value = |schema: &SchemaObject| -> String {
        let properties = &schema.object.as_ref().unwrap().properties;
        let type_enum: Vec<serde_json::Value> = properties
            .get(type_key_name)
            .unwrap()
            .clone()
            .into_object()
            .enum_values
            .unwrap();
        type_enum[0].as_str().unwrap().to_owned()
    };

    let name: Rc<str> = name.into();
    let mut page = DocPage::new(name.clone());
    let subschemas = flatten_subschemas(root_schema.schema.subschemas.as_ref().unwrap());
    let schema = subschemas
        .iter()
        .find(|schema| retrieve_type_value(schema) == variant_name)
        .unwrap();

    populate_page(&mut page, name, schema, root_schema);
    page.simplify();
    page
}

fn populate_page(
    page: &mut DocPage,
    name: Rc<str>,
    schema: &SchemaObject,
    root_schema: &RootSchema,
) {
    let root_schema_name = root_schema
        .schema
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.title.as_ref())
        .map(String::as_str);
    let mut definition = parse_schema(schema);
    definition.name = Some(name.clone());

    let references = definition.references.clone();
    page.add_definition(definition);

    // Parse every definition mentioned in `schema`
    for refer in references {
        if root_schema_name == Some(refer.deref()) {
            continue;
        }
        if page.contains_definition(&refer) {
            continue;
        }
        let Some(schema) = root_schema.definitions.get(refer.deref()) else {
            continue;
        };
        populate_page(page, refer, &schema.clone().into_object(), root_schema);
    }
}

fn parse_schema(schema: &SchemaObject) -> TypeDefinition {
    let (name, description) = schema
        .metadata
        .clone()
        .map(|metadata| (metadata.title, metadata.description))
        .unwrap_or_default();
    if let Some(reference) = &schema.reference {
        return TypeDefinition::complex(
            name,
            description,
            Kind::Ref(reference.as_str().into()),
            false,
        );
    }

    let (ty, is_optional) = match &schema.instance_type {
        Some(SingleOrVec::Single(ty)) => (ty.deref(), false),
        Some(SingleOrVec::Vec(types)) => match types.as_slice() {
            [ty, InstanceType::Null] => (ty, true),
            [InstanceType::Null, ty] => (ty, true),
            types => unimplemented!("Unsupported type: Vec({types:?})"),
        },
        None => {
            if let Some(subschemas) = &schema.subschemas {
                let mut types = flatten_subschemas(subschemas)
                    .iter()
                    .map(parse_schema)
                    .collect::<Vec<_>>();
                let is_optional = types.iter().any(|def| def.kind == Kind::Null);
                types.retain(|def| def.kind != Kind::Null);

                return TypeDefinition::complex(name, description, Kind::Union(types), is_optional);
            }

            unimplemented!("Unsupported type");
        }
    };

    let ty = match ty {
        InstanceType::Boolean => Kind::Bool,
        InstanceType::Array => parse_array_or_tuple(schema),
        InstanceType::String => match &schema.enum_values {
            Some(values) if values.is_empty() => Kind::String(None),
            Some(values) if values.len() == 1 => Kind::String(values[0].as_str().map(Into::into)),
            Some(values) => Kind::Union(
                values
                    .iter()
                    .map(|v| {
                        TypeDefinition::simple(
                            Kind::String(v.as_str().map(Into::into)),
                            is_optional,
                        )
                    })
                    .collect(),
            ),
            None => Kind::String(None),
        },
        InstanceType::Number => match schema.format.as_ref().unwrap().as_str() {
            "float" => Kind::F32,
            "double" => Kind::F64,
            format => unimplemented!("Unknown number format \"{format}\""),
        },
        InstanceType::Integer => match schema.format.as_ref().unwrap().as_str() {
            "uint32" | "uint" => Kind::U32,
            "int32" | "int" => Kind::I32,
            "uint16" => Kind::U16,
            format => unimplemented!("Unknown integer format \"{format}\""),
        },
        InstanceType::Object => parse_object(schema),
        InstanceType::Null => Kind::Null,
    };

    TypeDefinition::complex(name.clone(), description.clone(), ty, is_optional)
}

fn parse_object(schema: &SchemaObject) -> Kind {
    let mut properties = Vec::new();
    let object = schema.object.as_ref().unwrap();
    for (name, prop) in object.properties.clone() {
        let prop = prop.into_object();
        properties.push(ObjectProperty {
            name: name.into(),
            type_def: parse_schema(&prop),
        })
    }

    if let Some(subschemas) = &schema.subschemas {
        let objects = flatten_subschemas(subschemas)
            .iter()
            .map(parse_schema)
            .map(|mut def| {
                let ty = match def.kind {
                    Kind::Object(sub_properties) => {
                        Kind::Object([sub_properties, properties.clone()].concat())
                    }
                    _ => unreachable!("Expected object"),
                };

                def.kind = ty;
                def
            })
            .collect::<Vec<_>>();
        return Kind::Union(objects);
    }

    Kind::Object(properties)
}

fn parse_array_or_tuple(schema: &SchemaObject) -> Kind {
    let array = schema.array.as_ref().unwrap();
    match (array.min_items, array.max_items) {
        (Some(min), Some(max)) if min == max => {
            let Some(SingleOrVec::Vec(items)) = &array.items else {
                panic!("Expected typle types");
            };
            let tuple_ty = items
                .iter()
                .cloned()
                .map(|schema| parse_schema(&schema.into_object()))
                .collect();
            Kind::Tuple(tuple_ty)
        }
        _ => {
            let Some(SingleOrVec::Single(items)) = &array.items else {
                panic!("Arrays can accept only one type");
            };
            let array_ty = parse_schema(&items.clone().into_object());
            Kind::Array(Box::new(array_ty))
        }
    }
}

fn flatten_subschemas(subschemas: &SubschemaValidation) -> Vec<SchemaObject> {
    let mut schemas = Vec::new();

    // These subschemas are represented by an enum in rust. Only one variant can be used at the time.
    if let Some(mut one_of) = subschemas.one_of.clone() {
        schemas.append(&mut one_of);
    }
    if let Some(mut any_of) = subschemas.any_of.clone() {
        schemas.append(&mut any_of);
    }
    if let Some(mut all_of) = subschemas.all_of.clone() {
        schemas.append(&mut all_of);
    }

    schemas.into_iter().map(Schema::into_object).collect()
}