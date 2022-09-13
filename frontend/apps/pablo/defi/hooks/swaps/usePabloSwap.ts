import { APP_NAME } from "@/defi/polkadot/constants";
import {
  DEFAULT_NETWORK_ID,
  isValidAssetPair,
  toChainUnits,
} from "@/defi/utils";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import {
  useSigner,
  useExecutor,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";

type PabloSwapProps = {
  baseAssetId: string;
  quoteAssetId: string;
  minimumReceived: BigNumber;
  quoteAmount: BigNumber;
};

export function usePabloSwap({
  quoteAssetId,
  baseAssetId,
  quoteAmount,
  minimumReceived,
}: PabloSwapProps) {
  const signer = useSigner();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();
  const executor = useExecutor();

  const pabloSwap = useCallback(async (): Promise<string> => {
    return new Promise(async (res, rej) => {
      if (
        parachainApi &&
       signer !== undefined && executor &&
        isValidAssetPair(baseAssetId, quoteAssetId) &&
        selectedAccount

        ) {
          try {

          let pair = {
            base: baseAssetId,
            quote: quoteAssetId,
          };

          await executor.execute(
            parachainApi.tx.dexRouter.exchange(
              pair,
              parachainApi.createType(
                "u128",
                toChainUnits(quoteAmount).toString()
              ),
              parachainApi.createType(
                "u128",
                toChainUnits(minimumReceived).toString()
              )
            ),
            selectedAccount.address,
            parachainApi,
            signer,
            (txHash: string) => {
              console.log("TX Started: ", txHash);
              enqueueSnackbar(`Tx Hash: ${txHash}`);
            },
            (txHash: string, _events) => {
              console.log("TX Finalized: ", txHash);
              enqueueSnackbar(`Tx Finalized: ${txHash}`);
              res(txHash);
            },
            (txError: string) => {
              console.error(txError);
              enqueueSnackbar(`Tx Errored: ${txError}`);
              rej(txError);
            }
          );
        } catch (err: any) {
          console.error(err);
          enqueueSnackbar(`Tx Error: ${err.message}`);
          return rej(err);
        }
      }
    });
  }, [
    baseAssetId,
    quoteAssetId,
    quoteAmount,signer,
    minimumReceived,
    enqueueSnackbar,
    selectedAccount,
    parachainApi,
    executor,
  ]);

  return pabloSwap;
}
