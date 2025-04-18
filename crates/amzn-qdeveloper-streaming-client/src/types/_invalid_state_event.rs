// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Streaming Response Event when an Invalid State is reached
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvalidStateEvent {
    /// Reasons for Invalid State Event
    pub reason: crate::types::InvalidStateReason,
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::string::String,
}
impl InvalidStateEvent {
    /// Reasons for Invalid State Event
    pub fn reason(&self) -> &crate::types::InvalidStateReason {
        &self.reason
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn message(&self) -> &str {
        use std::ops::Deref;
        self.message.deref()
    }
}
impl InvalidStateEvent {
    /// Creates a new builder-style object to manufacture
    /// [`InvalidStateEvent`](crate::types::InvalidStateEvent).
    pub fn builder() -> crate::types::builders::InvalidStateEventBuilder {
        crate::types::builders::InvalidStateEventBuilder::default()
    }
}

/// A builder for [`InvalidStateEvent`](crate::types::InvalidStateEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InvalidStateEventBuilder {
    pub(crate) reason: ::std::option::Option<crate::types::InvalidStateReason>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl InvalidStateEventBuilder {
    /// Reasons for Invalid State Event
    /// This field is required.
    pub fn reason(mut self, input: crate::types::InvalidStateReason) -> Self {
        self.reason = ::std::option::Option::Some(input);
        self
    }

    /// Reasons for Invalid State Event
    pub fn set_reason(mut self, input: ::std::option::Option<crate::types::InvalidStateReason>) -> Self {
        self.reason = input;
        self
    }

    /// Reasons for Invalid State Event
    pub fn get_reason(&self) -> &::std::option::Option<crate::types::InvalidStateReason> {
        &self.reason
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }

    /// Consumes the builder and constructs a
    /// [`InvalidStateEvent`](crate::types::InvalidStateEvent). This method will fail if any of
    /// the following fields are not set:
    /// - [`reason`](crate::types::builders::InvalidStateEventBuilder::reason)
    /// - [`message`](crate::types::builders::InvalidStateEventBuilder::message)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::InvalidStateEvent, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::InvalidStateEvent {
            reason: self.reason.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "reason",
                    "reason was not specified but it is required when building InvalidStateEvent",
                )
            })?,
            message: self.message.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "message",
                    "message was not specified but it is required when building InvalidStateEvent",
                )
            })?,
        })
    }
}
