use openapiv3::{OpenAPI, SchemaKind, Type};
use serde_yaml::Value;

pub fn modify_sendgrid(mut yaml: Value) -> OpenAPI {
     let mut spec: OpenAPI =
        serde_yaml::from_value(yaml).expect("Could not structure OpenAPI file.");
    spec.paths.paths.get_mut("/v3/contactdb/recipients/search")
        .unwrap()
        .as_mut()
        .unwrap()
        .get = None;
    spec
}

pub fn modify_recurly(mut yaml: Value) -> OpenAPI {
    println!("modifying recurly:\n{}", serde_json::to_string(&yaml).unwrap());
    yaml["paths"]["/invoices/{invoice_id}/apply_credit_balance"]["put"]["parameters"].as_sequence_mut().unwrap().retain(|param| {
        param["$ref"].as_str().unwrap() != "#/components/parameters/site_id"
    });
    serde_yaml::from_value(yaml).unwrap()
}

pub fn modify_openai(mut yaml: Value) -> OpenAPI {
    let mut spec: OpenAPI =
        serde_yaml::from_value(yaml).expect("Could not structure OpenAPI file.");
    spec.security = vec![{
        let mut map = indexmap::IndexMap::new();
        map.insert("Bearer".to_string(), vec![]);
        map
    }];
    spec.security_schemes.insert("Bearer".to_string(), openapiv3::ReferenceOr::Item(openapiv3::SecurityScheme::HTTP {
        scheme: "bearer".to_string(),
        bearer_format: None,
        description: None,
    }));
    spec
}