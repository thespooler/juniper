//! GraphQL implementation for [reference].
//!
//! [reference]: primitive@std::reference

use crate::{
    graphql, meta::MetaType, resolve, Arguments, BoxFuture, ExecutionResult, Executor, Registry,
    Selection,
};

impl<'me, T, Info, S> resolve::Type<Info, S> for &'me T
where
    T: resolve::Type<Info, S> + ?Sized,
    Info: ?Sized,
{
    fn meta<'r>(registry: &mut Registry<'r, S>, info: &Info) -> MetaType<'r, S>
    where
        S: 'r,
    {
        T::meta(registry, info)
    }
}

impl<'me, T, Info> resolve::TypeName<Info> for &'me T
where
    T: resolve::TypeName<Info> + ?Sized,
    Info: ?Sized,
{
    fn type_name(info: &Info) -> &str {
        T::type_name(info)
    }
}

impl<'me, T, Info> resolve::ConcreteTypeName<Info> for &'me T
where
    T: resolve::ConcreteTypeName<Info> + ?Sized,
    Info: ?Sized,
{
    fn concrete_type_name<'i>(&self, info: &'i Info) -> &'i str {
        (**self).concrete_type_name(info)
    }
}

impl<'me, T, Info, Ctx, S> resolve::Value<Info, Ctx, S> for &'me T
where
    T: resolve::Value<Info, Ctx, S> + ?Sized,
    Info: ?Sized,
    Ctx: ?Sized,
{
    fn resolve_value(
        &self,
        selection_set: Option<&[Selection<'_, S>]>,
        info: &Info,
        executor: &Executor<Ctx, S>,
    ) -> ExecutionResult<S> {
        (**self).resolve_value(selection_set, info, executor)
    }
}

impl<'me, T, Info, Ctx, S> resolve::ValueAsync<Info, Ctx, S> for &'me T
where
    T: resolve::ValueAsync<Info, Ctx, S> + ?Sized,
    Info: ?Sized,
    Ctx: ?Sized,
{
    fn resolve_value_async<'r>(
        &'r self,
        selection_set: Option<&'r [Selection<'_, S>]>,
        info: &'r Info,
        executor: &'r Executor<Ctx, S>,
    ) -> BoxFuture<'r, ExecutionResult<S>> {
        (**self).resolve_value_async(selection_set, info, executor)
    }
}

impl<'me, T, Info, Ctx, S> resolve::ConcreteValue<Info, Ctx, S> for &'me T
where
    T: resolve::ConcreteValue<Info, Ctx, S> + ?Sized,
    Info: ?Sized,
    Ctx: ?Sized,
{
    fn resolve_concrete_value(
        &self,
        type_name: &str,
        selection_set: Option<&[Selection<'_, S>]>,
        info: &Info,
        executor: &Executor<Ctx, S>,
    ) -> ExecutionResult<S> {
        (**self).resolve_concrete_value(type_name, selection_set, info, executor)
    }
}

impl<'me, T, Info, Ctx, S> resolve::ConcreteValueAsync<Info, Ctx, S> for &'me T
where
    T: resolve::ConcreteValueAsync<Info, Ctx, S> + ?Sized,
    Info: ?Sized,
    Ctx: ?Sized,
{
    fn resolve_concrete_value_async<'r>(
        &'r self,
        type_name: &str,
        selection_set: Option<&'r [Selection<'_, S>]>,
        info: &'r Info,
        executor: &'r Executor<Ctx, S>,
    ) -> BoxFuture<'r, ExecutionResult<S>> {
        (**self).resolve_concrete_value_async(type_name, selection_set, info, executor)
    }
}

impl<'me, T, Info, Ctx, S> resolve::Field<Info, Ctx, S> for &'me T
where
    T: resolve::Field<Info, Ctx, S> + ?Sized,
    Info: ?Sized,
    Ctx: ?Sized,
{
    fn resolve_field(
        &self,
        field_name: &str,
        arguments: &Arguments<S>,
        info: &Info,
        executor: &Executor<Ctx, S>,
    ) -> ExecutionResult<S> {
        (**self).resolve_field(field_name, arguments, info, executor)
    }
}

impl<'me, T, Info, Ctx, S> resolve::FieldAsync<Info, Ctx, S> for &'me T
where
    T: resolve::FieldAsync<Info, Ctx, S> + ?Sized,
    Info: ?Sized,
    Ctx: ?Sized,
{
    fn resolve_field_async<'r>(
        &'r self,
        field_name: &'r str,
        arguments: &'r Arguments<S>,
        info: &'r Info,
        executor: &'r Executor<Ctx, S>,
    ) -> BoxFuture<'r, ExecutionResult<S>> {
        (**self).resolve_field_async(field_name, arguments, info, executor)
    }
}

impl<'me, T, S> graphql::InputType<S> for &'me T
where
    T: graphql::InputType<S> + ?Sized,
{
    fn assert_input_type() {
        T::assert_input_type()
    }
}

impl<'me, T, S> graphql::OutputType<S> for &'me T
where
    T: graphql::OutputType<S> + ?Sized,
{
    fn assert_output_type() {
        T::assert_output_type()
    }
}

impl<'me, T, S> graphql::Interface<S> for &'me T
where
    T: graphql::Interface<S> + ?Sized,
{
    fn assert_interface() {
        T::assert_interface()
    }
}

impl<'me, T, S> graphql::Object<S> for &'me T
where
    T: graphql::Object<S> + ?Sized,
{
    fn assert_object() {
        T::assert_object()
    }
}

impl<'me, T, S> graphql::Union<S> for &'me T
where
    T: graphql::Union<S> + ?Sized,
{
    fn assert_union() {
        T::assert_union()
    }
}
