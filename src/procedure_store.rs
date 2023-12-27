use std::{borrow::Cow, convert::Infallible};

use specta::{ts, DataType, DataTypeFrom, NamedDataType, StructType, Type, TypeMap};

use crate::layer::DynLayer;

/// @internal
#[derive(DataTypeFrom)]
#[cfg_attr(test, derive(specta::Type))]
pub struct ProceduresDef {
    #[specta(type = ProcedureDef)]
    queries: Vec<ProcedureDef>,
    #[specta(type = ProcedureDef)]
    mutations: Vec<ProcedureDef>,
    #[specta(type = ProcedureDef)]
    subscriptions: Vec<ProcedureDef>,
}

impl ProceduresDef {
    pub fn new<'a, TCtx: 'a>(
        queries: impl Iterator<Item = &'a ProcedureTodo<TCtx>>,
        mutations: impl Iterator<Item = &'a ProcedureTodo<TCtx>>,
        subscriptions: impl Iterator<Item = &'a ProcedureTodo<TCtx>>,
    ) -> Self {
        ProceduresDef {
            queries: queries.map(|i| &i.ty).cloned().collect(),
            mutations: mutations.map(|i| &i.ty).cloned().collect(),
            subscriptions: subscriptions.map(|i| &i.ty).cloned().collect(),
        }
    }

    pub fn to_named(self) -> NamedDataType {
        let struct_type: StructType = self.into();
        struct_type.to_named("Procedures")
    }
}

/// Represents a Typescript procedure file which is generated by the Rust code.
/// This is codegenerated Typescript file is how we can validate the types on the frontend match Rust.
///
/// @internal
#[derive(Debug, Clone, DataTypeFrom)]
#[cfg_attr(test, derive(specta::Type))]
pub struct ProcedureDef {
    pub key: Cow<'static, str>,
    #[specta(type = serde_json::Value)]
    pub input: DataType,
    #[specta(type = serde_json::Value)]
    pub result: DataType,
    #[specta(type = serde_json::Value)]
    pub error: DataType,
}

fn never() -> DataType {
    Infallible::inline(&mut Default::default(), &[])
}

impl ProcedureDef {
    pub fn from_tys<TArg, TResult, TError>(
        key: Cow<'static, str>,
        type_map: &mut TypeMap,
    ) -> Result<Self, ts::ExportError>
    where
        TArg: Type,
        TResult: Type,
        TError: Type,
    {
        Ok(ProcedureDef {
            key,
            input: match TArg::reference(type_map, &[]).inner {
                DataType::Tuple(tuple) if tuple.elements().is_empty() => never(),
                t => t,
            },
            result: TResult::reference(type_map, &[]).inner,
            error: TError::reference(type_map, &[]).inner,
        })
    }
}

// TODO: Rename this
pub struct ProcedureTodo<TCtx> {
    // TODO: Back to `pub(crate)`
    pub exec: Box<dyn DynLayer<TCtx>>,
    pub ty: ProcedureDef,
}

impl<TCtx> ProcedureTodo<TCtx> {
    #[cfg(feature = "unstable")]
    pub fn ty(&self) -> &ProcedureDef {
        &self.ty
    }
}

// TODO: Using track caller style thing for the panics in this function
// pub fn build<TCtx>(
//     key: Cow<'static, str>,
//     ctx: &mut Router2<TCtx>,
//     kind: ProcedureKind,
//     layer: impl Layer<TCtx> + 'static,
// ) where
//     TCtx: Send + 'static,
// {
//     let (map, type_name) = match kind {
//         ProcedureKind::Query => (&mut ctx.queries, "query"),
//         ProcedureKind::Mutation => (&mut ctx.mutations, "mutation"),
//         ProcedureKind::Subscription => (&mut ctx.subscriptions, "subscription"),
//     };

//     let key_org = key;
//     let key = key_org.to_string();
//     let type_def = layer
//         .into_procedure_def(key_org, &mut ctx.typ_store)
//         .expect("error exporting types");

//     // TODO: Cleanup this logic and do better router merging
//     #[allow(clippy::panic)]
//     if key.is_empty() || key == "ws" || key.starts_with("rpc.") || key.starts_with("rspc.") {
//         panic!("rspc error: attempted to create {type_name} operation named '{key}', however this name is not allowed.");
//     }

//     #[allow(clippy::panic)]
//     if map.contains_key(&key) {
//         panic!("rspc error: {type_name} operation already has resolver with name '{key}'");
//     }

//     map.insert(
//         key,
//         ProcedureTodo {
//             exec: boxed(layer),
//             ty: type_def,
//         },
//     );
// }
