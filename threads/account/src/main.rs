use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
struct Account {
    transactions: Vec<i32>,
    balance: i32,
}

impl Account {
    fn new() -> Self {
        Account {
            balance: 0,
            ..Default::default()
        }
    }

    fn withdrawal(&mut self, request: u32) -> u32 {
        if self.balance <= 0 {
            return 0;
        }

        let balance = self.balance as u32;
        let withdrawal = if balance <= request {
            self.balance
        } else {
            request as i32
        };

        self.transactions.push(-withdrawal);
        self.balance -= withdrawal;

        withdrawal as u32
    }

    fn deposit(&mut self, deposit: i32) {
        self.transactions.push(deposit);
        self.balance += deposit;
    }

    fn balance(&self) -> i32 {
        self.balance
    }
}

const GRAND_TOTAL: u32 = 400;

fn main() {
    let account = Arc::new(Mutex::new(Account::new()));

    let child_account = Arc::clone(&account);
    let parent_account = Arc::clone(&account);
    let banker_account = Arc::clone(&account);

    let (notification_sender, notification_receiver) = channel();

    let child_notification_sender = notification_sender.clone();

    let child_thread = thread::spawn(move || {
        let mut total_withdrawal = 0;

        loop {
            thread::sleep(Duration::from_millis(1500));
            let mut locked_account = child_account.lock().unwrap();
            let withdrawal = locked_account.withdrawal(50);
            total_withdrawal += withdrawal;
            if withdrawal > 0 {
                child_notification_sender
                    .send(format!("WITHDRAWAL <- {}", withdrawal))
                    .unwrap();
            }

            if total_withdrawal >= GRAND_TOTAL {
                break;
            }
        }
    });

    let parent_thread = thread::spawn(move || {
        let mut total_deposit = 0;

        loop {
            thread::sleep(Duration::from_secs(5));
            let mut locked_account = parent_account.lock().unwrap();

            locked_account.deposit(80);
            notification_sender
                .send(format!("DEPOSIT    -> {}", 80))
                .unwrap();
            total_deposit += 80;

            if total_deposit >= GRAND_TOTAL {
                break;
            }
        }
    });

    // Monitoring thread
    thread::spawn(move || loop {
        for notification in &notification_receiver {
            let locked_account = banker_account.lock().unwrap();
            println!(
                "Transaction: {} Balance {}",
                notification,
                locked_account.balance()
            );
        }
    });

    child_thread.join().unwrap();
    parent_thread.join().unwrap();

    println!("Account {:?}", account);
}
