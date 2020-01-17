{
    let mut doc = bson::Document::new();
    {% for field in object.fields %}
        doc.insert(
            vec![
                {% for part in field.attr.parts %}
                    {% match part %}
                        {% when Part::Key with (name) %}
                            "{{ name }}".to_string(),
                        {% when Part::Index with (index) %}
                            {{ index }}.to_string(),
                        {% when Part::Code with (code) %}
                            {
                                let result: usize = { {{ code }} };
                                result.to_string()
                            },
                        {% when Part::Dollar  %}
                            "$".to_string(),
                    {% endmatch %}
                {% endfor %}
            ].join("."),
            {% match field.value -%}
                {%- when Value::F64 with (value) -%}
                    bson::Bson::Double({{ value }})
                {%- when Value::String with (string) -%}
                    bson::Bson::String("{{ string }}".to_string())
                {%- when Value::ObjectId with (value) -%}
                {
                    let oid = bson::oid::ObjectId::with_string("{{ value }}")
                        .expect("Huus: Failed to convert the given string to an ObjectId");
                    bson::Bson::ObjectId(oid)
                }
                {%- when Value::Bool with (value) -%}
                    bson::Bson::Boolean({{ value }})
                {%- when Value::Date with (value) -%}
                {
                    let date = "{{ value.to_rfc3339() }}".parse::<chrono::DateTime<chrono::Utc>>();
                    bson::Bson::UtcDatetime(date.expect("Huus: Failed"))
                }
                {%- when Value::I32 with (value) -%}
                    bson::Bson::I32({{ value }})
                {%- when Value::I64 with (value) -%}
                    bson::Bson::I64({{ value }})
                {%- when Value::Object with (object) -%}
                    {{ generator.object(object) }}
                {%- when Value::Code with { code, cast } -%}
                {
                    let value: {{ cast.to_data() }} = {{ code }};
                    value.huus_into_bson()
                }
            {%- endmatch %}
        );
    {% endfor %}
    doc
}
