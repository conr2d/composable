{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    apps = let
      rust_log =
        "RUST_LOG=info,runtime=info,parachain=trace,cumulus-collator=trace,aura=debug,xcm=trace,pallet_ibc=debug,hyperspace=trace,hyperspace_parachain=trace,ics=trace,ics::routing=trace,ics::channel=trace,parachain::network-bridge-rx=debug,parachain::availability-store=info,parachain::approval-distribution=info,parachain::approval-voting=info,parachain::bitfield-distribution=debug,runtime::system=info,parachain::chain-api=debug,ics::routing=info,orml_xtokens=debug,wasmtime_cranelift=info";
    in {
      composable-follow-archive = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "composable-follow";
          runtimeInputs = [ self'.packages.composable-node ];

          text = ''
            ${rust_log} composable --chain=composable --listen-addr=/ip4/0.0.0.0/tcp/30334 --prometheus-port 9615 --base-path /tmp/composable-follownet/composable/archieve --execution=wasm --ws-external --state-pruning=archive --blocks-pruning=archive --rpc-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=unsafe --ws-port 9988 --rpc-port 39988 --in-peers 1000 --out-peers 1000 --ws-max-connections 10000  --sync=full -- --execution=wasm --listen-addr=/ip4/0.0.0.0/tcp/30333 --sync=full  --state-pruning=archive --blocks-pruning=archive
          '';
        };
      };

      kusama-follow = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "kusama-follow";
          runtimeInputs = [ pkgs.polkadot ];

          text = ''

            RUST_LOG="debug,runtime=info,parachain=trace,cumulus-collator=trace,aura=debug,xcm=trace,pallet_ibc=debug,hyperspace=trace,hyperspace_parachain=trace,ics=trace,ics::routing=trace,ics::channel=trace,parachain::network-bridge-rx=debug,parachain::availability-store=info,parachain::approval-distribution=info,parachain::approval-voting=info,parachain::bitfield-distribution=debug,runtime::system=info,parachain::chain-api=debug,ics::routing=info,orml_xtokens=debug,wasmtime_cranelift=info,netlink_proto=info,libp2p_tcp=info,libp2p_websocket=info,libp2p_swarm=info,trust_dns_resolver=info,libp2p_mdns=info,trie-cache=info,db=info,multistream_select=info,sub-libp2p=info,libp2p_ping=info,grandpa=info,txpool=info,wasm_overrides=info,sync=info,wasm-heap=info,sc_service=info,libp2p_kad=info,libp2p_core=info,libp2p_dns=info,babe=info" polkadot --chain=kusama --base-path /tmp/composable-follow/kusama/warp --execution=wasm --rpc-external --rpc-cors=all --rpc-methods=unsafe --rpc-port 9944 --sync=warp 2>&1 | tee /tmp/composable-follow/kusama/warp/log.txt
          '';
        };
      };
    };
  };
}
