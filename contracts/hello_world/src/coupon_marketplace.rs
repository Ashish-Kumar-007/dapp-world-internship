#![no_std]

use soroban_sdk::{ contract, contractimpl, contracttype, Address, Env, token };

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    CouponID,
}

#[derive(Clone)]
#[contracttype]
pub struct Coupon {
    owner: Address,
    token: Address,
    amount: u64,
    price: u64,
    id: u64,
}

#[contract]
pub struct CouponMarketplaceContract;

#[contractimpl]
impl CouponMarketplaceContract {
    pub fn create_coupon(env: Env, from: Address, token: Address, amount: u64, price: u64) {
        // Ensure that the transaction sender is authorized to create coupons
        from.require_auth();

        // Store the coupon in the contract storage
        let mut coupon_id = env.storage().instance().get(&DataKey::CouponID).unwrap_or(0);
        coupon_id = coupon_id + 1;

                // Create a new coupon with the provided details
        let coupon = Coupon {
            owner: from,
            token,
            amount,
            price,
            id: coupon_id,
        };

        
        env.storage().instance().set(&coupon_id, &coupon);
    }

    pub fn buy_coupon(env: Env, buyer: Address, coupon_id: u64) {
        // Retrieve the coupon from the contract storage using the correct key type
        let coupon: Coupon = env.storage().instance().get(&DataKey::CouponID).unwrap();
    
        // Ensure that the coupon exists
        if coupon.id == 0 {
            panic!("Coupon does not exist");
        }
    
        // Ensure that the buyer is not the owner of the coupon
        if buyer == coupon.owner {
            panic!("Cannot buy own coupon");
        }
    
        // Transfer tokens from the buyer to the owner of the coupon
        token::Client
            ::new(&env, &coupon.token)
            .transfer(&buyer, &coupon.owner, &(coupon.price as i128));
    
        // Transfer the coupon ownership to the buyer
        let new_coupon = Coupon {
            owner: buyer,
            token: coupon.token,
            amount: coupon.amount,
            price: coupon.price,
            id: coupon.id,
        };
        env.storage().instance().set(&DataKey::CouponID, &new_coupon);
    }
    
}