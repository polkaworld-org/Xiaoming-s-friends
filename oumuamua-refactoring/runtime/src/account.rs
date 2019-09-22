use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Codec, Decode, Encode};
use runtime_primitives::traits::{
    As, CheckedAdd, Member, SimpleArithmetic, Hash,
};

use crate::asset;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct AccountSt<AccountId, TokenBalance> {
    id: AccountId,
    illegal_times: u32,       // 违约次数
    legal_times: u32,         // 守约次数
    illegal_total: TokenBalance, // 总违约金额
    legal_total: TokenBalance,   // 总守约金额
}

pub trait Trait: system::Trait + asset::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


decl_storage! {
	trait Store for Module<T: Trait> as SubstrateModuleTemplate {
		AccountDetail get(account_detail): map T::AccountId => Option<AccountSt>;

	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn new_account(origin) -> Result {
			let _sender = ensure_signed(origin)?;
            ensure!(!<AccountDetail<T>>::exists(_sender), "the Account already exists");

            let _accountst = AccountSt {
                id: _sender,
                illegal_times: 0u32,
                legal_time: 0u32,
                illegal_total: T::TokenBalance::from(0u64);
                legal_total: T::TokenBalance::from(0u64);
            }

            <AccountDetail<T>>::insert(_sender, _accountst);

			Self::deposit_event(RawEvent::NewAccount(_sender));
			Ok(())
		}

        pub fn slash_account(origin, _who: T::AccountId) -> Result {
            let _sender = ensure_signed(origin)?;

            ensure!(_sender == Self::admin(), "only Admin can slash accounts");
            ensure!(<AccountDetail<T>>::exists(_who), "the Account don't exists");

            <AccountDetail<T>>::remove(_who);

            Self::deposit_event(RawEvent::SlashAccount(_who));

            Ok(())
        }
	}
}

decl_event!(
	pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as self::Trait>::TokenBalance,
    {

		NewAccount(AccountId),
        SlashAccount(AccountId),
        IllegalHappen(AccountId, TokenBalance),
        LegalHappen(AccountId, TokenBalance),
	}
);

impl<T: Trait> Module<T> {
    pub fn add_illegal(accountid: T::AccountId, value: T::TokenBalance) -> Result {
        ensure!(<AccountDetail<T>>::exists(accountid), "the Account don't exists");

        let mut _accountst = Self::account_detail(accountid);
        _accountst.illegal_times = _accountst.illegal_times.checked_add(1u32)
            .ok_or("overflow in calculating illegal_times")?;

        _accountst.illegal_total = _accountst.illegal_total.checked_add(value)
            .ok_or("overflow in calculating illegal_total")?;

        <AccountDetail<T>>::insert(accountid, _accountst);

        Self::deposit_event(RawEvent::IllegalHappen(accountid, value));

        Ok(())
    }

    pub fn add_legal(accountid: T::AccountId, value: T::TokenBalance) -> Result {
        ensure!(<AccountDetail<T>>::exists(accountid), "the Account don't exists");

        let mut _accountst = Self::account_detail(accountid);
        _accountst.legal_times = _accountst.legal_times.checked_add(1u32)
            .ok_or("overflow in calculating legal_times")?;

        _accountst.legal_total = _accountst.legal_total.checked_add(value)
            .ok_or("overflow in calculating illegal_total")?;

        <AccountDetail<T>>::insert(accountid, _accountst);

        Self::deposit_event(RawEvent::IllegalHappen(accountid, value));

        Ok(())
    }
}