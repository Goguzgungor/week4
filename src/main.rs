
fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        balance: 0,
        holder_name: String::from("John"),
        account_number: 1234,
    };
    
    let mut account2 = BankAccount {
        balance: 0,
        holder_name: String::from("Jane"),
        account_number: 5678,
    };
    
    // Call deposit on account1
    let deposit_amount = 100;
    account1.deposit(deposit_amount);
    
    // Call withdraw on account2
    let withdraw_amount = 10000;
    account2.withdraw(withdraw_amount);
    
    // Call balance on both accounts and print the result
    println!("Account 1 balance: {}", account1.balance());
    println!("Account 2 balance: {}", account2.balance());
}


trait Account<T>{
    fn deposit(&mut self,new:T) -> Result<(), String>;
    fn withdraw(&mut self,new:T) -> Result<(), String>;
    fn balance(&mut self)   -> i32;
}

struct BankAccount{
    balance: i32,
    holder_name: String,
    account_number: i32,
}

impl Account<i32> for BankAccount {
    fn deposit(&mut self,deposit:i32) -> Result<(), String> {
        self.balance+=deposit;
        if (self.balance<0){
            Err(String::from("Balance is negative!"))
    }
        else{
            Ok(())
        }
    }

    fn withdraw(&mut self,amaount:i32) -> Result<(), String> {
        self.balance-=amaount;
        println!("{}",self.balance);
        if (self.balance<0){
            panic!("Balance is negative!");
        }
        else{
            Ok(())
        }
    }

    fn balance(&mut self) -> i32{
        self.balance
    }
}

