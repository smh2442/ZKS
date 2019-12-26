use bigdecimal::BigDecimal;
use models::node::tx::TxSignature;
use models::node::{
    priv_key_from_fs, AccountAddress, Nonce, PrivateKey, PublicKey, TokenId, Transfer,
};
use rand::{thread_rng, Rng};

pub struct ZksyncAccount {
    pub private_key: PrivateKey,
    pub address: AccountAddress,
}

impl ZksyncAccount {
    pub fn rand() -> Self {
        let mut rng = &mut thread_rng();

        let pk = priv_key_from_fs(rng.gen());
        Self::new(pk)
    }

    pub fn new(private_key: PrivateKey) -> Self {
        Self {
            address: AccountAddress::from_privkey(&private_key),
            private_key,
        }
    }
    pub fn sign_transfer(
        &self,
        token_id: TokenId,
        amount: BigDecimal,
        fee: BigDecimal,
        to: &AccountAddress,
        nonce: Nonce,
    ) -> Transfer {
        let mut transfer = Transfer {
            from: self.address.clone(),
            to: to.clone(),
            token: token_id,
            amount,
            fee,
            nonce,
            signature: TxSignature::default(),
        };
        transfer.signature =
            TxSignature::sign_musig_pedersen(&self.private_key, &transfer.get_bytes());
        transfer
    }
}
