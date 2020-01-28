use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::parameter::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Key {
    String(String),
    Id(TypeId),
}

/// Used to store parameters passed to a [`RegisteredType`]. The parameters are
/// later used in [`Component::build`]
///
/// [`RegisteredType`]: ../container/struct.RegisteredType.html
/// [`Component::build`]: ../component/trait.Component.html#tymethod.build
#[derive(Debug)]
pub struct ParameterMap {
    map: HashMap<Key, Parameter>,
}

impl ParameterMap {
    pub(crate) fn new() -> Self {
        ParameterMap {
            map: HashMap::new(),
        }
    }

    /// Insert a parameter based on property name. If a parameter was already inserted
    /// with that name and type (via this method), the old value is returned.
    pub(crate) fn insert_with_name<V: Any>(&mut self, key: &str, value: V) -> Option<V> {
        self.map
            .insert(Key::String(key.to_string()), Parameter::new(key, value))
            .and_then(Parameter::get_value)
    }

    /// Insert a parameter based on property type. If a parameter was already inserted
    /// with that type (via this method), the old value is returned.
    pub(crate) fn insert_with_type<V: Any>(&mut self, value: V) -> Option<V> {
        self.map
            .insert(
                Key::Id(TypeId::of::<V>()),
                Parameter::new("(dummy name)", value),
            )
            .and_then(Parameter::get_value)
    }

    /// Remove a parameter based on property name. It must have been inserted
    /// via [`with_named_parameter`]
    ///
    /// [`with_named_parameter`]: ../container/struct.RegisteredType.html#method.with_named_parameter
    pub fn remove_with_name<V: Any>(&mut self, key: &str) -> Option<V> {
        let key = Key::String(key.to_string());
        let parameter = self.map.get(&key)?;

        if parameter.type_id == TypeId::of::<V>() {
            self.map.remove(&key).and_then(Parameter::get_value)
        } else {
            None
        }
    }

    /// Remove a parameter based on property type. It must have been inserted
    /// via [`with_typed_parameter`]
    ///
    /// [`with_typed_parameter`]: ../container/struct.RegisteredType.html#method.with_typed_parameter
    pub fn remove_with_type<V: Any>(&mut self) -> Option<V> {
        let key = Key::Id(TypeId::of::<V>());
        let parameter = self.map.get(&key)?;

        if parameter.type_id == TypeId::of::<V>() {
            self.map.remove(&key).and_then(Parameter::get_value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_map() {
        let mut map = ParameterMap::new();

        // Can insert any type of value
        map.insert_with_name("key 1", "value 1".to_string());
        map.insert_with_name("key 2", "value 2");
        map.insert_with_name("key 3", 123 as usize);
        map.insert_with_name("key 4", 123.323 as f32);
        map.insert_with_name("key 4", true);

        // Can get typed data back
        let x = map.remove_with_name::<String>("key 1").unwrap();
        assert_eq!(x, "value 1".to_string());

        // Can't cast into anything
        let x = map.remove_with_name::<Parameter>("key 2");
        assert!(x.is_none());

        assert_eq!(
            map.remove_with_name::<usize>(&"key 3".to_string()).unwrap(),
            123
        );
        assert_eq!(map.remove_with_name::<bool>("key 4").unwrap(), true); // overwrite data

        let mut map = ParameterMap::new();

        // Can insert any type of value
        map.insert_with_type("value 1".to_string());
        map.insert_with_type("value 2");
        map.insert_with_type(123 as usize);
        map.insert_with_type(123.323 as f32);
        map.insert_with_type(true);

        // Can get typed data back
        let x = map.remove_with_type::<String>().unwrap();
        assert_eq!(x, "value 1".to_string());

        // Can't remove anything
        let x = map.remove_with_type::<Parameter>();
        assert!(x.is_none());

        assert_eq!(map.remove_with_type::<usize>().unwrap(), 123);
        assert_eq!(map.remove_with_type::<bool>().unwrap(), true); // overwrite data
    }
}
