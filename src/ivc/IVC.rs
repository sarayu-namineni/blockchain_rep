pub trait IVC {
	type SecurityParam;
	type PublicParams;
	type Func;
	type ProveKey;
	type VerifyKey;
	type Index;
	type Output;
	type Witness;
	type Proof;

	fn generate(lambda : Self::SecurityParam) -> Self::PublicParams;
	fn encode(pp : Self::PublicParams, func : Self::Func) -> (Self::ProveKey, Self::VerifyKey);
	fn prove(pk : Self::ProveKey, i : Self::Index, z_0 : Self::Output, z_i : Self::Output, omega : Self::Witness, pi_i : Self::Proof) -> Self::Proof;
	fn verify(vk : Self::VerifyKey, i : Self::Index, z_0 : Self::Output, z_i : Self::Output, pi_i : Self::Proof) -> bool;
}