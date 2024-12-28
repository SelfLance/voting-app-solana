use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod voting {
    use super::*;
    pub fn initialize_poll(_ctx: Context<InitilizePoll> , _poll_id: u64) -> Results<()>{
        OK(())
    }


}
