use std::any::Any;

use serde::{Serialize, Deserialize};
use struct_field_names_as_array::FieldNamesAsArray;
use yew::Properties;


pub trait Units {
    fn get_fields(&self) -> Vec<&str>;
    fn get_value(&self, field: &str) -> f32;
    fn as_any(&self) -> &dyn Any;
}


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CalcType {
    Metric(MetricUnit),
    Imperial(ImperialUnit)
}

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize, FieldNamesAsArray)]
pub struct MetricUnit {
    pub height: MetricHeightUnit,
    pub weight: MetricWeightUnit
}

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize, FieldNamesAsArray)]
pub struct ImperialUnit {
    pub height: ImperialHeightUnit,
    pub weight: ImperialWeightUnit
}

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize, FieldNamesAsArray)]
pub struct MetricHeightUnit {
    pub cm: f32
}

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize, FieldNamesAsArray)]
pub struct MetricWeightUnit {
    pub kg: f32
}

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize, FieldNamesAsArray)]
pub struct ImperialHeightUnit {
    pub ft: f32,
    pub inches: f32
}

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize, FieldNamesAsArray)]
pub struct ImperialWeightUnit {
    pub st: f32,
    pub lbs: f32
}

impl MetricUnit {
    pub fn new() -> Self {
        Self {
            height: MetricHeightUnit {cm: 0.0},
            weight: MetricWeightUnit {kg: 0.0}
        }
    }
}

impl ImperialUnit {
    pub fn new() -> Self {
        Self {
            height: ImperialHeightUnit {ft: 0.0, inches: 0.0},
            weight: ImperialWeightUnit {st: 0.0, lbs: 0.0}
        }
    }
}

impl Units for MetricHeightUnit {
    fn get_fields(&self) -> Vec<&str> {
        vec!["cm"]
    }
    fn get_value(&self, field: &str) -> f32 {
        match field {
            "cm" => self.cm,
            _ => 0.0
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Units for MetricWeightUnit {
    fn get_fields(&self) -> Vec<&str> {
        vec!["kg"]
    }
    fn get_value(&self, field: &str) -> f32 {
        match field {
            "kg" => self.kg,
            _ => 0.0
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Units for ImperialHeightUnit {
    fn get_fields(&self) -> Vec<&str> {
        vec!["ft", "in"]
    }
    fn get_value(&self, field: &str) -> f32 {
        match field {
            "ft" => self.ft,
            "in" => self.inches,
            _ => 0.0
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Units for ImperialWeightUnit {
    fn get_fields(&self) -> Vec<&str> {
        vec!["st", "lbs"]
    }
    fn get_value(&self, field: &str) -> f32 {
        match field {
            "st" => self.st,
            "lbs" => self.lbs,
            _ => 0.0
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}