package main

import (
	"net/http"
	"math/big"
	"encoding/json"
	"fmt"
	"bytes"
	"io/ioutil"
	sdk "github.com/zkLinkProtocol/zklink_sdk/go_example/generated/uniffi/zklink_sdk"
)

type RPCTransaction struct {
     Id      int64             `json:"id"`
     JsonRpc string            `json:"jsonrpc"`
     Method  string            `json:"method"`
     Params  []json.RawMessage `json:"params"`
}

type SubmiterSignature struct {
    PubKey string `json:"pubKey"`
    Signature string `json:"signature"`
}

func HighLevelAutoDeleveraging() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    contract_price1 := sdk.ContractPrice{
        sdk.PairId(1),
        *big.NewInt(656566),
    }

    contract_price2 := sdk.ContractPrice{
        sdk.PairId(3),
        *big.NewInt(52552131),
    }

    var contract_prices = make([]sdk.ContractPrice,2)
    contract_prices[0] = contract_price1
    contract_prices[1] = contract_price2

    margin_price1 := sdk.SpotPriceInfo {
       sdk.TokenId(17),
       *big.NewInt(3236653653635635),
    }
    margin_price2 := sdk.SpotPriceInfo {
      sdk.TokenId(18),
      *big.NewInt(549574875297),
    }

    var margin_prices = make([]sdk.SpotPriceInfo,2)
    margin_prices[0] = margin_price1
    margin_prices[1] = margin_price2

    builder := sdk.AutoDeleveragingBuilder{
        sdk.AccountId(3),
        sdk.SubAccountId(1),
        sdk.Nonce(9),
        contract_prices,
        margin_prices,
        sdk.AccountId(7),
        sdk.PairId(3),
        *big.NewInt(1000),
        *big.NewInt(100000000000),
         *big.NewInt(10000),
        sdk.TokenId(18),
    }
    tx := sdk.NewAutoDeleveraging(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1TypeEth)
    if err != nil {
        return
    }
    txSignature, err := signer.SignAutoDeleveraging(tx)
    if err != nil {
        return
    }
    fmt.Println("tx signature: %s", txSignature)

    // get submitter signature
    zklinkTx := tx.ToZklinkTx()
    submitterSignature, err := signer.SubmitterSignature(zklinkTx)
    submitterSignature2, err := json.Marshal(SubmiterSignature {
        PubKey: submitterSignature.PubKey,
        Signature: submitterSignature.Signature,
    })
	rpc_req := RPCTransaction {
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
		    []byte(txSignature.Tx),
		    nil,
		    submitterSignature2,
		},
    }
	JsonTx, err := json.Marshal(rpc_req)
	fmt.Println("AutoDeleveraging rpc request:",  string(JsonTx))
	// get the testnet url or main net url
	zklinkUrl := sdk.ZklinkTestNetUrl()
	response, err := http.Post(zklinkUrl, "application/json", bytes.NewBuffer(JsonTx))
	if err != nil {
        fmt.Println(err)
        return
    }
    defer response.Body.Close()
    body, _ := ioutil.ReadAll(response.Body)
    fmt.Println(string(body))
}

func main() {
    HighLevelAutoDeleveraging()
}
