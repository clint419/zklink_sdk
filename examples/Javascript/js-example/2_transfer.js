import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    const to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let amount = wasm.closestPackableTransactionAmount("1234567899808787");
        let fee = wasm.closestPackableTransactionFee("10000567777")
        let tx_builder = new wasm.TransferBuilder(10, to_address, 1,
            1, 18, fee, amount, 1,ts);
        let transfer = wasm.newTransfer(tx_builder);
        const provider = window.bitkeep && window.bitkeep.ethereum;
        await provider.request({ method: 'eth_requestAccounts' });
        const signer = new wasm.JsonRpcSigner(provider);
        await signer.initZklinkSigner(null);
        console.log(signer);

        let signature = await signer.signTransfer(transfer,"USDC")
        console.log(signature);

        let submitter_signature = signer.submitterSignature(signature.tx);
        console.log(submitter_signature);
        let rpc_client = new wasm.RpcClient("testnet");
        let l1_signature = new wasm.TxLayer1Signature(wasm.L1SignatureType.Eth,signature.eth_signature);
        let tx_hash = await rpc_client.sendTransaction(signature.tx,l1_signature,submitter_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

main();
