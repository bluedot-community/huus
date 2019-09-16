{% let data_name = spec.struct_name.to_data() %}
{% let filter_name = spec.struct_name.to_filter() %}
{% let value_name = spec.struct_name.to_value() %}
{% let update_name = spec.struct_name.to_update() %}

#[derive(Clone, Debug, PartialEq)]
pub struct {{ data_name }} {
    {% for member in spec.members %}
        {% if member.is_optional %}
            pub {{ member.rust_name }}: Option<{{ member.to_data() }}>,
        {% else %}
            pub {{ member.rust_name }}: {{ member.to_data() }},
        {% endif %}
    {% endfor %}
}

impl huus::conversions::FromDoc for {{ data_name }} {
    fn from_doc(doc: bson::Document)
    -> Result<{{ data_name }}, huus::errors::ConversionError> {
        use huus::conversions::{HuusKey, HuusIntoStruct};
        Ok({{ data_name }} {
            {% for member in spec.members %}
                {{ member.rust_name }}:
                {% if member.is_optional %}
                    match doc.{{ member.from_doc_getter() }}("{{ member.db_name }}") {
                        Ok(value) => Some({ {{ member.conversion() }} }),
                        Err(bson::ordered::ValueAccessError::NotPresent) => None,
                        Err(bson::ordered::ValueAccessError::UnexpectedType) => {
                            return Err(huus::errors::ConversionError::wrong_type(
                                "{{ member.db_name }}".to_string()
                            ))
                        }
                    },
                {% else %}
                    match doc.{{ member.from_doc_getter() }}("{{ member.db_name }}") {
                        Ok(value) => { {{ member.conversion() }} }
                        Err(bson::ordered::ValueAccessError::NotPresent) => {
                            return Err(huus::errors::ConversionError::missing_key(
                                "{{ member.db_name }}".to_string()
                            ))
                        }
                        Err(bson::ordered::ValueAccessError::UnexpectedType) => {
                            return Err(huus::errors::ConversionError::wrong_type(
                                "{{ member.db_name }}".to_string()
                            ))
                        }
                    },
                {% endif %}
            {% endfor %}
        })
    }
}

impl huus::conversions::IntoDoc for {{ data_name }} {
    fn into_doc(self) -> bson::Document {
        use huus::conversions::HuusIntoBson;
        let mut doc = bson::Document::new();
        {% for member in spec.members %}
            {% if member.is_optional %}
                if let Some(data) = self.{{ member.rust_name }} {
                    doc.insert("{{ member.db_name }}", data.huus_into_bson());
                }
            {% else %}
                doc.insert("{{ member.db_name }}", self.{{ member.rust_name }}.huus_into_bson());
            {% endif %}
        {% endfor %}
        doc
    }
}

#[derive(Clone, Debug)]
pub struct {{ filter_name }} {
    {% for member in spec.members %}
        pub {{ member.rust_name }}: {{ member.to_filter() }},
    {% endfor %}
}

{% match spec.collection_name %}
    {% when Some with (_) %}
        impl huus::filters::BuildFilter for {{ filter_name }} {
            fn build_filter(self) -> huus::filters::Filter {
                let mut filter = huus::filters::Filter::empty();
                {% for member in spec.members %}
                    filter.incorporate(self.{{ member.rust_name }}.build_filter(
                        "{{ member.db_name }}".to_string()
                    ));
                {% endfor %}
                filter
            }
        }
    {% when None %}
        impl huus::filters::BuildInnerFilter for {{ filter_name }} {
            fn build_filter(self, field: String) -> huus::filters::Filter {
                let mut filter = huus::filters::Filter::empty();
                {% for member in spec.members %}
                    filter.incorporate(self.{{ member.rust_name }}.build_filter(
                        field.clone() + ".{{ member.db_name }}"
                    ));
                {% endfor %}
                filter
            }
        }
{% endmatch %}

impl Default for {{ filter_name }} {
    fn default() -> Self {
        Self {
            {% for member in spec.members %}
                {{ member.rust_name }}: <{{ member.to_filter() }}>::default(),
            {% endfor %}
        }
    }
}

#[derive(Clone, Debug)]
pub struct {{ value_name }} {
    {% for member in spec.members %}
        pub {{ member.rust_name }}: Option<{{ member.to_value() }}>,
    {% endfor %}
}

impl huus::values::BuildValue for {{ value_name }} {
    fn build_value(self) -> huus::values::Value {
        use huus::conversions::HuusIntoBson;
        let mut value = huus::values::ObjectValue::empty();
        {% for member in spec.members %}
            if let Some(data) = self.{{ member.rust_name }} {
                value.insert("{{ member.db_name }}".to_string(), data.build_value().into_bson());
            }
        {% endfor %}
        value.build_value()
    }
}

impl Default for {{ value_name }} {
    fn default() -> Self {
        Self {
            {% for member in spec.members %}
                {{ member.rust_name }}: None,
            {% endfor %}
        }
    }
}

#[derive(Clone, Debug)]
pub struct {{ update_name }} {
    {% for member in spec.members %}
        pub {{ member.rust_name }}: {{ member.to_update() }},
    {% endfor %}
}

{% match spec.collection_name %}
    {% when Some with (_) %}
        impl huus::updates::BuildUpdate for {{ update_name }} {
            fn build_update(self) -> huus::updates::Update {
                let mut update = huus::updates::Update::empty();
                {% for member in spec.members %}
                    {% if member.db_name != "_id" %}
                        update.incorporate(
                            self.{{ member.rust_name }}
                                .build_update("{{ member.db_name }}".to_string())
                        );
                    {% endif %}
                {% endfor %}
                update
            }
        }
    {% when None %}
        impl huus::updates::BuildInnerUpdate for {{ update_name }} {
            fn build_update(self, field: String) -> huus::updates::Update {
                let mut update = huus::updates::Update::empty();
                {% for member in spec.members %}
                    {% if member.db_name != "_id" %}
                        update.incorporate(
                            self.{{ member.rust_name }}
                                .build_update(field.clone() + ".{{ member.db_name}}")
                        );
                    {% endif %}
                {% endfor %}
                update
            }
        }
{% endmatch %}

impl Default for {{ update_name }} {
    fn default() -> Self {
        Self {
            {% for member in spec.members %}
                {{ member.rust_name}}: <{{ member.to_update() }}>::default(),
            {% endfor %}
        }
    }
}

{% match spec.collection_name %}
    {% when Some with (collection_name) %}
        impl huus::query::Query for {{ data_name }} {
            type Data = {{ data_name }};
            type Update = {{ update_name }};
            fn get_collection_name() -> &'static str {
                "{{ collection_name }}"
            }
            fn get_indexed_fields() -> Vec<&'static str> {
                let mut fields = Vec::new();
                {%for field in indexed_fields %}
                    fields.push("{{ field }}");
                {% endfor %}
                fields
            }
        }
        impl huus::query::Query for {{ filter_name }} {
            type Data = {{ data_name }};
            type Update = {{ update_name }};
            fn get_collection_name() -> &'static str {
                "{{ collection_name }}"
            }
            fn get_indexed_fields() -> Vec<&'static str> {
                let mut fields = Vec::new();
                {%for field in indexed_fields %}
                    fields.push("{{ field }}");
                {% endfor %}
                fields
            }
        }
    {% when None %}
{% endmatch %}

