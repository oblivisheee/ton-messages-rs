use tonlib_core::{
    TonAddress,
    cell::Cell,
    message::{
        CommonMsgInfo, JettonTransferMessage as TonLibJettonTransferMessage, TonMessage,
        TonMessageError, TransferMessage as TonLibTransferMessage,
    },
};

pub struct TransferMessage(TonLibTransferMessage);
impl TransferMessage {
    pub fn new(value: impl Into<f64>, dest: impl Into<TonAddress>) -> Self {
        let value = f64_to_biguint_ton(value.into());

        let dest = dest.into();
        let mut common = CommonMsgInfo::new_default_internal(&dest, &value);
        if let tonlib_core::message::CommonMsgInfo::InternalMessage(internal) = &mut common {
            internal.bounce = false;
        }
        let msg = TonLibTransferMessage::new(common);
        Self(msg)
    }
    pub fn build(&self) -> Result<Cell, TonMessageError> {
        let cell = self.0.build()?;
        Ok(cell)
    }
}

fn f64_to_biguint_ton(value: f64) -> num_bigint::BigUint {
    num_bigint::BigUint::from((value * 1_000_000_000.0) as u64)
}
// pub struct JettonTransferMessage(TonLibJettonTransferMessage);
// impl JettonTransferMessage {
//     pub fn new(
//         value: impl Into<f64>,
//         dest: impl Into<TonAddress>,
//         jetton: impl Into<TonAddress>,
//     ) -> Self {

//     }
// }
