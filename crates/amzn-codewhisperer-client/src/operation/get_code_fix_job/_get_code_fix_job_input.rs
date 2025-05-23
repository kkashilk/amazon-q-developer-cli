// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCodeFixJobInput {
    #[allow(missing_docs)] // documentation missing in model
    pub job_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub profile_arn: ::std::option::Option<::std::string::String>,
}
impl GetCodeFixJobInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(&self) -> ::std::option::Option<&str> {
        self.profile_arn.as_deref()
    }
}
impl GetCodeFixJobInput {
    /// Creates a new builder-style object to manufacture
    /// [`GetCodeFixJobInput`](crate::operation::get_code_fix_job::GetCodeFixJobInput).
    pub fn builder() -> crate::operation::get_code_fix_job::builders::GetCodeFixJobInputBuilder {
        crate::operation::get_code_fix_job::builders::GetCodeFixJobInputBuilder::default()
    }
}

/// A builder for [`GetCodeFixJobInput`](crate::operation::get_code_fix_job::GetCodeFixJobInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetCodeFixJobInputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) profile_arn: ::std::option::Option<::std::string::String>,
}
impl GetCodeFixJobInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_id
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_profile_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.profile_arn
    }

    /// Consumes the builder and constructs a
    /// [`GetCodeFixJobInput`](crate::operation::get_code_fix_job::GetCodeFixJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_code_fix_job::GetCodeFixJobInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_code_fix_job::GetCodeFixJobInput {
            job_id: self.job_id,
            profile_arn: self.profile_arn,
        })
    }
}
