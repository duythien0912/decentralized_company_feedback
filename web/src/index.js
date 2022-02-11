// src/index.js
import React from "react";
import ReactDOM from "react-dom";
import App from "./App";
import getConfig from "./config.js";
import * as nearAPI from "near-api-js";

// Buffer is not defined:: https://github.com/near/near-api-js/issues/757#issuecomment-1002754955
import { Buffer } from "buffer";
global.Buffer = Buffer;

// Initializing contract
async function initContract() {
  const nearConfig = getConfig(process.env.NODE_ENV || "testnet");

  // Initializing connection to the NEAR TestNet
  const near = await nearAPI.connect({
    deps: {
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
    },
    ...nearConfig,
  });

  // Needed to access wallet
  const walletConnection = new nearAPI.WalletConnection(near);

  // Load in account data
  let currentUser;
  if (walletConnection.getAccountId()) {
    currentUser = {
      accountId: walletConnection.getAccountId(),
      balance: (await walletConnection.account().state()).amount,
    };
  }

  // Initializing our contract APIs by contract name and configuration
  const contract = await new nearAPI.Contract(
    walletConnection.account(),
    nearConfig.contractName,
    {
      // View methods are read-only – they don't modify the state, but usually return some value
      viewMethods: [
        "get_feedbacks",
        "get_feedback",
        "get_feedbacks_by_user_id_paging",
        "get_feedbacks_by_parent_id_paging",
        "get_feedbacks_by_company_id_paging",
        "get_companies_paging",
        "get_company",
      ],
      // Change methods can modify the state, but you don't receive the returned value when called
      changeMethods: [
        "create_feedback",
        "create_user",
        "create_company",
        "update_feedback",
        "update_user",
        "update_company",
        "update_active_feedback",
        "update_active_user",
        "update_active_company",
      ],
      // Sender is the account ID to initialize transactions.
      // getAccountId() will return empty string if user is still unauthorized
      sender: walletConnection.getAccountId(),
    }
  );

  return { contract, currentUser, nearConfig, walletConnection };
}

window.nearInitPromise = initContract().then(
  ({ contract, currentUser, nearConfig, walletConnection }) => {
    ReactDOM.render(
      <App
        contract={contract}
        currentUser={currentUser}
        nearConfig={nearConfig}
        wallet={walletConnection}
      />,
      document.getElementById("root")
    );
  }
);
