// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`DeleteAssignment`](crate::operation::delete_assignment::builders::DeleteAssignmentFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`principal_id(impl Into<String>)`](crate::operation::delete_assignment::builders::DeleteAssignmentFluentBuilder::principal_id) / [`set_principal_id(Option<String>)`](crate::operation::delete_assignment::builders::DeleteAssignmentFluentBuilder::set_principal_id):<br>required: **true**<br>Identity Store User or Group ID<br>
    ///   - [`principal_type(PrincipalType)`](crate::operation::delete_assignment::builders::DeleteAssignmentFluentBuilder::principal_type) / [`set_principal_type(Option<PrincipalType>)`](crate::operation::delete_assignment::builders::DeleteAssignmentFluentBuilder::set_principal_type):<br>required: **true**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`DeleteAssignmentOutput`](crate::operation::delete_assignment::DeleteAssignmentOutput)
    /// - On failure, responds with
    ///   [`SdkError<DeleteAssignmentError>`](crate::operation::delete_assignment::DeleteAssignmentError)
    pub fn delete_assignment(&self) -> crate::operation::delete_assignment::builders::DeleteAssignmentFluentBuilder {
        crate::operation::delete_assignment::builders::DeleteAssignmentFluentBuilder::new(self.handle.clone())
    }
}
