use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::{
    error::MyProgramError,
    state::{
        try_from_account_info_mut,
        utils::{load_ix_data, DataLen},
        Initialized, MyState,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateMyStateIxData {
    pub data: [u8; 32],
}

impl DataLen for UpdateMyStateIxData {
    const LEN: usize = core::mem::size_of::<UpdateMyStateIxData>(); // 32 bytes for data
}

pub fn process_update_state(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer_acc, state_acc] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let my_state = unsafe { try_from_account_info_mut::<MyState>(state_acc)? };

    // CHECK if my_state is initialized
    if !my_state.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Validate PDA
    MyState::validate_pda(my_state.bump, state_acc.key(), payer_acc.key())?;

    if my_state.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let ix_data = unsafe { load_ix_data::<UpdateMyStateIxData>(data)? };

    my_state.update(ix_data)?;

    Ok(())
}
