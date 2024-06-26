use crate::*;
use numbat_codec::*;

/// Some info to display in endpoint argument deserialization error messages,
/// to help users identify the faulty argument.
/// Generated automatically.
/// Current version uses argument names,
/// but in principle it could be changed to argument index to save some bytes from the wasm output.
pub type ArgId = &'static [u8];

pub trait DynArgLoader<T>: Sized {
    fn has_next(&self) -> bool;

    fn next_arg(&mut self, arg_id: ArgId) -> Result<Option<T>, SCError>;
}

pub trait ArgType<D>: Sized {
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError>;
}

#[inline]
pub fn load_dyn_arg<T, D, E>(loader: &mut D, err_handler: &E, arg_id: ArgId) -> T
where
    T: ArgType<D>,
    E: DynArgErrHandler,
{
    match T::load(loader, arg_id) {
        Ok(arg) => arg,
        Err(sc_err) => err_handler.handle_sc_error(sc_err),
    }
}

#[inline]
pub fn check_no_more_args<D, E>(loader: &D, err_handler: &E)
where
    D: DynArgLoader<()>,
    E: DynArgErrHandler,
{
    if D::has_next(loader) {
        err_handler.handle_sc_error(SCError::Static(err_msg::ARG_WRONG_NUMBER));
    }
}

impl<T, D> ArgType<D> for T
where
    T: Decode,
    D: DynArgLoader<T>,
{
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        if let TypeInfo::Unit = T::TYPE_INFO {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return Ok(cast_unit);
        }

        match loader.next_arg(arg_id) {
            Ok(Some(arg)) => Ok(arg),
            Ok(None) => Err(SCError::Static(err_msg::ARG_WRONG_NUMBER)),
            Err(sc_err) => Err(sc_err),
        }
    }
}

pub struct VarArgs<T>(pub Vec<T>);

impl<T> From<Vec<T>> for VarArgs<T> {
    fn from(v: Vec<T>) -> Self {
        VarArgs(v)
    }
}

impl<T> VarArgs<T> {
    #[inline]
    pub fn new() -> Self {
        VarArgs(Vec::new())
    }
}

impl<T> Default for VarArgs<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> VarArgs<T> {
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }

}

impl<T, D> ArgType<D> for VarArgs<T>
where
    T: ArgType<D>,
    D: DynArgLoader<()>,
{
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        let mut result_vec: Vec<T> = Vec::new();
        while DynArgLoader::<()>::has_next(&*loader) {
            result_vec.push(T::load(loader, arg_id)?);
        }
        Ok(VarArgs(result_vec))
    }
}

pub enum OptionalArg<T> {
    Some(T),
    None
}

impl<T> From<Option<T>> for OptionalArg<T> {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(arg) => OptionalArg::Some(arg),
            None => OptionalArg::None
        }
    }
}

impl<T> OptionalArg<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            OptionalArg::Some(arg) => Some(arg),
            OptionalArg::None => None
        }
    }
}

impl<T, D> ArgType<D> for OptionalArg<T>
where
    T: ArgType<D>,
    D: DynArgLoader<()>,
{
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        if DynArgLoader::<()>::has_next(&*loader) {
            Ok(OptionalArg::Some(T::load(loader, arg_id)?))
        } else {
            Ok(OptionalArg::None)
        }
    }
}

pub struct AsyncCallError {
    pub err_code: i32,
    pub err_msg: Vec<u8>,
}

pub enum AsyncCallResult<T> {
    Ok(T),
    Err(AsyncCallError)
}

impl<T, D> ArgType<D> for AsyncCallResult<T>
where
    T: ArgType<D>,
    D: DynArgLoader<()> + DynArgLoader<i32> + DynArgLoader<Vec<u8>>,
{
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        let err_code = i32::load(loader, arg_id)?;
        if err_code == 0 {
            let arg = T::load(loader, arg_id)?;
            Ok(AsyncCallResult::Ok(arg))
        } else {
            let err_msg = Vec::<u8>::load(loader, arg_id)?;
            Ok(AsyncCallResult::Err(AsyncCallError {
                err_code,
                err_msg,
            }))
        }
    }
}

macro_rules! multi_arg_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            pub struct $mr<$($name,)+>(pub ($($name,)+));

            impl<$($name,)+ D> ArgType<D> for $mr<$($name,)+>
            where
                $($name: ArgType<D>,)+
                D: $(DynArgLoader<$name> + )+ Sized
            {
                fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
                    Ok($mr((
                        $(
                            $name::load(loader, arg_id)?
                        ),+
                    )))
                }
            }

            impl<$($name,)+> $mr<$($name,)+> {
                #[inline]
                pub fn into_tuple(self) -> ($($name,)+) {
                    self.0
                }
            }
        )+
    }
}

multi_arg_impls! {
    (MultiArg2  0 T0 1 T1)
    (MultiArg3  0 T0 1 T1 2 T2)
    (MultiArg4  0 T0 1 T1 2 T2 3 T3)
    (MultiArg5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiArg6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiArg7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiArg8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiArg9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiArg10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiArg11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiArg12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiArg13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiArg14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiArg15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiArg16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
