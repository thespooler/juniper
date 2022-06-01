//! GraphQL implementation for [`Box`].

use crate::{
    graphql,
    meta::MetaType,
    parser::{ParseError, ScalarToken},
    reflect, resolve, Arguments, BoxFuture, DefaultScalarValue, ExecutionResult, Executor,
    IntoFieldError, Registry, Selection,
};

impl<T, Info, S> resolve::Type<Info, S> for Box<T>
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

impl<T, Info> resolve::TypeName<Info> for Box<T>
where
    T: resolve::TypeName<Info> + ?Sized,
    Info: ?Sized,
{
    fn type_name(info: &Info) -> &str {
        T::type_name(info)
    }
}

impl<T, Info> resolve::ConcreteTypeName<Info> for Box<T>
where
    T: resolve::ConcreteTypeName<Info> + ?Sized,
    Info: ?Sized,
{
    fn concrete_type_name<'i>(&self, info: &'i Info) -> &'i str {
        (**self).concrete_type_name(info)
    }
}

impl<T, Info, Ctx, S> resolve::Value<Info, Ctx, S> for Box<T>
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

impl<T, Info, Ctx, S> resolve::ValueAsync<Info, Ctx, S> for Box<T>
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

impl<T, Info, Ctx, S> resolve::ConcreteValue<Info, Ctx, S> for Box<T>
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

impl<T, Info, Ctx, S> resolve::ConcreteValueAsync<Info, Ctx, S> for Box<T>
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

impl<T, Info, Ctx, S> resolve::Field<Info, Ctx, S> for Box<T>
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

impl<T, Info, Ctx, S> resolve::FieldAsync<Info, Ctx, S> for Box<T>
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

impl<T, S> resolve::ToInputValue<S> for Box<T>
where
    T: resolve::ToInputValue<S> + ?Sized,
{
    fn to_input_value(&self) -> graphql::InputValue<S> {
        (**self).to_input_value()
    }
}

impl<'inp, T, S> resolve::InputValue<'inp, S> for Box<T>
where
    T: resolve::InputValueAsBox<'inp, S> + ?Sized,
    S: 'inp,
{
    type Error = <T as resolve::InputValueAsBox<'inp, S>>::Error;

    fn try_from_input_value(v: &'inp graphql::InputValue<S>) -> Result<Self, Self::Error> {
        <T as resolve::InputValueAsBox<'inp, S>>::try_from_input_value(v)
    }

    fn try_from_implicit_null() -> Result<Self, Self::Error> {
        <T as resolve::InputValueAsBox<'inp, S>>::try_from_implicit_null()
    }
}

pub trait TryFromInputValue<'input, S: 'input = DefaultScalarValue> {
    type Error: IntoFieldError<S>;

    fn try_from_input_value(v: &'input graphql::InputValue<S>) -> Result<Box<Self>, Self::Error>;

    fn try_from_implicit_null() -> Result<Box<Self>, Self::Error> {
        Self::try_from_input_value(&graphql::InputValue::<S>::Null)
    }
}

impl<'inp, T, S> TryFromInputValue<'inp, S> for T
where
    T: resolve::InputValue<'inp, S>,
    S: 'inp,
{
    type Error = <T as resolve::InputValue<'inp, S>>::Error;

    fn try_from_input_value(v: &'inp graphql::InputValue<S>) -> Result<Box<Self>, Self::Error> {
        <T as resolve::InputValue<'inp, S>>::try_from_input_value(v).map(Box::new)
    }

    fn try_from_implicit_null() -> Result<Box<Self>, Self::Error> {
        <T as resolve::InputValue<'inp, S>>::try_from_implicit_null().map(Box::new)
    }
}

impl<T, S> resolve::ScalarToken<S> for Box<T>
where
    T: resolve::ScalarToken<S> + ?Sized,
{
    fn parse_scalar_token(token: ScalarToken<'_>) -> Result<S, ParseError<'_>> {
        T::parse_scalar_token(token)
    }
}

impl<T, S> graphql::InputType<S> for Box<T>
where
    T: graphql::InputType<S> + ?Sized,
{
    fn assert_input_type() {
        T::assert_input_type()
    }
}

impl<T, S> graphql::OutputType<S> for Box<T>
where
    T: graphql::OutputType<S> + ?Sized,
{
    fn assert_output_type() {
        T::assert_output_type()
    }
}

impl<T, S> graphql::Interface<S> for Box<T>
where
    T: graphql::Interface<S> + ?Sized,
{
    fn assert_interface() {
        T::assert_interface()
    }
}

impl<T, S> graphql::Object<S> for Box<T>
where
    T: graphql::Object<S> + ?Sized,
{
    fn assert_object() {
        T::assert_object()
    }
}

impl<T, S> graphql::Scalar<S> for Box<T>
where
    T: graphql::Scalar<S> + ?Sized,
{
    fn assert_scalar() {
        T::assert_scalar()
    }
}

impl<T, S> graphql::Union<S> for Box<T>
where
    T: graphql::Union<S> + ?Sized,
{
    fn assert_union() {
        T::assert_union()
    }
}

impl<T, S> reflect::BaseType<S> for Box<T>
where
    T: reflect::BaseType<S> + ?Sized,
{
    const NAME: reflect::Type = T::NAME;
}

impl<T, S> reflect::BaseSubTypes<S> for Box<T>
where
    T: reflect::BaseSubTypes<S> + ?Sized,
{
    const NAMES: reflect::Types = T::NAMES;
}

impl<T, S> reflect::WrappedType<S> for Box<T>
where
    T: reflect::WrappedType<S> + ?Sized,
{
    const VALUE: reflect::WrappedValue = T::VALUE;
}
