// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// The input schema for the tool in JSON format.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ToolInputSchema {
    #[allow(missing_docs)] // documentation missing in model
    pub json: ::std::option::Option<::aws_smithy_types::Document>,
}
impl ToolInputSchema {
    #[allow(missing_docs)] // documentation missing in model
    pub fn json(&self) -> ::std::option::Option<&::aws_smithy_types::Document> {
        self.json.as_ref()
    }
}
impl ::std::fmt::Debug for ToolInputSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ToolInputSchema");
        formatter.field("json", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ToolInputSchema {
    /// Creates a new builder-style object to manufacture
    /// [`ToolInputSchema`](crate::types::ToolInputSchema).
    pub fn builder() -> crate::types::builders::ToolInputSchemaBuilder {
        crate::types::builders::ToolInputSchemaBuilder::default()
    }
}

/// A builder for [`ToolInputSchema`](crate::types::ToolInputSchema).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct ToolInputSchemaBuilder {
    pub(crate) json: ::std::option::Option<::aws_smithy_types::Document>,
}
impl ToolInputSchemaBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn json(mut self, input: ::aws_smithy_types::Document) -> Self {
        self.json = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_json(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
        self.json = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_json(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
        &self.json
    }

    /// Consumes the builder and constructs a [`ToolInputSchema`](crate::types::ToolInputSchema).
    pub fn build(self) -> crate::types::ToolInputSchema {
        crate::types::ToolInputSchema { json: self.json }
    }
}
impl ::std::fmt::Debug for ToolInputSchemaBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ToolInputSchemaBuilder");
        formatter.field("json", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
