const {
    Connection,
    sendAndConfirmTransaction,
    Keypair,
    Transaction,
    SystemProgram,
    PublicKey,
    TransactionInstruction,
} = require("solana/web3.js");

const BN = require("bn.js");

const main = async() => {
    /*
    process.argv is an inbuilt application programming interface of the process module which
     is used to get the arguments passed to the node.js process when run in the command line.
    */
    var args = process.argv.slice(2);
    // args[0]: Program ID
    // args[1]: (Optional): Counter buffer account
    const programId = new PublicKey(args[0]);

    console.log(programId.toBase58());
    const connection = new Connection("https://api.devnet.solana.com");
    const feePayer = new Keypair();

    console.log("Requesting Airdrop of 2 SOL - DevNet");
    await connection.requestAirdrop(feePayer.publicKey, 2e9);
    console.log("Airdrop received");

    // Declaring counterKey ahead of assignment
    let counterKey;

    let tx = new Transaction();
    let signers = [feePayer];

    if (args.length > 1) {
        console.log("Found counter address");
        counterKey = new PublicKey(args[1]);
    } else {
        console.log("Generating new counter address");
        const counter = new Keypair();
        counterKey =  counter.publicKey;

        let createIx = SystemProgram.createAccount({
            fromPubkey: feePayer.publicKey,
            newAccountPubkey: counterKey,
            // Amount of lamports to transfer to the created account
            lamports: await connection.getMinimumBalanceForREntExemption(8),
            // Amount of space in bytes to allocate to the created account
            space: 8,
            // Public Key of the program to assign as the owner of the created account
            programId: programId,
        });

        // add counter keypair to signers array
        signers.push(counter)
        // Add instruction to transaction
        tx.add(createIx)
    }
    
    // Buffer objects are used to represent a fixed-length sequence of bytes
    // Corresponds to initial index of the counter, which is zero (represented by 8 bits of 0)
    const idx = Buffer.from(new Uint8Array([0])); // this corresponds to CounterInstruction::Increment which is 8 bit / 1 byte object

    let incrIx = new TransactionInstruction({
        keys: [
            {
                pubkey: counterKey,
                isSigner: false,
                isWritable: true,
            }
        ],
        programId: programId,
        data: idx
    })

    /*
    TransactionInstruction({
        keys: Array<AccountMeta>,
        programId: PublicKey,
        data: Buffer,
    });
    */

    // Adding counter increment instruction
    tx.add(incrIx);

    let txid = await sendAndConfirmTransaction(connection, tx, signers, {
        skipPreflight: true,
        preflightCommitment: "confirmed",
        confirmation: "confirmed",
    });
    console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

    data = (await connection.getAccountInfo(counterKey)).data;
    count = new BN(data, "le");

    console.log("Counter key:", counterKey.toBase58());
    console.log("Count:", count.toNumber());
};

main()
    .then(() => {
        console.log("Success");
    })
    .catch((e) => {
        console.error(e);
    });