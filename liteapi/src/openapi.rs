use crate::Routes;
use serde_json::json;

pub(crate) async unsafe fn openapi(routes: &Routes) -> String {
    // Create a new JSON object to represent the OpenAPI schema
    let mut schema = json!({
        "openapi": "3.0.0",
        "info": {
            "title": "LiteAPI"
        },
        "paths": {}
    });

    // Iterate over the routes
    for (path, (method, _)) in routes {
        let method = method.to_lowercase();

        // Check if the path already exists in the schema
        if !schema["paths"][&path].is_object() {
            // If not, create a new object for the path
            schema["paths"][&path] = json!({});
        }

        // Add the method to the path object
        schema["paths"][&path][&method] = json!({
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        });
    }

    serde_json::to_string_pretty(&schema).unwrap()
}
