const {
    Connection,
    PublicKey,
    Keypair,
    LAMPORTS_PER_SOL,
    Transaction,
    sendAndConfirmTransaction,
    SystemProgram,
  } = require("@solana/web3.js");
  
  // Connect to the Solana devnet
  const connection = new Connection("https://api.devnet.solana.com");
  
  // Create a new wallet keypair
  const wallet = Keypair.generate();
  
  // Get the public key (address) of the wallet
  const publicKey = wallet.publicKey;
  
  console.log("Public Key:", publicKey.toBase58());
  
  // Request an airdrop of 1 SOL
  (async () => {
    const airdropSignature = await connection.requestAirdrop(
      publicKey,
      LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignature);
    console.log("Airdrop completed!");
  
    // Check the balance of the wallet
    const balance = await connection.getBalance(publicKey);
    console.log(`Wallet balance: ${balance / LAMPORTS_PER_SOL} SOL`);
  
    // Create a new wallet to transfer tokens to
    const recipientWallet = Keypair.generate();
    const recipientPublicKey = recipientWallet.publicKey;
  
    // Transfer 0.5 SOL to the recipient wallet
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: publicKey,
        toPubkey: recipientPublicKey,
        lamports: 0.5 * LAMPORTS_PER_SOL,
      })
    );
  
    // Sign the transaction with the sender's private key
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      wallet,
    ]);
    console.log("Transfer completed. Transaction signature:", signature);
  
    // Check the balance of both wallets
    const senderBalance = await connection.getBalance(publicKey);
    const recipientBalance = await connection.getBalance(recipientPublicKey);
    console.log(`Sender balance: ${senderBalance / LAMPORTS_PER_SOL} SOL`);
    console.log(`Recipient balance: ${recipientBalance / LAMPORTS_PER_SOL} SOL`);
  })();