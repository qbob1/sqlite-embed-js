use std::collections::HashMap;

use quick_js::{Context, JsValue};
use serde_json::Value;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, define_scalar_function, Result};

fn convert(value: &JsValue) -> Option<Value> {
    match value {
        JsValue::Undefined => Some(Value::Null),
        JsValue::Null => Some(Value::Null),
        JsValue::Bool(b) => Some(Value::Bool(*b)),
        JsValue::Int(n) => Some(Value::Number(serde_json::Number::from(*n))),
        JsValue::Float(f) => Some(Value::Number(serde_json::Number::from_f64(*f).unwrap())),
        JsValue::String(s) => Some(Value::String(s.to_string())),
        JsValue::Object(o) => jsobjct_to_serdevalue(&o),
        JsValue::Array(a) => jsarray_to_serdevalue(&a),
        _ => None,
    }
}

fn jsarray_to_serdevalue(obj: &Vec<JsValue>) -> Option<Value> {
    let mut arr = Vec::new();
    for value in obj.iter() {
        let json_value = convert(&value);
        arr.push(json_value.unwrap());
    }
    Some(Value::Array(arr))
}

fn jsobjct_to_serdevalue(obj: &HashMap<String, JsValue>) -> Option<Value> {
    let mut map = serde_json::Map::new();
    for (key, value) in obj {
        let json_value = convert(&value);
        map.insert(key.to_string(), json_value.unwrap());
    }
    Some(Value::Object(map))
}

pub fn js(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ctx = Context::new().unwrap();
    let js = api::value_text_notnull(values.get(0).expect("1st argument as js"))?;
    let value = ctx.eval(js).unwrap();
    let _d = match value {
        JsValue::Undefined => api::result_text(context, "undefined"),
        JsValue::Null => Ok(api::result_null(context)),
        JsValue::Bool(b) => Ok(api::result_bool(context, b)),
        JsValue::Int(n) => Ok(api::result_int(context, n)),
        JsValue::Float(f) => Ok(api::result_double(context, f)),
        JsValue::String(s) => api::result_text(context, &s),
        JsValue::Object(o) => api::result_json(context, jsobjct_to_serdevalue(&o).unwrap()),
        JsValue::Array(a) => api::result_json(context, jsarray_to_serdevalue(&a).unwrap()),
        _ => todo!(),
    };
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_embedjs_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(
        db,
        "js",
        1,
        js,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;
    Ok(())
}
