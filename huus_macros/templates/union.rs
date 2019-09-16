{% let data_name = spec.name.to_data() %}
{% let filter_name = spec.name.to_filter() %}
{% let value_name = spec.name.to_value() %}
{% let update_name = spec.name.to_update() %}

#[derive(Clone, Debug, PartialEq)]
pub enum {{ data_name }} {
    {% for choice in spec.choices %}
        {{ choice.rust_name }}({{ choice.variant.to_data() }}),
    {% endfor %}
}

impl huus::conversions::FromDoc for {{ data_name }} {
    fn from_doc(doc: bson::Document)
    -> Result<{{ data_name }}, huus::errors::ConversionError> {
        use huus::errors::ConversionError;
        match doc.get_str("_huus_variant") {
            Ok(name) => {
                match name {
                    {% for choice in spec.choices %}
                        "{{ choice.db_name }}" => Ok({{ data_name }}::{{ choice.rust_name }}(
                            {{ choice.variant.to_data() }}::from_doc(doc)?)
                        ),
                    {% endfor %}
                    _ => Err(huus::errors::ConversionError::unexpected_value(name.to_string())),
                }
            }
            Err(_) => {
                Err(huus::errors::ConversionError::missing_key("_huus_variant".to_string()))
            }
        }
    }
}

impl huus::conversions::IntoDoc for {{ data_name }} {
    fn into_doc(self) -> bson::Document {
        match self {
            {% for choice in spec.choices %}
                Self::{{ choice.rust_name }}(data) => {
                    let mut doc = data.into_doc();
                    doc.insert_bson(
                        "_huus_variant".to_string(),
                        bson::Bson::String("{{ choice.db_name }}".to_string())
                    );
                    doc
                }
            {% endfor %}
        }
    }
}

#[derive(Clone, Debug)]
pub enum {{ filter_name }} {
    {% for choice in spec.choices %}
        {{ choice.rust_name }}({{ choice.variant.to_filter() }}),
    {% endfor %}
}

impl huus::filters::BuildInnerFilter for {{ filter_name }} {
    fn build_filter(self, field: String) -> huus::filters::Filter {
        match self {
            {% for choice in spec.choices %}
                Self::{{ choice.rust_name }}(filter) => {
                    filter.build_filter(field)
                }
            {% endfor %}
        }
    }
}

#[derive(Clone, Debug)]
pub enum {{ value_name }} {
    {% for choice in spec.choices %}
        {{ choice.rust_name }}({{ choice.variant.to_value() }}),
    {% endfor %}
}

impl huus::values::BuildValue for {{ value_name }} {
    fn build_value(self) -> huus::values::Value {
        match self {
            {% for choice in spec.choices %}
                Self::{{ choice.rust_name }}(value) => {
                    match value.build_value().into_bson() {
                        bson::Bson::Document(mut doc) => {
                            let value = bson::Bson::String("{{ choice.db_name }}".to_string());
                            doc.insert_bson("_huus_variant".to_string(), value);
                            huus::values::Value::new(bson::Bson::Document(doc))
                        }
                        _ => panic!("Huus: Failed to cast union into a document"),
                    }
                }
            {% endfor %}
        }
    }
}

#[derive(Clone, Debug)]
pub enum {{ update_name }} {
    {% for choice in spec.choices %}
        {{ choice.rust_name }}({{ choice.variant.to_update() }}),
    {% endfor %}
}

impl huus::updates::BuildInnerUpdate for {{ update_name }} {
    fn build_update(self, field: String) -> huus::updates::Update {
        match self {
            {% for choice in spec.choices %}
                Self::{{ choice.rust_name }}(update) => {
                    let key = field.clone() + "._huus_variant";
                    let value = bson::Bson::String("{{ choice.db_name }}".to_string());
                    let variant_update = huus::updates::Update::with_field(key, value);
                    let mut result = update.build_update(field);
                    result.incorporate(variant_update);
                    result
                }
            {% endfor %}
        }
    }
}

