use fluvio_smartmodule::dataplane::smartmodule::{
    SmartModuleInput, SmartModuleOutput, SmartModuleTransformErrorStatus,
};
use wasmtime::{TypedFunc, AsContextMut};
use anyhow::Result;

use crate::{
    instance::{SmartModuleInstanceContext, SmartModuleTransform},
    state::WasmState,
};

type WasmFn = TypedFunc<(i32, i32, u32), i32>;

pub const FILTER_FN_NAME: &str = "filter";
pub const MAP_FN_NAME: &str = "map";
pub const FILTER_MAP_FN_NAME: &str = "filter_map";
pub const ARRAY_MAP_FN_NAME: &str = "array_map";

pub struct SimpleTansform {
    f: WasmFn,
    name: String,
}

impl std::fmt::Debug for SimpleTansform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name.as_str() {
            FILTER_FN_NAME => write!(f, "FilterFn"),
            MAP_FN_NAME => write!(f, "MapFnWithParam"),
            FILTER_MAP_FN_NAME => write!(f, "FilterMapFnWithParam"),
            ARRAY_MAP_FN_NAME => write!(f, "ArrayMapFnWithParam"),
            _ => unreachable!("unknown transform function"),
        }
    }
}

impl SimpleTansform {
    #[tracing::instrument(skip(ctx, store))]
    pub(crate) fn try_instantiate(
        name: &str,
        ctx: &SmartModuleInstanceContext,
        store: &mut impl AsContextMut,
    ) -> Result<Option<Self>> {
        match ctx.get_wasm_func(store, name) {
            Some(func) => {
                // check type signature
                func.typed(&mut *store)
                    .or_else(|_| func.typed(&mut *store))
                    .map(|f| {
                        Some(Self {
                            f,
                            name: name.to_string(),
                        })
                    })
            }
            None => Ok(None),
        }
    }
}

impl SmartModuleTransform for SimpleTansform {
    fn process(
        &mut self,
        input: SmartModuleInput,
        ctx: &mut SmartModuleInstanceContext,
        store: &mut WasmState,
    ) -> Result<SmartModuleOutput> {
        let slice = ctx.write_input(&input, &mut *store)?;
        let map_output = self.f.call(&mut *store, slice)?;

        if map_output < 0 {
            let internal_error = SmartModuleTransformErrorStatus::try_from(map_output)
                .unwrap_or(SmartModuleTransformErrorStatus::UnknownError);
            return Err(internal_error.into());
        }

        let output: SmartModuleOutput = ctx.read_output(store)?;
        Ok(output)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
