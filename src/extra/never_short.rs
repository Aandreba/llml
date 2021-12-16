use std::ops::{Try, ControlFlow, FromResidual, Residual};

pub(crate) struct NeverShortCircuit<T>(pub T);

pub(crate) enum NeverShortCircuitResidual {}

impl<T> Try for NeverShortCircuit<T> {
    type Output = T;
    type Residual = NeverShortCircuitResidual;

    #[inline]
    fn branch(self) -> ControlFlow<NeverShortCircuitResidual, T> {
        ControlFlow::Continue(self.0)
    }

    #[inline]
    fn from_output(x: T) -> Self {
        NeverShortCircuit(x)
    }
}

impl<T> FromResidual for NeverShortCircuit<T> {
    #[inline]
    fn from_residual(never: NeverShortCircuitResidual) -> Self {
        match never {}
    }
}

impl<T> Residual<T> for NeverShortCircuitResidual {
    type TryType = NeverShortCircuit<T>;
}