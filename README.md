To set up the capture and score actions program:-
Run anchor test, to test program, to be sure it's production ready.

Make sure you have a solana account keypair or an account for signing transactions

Then set you solana config to where you want the program deployed using
solana config set --url <config: {devnet, localnet, mainnet}>
solana config get:- gets the config so you can check the set value of the field

You can run to deploy program
solana program delpoy target/deploy/capture_actions.so


Then to interact with the program you will need to get the IDL from the target/idl/capture_actions.json and then interact with the program..
You can use the sample in the unit tests


A little explanation on how to set up the frontend
After program delpoyment and complete setup on the frontend you read the data in the frontend by passing in the account public address into the fetch method present on the account, then you can read the scores in the scoreAccount then when you read it you can show it to whomever is disbursing the tokens,
then for any user you disburse token to, you call the clear_user_score instruction present on the program to reset the user score to 0.



NOTE: You would need to setup the provider for the frontend so you can sign some transactions/instructions.

