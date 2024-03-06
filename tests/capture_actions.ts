import assert from 'assert';
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CaptureActions } from "../target/types/capture_actions";



describe("capture_actions", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CaptureActions as Program<CaptureActions>;
  const programId = new anchor.web3.PublicKey(require("../target/idl/capture_actions.json").metadata.address);
  const network = 'http://localhost:8899';
  const connection = new anchor.web3.Connection(network, 'confirmed');
  const keypair = anchor.web3.Keypair.generate();
  const keypair2 = anchor.web3.Keypair.generate();
  const keypair3 = anchor.web3.Keypair.generate();




  it('creates a post', async() => {
    const tx = await program.rpc.createPost('12', {
      accounts: {
        postAccount: keypair.publicKey,
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [keypair, keypair2]
    })
    await connection.confirmTransaction(tx);
    const feedback = await connection.getTransaction(tx);


    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    assert.equal(scoreAccount.scores[0].score.toString(), "10");
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores});
  })


  it('downloads a post', async() => {
    const tx = await program.rpc.downloadPost('12', {
      accounts: {
        postAccount: keypair.publicKey,
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey
      },
      signers: []
    })
    await connection.confirmTransaction(tx);
    const feedback = await connection.getTransaction(tx);

    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores, score: scoreAccount.scores[0].score.toString()});
  })


  it('likes a post', async() => {
    const tx = await program.rpc.likePost('12', {
      accounts: {
        postAccount: keypair.publicKey,
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey
      },
      signers: []
    })
    await connection.confirmTransaction(tx);
    const feedback = await connection.getTransaction(tx);

    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores, score: scoreAccount.scores[0].score.toString()});
  })


  it('comments on a post', async() => {
    const tx = await program.rpc.commentPost('12', {
      accounts: {
        postAccount: keypair.publicKey,
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey
      },
      signers: []
    })
    await connection.confirmTransaction(tx);
    const feedback = await connection.getTransaction(tx);

    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores, score: scoreAccount.scores[0].score.toString()});
  })


  it('shares a post', async() => {
    const tx = await program.rpc.sharePost('12', {
      accounts: {
        postAccount: keypair.publicKey,
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey
      },
      signers: []
    })
    await connection.confirmTransaction(tx);
    const feedback = await connection.getTransaction(tx);

    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores, score: scoreAccount.scores[0].score.toString()});
  })


  it('views a post completely', async() => {
    const tx = await program.rpc.completePostView('12', {
      accounts: {
        postAccount: keypair.publicKey,
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey
      },
      signers: []
    })
    await connection.confirmTransaction(tx);
    const feedback = await connection.getTransaction(tx);

    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores, score: scoreAccount.scores[0].score.toString()});
  })

  it('views a post multiple times', async() => {
    const tx = await program.rpc.completePostView('12', {
      accounts: {
        postAccount: keypair.publicKey,
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey
      },
      signers: []
    })
    await connection.confirmTransaction(tx);
    const feedback = await connection.getTransaction(tx);

    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores, score: scoreAccount.scores[0].score.toString()});
  })



  it('clears a score', async() => {
    const tx = await program.rpc.resetUserScore({
      accounts: {
        scoreAccount: keypair2.publicKey,
        user: provider.wallet.publicKey
      },
      signers: []
    })


    await connection.confirmTransaction(tx);
    const feedback = connection.getTransaction(tx);

    const postAccount = await program.account.postAccount.fetch(keypair.publicKey);
    const scoreAccount = await program.account.scoreAccount.fetch(keypair2.publicKey);
    console.log({postAccount, feedback, tx, posts: postAccount.posts, scoreAccount, scores: scoreAccount.scores, score: scoreAccount.scores[0].score.toString()});
  })

});
