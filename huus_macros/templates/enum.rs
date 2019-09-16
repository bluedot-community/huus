{% let data_name = spec.name.to_data() %}
{% let value_name = spec.name.to_value() %}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum {{ data_name }} {
    {% for choice in spec.choices %}
        {{ choice.rust_name }},
    {% endfor %}
}

impl huus::conversions::HuusKey for {{ data_name }} {
    fn from_str(string: &str) -> Result<Self, huus::errors::ConversionError> {
        match string {
            {% for choice in spec.choices %}
                "{{ choice.db_name }}" => Ok(Self::{{ choice.rust_name }}),
            {% endfor %}
            _ => Err(huus::errors::ConversionError::incorrect_value(string.to_string())),
        }
    }
    fn to_str(&self) -> &'static str {
        match self {
            {% for choice in spec.choices %}
                Self::{{ choice.rust_name }} => "{{ choice.db_name }}",
            {% endfor %}
        }
    }
}

impl huus::conversions::HuusIntoBson for {{ data_name }} {
    fn huus_into_bson(self) -> bson::Bson {
        use huus::conversions::HuusKey;
        bson::Bson::String(self.to_str().to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum {{ value_name }} {
    {% for choice in spec.choices %}
        {{ choice.rust_name }},
    {% endfor %}
}

impl huus::values::BuildValue for {{ value_name }} {
    fn build_value(self) -> huus::values::Value {
        use huus::conversions::HuusKey;
        huus::values::Value::new(bson::Bson::String(self.to_str().to_string()))
    }
}

impl huus::conversions::HuusKey for {{ value_name }} {
    fn from_str(string: &str) -> Result<Self, huus::errors::ConversionError> {
        match string {
            {% for choice in spec.choices %}
                "{{ choice.db_name }}" => Ok(Self::{{ choice.rust_name }}),
            {% endfor %}
            _ => Err(huus::errors::ConversionError::incorrect_value(string.to_string())),
        }
    }
    fn to_str(&self) -> &'static str {
        match self {
            {% for choice in spec.choices %}
                Self::{{ choice.rust_name }} => "{{ choice.db_name }}",
            {% endfor %}
        }
    }
}

