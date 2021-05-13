const anchor = require('@project-serum/anchor');

describe('t1_basics', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    console.log('Workspace', anchor.workspace)
    const program = anchor.workspace.T1Basics
    const tx = await program.rpc.initialize()
    console.log("Your transaction signature", tx)

  });
});
