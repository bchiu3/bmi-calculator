use std::any::Any;

use serde::{Serialize, Deserialize};
use struct_field_names_as_array::FieldNamesAsArray;
use yew::Properties;


pub trait Units {
    fn get_weight_fields(&self) -> Vec<&str>;
    fn get_height_fields(&self) -> Vec<&str>;
    fn set_value(&mut self, field: &str, value: f32);
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
    pub cm: f32,
    pub kg: f32
}

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize, FieldNamesAsArray)]
pub struct ImperialUnit {
    pub ft: f32,
    pub inches: f32,
    pub st: f32,
    pub lbs: f32

}

impl MetricUnit {
    pub fn new() -> Self {
        MetricUnit {
            cm: 0.0,
            kg: 0.0
        }
    }
}

impl ImperialUnit {
    pub fn new() -> Self {
        ImperialUnit {
            ft: 0.0,
            inches: 0.0,
            st: 0.0,
            lbs: 0.0
        }
    }
}

impl Units for MetricUnit {
    fn get_weight_fields(&self) -> Vec<&str> {
        vec!["kg"]
    }

    fn get_height_fields(&self) -> Vec<&str> {
        vec!["cm"]
    }

    fn get_value(&self, field: &str) -> f32 {
        match field {
            "kg" => self.kg,
            "cm" => self.cm,
            _ => 0.0
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_value(&mut self, field: &str, value: f32) {
        match field {
            "kg" => self.kg = value,
            "cm" => self.cm = value,
            _ => ()
        }
    }
}

impl Units for ImperialUnit {
    fn get_weight_fields(&self) -> Vec<&str> {
        vec!["st", "lbs"]
    }

    fn get_height_fields(&self) -> Vec<&str> {
        vec!["ft", "in"]
    }

    fn get_value(&self, field: &str) -> f32 {
        match field {
            "ft" => self.ft,
            "in" => self.inches,
            "st" => self.st,
            "lbs" => self.lbs,
            _ => 0.0
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_value(&mut self, field: &str, value: f32) {
        match field {
            "ft" => self.ft = value,
            "in" => self.inches = value,
            "st" => self.st = value,
            "lbs" => self.lbs = value,
            _ => ()
        }
    }
}