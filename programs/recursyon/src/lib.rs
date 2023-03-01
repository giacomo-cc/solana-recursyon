use anchor_lang::{
    prelude::*,
    solana_program::{self, hash::hashv, instruction::Instruction},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod recursyon {
    use super::*;

    pub fn execute(ctx: Context<Execute>, input_value: u8) -> Result<()> {
        require_keys_eq!(ctx.accounts.myself.key(), *ctx.program_id);

        msg!("received value: {}", input_value);
        if input_value > 0 {
            let mut data = hashv(&[b"global:execute"]).to_bytes()[..8].to_vec();
            data.push(input_value - 1);

            let ix = Instruction::new_with_bytes(*ctx.program_id, &data, vec![AccountMeta::new_readonly(*ctx.program_id, false)]);
            msg!("invoke recursion now");
            solana_program::program::invoke(&ix, &[ctx.accounts.myself.to_account_info()])?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {

    /// CHECK:
    #[account()]
    pub myself: AccountInfo<'info>,
}

