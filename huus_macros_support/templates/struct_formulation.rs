{% let insert_name = spec.struct_name.to_insert() %}
{% let data_name = spec.struct_name.to_data() %}
{% let filter_name = spec.struct_name.to_filter() %}
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
                        Ok(value) => Some({ {{ member.to_conversion() }} }),
                        Err(bson::ordered::ValueAccessError::NotPresent) => None,
                        Err(bson::ordered::ValueAccessError::UnexpectedType) => {
                            return Err(huus::errors::ConversionError::wrong_type(
                                "{{ member.db_name }}".to_string()
                            ))
                        }
                    },
                {% else %}
                    match doc.{{ member.from_doc_getter() }}("{{ member.db_name }}") {
                        Ok(value) => { {{ member.to_conversion() }} }
                        Err(bson::ordered::ValueAccessError::NotPresent) => {
                            {% match member.to_default() %}
                                {% when Some with (default) %}
                                    {{ default }}
                                {% when None %}
                                    return Err(huus::errors::ConversionError::missing_key(
                                        "{{ member.db_name }}".to_string()
                                    ))
                            {% endmatch %}
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

{% match spec.collection_name %}
    {% when Some with (collection_name) %}
        #[derive(Clone, Debug)]
        pub struct {{ insert_name }} {
            doc: bson::Document,
        }

        impl {{ insert_name }} {
            pub fn new(doc: bson::Document) -> Self {
                Self { doc }
            }
        }

        impl huus::conversions::IntoDoc for {{ insert_name }} {
            fn into_doc(self) -> bson::Document {
                self.doc
            }
        }

        #[derive(Clone, Debug)]
        pub struct {{ filter_name }} {
            doc: bson::Document,
        }

        impl {{ filter_name }} {
            pub fn new(doc: bson::Document) -> Self {
                Self { doc }
            }
        }

        impl huus::conversions::IntoDoc for {{ filter_name }} {
            fn into_doc(self) -> bson::Document {
                self.doc
            }
        }

        #[derive(Clone, Debug)]
        pub struct {{ update_name }} {
            doc: bson::Document,
        }

        impl {{ update_name }} {
            pub fn new(doc: bson::Document) -> Self {
                Self { doc }
            }
        }

        impl huus::conversions::IntoDoc for {{ update_name }} {
            fn into_doc(self) -> bson::Document {
                self.doc
            }
        }

        {% let coll_name = generator.make_coll_name(collection_name) %}
        pub struct {{ coll_name }};

        impl huus::query::Query for {{ coll_name }} {
            type Data = {{ data_name }};
            type Insert = {{ insert_name }};
            type Filter = {{ filter_name }};
            type Update = {{ update_name }};
            fn get_collection_name() -> &'static str {
                "{{ collection_name }}"
            }
            fn get_indexed_fields() -> Vec<&'static str> {
                let mut fields = Vec::with_capacity({{ spec.indexed_fields.len() }});
                {% for field in  spec.indexed_fields %}
                    fields.push("{{ field }}");
                {% endfor %}
                fields
            }
        }
    {% when None %}
{% endmatch %}

