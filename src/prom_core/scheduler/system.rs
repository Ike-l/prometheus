// https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch

use std::{any::{type_name, TypeId}, fmt::Debug, marker::PhantomData};

use super::{AccessMap, TypeMap};

pub trait System {
    fn run(&mut self, resources: &TypeMap, access: &mut AccessMap);
}

impl Debug for dyn System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "System trait object")
    }
}

pub trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

pub trait SystemParam {
    type Item<'new>;

    fn accesses(access: &mut AccessMap);

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_>;

    unsafe fn typed_retrieve<T: 'static>(resources: &TypeMap) -> &T {
        let unsafe_cell = resources
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("Expected type: {} in type map", type_name::<T>()));

        let boxed = &*unsafe_cell.get();

        let value = boxed.downcast_ref::<T>()
            .unwrap_or_else(|| panic!("Downcasting resource: {}", type_name::<T>()));

        value
    }

    unsafe fn typed_mut_retrieve<T: 'static>(resources: &TypeMap) -> &mut T {
        let unsafe_cell = resources
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("Expected type: {} in type map", type_name::<T>()));
        
        let boxed = &mut *unsafe_cell.get();

        let value = boxed.downcast_mut::<T>()
            .unwrap_or_else(|| panic!("Downcasting resource: {}", type_name::<T>()));

        value
    }
}

macro_rules! impl_system {
    (
        $($params:ident),*
    ) => {
        #[allow(clippy::too_many_arguments)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: SystemParam),*> System for FunctionSystem<($($params,)*), F>
            where
                for<'a, 'b> &'a mut F:
                    FnMut( $($params),* ) +
                    FnMut( $(<$params as SystemParam>::Item<'b>),* )
        {
            fn run(&mut self, resources: &TypeMap, accesses: &mut AccessMap) {
                fn call_inner<$($params),*>(
                    mut f: impl FnMut($($params),*),
                    $($params: $params),*
                ) {
                    f($($params),*)
                }

                $(
                    $params::accesses(accesses);
                )*

                $(
                    let $params = unsafe { $params::retrieve(resources) };
                )*

                call_inner(&mut self.f, $($params),*)
            }
        }
    }
}

macro_rules! impl_into_system {
    (
        $($params:ident),*
    ) => {
        impl<F, $($params: SystemParam),*> IntoSystem<($($params,)*)> for F
            where
                for<'a, 'b> &'a mut F:
                    FnMut( $($params),* ) +
                    FnMut( $(<$params as SystemParam>::Item<'b>),* )
        {
            type System = FunctionSystem<($($params,)*), Self>;

            fn into_system(self) -> Self::System {
                FunctionSystem {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct FunctionSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

impl_system!();
impl_system!(T1);
impl_system!(T1, T2);
impl_system!(T1, T2, T3);
impl_system!(T1, T2, T3, T4);
impl_system!(T1, T2, T3, T4, T5);
impl_system!(T1, T2, T3, T4, T5, T6);
impl_system!(T1, T2, T3, T4, T5, T6, T7);
impl_system!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

impl_into_system!();
impl_into_system!(T1);
impl_into_system!(T1, T2);
impl_into_system!(T1, T2, T3);
impl_into_system!(T1, T2, T3, T4);
impl_into_system!(T1, T2, T3, T4, T5);
impl_into_system!(T1, T2, T3, T4, T5, T6);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);