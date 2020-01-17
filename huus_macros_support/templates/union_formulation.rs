{% let data_name = spec.name.to_data() %}

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

