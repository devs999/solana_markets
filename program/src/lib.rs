use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Bet {
    pub bettor: Pubkey,
    pub amount: u64,
    pub prediction: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Market {
    pub id: u64,
    pub description: String,
    pub outcomes: Vec<String>,
    pub bets: Vec<Bet>,
    pub resolved: bool,
    pub winning_outcome: Option<String>,
    pub total_staked: u64,
    pub creator: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PredictionMarketContract {
    markets: Vec<Market>,
    market_count: u64,
}

impl Default for PredictionMarketContract {
    fn default() -> Self {
        Self {
            markets: Vec::new(),
            market_count: 0,
        }
    }
}

impl PredictionMarketContract {
    fn create_market(&mut self, description: String, outcomes: Vec<String>, admin: Pubkey) {
        let market = Market {
            id: self.market_count,
            description,
            outcomes,
            bets: Vec::new(),
            resolved: false,
            winning_outcome: None,
            creator: admin,
            total_staked: 0,
        };
        self.markets.push(market);
        self.market_count += 1;
    }

    fn place_bet(
        &mut self,
        market_id: u64,
        prediction: String,
        bettor: Pubkey,
        amount: u64,
    ) -> ProgramResult {
        let market = self
            .markets
            .get_mut(market_id as usize)
            .ok_or(ProgramError::InvalidArgument)?;
        if market.resolved {
            return Err(ProgramError::InvalidArgument);
        }

        let bet = Bet {
            bettor,
            amount,
            prediction,
        };

        market.bets.push(bet);
        market.total_staked += amount;

        Ok(())
    }

    fn settle_market(&mut self, market_id: u64, winning_outcome: String) -> ProgramResult {
        let market = self
            .markets
            .get_mut(market_id as usize)
            .ok_or(ProgramError::InvalidArgument)?;
        if market.resolved {
            return Err(ProgramError::InvalidArgument);
        }

        market.winning_outcome = Some(winning_outcome.clone());
        market.resolved = true;

        let total_staked_on_winner = market
            .bets
            .iter()
            .filter(|bet| bet.prediction == winning_outcome)
            .map(|bet| bet.amount)
            .sum::<u64>();

        for bet in market.bets.iter() {
            if bet.prediction == winning_outcome {
                let share = bet.amount as f64 / total_staked_on_winner as f64;
                let payout = (market.total_staked as f64 * share) as u64;
                // Handle the transfer logic in Solana (this is just a placeholder)
                msg!("Transfer {} SOL to {:?}", payout, bet.bettor);
            }
        }

        Ok(())
    }

    fn withdraw_funds(&mut self, market_id: u64, admin: Pubkey) -> ProgramResult {
        let market = self
            .markets
            .get_mut(market_id as usize)
            .ok_or(ProgramError::InvalidArgument)?;
        if !market.resolved {
            return Err(ProgramError::InvalidArgument);
        }

        // Calculate the remaining funds after payouts
        let funds_to_withdraw = market.total_staked;
        market.total_staked = 0;

        // Handle the transfer logic in Solana (this is just a placeholder)
        msg!("Transfer {} SOL to {:?}", funds_to_withdraw, admin);

        Ok(())
    }

    fn get_total_staked(&self, market_id: u64) -> u64 {
        self.markets
            .get(market_id as usize)
            .map_or(0, |market| market.total_staked)
    }
}

// Entry point
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
