use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InputData {
    pub field_type: FieldType,
    pub label: String,
    pub name: String,
    pub value: String,
    pub error: Option<String>,
    pub required: bool,
}

#[derive(Clone, PartialEq)]
pub enum FieldType {
    Text,
    EMail,
    Password,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub input_node_ref: NodeRef,
    pub data: InputData,
}

#[function_component(InputField)]
pub fn input_field(
    InputFieldProps {
        input_node_ref,
        data,
    }: &InputFieldProps,
) -> Html {
    let field_type = match data.field_type {
        FieldType::Text => "text",
        FieldType::EMail => "email",
        FieldType::Password => "password",
    };

    html! {
        <div>
            <label
                for={data.name.clone()}
                class={classes!(if data.required {"required"} else {""})}
            >
                {data.label.clone()}{if data.required {":*"} else {":"}}
                <input
                    type={field_type}
                    name={data.name.clone()}
                    value={data.value.clone()}
                    required={data.required}
                    ref={input_node_ref.clone()}
                />
            </label>
            if let Some(error) = &data.error {
                <div class="error_text"><i class="arrow up"></i>{error.clone()}</div>
            }
        </div>
    }
}

pub fn field_props(
    node_ref: NodeRef,
    field_type: FieldType,
    label: &str,
    name: &str,
    required: bool,
) -> (NodeRef, InputFieldProps) {
    let props = InputFieldProps {
        input_node_ref: node_ref.clone(),
        data: InputData {
            field_type: field_type,
            label: label.to_string(),
            name: name.to_string(),
            value: "".to_string(),
            error: None,
            required,
        },
    };

    (node_ref, props)
}
