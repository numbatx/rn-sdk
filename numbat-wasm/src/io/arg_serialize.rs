use crate::*;
use numbat_codec::*;
use crate::call_data::*;

pub trait AsynCallArg: Sized {
    fn push_async_arg(&self, serializer: &mut CallDataSerializer) -> Result<(), SCError>;

    fn push_async_arg_exact(&self, _serializer: &mut CallDataSerializer, _expected_len: usize) -> Result<(), SCError> {
        Err(SCError::Static(&b"not supported"[..]))
    }
}

impl<T> AsynCallArg for T
where
    T: Encode,
{
    #[inline]
    fn push_async_arg(&self, serializer: &mut CallDataSerializer) -> Result<(), SCError> {
        self
            .using_top_encoded(|buf| serializer.push_argument_bytes(buf))
            .map_err(SCError::PushAsyncEncodeErr)
    }
}

impl<T> AsynCallArg for VarArgs<T>
where
    T: AsynCallArg,
{
    fn push_async_arg(&self, serializer: &mut CallDataSerializer) -> Result<(), SCError> {
        for elem in self.0.iter() {
            elem.push_async_arg(serializer)?;
        }
        Ok(())
    }

    fn push_async_arg_exact(&self, serializer: &mut CallDataSerializer, expected_len: usize) -> Result<(), SCError> {
        if self.len() != expected_len {
            return Err(SCError::Static(err_msg::ARG_ASYNC_WRONG_NUMBER));
        }
        self.push_async_arg(serializer)?;
        Ok(())
    }
}

impl<T> AsynCallArg for OptionalArg<T>
where
    T: AsynCallArg,
{
    #[inline]
    fn push_async_arg(&self, serializer: &mut CallDataSerializer) -> Result<(), SCError> {
        if let OptionalArg::Some(t) = self {
            t.push_async_arg(serializer)?;
        }
        Ok(())
    }
}

macro_rules! multi_result_impls {
    ($(($mr:ident $($n:tt $name:ident)+) )+) => {
        $(
            impl<$($name),+> AsynCallArg for $mr<$($name,)+>
            where
                $($name: AsynCallArg,)+
            {
                #[inline]
                fn push_async_arg(&self, serializer: &mut CallDataSerializer) -> Result<(), SCError> {
                    $(
                        (self.0).$n.push_async_arg(serializer)?;
                    )+
                    Ok(())
                }
            }
        )+
    }
}

multi_result_impls! {
    (MultiResult1  0 T0)
    (MultiResult2  0 T0 1 T1)
    (MultiResult3  0 T0 1 T1 2 T2)
    (MultiResult4  0 T0 1 T1 2 T2 3 T3)
    (MultiResult5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiResult6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiResult7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiResult8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiResult9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiResult10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiResult11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiResult12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiResult13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiResult14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiResult15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiResult16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
