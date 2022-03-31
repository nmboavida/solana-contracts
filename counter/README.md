This program increments a counter everytime an address sends a transaction with an Increment instruction.

Program was deployed on devnet: Program Id = 7ws3s9eaXEzC5AUbuBrewJxZEMbhaKhfBYyyWCwKXCw7

##### Client

Client code does the following:

1. Creates account A and funds it with 2 SOL
2. If the more than 1 cli parameter is given then it takes the second parameter as the pubkey of the Program owned account B
3. If only 2 cli parameter then it creates itself a program owned account B and trnasfer SOL from account A in order to fund the rent (this last step is handled by the System Program)
4. Client adds a Increment instruction the transaction
5. Client sends transaction through the network and waits for confirmation

To run the client (make sure you are in the client directory):
`node index.js <Program id>` --> will generate a keypair for you

or 
`node index.js <Program id> <Pubkey>` --> if you want to use your pubkey


To consider regarding accounts on Solana:
A created account is initialized to be owned by a built-in program called the System program and is called a system account aptly. An account includes "owner" metadata. The owner is a program id. The runtime grants the program write access to the account if its id matches the owner. For the case of the System program, the runtime allows clients to transfer lamports and importantly assign account ownership, meaning changing the owner to a different program id. If an account is not owned by a program, the program is only permitted to read its data and credit the account.


