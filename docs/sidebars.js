/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

 const isProd = process.env.NODE_ENV === "production";

 // @ts-check
 
 /** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
 const sidebars = {
   // By default, Docusaurus generates a sidebar from the docs folder structure
   // tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],
 
   // But you can create a sidebar manually
   internalSidebar: [{ type: "autogenerated", dirName: "internal" }],
   user_guides: [
     {
       type: "category",
       label: "User Guides",
       link: {
         type: "generated-index",
         slug: "user-guides",
       },
       collapsible: false,
       items: [
         {
           type: "category",
           label: "Accounts and Wallets",
           link: {
             type: "generated-index",
             slug: "accounts-wallets",
           },
           collapsible: false,
           items: [
             "user-guides/polkadotjs-extension-create-account",
             "user-guides/talisman-create-account",
             "user-guides/layr-guide",
           ],
         },
         {
           type: "category",
           label: "Transactions and Trading",
           link: {
             type: "generated-index",
             slug: "transactions-and-trading",
           },
           collapsible: false,
           items: [
             "user-guides/claim-rewards-guide",
             "user-guides/how-to-provide-liquidity",
             "user-guides/how-to-trade-pica-on-pablo",
             "user-guides/centauri-staking",
             "user-guides/centauri-guide",
             "user-guides/centauri-transfers",
           ],
         },
 
         "user-guides/polkassembly-picasso-governance",
       ],
     },
   ],
   networks: [
     {
       type: "category",
       label: "Picasso",
       link: {
         type: "doc",
         id: "networks/picasso-parachain-overview",
       },
       collapsible: false,
       collapsed: false,
       items: [
         "networks/picasso/governance",
         "networks/picasso/asset-list",
         "networks/picasso/pica-use-cases",
         "networks/picasso/tokenomics",
         "networks/picasso/token-transparency",
         "networks/picasso/crowdloan",
         {
           type: "category",
           label: "CosmWasm",
           link: {
             type: "doc",
             id: "products/cosmwasm-vm-overview",
           },
           collapsible: true,
           collapsed: true,
           items: [
             "products/cosmwasm/existing-cosmwasm-projects-to-deploy-on-ccw-vm",
             "products/cosmwasm/syngery-with-centauri-and-xcvm",
             "products/cosmwasm/writing-smart-contracts-with-cosmwasm",
           ],
         },
         {
           type: "category",
           label: "Apollo",
           link: {
             type: "doc",
             id: "products/apollo-overview",
           },
           collapsible: true,
           collapsed: true,
           items: [
             "products/apollo/apollo-how-it-works",
             "products/apollo/technical-details",
             "products/apollo/apollo-deployment",
           ],
         },
         {
          type: "category",
          label: "Pablo",
          link: {
            type: "doc",
            id: "products/pablo-overview",
          },
          collapsible: true,
          collapsed: true,
          items: [
            "products/pablo/swaps-trading",
            "products/pablo/launch-pools",
            "products/pablo/auctions-bonding",
            "products/pablo/cross-chain-DEX",
          ],
        },
       ],
     },
 
     {
       type: "category",
       label: "Composable",
       link: {
         type: "doc",
         id: "networks/composable-parachain-overview",
       },
       collapsible: false,
       collapsed: false,
       items: [
         "networks/composable/composie-asset-list",
         "networks/composable/composable-crowdloan",
         "networks/composable/LAYR-tokenomics",
         "networks/composable/composable-council",
       ],
     },
 
     "networks/centauri-chain",
   ],
 
   centauri: [
     "products/centauri-overview",
     "products/centauri/Dotsama-ibc",
     "products/centauri/near-ibc-bridge",
     "products/centauri/hyperspace-relayer",
     "products/centauri/light-clients",
     "products/centauri/merkle-mountain-ranges",
     "products/centauri/cosmos11-BEEFY-COSMOS-IBC-light-client",
   ],
   technology: [
     {
       type: "category",
       label: "XCVM",
       link: {
         type: "doc",
         id: "products/xcvm",
       },
       collapsible: false,
       collapsed: false,
       items: [
         "products/xcvm/how-it-works",
         {
           type: "category",
           label: "Use Cases",
           link: {
             type: "generated-index",
             slug: "use-cases",
           },
           collapsible: false,
           collapsed: false,
           items: ["products/xcvm/use-cases/swap"],
         },
       ],
     },
   ],
   develop: [
    {
      type: "category",
      label: "Nix",
      link: {
        type: "doc",
        id: "nix"
      },
      collapsible: true,
      collapsed: true,
      items: [
        "nix/install",
        "nix/run-packages",
        "nix/development-environments",
        "nix/running-checks",
        "nix/reading-logs",
        "nix/defining-your-own-packages",
        "nix/editing-docs",
        "nix/troubleshooting",
      ],
    },
    {
      type: "doc",
      id: "codespaces",
    },
    {
      type: "category",
      label: "Cosmwasm Orchestrate",
      link: {
        type: "doc",
        id: "developer-guides/cosmwasm-orchestrate",
      },
      collapsible: true,
      collapsed: true,
      items: [
        {
          type: "category",
          label: "Concepts",
          link: {
            type: "doc",
            id: "developer-guides/cosmwasm/cw-orchestrate/concepts/concepts",
          },
          collapsible: true,
          collapsed: true,
          items: [
            "developer-guides/cosmwasm/cw-orchestrate/concepts/direct-dispatch",
            "developer-guides/cosmwasm/cw-orchestrate/concepts/address-handlers",
            "developer-guides/cosmwasm/cw-orchestrate/concepts/custom-handler",
          ],
        },
        "developer-guides/cosmwasm/cw-orchestrate/tutorial-dex",
      ],
    },
    {
      type: "category",
      label: "Cosmwasm CLI",
      link: {
        type: "doc",
        id: "developer-guides/cosmwasm-cli",
      },
      collapsible: true,
      collapsed: true,
      items: ["developer-guides/cosmwasm/walkthrough"],
    },
    "developer-guides/oracle-set-up-guide",
    "developer-guides/collator-guide",
    "developer-guides/local-picasso-guide",
  ],
   ecosystem: [
     {
       type: "category",
       label: "Ecosystem",
 
       link: {
         type: "generated-index",
         slug: "ecosystem",
       },
       collapsible: false,
       items: [
         {
           type: "category",
           label: "Build on Composable: Ecosystem Development",
           link: {
             type: `doc`,
             id: `ecosystem/build-on-composable-ecosystem-development`,
           },
           collapsible: false,
           items: [
             "ecosystem/rfp-canonical-stablecoin-design-and-integration",
             "ecosystem/rfp-explorer",
           ],
         },
         "ecosystem/composable-grants",
         "ecosystem/press-kit",
         "ecosystem/the-composable-team",
       ],
     },
 
     {
       type: "doc",
       label: "Audits, Fixes & Bug Bounties",
       id: "audits/audit-results-recommendations-and-remediations",
     },
     {
       type: "category",
       label: "Legal Disclaimers and Disclosures",
       collapsible: true,
       collapsed: true,
       items: [
         "faqs/disclaimers-disclosures-for-composable-tokens",
         "faqs/risk-factors",
         "faqs/terms-of-use",
       ],
     },
     {
      "type": "category",
      "label": "Archived",
      "collapsible": true,
      "collapsed": true,
      "items": [
        {
          "type": "doc",
          "label": "Mosaic (Discontinued)",
          "id": "products/mosaic/mosaic-withdrawal-guide"
        },
        {
          "type": "category",
          "label": "Parachain Vault Strategy (Discontinued)",
          "link": {
            "type": "doc",
            "id": "products/parachain-vault-strategy/composable-strategies-withdrawal-guide"
          },
          "collapsible": true,
          "collapsed": true,
          "items": [
            "products/parachain-vault-strategy/vault-process-in-detail",
            "products/parachain-vault-strategy/contracts-technical-details"
          ],
        },
      ],
    },    
   ],
 };
 
 // if (!isProd) {
 //     sidebars.tutorialSidebar.unshift({
 //         type: 'category',
 //         label: 'test-SCDI',
 //         link: {
 //             type: 'doc',
 //             id: 'testSCDI/entry',
 //         },
 //         collapsible: true,
 //         collapsed: true,
 //         items: [
 //             {
 //                 type: 'link',
 //                 label: 'test-SCDI',
 //                 href: '/test-vm',
 //             },
 //         ],
 //     });
 // }
 
 module.exports = sidebars;