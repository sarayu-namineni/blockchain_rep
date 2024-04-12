use crate::ivc::IVC::IVC;

struct DummyIVCImpl;

impl IVC for DummyIVCImpl {
	type SecurityParam = u64;
	type PublicParams = ();
	type Func = fn(u64)-> u64;
	type ProveKey = ();
	type VerifyKey = ();
	type Index = u64;
	type Output = u64;
	type Witness = ();
	type Proof = ();

	fn generate(lambda : Self::SecurityParam) -> Self::PublicParams {
		return ();
	}

	fn encode(pp : Self::PublicParams, func : Self::Func) -> (Self::ProveKey, Self::VerifyKey) {
		return ((), ());
	}

	fn prove(pk : Self::ProveKey, i : Self::Index, z_0 : Self::Output, z_i : Self::Output, omega : Self::Witness, pi_i : Self::Proof) -> Self::Proof {
		return ();
	}

	fn verify(vk : Self::VerifyKey, i : Self::Index, z_0 : Self::Output, z_i : Self::Output, pi_i : Self::Proof) -> bool {
		return true;
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!((), DummyIVCImpl::generate(0));
    }

	#[test]
    fn test_prove() {
        assert_eq!((), DummyIVCImpl::prove((), 0, 0, 0, (), ()));
    }

	#[test]
    fn test_verify() {
        assert_eq!(true, DummyIVCImpl::verify((), 0, 0, 0, ()));
    }
}