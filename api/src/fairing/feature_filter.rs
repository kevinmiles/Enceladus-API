use hashbrown::HashSet;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Request,
    Response,
};
use serde_json::{Map, Value as Json};
use std::io::Cursor;

/// Remove any feature-specific fields unless requested.
///
/// A feature-specific field is one whose key contains two consecutive underscores.
/// All characters preceding those underscores is the feature name.
///
/// To enable a feature,
/// you should use the `feature` queryparam,
/// passing a comma separated list of features to enable.
///
/// By default, no features are enabled.
///
/// Usage:
/// ```rust
/// rocket::ignite.attach(FeatureFilter::default()).launch()
/// ```
#[derive(Default)]
pub struct FeatureFilter;

impl Fairing for FeatureFilter {
    /// Give Rocket some information about the fairing, including when to call it.
    #[inline]
    fn info(&self) -> Info {
        Info {
            name: "Feature filter",
            kind: Kind::Response,
        }
    }

    /// After a request is completed,
    /// call `filter_array` and `filter_object` as necessary to remove any unwanted fields.
    ///
    /// FIXME Is there any valid use case for an "all" feature flag?
    #[inline]
    fn on_response(&self, request: &Request<'_>, response: &mut Response<'_>) {
        let features: String = request
            .get_query_value("features")
            .unwrap_or_else(|| Ok("".into()))
            .unwrap();
        let features: &HashSet<String> = &features.split(',').map(str::to_lowercase).collect();

        let mut body: Json = {
            let body_string = response.body_string();
            if body_string.is_none() {
                // Error converting the body to a String;
                // there aren't any fields to remove.
                return;
            }
            let body_string = body_string.unwrap();

            match serde_json::from_str(&body_string) {
                Ok(body) => body,
                Err(_) => {
                    // Not a JSON body, so there's no fields to remove.
                    return response.set_sized_body(Cursor::new(body_string));
                }
            }
        };

        if body.is_array() {
            filter_array(body.as_array_mut().unwrap(), features);
        } else if body.is_object() {
            filter_object(body.as_object_mut().unwrap(), features);
        }

        response.set_sized_body(Cursor::new(body.to_string()));
    }
}

/// Recursively filter the fields of an object in-place.
#[inline]
fn filter_object(object: &mut Map<String, Json>, features: &HashSet<String>) {
    for (key, _) in object.clone().iter() {
        let value = &mut object[key];

        // Recursively reach each value.
        if value.is_object() {
            filter_object(value.as_object_mut().unwrap(), features);
        } else if value.is_array() {
            filter_array(value.as_array_mut().unwrap(), features)
        }

        // This field requires a feature that wasn't requested.
        if key.contains("__")
            && !features.contains(&*key.splitn(2, "__").next().unwrap().to_lowercase())
        {
            object.remove(key);
        }
    }
}

/// Recursively filter the fields of any child objects of an array in-place.
#[inline]
fn filter_array(array: &mut Vec<Json>, features: &HashSet<String>) {
    for (i, _) in array.clone().iter().enumerate() {
        let val = &mut array[i];

        if val.is_object() {
            filter_object(val.as_object_mut().unwrap(), features);
        } else if val.is_array() {
            filter_array(val.as_array_mut().unwrap(), features);
        }
    }
}
